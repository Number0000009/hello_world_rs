pub struct MMU;

/* TODO:
 * ````
 * TCR_EL1.Tx2SZ = 0x10 - Initial lookup table level = 0
 * TCR_EL1.Tx2SZ = 0x19 - Initial lookup table level = 1
 * (D4-11)
 * TCR_EL1.TxSZ = 16 -> 0 - 0_FFFF_FFFF_FFFF
 * (Address Space size = 2^(64 - TxSZ))
 * 0b01 = block (i.e. 0x711 = R/W @ EL1, block / 0x751 = R/W @ any EL, block)
 * 0b11 = table next level
 * descriptor size 64bits (i.e. 80000711 00000000 = block R/W @ EL1 only)
 *
 * example:
 * TxSZ = 0x19 (0b11 - table next level / 0x01 - 2MB block)
 * Initial lookup level = 1
 * Descriptor lvl 2 (D4-15):
 * 2e000711 00000000 [level 2 2MB Block R/W @ EL1 ]

 * Initial lookup level = 1
 * Descriptor lvl 2 (D4-15):
 * 2e000003 00000000 [level 2 table (D4-15)] -> xxxxx713 00000000 [4KB page (D4-16)]
 */

impl MMU {

    pub fn setup_ttbr0_el1(&self, ttbr: u64)
    {
        unsafe {
        asm!("msr ttbr0_el1, $0\n\t
              isb" ::"r"(ttbr)::);
        }
    }

    pub fn setup_ttbr1_el1(&self, ttbr: u64)
    {
        unsafe {
        asm!("msr ttbr1_el1, $0\n\t
              isb" ::"r"(ttbr)::);
        }
    }

    pub fn invalidate_tlb(&self)
    {
        unsafe {
        asm!("tlbi vmalle1is\n\t
              isb\n\t
              dsb sy" ::::);
        }
    }

    pub fn enable(&self) {
        unsafe {
        asm!("mrs x0, sctlr_el1\n\t
              orr x0, x0, #1\n\t    // SCTLR_EL1.M
              isb"
            :::"x0" :);
        }
    }

    // u32 because we'll be using TCR_EL1.IPS = 0b000 (32 bit)
    pub fn translate_el1_s1r(&self, input_addr: u64) -> u32 {
        let output_addr: u64;

        unsafe {
        asm!("at s1e1r, $1\n\t
              mrs $0, par_el1"
            :"=r"(output_addr):"r"(input_addr)::);
        }

        assert_eq!(output_addr & 1, 1);                // PAR_EL1.F

        (output_addr & 0x0fff_ffff_ffff_ffff).wrapping_shr(12) as u32
    }
}