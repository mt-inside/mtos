#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

// re-export these
pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error for: {:?}", layout)
}

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
