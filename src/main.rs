#![feature(lang_items, start)]
#![feature(asm)]
#![feature(global_asm)]

#![no_std]
#![no_main]

#![allow(non_snake_case)]
#![allow(unreachable_code)]

use core::panic::PanicInfo;

mod UART;
mod LEDs;
mod CPU;
mod MMU;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    LEDs::LEDs::new().light_failure();

    UART::UART::new().puts("\n***PANIC!***");

    if let Some(location) = _info.location() {
        UART::UART::new().puts(" at the ");
        UART::UART::new().puts(location.file());
        UART::UART::new().puts("\nline# ");
        UART::UART::new().putu32(location.line());
    }

    UART::UART::new().puts("\n");

    CPU::CPU::new().stop_fail();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    let CPU = CPU::CPU::new();
    CPU.init();

    let UART = UART::UART::new();

    UART.puts("Currently at EL");

    let el = CPU.get_current_EL();
    UART.putu32(el as u32);
    UART.puts("\n");

    assert_eq!(el, 3);

    CPU.goto_EL(CPU::EL::EL2t);

    UART.puts("And now at EL");
    UART.putu32(CPU.get_current_EL() as u32);
    UART.puts("\n");

    let LEDs = LEDs::LEDs::new();
    LEDs.light_ok();

    CPU::CPU::new().stop_ok();
}

#[lang = "eh_personality"] extern fn eh_personality() {}

global_asm!(include_str!("setup_stack.S"));