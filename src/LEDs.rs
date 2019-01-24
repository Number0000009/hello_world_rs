pub struct LEDs;

impl LEDs {

    const FVP_SYSREG: u32 = 0x1c01_0000;                    // FVP System Register base MMIO
    const FVP_SYSREG_SYSLEDS: u32 = LEDs::FVP_SYSREG + 0x8; // LEDs MMIO

    pub fn new() -> LEDs {
        LEDs
    }

    fn light_mask(&self, mask: u8) {
        unsafe {
        let leds_ptr: *mut u8 = LEDs::FVP_SYSREG_SYSLEDS as *mut u8;
        *leds_ptr = mask;
        }
    }

    pub fn light_ok(&self) {
        self.light_mask(0xaa as u8);
    }

    pub fn light_failure(&self) {
        self.light_mask(0x55 as u8);
    }
}