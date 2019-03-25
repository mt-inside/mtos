use crate::{print, println};

pub fn dump_page_tables() -> () {
    use x86_64::structures::paging::PageTable;

    let (l4_table, _) = x86_64::registers::control::Cr3::read();
    /* gives a phy addr, which we can't directly access */
    println!("L4 page table at: {:?}", l4_table.start_address());
    /* however, the bootloader maps the last page of the kernel's virtual address space to the
     * frame of the L4 page table */
    let l4_virt_ptr = 0xffff_ffff_ffff_f000 as *const PageTable;
    let l4_tbl = unsafe { &*l4_virt_ptr };
    for i in 0..10 {
        println!("Entry {}: {:?}", i, l4_tbl[i]);
    }
}
