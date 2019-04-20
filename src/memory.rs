use crate::{print, println};

use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::structures::paging::{
    FrameAllocator, MappedPageTable, Mapper, MapperAllSizes, Page, PageTable, PhysFrame, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};

const X86_64_PAGE_TABLE_DEPTH: usize = 4;
type PageTableOffsets = [ux::u9; X86_64_PAGE_TABLE_DEPTH];

pub unsafe fn init(phys_mem_offset: u64) -> impl MapperAllSizes {
    let l4_table = active_l4_table(phys_mem_offset);
    let c = move |f: PhysFrame| -> *mut PageTable { _frame_to_page_table(phys_mem_offset, f) };

    MappedPageTable::new(l4_table, c)
}

pub fn active_l4_table(phys_mem_offset: u64) -> &'static mut PageTable {
    let (l4_table_phys, _) = x86_64::registers::control::Cr3::read();
    unsafe { _frame_to_page_table(phys_mem_offset, l4_table_phys) }
}

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

unsafe fn _frame_to_page_table(phys_mem_offset: u64, frame: PhysFrame) -> &'static mut PageTable {
    let virt = phys_mem_offset + frame.start_address().as_u64();
    let ptr = VirtAddr::new(virt).as_mut_ptr();
    &mut *ptr
}

pub fn dump_page_tables_2(phys_mem_offset: u64, stop: usize) -> () {
    if stop < 1 || stop > X86_64_PAGE_TABLE_DEPTH {
        return;
    }

    let (l4_table_phys, _) = x86_64::registers::control::Cr3::read();

    _dump_table(
        phys_mem_offset,
        l4_table_phys,
        stop,
        X86_64_PAGE_TABLE_DEPTH,
    );
}

fn _dump_table(phys_mem_offset: u64, phys: PhysFrame, stop: usize, level: usize) -> () {
    let table = unsafe { _frame_to_page_table(phys_mem_offset, phys) };

    for (i, entry) in table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L{} Entry {}: {:?}", level, i, entry);

            if level > stop {
                _dump_table(phys_mem_offset, entry.frame().unwrap(), stop, level - 1);
            }
        }
    }
}

pub unsafe fn translate_addr_mt(physical_memory_offset: u64, addr: VirtAddr) -> Option<PhysAddr> {
    _translate_addr(physical_memory_offset, addr)
}

fn _translate_addr(phys_mem_offset: u64, addr: VirtAddr) -> Option<PhysAddr> {
    let (l4_table_phys, _) = x86_64::registers::control::Cr3::read();
    let table_indices: PageTableOffsets = [
        addr.p1_index(),
        addr.p2_index(),
        addr.p3_index(),
        addr.p4_index(),
    ];

    let frame_base = _traverse_table(
        phys_mem_offset,
        l4_table_phys,
        table_indices,
        X86_64_PAGE_TABLE_DEPTH,
    );
    frame_base.map(|b| b + u64::from(addr.page_offset()))
}
fn _traverse_table(
    phys_mem_offset: u64,
    table_phys: PhysFrame,
    table_indices: PageTableOffsets,
    level: usize,
) -> Option<PhysAddr> {
    use x86_64::structures::paging::page_table::FrameError;

    let table = unsafe { _frame_to_page_table(phys_mem_offset, table_phys) };

    let entry = &table[table_indices[level - 1]];
    let frame = match entry.frame() {
        Ok(f) => f,
        Err(FrameError::FrameNotPresent) => return None,
        Err(FrameError::HugeFrame) => panic!("Huge pages not supported"),
    };

    if level == 1 {
        // base case
        return Some(frame.start_address());
    } else {
        // recursive case
        return _traverse_table(phys_mem_offset, frame, table_indices, level - 1);
    }
}

pub unsafe fn translate_addr_ref(addr: VirtAddr, physical_memory_offset: u64) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}
fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: u64) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::page_table::FrameError;

    // read the active level 4 frame from the CR3 register
    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut frame = level_4_table_frame;

    // traverse the multi-level page table
    for &index in &table_indexes {
        // convert the frame into a page table reference
        let virt = frame.start_address().as_u64() + physical_memory_offset;
        let table_ptr: *const PageTable = VirtAddr::new(virt).as_ptr();
        let table = unsafe { &*table_ptr };

        // read the page table entry and update `frame`
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }

    // calculate the physical address by adding the page offset
    Some(frame.start_address() + u64::from(addr.page_offset()))
}

pub fn create_example_mapping(
    page: Page,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let fs = Flags::PRESENT | Flags::WRITABLE;

    let res = unsafe { mapper.map_to(page, frame, fs, frame_allocator) };
    res.expect("Failed to create new mapping").flush();
}

pub struct EmptyFrameAllocator;
impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

pub struct BootInfoFrameAllocator<I>
where
    I: Iterator<Item = PhysFrame>,
{
    frames: I,
}

impl<I> BootInfoFrameAllocator<I>
where
    I: Iterator<Item = PhysFrame>,
{
    pub fn new(
        memory_map: &'static MemoryMap,
    ) -> BootInfoFrameAllocator<impl Iterator<Item = PhysFrame>> {
        let frames = memory_map
            .iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable)
            .map(|r| r.range.start_addr()..r.range.end_addr())
            .flat_map(|r| r.step_by(4096))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)));

        BootInfoFrameAllocator { frames }
    }
}

pub fn BootInfoFrameAllocator_new(
    memory_map: &'static MemoryMap,
) -> BootInfoFrameAllocator<impl Iterator<Item = PhysFrame>> {
    let frames = memory_map
        .iter()
        .filter(|r| r.region_type == MemoryRegionType::Usable)
        .map(|r| r.range.start_addr()..r.range.end_addr())
        .flat_map(|r| r.step_by(4096))
        .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)));

    BootInfoFrameAllocator { frames }
}

impl<I> FrameAllocator<Size4KiB> for BootInfoFrameAllocator<I>
where
    I: Iterator<Item = PhysFrame>,
{
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.frames.next()
    }
}
