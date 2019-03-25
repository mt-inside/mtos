#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::panic::PanicInfo;
use mtos::*;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    mtos::interrupts::init();

    x86_64::instructions::int3();

    serial_println!("ok");

    unsafe {
        exit_qemu();
    }

    loop {} // don't know how to mark exit_qemu as -> !, so still need this
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("failed");
    println!("{}", info);

    unsafe {
        exit_qemu();
    }
    loop {}
}
