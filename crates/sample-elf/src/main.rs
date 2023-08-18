#![no_std]
#![no_main]

use core::fmt::{self, Write};

//

struct SyscallLog;

//

impl Write for SyscallLog {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if hyperion_syscall::log(s) == 0 {
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}

//

#[no_mangle]
pub extern "C" fn _start() -> ! {
    /* // page fault test:
    let null_ptr = core::hint::black_box(0x0) as *const u8;
    core::hint::black_box(unsafe { *null_ptr }); */

    for i in 0u64.. {
        writeln!(&mut SyscallLog, "testing `{i}`").unwrap();

        for j in 0x0u64..0x4_000_000u64 {
            core::hint::black_box(j);
        }
    }

    hyperion_syscall::exit(0);
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    hyperion_syscall::log("panic");
    hyperion_syscall::exit(-1);
}
