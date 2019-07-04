#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

use mtos::*;

entry_point!(kernel_main);

#[cfg(not(test))]
pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    gdt::init();
    interrupts::init();
    let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };
    let mut frame_allocator = memory::BootInfoFrameAllocator_new(&boot_info.memory_map);
    allocator::init(&mut mapper, &mut frame_allocator).expect("Heap initialisation failed");

    use x86_64::structures::paging::{Page, PhysFrame};
    use x86_64::{PhysAddr, VirtAddr};
    // Map VGA buffer to 0x1000
    memory::create_mapping(
        Page::containing_address(VirtAddr::new(0x1000)),
        PhysFrame::containing_address(PhysAddr::new(0xb8000)),
        &mut mapper,
        &mut frame_allocator,
    );

    println!("Welcome to mTOS.");
    serial_println!("Hello Host!");

    let x = Box::new(42);
    println!("value on the heap: {} at {:p}", x, x);

    let mut v = Vec::new();
    for i in 0..69 {
        v.push(i);
    }
    println!("vec at {:p}; item 42: {}", v.as_slice(), v[42]);

    let rc = Rc::new(vec![0,1,2]);
    let clone = rc.clone();
    println!("current ref count is {}", Rc::strong_count(&clone));
    core::mem::drop(rc);
    println!("current ref count is {}", Rc::strong_count(&clone));

    //unsafe { exit_qemu() };
    mtos::sleep_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    mtos::sleep_loop();
}
