pub struct CPU;

impl CPU {
    pub fn new() -> CPU {
        CPU
    }

    pub fn init(&self) {
        let mpidr_el1: u32;

        unsafe {
        asm!("mrs x0, mpidr_el1" : "={x0}"(mpidr_el1));
        }

        if mpidr_el1 as u8 & 0xf != 0 {
            unsafe {
            asm!("wfe");
            }
            loop {}
        }
    }

    pub fn stop_ok(&self) {
    }

    pub fn stop_fail(&self) {
    }
}
