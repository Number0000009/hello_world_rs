#![feature(lang_items, start)]
#![feature(asm)]
#![feature(global_asm)]

#![no_std]
#![no_main]

#![allow(non_snake_case)]
#![allow(unreachable_code)]

use core::panic::PanicInfo;

mod LEDs;
mod UART;
mod CPU;
mod MMU;

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

#[no_mangle]
pub extern "C" fn _start() -> ! {

    CPU::CPU.init();

    UART::UART.puts("Currently at EL");

    let mut el = CPU::CPU.get_current_EL();
    UART::UART.putu32(el as u32);
    UART::UART.puts("\n");

    assert_eq!(el, 3);

    CPU::CPU.goto_EL(CPU::EL::EL2t);

    UART::UART.puts("And now at EL");

    el = CPU::CPU.get_current_EL();
    UART::UART.putu32(el as u32);
    assert_eq!(el, 2);

    UART::UART.puts("\n");

    CPU::CPU.stop_ok();
}

#[lang = "eh_personality"] extern fn eh_personality() {}

global_asm!(include_str!("setup_stack.S"));
