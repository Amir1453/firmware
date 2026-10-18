#![no_std]
#![no_main]

mod em_check;
mod uart;

/// Forces recompile if linker.ld is changed.
const _: &str = include_str!("../../linker.ld");

/// The default exception handler, just loops forever.
#[unsafe(no_mangle)]
pub extern "C" fn dh() {
    loop {
        core::hint::spin_loop();
    }
}

/// There are specific slots reserved by ARM, so we need to set them to 0.
pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

/// Populate the exception handler table, with only dh() for now!
#[unsafe(link_section = ".vector_table.exceptions")]
#[unsafe(no_mangle)]
pub static EXCEPTIONS: [Vector; 14] = [
    // NMI
    Vector { handler: dh },
    // HardFault
    Vector { handler: dh },
    // MemManage
    Vector { handler: dh },
    // BusFault
    Vector { handler: dh },
    // UsageFault
    Vector { handler: dh },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    // SVCall
    Vector { handler: dh },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    // PendSV
    Vector { handler: dh },
    // SysTick
    Vector { handler: dh },
];

/// Pointer to the reset function (_start)
#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = _start;

// Sections that will be provided by the linker.
unsafe extern "C" {
    static mut _start_bss: u8;
    static mut _end_bss: u8;

    static mut _start_data: u8;
    static mut _end_data: u8;
    static _init_data: u8;
}

/// Taken from embedonomicon, initializes .data and .bss
#[allow(static_mut_refs)]
extern "C" fn init_mem() {
    // Ugliest code on the planet ?

    // Zeroes out bss
    let count = unsafe { &_end_bss as *const u8 as usize - &_start_bss as *const u8 as usize };
    unsafe { core::ptr::write_bytes(&mut _start_bss as *mut u8, 0, count) };

    // Copies from initial .data to the runtime .data
    let count = unsafe { &_end_data as *const u8 as usize - &_start_data as *const u8 as usize };
    unsafe {
        core::ptr::copy_nonoverlapping(&_init_data as *const u8, &mut _start_data as *mut u8, count)
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init_mem();

    main();
}

fn main() -> ! {
    uart::init();

    uart::print("Hello Quarkslab!");

    loop {
        core::hint::spin_loop();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
