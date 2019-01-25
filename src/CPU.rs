pub struct CPU;

use super::LEDs;

pub enum EL {
    EL0t = 0b0000,
    EL1t = 0b0100,
    EL1h = 0b0101,
    EL2t = 0b1000,
    EL2h = 0b1001,
}

impl CPU {

    pub fn new() -> CPU {
        CPU
    }

    pub fn init(&self) {
        let mpidr_el1: u32;

        unsafe {
        asm!("mrs $0, mpidr_el1" : "=r"(mpidr_el1):::);
        }

        if mpidr_el1 as u8 & 0xf != 0 {
            unsafe {
            asm!("wfe");
            }
            loop {}
        }

        unsafe {
        // EL2 and below ELs are AArch64 and Non-secure, HVC is defined
        asm!("mrs x0, scr_el3\n\t
              orr x0, x0, #(1 << 10)\n\t    // SCR_EL3.RW (EL2 and below are AArch64)
              orr x0, x0, #(1 << 0)\n\t     // SCR_EL3.NS (EL2 and below are Non-secure)
              orr x0, x0, #(1 << 8)\n\t     // SCR_EL3.HCE (HVC is not UNDEFINED)
              msr scr_el3, x0"
            :::"x0":)
        }
    }

    pub fn get_current_EL(&self) -> u8 {
        let current_el: u8;

        unsafe {
        asm!("mrs $0, CurrentEL" : "=r"(current_el):::);
        }

        current_el.wrapping_shr(2)
    }

    pub fn goto_EL(&self, el: EL) {
        unsafe {
        asm!("msr spsr_el3, $0\n\t
              adr $0, exit\n\t
              msr elr_el3, $0\n\t
              eret\n\t
              exit:" ::"r"(el)::);
        }
    }

    fn stop(&self) -> ! {
        unsafe { asm!("dsb nsh"); }
        loop {
        unsafe { asm!("wfe"); }
        }
    }

    pub fn stop_ok(&self) -> ! {
        LEDs::LEDs.light_ok();
        self.stop();
    }

    pub fn stop_fail(&self) -> ! {
        LEDs::LEDs.light_failure();
        self.stop();
    }
}