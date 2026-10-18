#![no_std]
#![no_main]

/// Forces recompile if linker.ld is changed.
const _: &str = include_str!("../../linker.ld");

#[unsafe(no_mangle)]
extern "C" fn _start() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
