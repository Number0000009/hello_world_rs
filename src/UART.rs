pub struct UART;

impl UART {

    const PL011_UART0: u32 = 0x1c090000;                // UART0 base MMIO
    const PL011_UARTFR: u32 = UART::PL011_UART0 + 0x18; // UART0 Flag register
    const UART_TXFE: u8 = (1<<7);                       // TXFE - Transmit FIFO empty

    pub fn new() -> UART {
        UART
    }

    fn putc(&self, c: char) {

        let txfe_ptr: *const u8 = UART::PL011_UARTFR as *const u8;
        let uart0_ptr: *mut u8 = UART::PL011_UART0 as *mut u8;

        unsafe {
        let txfe_val:u8 = *txfe_ptr;

        while txfe_val & UART::UART_TXFE == 0 {}

        *uart0_ptr = c as u8;
        }
    }

    pub fn puts(&self, s: &str) {

        for c in s.chars() {
            match c {
                '\n' => {
                            self.putc('\n');
                            self.putc('\r');
                },
                _ => self.putc(c),
            }
        }
    }

    pub fn putx(&self, n: u32) {

        self.puts("0x");

        let mut c: u8;

        for i in 0..8 {
            let d: u8 = n.wrapping_shr(32 - 4 - i * 4) as u8 & 0xf;

            if d > 9 {
                c = d + 'A' as u8 - 0xa;
            } else {
                c = d + '0' as u8;
            }
            self.putc(c as char);
        }
    }
}