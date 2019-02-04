macro_rules! inst_a {
    (
            $sys_reg:expr, $src_reg:expr
    ) => {
        unsafe {
        asm!(".inst ${0:c}"::"i"((0xd51 << 20) | (($sys_reg << 5) | ($src_reg & 0x1f)))::);
        }
    }
}

macro_rules! inst_b {
    (
            $imm16:expr
    ) => {
        unsafe {
        asm!(".inst ${0:c}"::"i"(0x12200009 | (($imm16 & 0xffff) << 5))::);
        }
    }
}

macro_rules! inst_c {
    (
            $imm16:expr
    ) => {
        unsafe {
        asm!(".inst ${0:c}"::"i"(0x1220000a | (($imm16 & 0xffff) << 5))::);
        }
    }
}
