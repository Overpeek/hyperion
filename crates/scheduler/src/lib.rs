#![no_std]
#![feature(new_uninit)]

//

use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::{
    any::Any,
    cell::UnsafeCell,
    mem::swap,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use crossbeam_queue::SegQueue;
use hyperion_arch::{
    context::Context,
    cpu::ints,
    stack::{AddressSpace, KernelStack, Stack, UserStack},
    tls::{self, Tls},
    vmm::PageMap,
};
use hyperion_mem::vmm::{PageFaultResult, PageMapImpl, Privilege};
use hyperion_scheduler_task::{AnyTask, CleanupTask, Task};
use spin::{Lazy, Mutex};

//

extern crate alloc;

pub mod executor;
pub mod keyboard;
pub mod process;
pub mod task;
pub mod timer;

//

pub struct TaskImpl {
    address_space: Arc<AddressSpace>,
    kernel_stack: Stack<KernelStack>,
    user_stack: Stack<UserStack>,

    // context is used 'unsafely' only in the switch
    context: UnsafeCell<Context>,
    job: Option<Box<dyn FnOnce() + Send + 'static>>,
    pid: usize,
}

impl TaskImpl {
    pub fn new(f: impl FnOnce() + Send + 'static) -> Self {
        hyperion_log::trace!("new task");
        static NEXT_PID: AtomicUsize = AtomicUsize::new(0);
        let pid = NEXT_PID.fetch_add(1, Ordering::Relaxed);

        hyperion_log::trace!("new address space");
        let address_space = Arc::new(AddressSpace::new(PageMap::new()));

        hyperion_log::trace!("new stack");
        let mut kernel_stack = address_space.kernel_stacks.take();
        kernel_stack.grow(&address_space.page_map, 4).unwrap();
        let stack_top = kernel_stack.top;
        hyperion_log::trace!("stack top: 0x{:0x}", stack_top);

        let user_stack = address_space.user_stacks.take();

        hyperion_log::trace!("initializing task stack");
        let context = UnsafeCell::new(Context::new(
            &address_space.page_map,
            stack_top,
            thread_entry,
        ));
        let job = Some(Box::new(f) as _);

        Self {
            address_space,
            kernel_stack,
            user_stack,

            context,
            job,
            pid,
        }
    }

    pub fn debug(&mut self) {
        hyperion_log::debug!(
            "TASK DEBUG: context: {:0x?}, job: {:?}, pid: {}",
            unsafe { &*self.context.get() },
            self.job.as_ref().map(|_| ()),
            self.pid
        )
    }
}

impl AnyTask for TaskImpl {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn take_job(&mut self) -> Option<Box<dyn FnOnce() + Send + 'static>> {
        self.job.take()
    }

    fn pid(&self) -> usize {
        self.pid
    }
}

pub static READY: SegQueue<Task> = SegQueue::new();

/// reset this processors scheduling
pub fn reset() -> ! {
    hyperion_arch::int::disable();

    ints::PAGE_FAULT_HANDLER.store(page_fault_handler);
    hyperion_driver_acpi::apic::APIC_TIMER_HANDLER.store(yield_now);

    let boot: Task = Box::new(TaskImpl::new(|| {}));
    swap_current(Some(boot));
    stop();
}

/// switch to another thread
#[track_caller]
pub fn yield_now() {
    // hyperion_log::debug!("yield_now");
    let Some(next) = next_task() else {
        // no other tasks, don't switch
        return;
    };
    let Some(mut current) = swap_current(None) else {
        unreachable!("cannot yield from a task that doesn't exist")
    };

    let Some(task): Option<&mut TaskImpl> = current.as_any().downcast_mut() else {
        unreachable!("the task was from another scheduler")
    };
    let context = task.context.get();

    // push the current thread back to the ready queue AFTER switching
    after().push(CleanupTask::Ready(current));

    // SAFETY: `current` is stored in the queue until the switch
    // and the boxed field `context` makes sure the context pointer doesn't move
    unsafe {
        // hyperion_log::debug!("switch");
        block(context, next);
    }
}

/// destroy the current thread
/// and switch to another thread
pub fn stop() -> ! {
    // hyperion_log::debug!("stop");

    // TODO: running out stack space after taking the task doesnt allow the stack to grow
    let Some(next) = next_task() else {
        todo!("no tasks, shutdown");
    };
    let Some(mut current) = swap_current(None) else {
        unreachable!("cannot stop a task that doesn't exist")
    };

    let Some(task): Option<&mut TaskImpl> = current.as_any().downcast_mut() else {
        unreachable!("the task was from another scheduler")
    };
    let context = task.context.get();

    // push the current thread to the drop queue AFTER switching
    after().push(CleanupTask::Drop(current));

    // SAFETY: `current` is stored in the queue until the switch
    // and the boxed field `context` makes sure the context pointer doesn't move
    unsafe {
        block(context, next);
    }

    unreachable!("a destroyed thread cannot continue executing");
}

