// Implemented w.r.t the following:
// Cortex-M System Design Kit, Revision: r1p0 Copyright © 2011, 2013 ARM. All rights reserved.

use core::ptr::{read_volatile, write_volatile};

const UART0: usize = 0x4000_4000;

// CMSDK APB UART register addresses
// Ideally should be u32 addr instead, and some nice wraps maybe.
// Would be best to use bit_flags! here, but no crates.

/// UART_DATA, Base offset 0x000.
const UART_DATA: *mut u32 = UART0 as *mut u32;
/// UART_STATE, Base offset 0x004.
/// [0] -> TX buffer full.
/// [1] -> RX buffer full.
const UART_STATE: *const u32 = (UART0 + 0x04) as *const u32;
/// UART_CTRL, Base offset 0x008.
/// [0] -> TX Enable.
/// [1] -> RX Enable.
const UART_CTRL: *mut u32 = (UART0 + 0x08) as *mut u32;
/// UART_BAUDDIV, Base offset 0x010.
const UART_BAUDDIV: *mut u32 = (UART0 + 0x10) as *mut u32;

// MPS2-AN385 25MhZ system/peripheral clock
const PCLK_HZ: u32 = 25_000_000;
const BAUD_RATE: u32 = 115_200;
const BAUD_DIV: u32 = (PCLK_HZ + BAUD_RATE / 2) / BAUD_RATE;

/// Initializes the UART communications.
pub fn init() {
    unsafe {
        write_volatile(UART_CTRL, 0); // Writes 0, disable everything.
        write_volatile(UART_BAUDDIV, BAUD_DIV); // Write the BAUD_DIV
        write_volatile(UART_CTRL, 1); // Writes 1, enable only TX.
    }
}

/// Write one byte to the UART.
pub fn putc(c: u8) {
    unsafe {
        // Wait until the transmit buffer is available.
        while read_volatile(UART_STATE) & 1 != 0 {
            core::hint::spin_loop();
        }

        write_volatile(UART_DATA, c as u32);
    }
}

struct UartWriter;

impl core::fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            putc(byte);
        }

        Ok(())
    }
}

pub fn print_fmt(args: core::fmt::Arguments<'_>) {
    core::fmt::write(&mut UartWriter, args).ok();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::uart::print_fmt(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::uart::print_fmt(format_args!("\r\n"))
    };

    ($($arg:tt)*) => {
        $crate::uart::print_fmt(
            format_args!("{}\r\n", format_args!($($arg)*))
        )
    };
}
