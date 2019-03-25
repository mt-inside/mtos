#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::panic::PanicInfo;

use mtos::*;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to mTOS.");
    serial_println!("Hello Host!");

    gdt::init();
    interrupts::init();

    let ptr = 0xdeadbeef as *mut u32;
    memory::dump_page_tables();
    unsafe {
        *ptr = 42;
    }

    // unsafe {
    //     exit_qemu();
    // }

    mtos::sleep_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    mtos::sleep_loop();
}
