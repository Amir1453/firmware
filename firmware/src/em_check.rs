use core::ptr::{read_volatile, write_volatile};

use crate::println;

/// AHB GPIO a general-purpose I/O interface unit.
/// Documented in Application Note AN385.
/// Start address: 0x40010000
/// End address:   0x40010FFF
/// Description: CMSDK AHB GPIO #0 
const GPIO0: usize = 0x40010000;

/// Peripheral ID Register 0
const PID0: *const u32 = (GPIO0 + 0xFE0) as *const u32;
/// Peripheral ID Register 1
const PID1: *const u32 = (GPIO0 + 0xFE4) as *const u32;
/// Peripheral ID Register 2
const PID2: *const u32 = (GPIO0 + 0xFE8) as *const u32;

/// Peripheral ID Register 0 Reset Value
const PID0_RESET_VAL: u32 = 0x20;
/// Peripheral ID Register 1 Reset Value
const PID1_RESET_VAL: u32 = 0xB8;
/// Peripheral ID Register 2 Reset Value
const PID2_RESET_VAL: u32 = 0x1B;

/// Since the GPIO0 is left unmapped on QEMU, it should return 0x00.
/// According to Chapter 3.5.2 of Arm Cortex-M System Design Kit,
/// PID0-PID2 have reset values documented as above.
pub fn check_pid() {
    println!();
    println!("Checking Peripheral ID Registers");
    println!();

    let pid0 = unsafe { read_volatile(PID0) & 0xFF };
    let pid1 = unsafe { read_volatile(PID1) & 0xFF };
    let pid2 = unsafe { read_volatile(PID2) & 0xFF };

    println!("PID0: {:#010X}, expected {:#04X}", pid0, PID0_RESET_VAL);
    println!("PID1: {:#010X}, expected {:#04X}", pid1, PID1_RESET_VAL);
    println!("PID2: {:#010X}, expected {:#04X}", pid2, PID2_RESET_VAL);
}

/// Component ID Register 0
const CID0: *const u32 = (GPIO0 + 0xFF0) as *const u32;
/// Component ID Register 1
const CID1: *const u32 = (GPIO0 + 0xFF4) as *const u32;
/// Component ID Register 2
const CID2: *const u32 = (GPIO0 + 0xFF8) as *const u32;
/// Component ID Register 3
const CID3: *const u32 = (GPIO0 + 0xFFC) as *const u32;

/// Component ID Register 0 Reset Value
const CID0_RESET_VAL: u32 = 0x0D;
/// Component ID Register 1 Reset Value
const CID1_RESET_VAL: u32 = 0xF0;
/// Component ID Register 2 Reset Value
const CID2_RESET_VAL: u32 = 0x05;
/// Component ID Register 3 Reset Value
const CID3_RESET_VAL: u32 = 0xB1;

/// Since the GPIO0 is left unmapped on QEMU, it should return 0x00.
/// According to Chapter 3.5.2 of Arm Cortex-M System Design Kit,
/// CID0-CID2 have reset values documented as above.
pub fn check_cid() {
    println!();
    println!("Checking Component ID Registers");
    println!();

    let cid0 = unsafe { read_volatile(CID0) & 0xFF };
    let cid1 = unsafe { read_volatile(CID1) & 0xFF };
    let cid2 = unsafe { read_volatile(CID2) & 0xFF };
    let cid3 = unsafe { read_volatile(CID3) & 0xFF };

    println!("CID0: {:#010X}, expected {:#04X}", cid0, CID0_RESET_VAL);
    println!("CID1: {:#010X}, expected {:#04X}", cid1, CID1_RESET_VAL);
    println!("CID2: {:#010X}, expected {:#04X}", cid2, CID2_RESET_VAL);
    println!("CID3: {:#010X}, expected {:#04X}", cid3, CID3_RESET_VAL);
}

/// APB test slave register
/// Data register, 32-bit. Zero wait state, two PCLK cycles per access.
const APB_SLAVE: *mut u32 = 0x4000B000 as *mut u32;
/// Get pwned
const APB_SLAVE_WB: u32 = 0xDEADBEEF;

/// This check is for whether APB test slaves are implemented or not.
/// Documented in 4.7.3 APB test slave.
///
/// A simple APB test slave is included for verification purposes. Arm uses the test slave to verify
/// the handling of wait states and error responses in the AHB to APB bridge. The APB test slave
/// contains a software-programmable register that supports word, halfword, and byte size accesses.
pub fn check_apb_slave() {
    println!();
    println!("Checking APB Test Slave state");
    println!();

    let apb_init_val = unsafe { read_volatile(APB_SLAVE) };

    println!("APB Test Slave initial value: {:#010X}", apb_init_val);

    unsafe {
        write_volatile(APB_SLAVE, APB_SLAVE_WB);
    }

    let read_the_write = unsafe { read_volatile(APB_SLAVE) };

    unsafe {
        write_volatile(APB_SLAVE, apb_init_val);
    }

    println!(
        "APB Test Slave readback: {:#010X}, expected: {:#010X}",
        read_the_write, APB_SLAVE_WB
    );
}