pub fn spawn(f: impl FnOnce() + Send + 'static) {
    schedule(Box::new(TaskImpl::new(f)))
}

/// schedule
fn schedule(new: Task) {
    READY.push(new);
}

fn swap_current(mut new: Option<Task>) -> Option<Task> {
    swap(&mut new, &mut active());
    new
}

/// # Safety
///
/// `current` must be correct and point to a valid exclusive [`Context`]
unsafe fn block(current: *mut Context, mut next: Task) {
    let Some(task): Option<&mut TaskImpl> = next.as_any().downcast_mut() else {
        unreachable!("the task was from another scheduler")
    };
    let context = task.context.get();

    after().push(CleanupTask::Next(next));

    // SAFETY: `next` is stored in the queue until the switch
    // and the boxed field `context` makes sure the context pointer doesn't move
    unsafe {
        hyperion_arch::context::switch(current, context);
    }

    cleanup();
}

fn next_task() -> Option<Task> {
    READY.pop()
}

fn cleanup() {
    let after = after();

    while let Some(next) = after.pop() {
        match next {
            CleanupTask::Ready(ready) => {
                schedule(ready);
            }
            CleanupTask::Next(next) => {
                swap_current(Some(next));
            }
            CleanupTask::Drop(mut drop) => {
                let Some(task): Option<&mut TaskImpl> = drop.as_any().downcast_mut() else {
                    unreachable!("the task was from another scheduler")
                };

                if Arc::strong_count(&task.address_space) != 1 {
                    continue;
                }

                // TODO: deallocate user pages
                // task.address_space;
            }
        };
    }
}

struct SchedulerTls {
    active: Mutex<Option<Task>>,
    after: SegQueue<CleanupTask>,
}

static TLS: Lazy<Tls<SchedulerTls>> = Lazy::new(|| {
    Tls::new(|| SchedulerTls {
        active: Mutex::new(None),
        after: SegQueue::new(),
    })
});

fn active() -> impl DerefMut<Target = Option<Task>> {
    TLS.active.lock()
    /* static ACTIVE: Lazy<ApicTls<Mutex<Option<Task>>>> =
        Lazy::new(|| ApicTls::new(|| Mutex::new(None)));
    ACTIVE.lock() */
    // tls::get().active.lock()
}

fn after() -> impl Deref<Target = SegQueue<CleanupTask>> {
    &TLS.after
    /* static AFTER: Lazy<ApicTls<SegQueue<CleanupTask>>> = Lazy::new(|| ApicTls::new(SegQueue::new));
    Lazy::force(&AFTER).deref() */
    // &tls::get().after_switch
}

fn page_fault_handler(addr: usize, user: Privilege) -> PageFaultResult {
    // hyperion_log::debug!("scheduler page fault");

    let Some(mut current) = swap_current(None) else {
        hyperion_log::debug!("no job");
        return PageFaultResult::NotHandled;
    };

    let Some(task): Option<&mut TaskImpl> = current.as_any().downcast_mut() else {
        hyperion_log::debug!("no task");
        swap_current(Some(current));
        return PageFaultResult::NotHandled;
    };

    let res = if user == Privilege::User {
        user_page_fault_handler(addr, task)
    } else {
        kernel_page_fault_handler(addr, task)
    };

    swap_current(Some(current));

    res
}

fn user_page_fault_handler(addr: usize, task: &mut TaskImpl) -> PageFaultResult {
    let result = task
        .user_stack
        .page_fault(&task.address_space.page_map, addr as u64);

    if result == PageFaultResult::Handled {
        return result;
    }

    hyperion_log::debug!("killing user-space process");
    stop();
}

fn kernel_page_fault_handler(addr: usize, task: &mut TaskImpl) -> PageFaultResult {
    let result = task
        .kernel_stack
        .page_fault(&task.address_space.page_map, addr as u64);

    if result == PageFaultResult::Handled {
        return result;
    }

    hyperion_log::error!("page fault from kernel-space");
    result
}

extern "sysv64" fn thread_entry() -> ! {
    cleanup();
    {
        let Some(mut current) = swap_current(None) else {
            unreachable!("cannot run a task that doesn't exist")
        };
        let Some(job) = current.take_job() else {
            unreachable!("cannot run a task that already ran")
        };
        swap_current(Some(current));
        job();
    }
    stop();
}
