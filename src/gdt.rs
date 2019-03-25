use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

struct SegmentSelectors {
    code_segment_selector: SegmentSelector,
    tss_segment_selector: SegmentSelector,
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, SegmentSelectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_segment_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_segment_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            SegmentSelectors {
                code_segment_selector,
                tss_segment_selector,
            },
        )
    };
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096;
             // no proper allocation now, so use this wherever it ends up
             // must be mut otherwise the compiler will put it in an RO page
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            // NB: no guard page
            stack_end
        };
        tss
    };
}

pub fn init() {
    GDT.0.load();
    unsafe {
        x86_64::instructions::segmentation::set_cs(GDT.1.code_segment_selector);
        x86_64::instructions::tables::load_tss(GDT.1.tss_segment_selector);
    }
}
