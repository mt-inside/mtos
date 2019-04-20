#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

use mtos::*;

entry_point!(kernel_main);

#[cfg(not(test))]
pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Welcome to mTOS.");
    serial_println!("Hello Host!");

    gdt::init();
    interrupts::init();

    //memory::dump_page_tables_2(boot_info.physical_memory_offset, 3);

    use x86_64::{structures::paging::MapperAllSizes, structures::paging::Page, VirtAddr};

    let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };
    let mut frame_allocator = memory::BootInfoFrameAllocator_new(&boot_info.memory_map);

    let page = Page::containing_address(VirtAddr::new(0x1000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    let addrs = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x20010a,
        // some stack page
        0x57ac_001f_fe48,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &a in &addrs {
        let v = VirtAddr::new(a);
        //let p_mt = unsafe { memory::translate_addr_mt(boot_info.physical_memory_offset, v) };
        //let p_ref = unsafe { memory::translate_addr_ref(v, boot_info.physical_memory_offset) };
        let p = mapper.translate_addr(v);
        println!("{:?} -> {:?}", v, p);
    }

    // let ptr = 0xdeadbeef as *mut u32;
    // unsafe {
    //     *ptr = 42;
    // }

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
