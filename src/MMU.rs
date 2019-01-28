pub struct MMU;

use super::UART;

// Rust enums cannot have duplicates
pub enum DescriptorType {
    ONE = 0b01,
    THREE = 0b11,
}

impl DescriptorType {
    pub const BLOCK : DescriptorType = DescriptorType::ONE;
    pub const TABLE : DescriptorType = DescriptorType::THREE;
    pub const PAGE : DescriptorType = DescriptorType::THREE;
}

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

bitfield! {
    #[derive(Copy, Clone)]
    pub struct descriptor_table_lvl2(u64);
    u8;
    pub desc_type, set_type: 1, 0;
    ignored, _: 11, 2;
//  res0, _: 11, 12;                          // Absent for 4KB
    pub u64, next_level_table_addr, set_next_level_table_addr: 47, 12;
    res0, _: 51, 48;
    ignored2, _: 58, 52;
    PXNTable, _: 59;
    XNTable, _: 60;
    APTable, _: 62, 61;
    NSTable, _: 63;
}

bitfield! {
    pub struct descriptor_page_4k_lvl3(u64);
    u8;
    pub desc_type, set_type: 1, 0;
    pub lower_attrs, set_lower_attrs: 11, 2;
    pub TA, set_TA: 15, 12;                         // bits[51:48] of the page address
    pub u32, output_addr, set_output_addr: 47, 16;
    res0, _: 50, 48;
    pub upper_attrs, set_upper_attrs: 63, 51;
}

impl MMU {

    pub fn setup_tcr_el1(&self)
    {
        let mut tcr_el1: u64;

        unsafe {
        asm!("mrs $0, tcr_el1" :"=r"(tcr_el1)::);
        }

        UART::UART.putx64(tcr_el1);
        UART::UART.puts("\n");

        // set t1sz[16 - 21] = 0x19
        // span 0xFFFFFF8000000000 - 0xFFFFFFFFFFFFFFFF (512 GB)
        tcr_el1 = tcr_el1 & !(0b000000 << 16);
        tcr_el1 = tcr_el1 | (0x19 << 16);

        // set tg1[30 - 31] = 0b10
        tcr_el1 = tcr_el1 & !(0x00 << 30);
        tcr_el1 = tcr_el1 | (0b10 << 30);

        UART::UART.putx64(tcr_el1);
        UART::UART.puts("\n");

        unsafe {
        asm!("msr tcr_el1, $0" ::"r"(tcr_el1):);
        }
    }

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

    pub fn dump_descriptor(&self, tt: &descriptor_table_lvl2)
    {
        UART::UART.puts("Type:");
        UART::UART.putx32(tt.desc_type() as u32);
        UART::UART.puts("\nNext level table address:");
        UART::UART.putx64(tt.next_level_table_addr());
        UART::UART.puts("\n");
    }

    pub fn set_type(&self, mut tt: descriptor_table_lvl2, _type: DescriptorType)
    {
        tt.set_type(_type as u8);
    }

    pub fn set_next_level_table_addr(&self, mut tt: descriptor_table_lvl2, addr: u64)
    {
        tt.set_next_level_table_addr(addr);
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