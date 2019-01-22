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
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    let CPU = CPU::CPU::new();
    CPU.init();

    let UART = UART::UART::new();
    UART.puts("hello\nworld!\n");

    UART.putx(0x11223344);
    UART.puts("\n");
    UART.putx(0x1122AABB);
    UART.puts("\n");

    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

global_asm!(include_str!("setup_stack.S"));