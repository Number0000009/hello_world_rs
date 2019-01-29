#![feature(lang_items, start)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(core_intrinsics)]

#![no_std]
#![no_main]

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unreachable_code)]

use core::panic::PanicInfo;

mod LEDs;
mod UART;
mod CPU;
mod MMU;

#[macro_use]
extern crate bitfield;

const NUM_ENTRIES_4KB: usize = 512;

// TODO: probably u64 is better as it's uniform for all descriptor formats
#[repr(C, align(0x1000))]
struct PageTable012 {
    entries: [MMU::descriptor_table_lvl012; NUM_ENTRIES_4KB]
}

#[repr(C, align(0x1000))]
struct PageTable3 {
    entries: [MMU::descriptor_page_4k_lvl3; NUM_ENTRIES_4KB]
}

//T1SZ=25 -> lookup starts at Level 1
//static mut PageTableLvl0: PageTable012 = PageTable012 {
//    entries: [MMU::descriptor_table_lvl012(0); NUM_ENTRIES_4KB]
//};

static mut PageTableLvl1: PageTable012 = PageTable012 {
    entries: [MMU::descriptor_table_lvl012(0); NUM_ENTRIES_4KB]
};

static mut PageTableLvl2: PageTable012 = PageTable012 {
    entries: [MMU::descriptor_table_lvl012(0); NUM_ENTRIES_4KB]
};

static mut PageTableLvl3: PageTable3 = PageTable3 {
    entries: [MMU::descriptor_page_4k_lvl3(0); NUM_ENTRIES_4KB]
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    UART::UART.puts("\n***PANIC!***");

    if let Some(location) = _info.location() {
        UART::UART.puts(" at the ");
        UART::UART.puts(location.file());
        UART::UART.puts("\nline# ");
        UART::UART.putu32(location.line());
    }

    UART::UART.puts("\n");

    CPU::CPU.stop_fail();
}

trait BaseAddr {
    fn get_base_addr(&self) -> u64;
}

impl BaseAddr for [MMU::descriptor_table_lvl012; 512] {
    fn get_base_addr(&self) -> u64 {
        self as *const MMU::descriptor_table_lvl012 as u64
    }
}

impl BaseAddr for [MMU::descriptor_page_4k_lvl3; 512] {
    fn get_base_addr(&self) -> u64 {
        self as *const MMU::descriptor_page_4k_lvl3 as u64
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    CPU::CPU.init();

    UART::UART.puts("Currently at EL");

    let mut el: u32 = CPU::CPU.get_current_EL() as u32;
    UART::UART.putu32(el);
    UART::UART.puts("\n");

    assert_eq!(el, 3);

    CPU::CPU.goto_EL(CPU::EL::EL1t);

    UART::UART.puts("And now at EL");
    el = CPU::CPU.get_current_EL() as u32;
    UART::UART.putu32(el);
    UART::UART.puts("\n");

    assert_eq!(el, 1);

    MMU::MMU.setup_tcr_el1();

    unsafe {
    PageTableLvl3.entries[0].set_type(MMU::DescriptorType::PAGE as u8);
    PageTableLvl3.entries[0].set_lower_attrs(0x71);
    PageTableLvl3.entries[0].set_output_addr(0x80000000);

    PageTableLvl2.entries[0].set_type(MMU::DescriptorType::TABLE as u8);
    PageTableLvl2.entries[0].set_next_level_table_addr(PageTableLvl3.entries.get_base_addr());

    PageTableLvl1.entries[0].set_type(MMU::DescriptorType::TABLE as u8);
    PageTableLvl1.entries[0].set_next_level_table_addr(PageTableLvl2.entries.get_base_addr());

//T1SZ=25 -> lookup starts at Level 1
//    PageTableLvl0.entries[0].set_type(MMU::DescriptorType::TABLE as u8);
//    PageTableLvl0.entries[0].set_next_level_table_addr(PageTableLvl1.entries.get_base_addr());

//T1SZ=25 -> lookup starts at Level 1
//    PageTableLvl0.entries[0].dump_descriptor();
    PageTableLvl1.entries[0].dump_descriptor();
    PageTableLvl2.entries[0].dump_descriptor();
    PageTableLvl3.entries[0].dump_descriptor();

    PageTableLvl3.entries[1].set_type(MMU::DescriptorType::PAGE as u8);
    PageTableLvl3.entries[1].set_lower_attrs(0x71);
    PageTableLvl3.entries[1].set_output_addr(0x81000000);

    PageTableLvl3.entries[1].dump_descriptor();

//T1SZ=25 -> lookup starts at Level 1
//    let ttbr: u64 = PageTableLvl0.entries.get_base_addr();
    let ttbr: u64 = PageTableLvl1.entries.get_base_addr();

    UART::UART.puts("TTBR1_EL1: ");
    UART::UART.putx64(ttbr);
    UART::UART.puts("\n");

    MMU::MMU.setup_ttbr1_el1(ttbr);
    asm!("b .");
    }

    MMU::MMU.invalidate_tlb();
    MMU::MMU.enable();

    CPU::CPU.stop_ok();
}

#[lang = "eh_personality"] extern fn eh_personality() {}

global_asm!(include_str!("setup_stack.S"));
