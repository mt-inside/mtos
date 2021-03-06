#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;
extern crate raw_cpuid;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use raw_cpuid::{CpuId,CacheType};

use mtos::*;

entry_point!(kernel_main);

#[cfg(not(test))]
pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    gdt::init();
    interrupts::init();
    let mut mapper = unsafe { memory::init(VirtAddr::new(boot_info.physical_memory_offset)) };
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

    serial_banner();
    console_banner();
    cpu_info();

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

fn serial_banner() {
    serial_println!("mtOS");
    serial_println!("Hello Host!");
}
fn console_banner() {
    println!("mtOS");
}

fn cpu_info() {
    let cpuid = CpuId::new();

    println!("{} {}",
        cpuid.get_vendor_info().as_ref().map_or_else(
            || "unknown",
            |vi| vi.as_string(),
        ),
        cpuid.get_extended_function_info().as_ref().map_or_else(
            || "unknown",
            |efi| efi.processor_brand_string().unwrap_or("unknown"),
        ),
    );

    cpu_cache_info();
    cpu_features();
}

fn cpu_features() {
    let cpuid = CpuId::new();

    let mut features = Vec::with_capacity(80);
    cpuid.get_feature_info().map(|finfo| {
        if finfo.has_sse3() {
            features.push("sse3")
        }
        if finfo.has_pclmulqdq() {
            features.push("pclmulqdq")
        }
        if finfo.has_ds_area() {
            features.push("ds_area")
        }
        if finfo.has_monitor_mwait() {
            features.push("monitor_mwait")
        }
        if finfo.has_cpl() {
            features.push("cpl")
        }
        if finfo.has_vmx() {
            features.push("vmx")
        }
        if finfo.has_smx() {
            features.push("smx")
        }
        if finfo.has_eist() {
            features.push("eist")
        }
        if finfo.has_tm2() {
            features.push("tm2")
        }
        if finfo.has_ssse3() {
            features.push("ssse3")
        }
        if finfo.has_cnxtid() {
            features.push("cnxtid")
        }
        if finfo.has_fma() {
            features.push("fma")
        }
        if finfo.has_cmpxchg16b() {
            features.push("cmpxchg16b")
        }
        if finfo.has_pdcm() {
            features.push("pdcm")
        }
        if finfo.has_pcid() {
            features.push("pcid")
        }
        if finfo.has_dca() {
            features.push("dca")
        }
        if finfo.has_sse41() {
            features.push("sse41")
        }
        if finfo.has_sse42() {
            features.push("sse42")
        }
        if finfo.has_x2apic() {
            features.push("x2apic")
        }
        if finfo.has_movbe() {
            features.push("movbe")
        }
        if finfo.has_popcnt() {
            features.push("popcnt")
        }
        if finfo.has_tsc_deadline() {
            features.push("tsc_deadline")
        }
        if finfo.has_aesni() {
            features.push("aesni")
        }
        if finfo.has_xsave() {
            features.push("xsave")
        }
        if finfo.has_oxsave() {
            features.push("oxsave")
        }
        if finfo.has_avx() {
            features.push("avx")
        }
        if finfo.has_f16c() {
            features.push("f16c")
        }
        if finfo.has_rdrand() {
            features.push("rdrand")
        }
        if finfo.has_fpu() {
            features.push("fpu")
        }
        if finfo.has_vme() {
            features.push("vme")
        }
        if finfo.has_de() {
            features.push("de")
        }
        if finfo.has_pse() {
            features.push("pse")
        }
        if finfo.has_tsc() {
            features.push("tsc")
        }
        if finfo.has_msr() {
            features.push("msr")
        }
        if finfo.has_pae() {
            features.push("pae")
        }
        if finfo.has_mce() {
            features.push("mce")
        }
        if finfo.has_cmpxchg8b() {
            features.push("cmpxchg8b")
        }
        if finfo.has_apic() {
            features.push("apic")
        }
        if finfo.has_sysenter_sysexit() {
            features.push("sysenter_sysexit")
        }
        if finfo.has_mtrr() {
            features.push("mtrr")
        }
        if finfo.has_pge() {
            features.push("pge")
        }
        if finfo.has_mca() {
            features.push("mca")
        }
        if finfo.has_cmov() {
            features.push("cmov")
        }
        if finfo.has_pat() {
            features.push("pat")
        }
        if finfo.has_pse36() {
            features.push("pse36")
        }
        if finfo.has_psn() {
            features.push("psn")
        }
        if finfo.has_clflush() {
            features.push("clflush")
        }
        if finfo.has_ds() {
            features.push("ds")
        }
        if finfo.has_acpi() {
            features.push("acpi")
        }
        if finfo.has_mmx() {
            features.push("mmx")
        }
        if finfo.has_fxsave_fxstor() {
            features.push("fxsave_fxstor")
        }
        if finfo.has_sse() {
            features.push("sse")
        }
        if finfo.has_sse2() {
            features.push("sse2")
        }
        if finfo.has_ss() {
            features.push("ss")
        }
        if finfo.has_htt() {
            features.push("htt")
        }
        if finfo.has_tm() {
            features.push("tm")
        }
        if finfo.has_pbe() {
            features.push("pbe")
        }
    });

    cpuid.get_extended_feature_info().map(|finfo| {
        if finfo.has_bmi1() {
            features.push("bmi1")
        }
        if finfo.has_hle() {
            features.push("hle")
        }
        if finfo.has_avx2() {
            features.push("avx2")
        }
        if finfo.has_fdp() {
            features.push("fdp")
        }
        if finfo.has_smep() {
            features.push("smep")
        }
        if finfo.has_bmi2() {
            features.push("bmi2")
        }
        if finfo.has_rep_movsb_stosb() {
            features.push("rep_movsb_stosb")
        }
        if finfo.has_invpcid() {
            features.push("invpcid")
        }
        if finfo.has_rtm() {
            features.push("rtm")
        }
        if finfo.has_rdtm() {
            features.push("rdtm")
        }
        if finfo.has_fpu_cs_ds_deprecated() {
            features.push("fpu_cs_ds_deprecated")
        }
        if finfo.has_mpx() {
            features.push("mpx")
        }
        if finfo.has_rdta() {
            features.push("rdta")
        }
        if finfo.has_rdseed() {
            features.push("rdseed")
        }
        if finfo.has_adx() {
            features.push("adx")
        }
        if finfo.has_smap() {
            features.push("smap")
        }
        if finfo.has_clflushopt() {
            features.push("clflushopt")
        }
        if finfo.has_processor_trace() {
            features.push("processor_trace")
        }
        if finfo.has_sha() {
            features.push("sha")
        }
        if finfo.has_sgx() {
            features.push("sgx")
        }
        if finfo.has_avx512f() {
            features.push("avx512f")
        }
        if finfo.has_avx512dq() {
            features.push("avx512dq")
        }
        if finfo.has_avx512_ifma() {
            features.push("avx512_ifma")
        }
        if finfo.has_avx512pf() {
            features.push("avx512pf")
        }
        if finfo.has_avx512er() {
            features.push("avx512er")
        }
        if finfo.has_avx512cd() {
            features.push("avx512cd")
        }
        if finfo.has_avx512bw() {
            features.push("avx512bw")
        }
        if finfo.has_avx512vl() {
            features.push("avx512vl")
        }
        if finfo.has_clwb() {
            features.push("clwb")
        }
        if finfo.has_prefetchwt1() {
            features.push("prefetchwt1")
        }
        if finfo.has_umip() {
            features.push("umip")
        }
        if finfo.has_pku() {
            features.push("pku")
        }
        if finfo.has_ospke() {
            features.push("ospke")
        }
        if finfo.has_rdpid() {
            features.push("rdpid")
        }
        if finfo.has_sgx_lc() {
            features.push("sgx_lc")
        }
    });

    println!("{}", features.join(" "));
}

fn cpu_cache_info() {
    let cpuid = CpuId::new();
    cpuid.get_cache_parameters().map_or_else(
        || println!("No cache parameter information available"),
        |cparams| {
            for cache in cparams {
                print!("L{}", cache.level());

                let typ = match cache.cache_type() {
                    CacheType::Data => "Instr",
                    CacheType::Instruction => "Data ",
                    CacheType::Unified => "Unify",
                    _ => "Unknown cache type",
                };
                print!(" {}: ", typ);

                let size = cache.associativity()
                    * cache.physical_line_partitions()
                    * cache.coherency_line_size()
                    * cache.sets();
                if size > 1024 * 1024 {
                    print!("{}MiB", size / (1024 * 1024));
                } else {
                    print!("{}KiB", size / 1024);
                };

                if cache.is_fully_associative() {
                    print!(", fully associative");
                } else {
                    print!(", {}-way associative", cache.associativity());
                };

                if cache.has_complex_indexing() {
                    print!(", hash-based-mapped");
                } else {
                    print!(", direct-mapped");
                };

                println!("");
            }
        },
    );
}
