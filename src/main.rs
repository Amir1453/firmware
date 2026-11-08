use std::process::Command;

fn main() {
    let kernel = env!("FIRMWARE_ELF");

    println!("{}", kernel);

    let gdb = std::env::args().any(|arg| arg == "--gdb");

    let mut qemu = Command::new("qemu-system-arm");
    qemu.args(["-M", "mps2-an385"]);
    qemu.args(["-kernel", kernel]);
    qemu.arg("-nographic");

    if gdb {
        qemu.args(["-gdb", "tcp::3333"]);
        qemu.arg("-S");
    }

    let mut child = qemu.spawn().expect("failed to start qemu");
    let status = child.wait().expect("failed to wait on qemu");

    println!("QEMU exit code: {status}");
}
