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
mod regs_defs;

#[macro_use]
mod macro_inst;

#[macro_use]
extern crate bitfield;

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

    inst_a!(regs_defs::REG_A, 0);
    inst_b!(0x666);
    inst_c!(0x666);

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

    MMU::EL01.setup_tcr();

    unsafe {
    MMU::PageTableLvl3.entries[0].set_type(MMU::DescriptorType::PAGE as u8);
    MMU::PageTableLvl3.entries[0].set_lower_attrs(0x71);
    MMU::PageTableLvl3.entries[0].set_output_addr(0x80000000);

    MMU::PageTableLvl2.entries[0].set_type(MMU::DescriptorType::TABLE as u8);
    MMU::PageTableLvl2.entries[0].set_next_level_table_addr(MMU::PageTableLvl3.entries.get_base_addr());

    MMU::PageTableLvl1.entries[0].set_type(MMU::DescriptorType::TABLE as u8);
    MMU::PageTableLvl1.entries[0].set_next_level_table_addr(MMU::PageTableLvl2.entries.get_base_addr());

//T1SZ=25 -> lookup starts at Level 1
//    MMU::PageTableLvl0.entries[0].set_type(MMU::DescriptorType::TABLE as u8);
//    MMU::PageTableLvl0.entries[0].set_next_level_table_addr(MMU::PageTableLvl1.entries.get_base_addr());

//T1SZ=25 -> lookup starts at Level 1
//    PageTableLvl0.entries[0].dump_descriptor();
    MMU::PageTableLvl1.entries[0].dump_descriptor();
    MMU::PageTableLvl2.entries[0].dump_descriptor();
    MMU::PageTableLvl3.entries[0].dump_descriptor();

    MMU::PageTableLvl3.entries[1].set_type(MMU::DescriptorType::PAGE as u8);
    MMU::PageTableLvl3.entries[1].set_lower_attrs(0x71);
    MMU::PageTableLvl3.entries[1].set_output_addr(0x81000000);

    MMU::PageTableLvl3.entries[1].dump_descriptor();

//T1SZ=25 -> lookup starts at Level 1
//    let ttbr: u64 = MMU::PageTableLvl0.entries.get_base_addr();
    let ttbr: u64 = MMU::PageTableLvl1.entries.get_base_addr();

    UART::UART.puts("TTBR1_EL1: ");
    UART::UART.putx64(ttbr);
    UART::UART.puts("\n");

    MMU::EL01.setup_ttbr1(ttbr);
    asm!("b .");
    }

    MMU::EL01.invalidate_tlb();
    MMU::EL01.enable();

    CPU::CPU.stop_ok();
}

#[lang = "eh_personality"] extern fn eh_personality() {}

global_asm!(include_str!("setup_stack.S"));
