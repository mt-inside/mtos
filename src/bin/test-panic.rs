#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::panic::PanicInfo;
use mtos::*;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("ok");

    unsafe {
        exit_qemu();
    }
    loop {}
}
