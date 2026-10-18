#![no_std]
#![no_main]

mod uart;

/// Forces recompile if linker.ld is changed.
const _: &str = include_str!("../../linker.ld");

#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = _start;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    main();

    loop {
        core::hint::spin_loop();
    }
}

fn main() {
    uart::init();

    uart::print("Hello Quarkslab!");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
