use std::process::Command;

fn main() {
    let kernel = env!("FIRMWARE_ELF");

    println!("{}", kernel);

    let mut qemu = Command::new("qemu-system-arm");
    qemu.args(["-M", "mps2-an385"]);
    qemu.args(["-kernel", kernel]);
    qemu.args(["-gdb", "tcp::3333"]);
    qemu.arg("-nographic");

    let mut child = qemu.spawn().expect("failed to start qemu");

    let status = child.wait().expect("failed to wait on qemu");
    match status.code().unwrap_or(1) {
        0x10 => 0, // success
        0x11 => 1, // failure
        _ => 2,    // unknown fault
    };
}
