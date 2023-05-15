use super::{ReadOnly, ReadWrite, Reg, WriteOnly, LOCAL_APIC};
use crate::{arch::cpu::idt::Irq, debug, driver::acpi::hpet::HPET, trace};
use spin::{Lazy, Mutex, MutexGuard};

//

pub fn apic_regs() -> MutexGuard<'static, &'static mut ApicRegs> {
    pub static APIC_REGS: Lazy<Mutex<&'static mut ApicRegs>> =
        Lazy::new(|| Mutex::new(unsafe { &mut *(*LOCAL_APIC as *mut ApicRegs) }));

    APIC_REGS.lock()
}

//

const IA32_APIC_BASE: u32 = 0x1B;
const IA32_APIC_XAPIC_ENABLE: u64 = 1 << 11;
const _IA32_APIC_X2APIC_ENABLE: u64 = 1 << 10;

const APIC_SW_ENABLE: u32 = 1 << 8;
const APIC_DISABLE: u32 = 1 << 16;

const APIC_NMI: u32 = 4 << 8;

const _APIC_TIMER_MODE_ONESHOT: u32 = 0b00 << 17;
const APIC_TIMER_MODE_PERIODIC: u32 = 0b01 << 17;
const _APIC_TIMER_MODE_TSC_DEADLINE: u32 = 0b10 << 17;

const _APIC_TIMER_DIV_BY_1: u32 = 0b1011;
const _APIC_TIMER_DIV_BY_2: u32 = 0b0000;
const _APIC_TIMER_DIV_BY_4: u32 = 0b0001;
const _APIC_TIMER_DIV_BY_8: u32 = 0b0010;
const APIC_TIMER_DIV_BY_16: u32 = 0b0011;
const _APIC_TIMER_DIV_BY_32: u32 = 0b1000;
const _APIC_TIMER_DIV_BY_64: u32 = 0b1001;
const _APIC_TIMER_DIV_BY_128: u32 = 0b1010;
const APIC_TIMER_DIV: u32 = APIC_TIMER_DIV_BY_16;

pub fn enable() {
    debug!("Writing ENABLE APIC");
    write_msr(
        IA32_APIC_BASE,
        read_msr(IA32_APIC_BASE) | IA32_APIC_XAPIC_ENABLE,
    );

    let apic_regs = unsafe { &mut *(*LOCAL_APIC as *mut ApicRegs) };
    trace!("Apic regs: {apic_regs:#?}");

    // reset to well-known state
    apic_regs.destination_format.write(0xFFFF_FFFF);
    apic_regs.lvt_timer.write(APIC_DISABLE);
    apic_regs.lvt_perf_mon_counters.write(APIC_NMI);
    apic_regs.lvt_lint_0.write(APIC_DISABLE);
    apic_regs.lvt_lint_1.write(APIC_DISABLE);
    apic_regs.task_priority.write(0);

    debug!("Writing ENABLE SIVR");
    apic_regs
        .spurious_interrupt_vector
        .write(apic_regs.spurious_interrupt_vector.read() | APIC_SW_ENABLE);

    /*     let apic_period = 1000000; */
    let apic_period = u32::MAX;

    apic_regs.timer_divide.write(APIC_TIMER_DIV);
    apic_regs
        .lvt_timer
        .write(Irq::ApicTimer as u32 | APIC_TIMER_MODE_PERIODIC);
    apic_regs.timer_init.write(apic_period);

    apic_regs.lvt_thermal_sensor.write(0);
    apic_regs.lvt_error.write(0);

    // buggy HW fix:
    apic_regs.timer_divide.write(APIC_TIMER_DIV);

    let _hpet = &*HPET;

    // loop { /* debug!("APIC TIMER {}", apic_regs.timer_current.read()); */ }
}

/* fn read_apic_reg(reg: usize) -> u32 {
    unsafe { ptr::read_volatile((*LOCAL_APIC + reg) as _) }
}

fn write_apic_reg(reg: usize, val: u32) {
    unsafe { ptr::write_volatile((*LOCAL_APIC + reg) as _, val) }
} */

fn read_msr(msr: u32) -> u64 {
    unsafe { x86_64::registers::model_specific::Msr::new(msr).read() }
}

fn write_msr(msr: u32, val: u64) {
    unsafe { x86_64::registers::model_specific::Msr::new(msr).write(val) }
}

//

/// Table 10-1 Local APIC Register Address Map
///
/// https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-3a-part-1-manual.pdf
///
/// 10-6 Vol. 3A
#[derive(Debug)]
#[repr(C)]
pub struct ApicRegs {
    pub _res0: Reg<3, (), [u32; 2]>,
    pub lapic_id: Reg<3, ReadWrite>,
    pub lapic_ver: Reg<3, ReadOnly>,
    pub _res1: Reg<3, (), [u32; 4]>,
    pub task_priority: Reg<3, ReadWrite>,
    pub arbitration_priority: Reg<3, ReadOnly>,
    pub processor_priority: Reg<3, ReadOnly>,
    pub eoi: Reg<3, WriteOnly>,
    pub remote_read: Reg<3, ReadOnly>,
    pub logical_destination: Reg<3, ReadWrite>,
    pub destination_format: Reg<3, ReadWrite>,
    pub spurious_interrupt_vector: Reg<3, ReadWrite>,
    pub _pad2: Reg<3, (), [u32; 34]>,
    pub lvt_timer: Reg<3, ReadWrite>,
    pub lvt_thermal_sensor: Reg<3, ReadWrite>,
    pub lvt_perf_mon_counters: Reg<3, ReadWrite>,
    pub lvt_lint_0: Reg<3, ReadWrite>,
    pub lvt_lint_1: Reg<3, ReadWrite>,
    pub lvt_error: Reg<3, ReadWrite>,
    pub timer_init: Reg<3, ReadWrite>,
    pub timer_current: Reg<3, ReadOnly>,
    pub _res2: Reg,
    pub timer_divide: Reg<3, ReadWrite>,
}
