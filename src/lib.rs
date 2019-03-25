#![feature(abi_x86_interrupt)]
#![cfg_attr(not(test), no_std)]

// re-export these
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga;

pub fn sleep_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

// Return type should really be bottom, but idk how to tell rustc that port.write() won't return.
pub unsafe fn exit_qemu() -> () {
    let mut port = x86_64::instructions::port::Port::<u32>::new(0xf4);
    port.write(0);
}
