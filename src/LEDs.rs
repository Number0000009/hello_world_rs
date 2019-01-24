pub struct LEDs;

impl LEDs {

    const FVP_SYSREG: u32 = 0x1c01_0000;                    // FVP System Register base MMIO
    const FVP_SYSREG_SYSLEDS: u32 = LEDs::FVP_SYSREG + 0x8; // LEDs MMIO

    pub fn new() -> LEDs {
        LEDs
    }
}
