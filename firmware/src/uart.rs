const UART0: usize = 0x4000_4000;

pub fn init() {
    unsafe {
        // BAUDDIV: minimum valid divider for QEMU
        core::ptr::write_volatile((UART0 + 0x10) as *mut u32, 16);

        // CTRL: enable TX (bit 0)
        core::ptr::write_volatile((UART0 + 0x08) as *mut u32, 1);
    }
}

pub fn putc(c: u8) {
    unsafe {
        // Wait until the transmit buffer is available.
        while core::ptr::read_volatile((UART0 + 0x04) as *const u32) & 1 != 0 {}

        core::ptr::write_volatile(UART0 as *mut u32, c as u32);
    }
}

pub fn print(s: &str) {
    for c in s.as_bytes() {
        putc(*c);
    }
}
