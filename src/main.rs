#![feature(lang_items, start)]
#![feature(asm)]
#![feature(global_asm)]

#![no_std]
#![no_main]

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unreachable_code)]

use core::panic::PanicInfo;

//use core::alloc::GlobalAlloc;
//use core::alloc::Layout;
//use core::ptr::null_mut;

mod LEDs;
mod UART;
mod CPU;
mod MMU;

#[macro_use]
extern crate bitfield;

const NUM_ENTRIES_4KB: usize = 512;

#[repr(C, align(0x1000))]
struct PageTable {
    entries: [MMU::descriptor_table_lvl2; NUM_ENTRIES_4KB]
}

static mut PageTableLvl2: PageTable = PageTable {
    entries: [MMU::descriptor_table_lvl2(0x4242424242424242); NUM_ENTRIES_4KB]
};

/*
struct PageTableLvl3 {
    entries: [0; NUM_ENTRIES_4KB]
}
*/
/*
struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
    }
}

#[global_allocator]
static A: Allocator = Allocator;
*/

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

impl BaseAddr for [MMU::descriptor_table_lvl2; 512] {
    fn get_base_addr(&self) -> u64 {
        self as *const MMU::descriptor_table_lvl2 as u64
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
/*
    let mut tt_lvl2 = MMU::descriptor_table_lvl2(0x4242424242424242);
    tt_lvl2.set_type(MMU::DescriptorType::TABLE as u8);
    tt_lvl2.set_next_level_table_addr(0xffffffff);
*/
/*
    MMU::MMU.dump_descriptor(&tt);
*/

//    let tt_lvl2: MMU::descriptor_table_lvl2 = &A.alloc(Layout::new::<MMU::descriptor_table_lvl2>()) as MMU::descriptor_table_lvl2;
//    assert!(tt_lvl2.is_null());

//    MMU::MMU.dump_descriptor(&tt_lvl2);

    MMU::MMU.setup_tcr_el1();

    unsafe {
    let ttbr: u64 = PageTableLvl2.entries.get_base_addr();

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
