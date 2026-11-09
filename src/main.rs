#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

fn main() {
    let kernel = env!("FIRMWARE_ELF");

    println!("Running firmware located at:\n{}", kernel);

    let gdb = std::env::args().any(|arg| arg == "--gdb");

    let mut qemu = Command::new("qemu-system-arm");
    qemu.args(["-M", "mps2-an385"]);
    qemu.args(["-kernel", kernel]);
    qemu.arg("-nographic");

    if gdb {
        #[cfg(unix)]
        {
            unsafe extern "C" {
                fn signal(
                    sig: std::ffi::c_int,
                    handler: extern "C" fn(std::ffi::c_int),
                ) -> *mut std::ffi::c_void;
            }

            extern "C" fn handle_sigint(_: std::ffi::c_int) {}

            // Keep the runner alive on Ctrl-C. Caught handlers reset on exec,
            // so GDB still receives interrupts normally.
            const SIGINT: std::ffi::c_int = 2;
            let previous = unsafe { signal(SIGINT, handle_sigint) };
            assert_ne!(previous as isize, -1, "failed to configure Ctrl-C handling");

            // GDB forwards interrupts through the debug connection.
            qemu.process_group(0);
        }
        qemu.args(["-gdb", "tcp:127.0.0.1:3333"]);
        qemu.arg("-S");
        qemu.stdin(Stdio::null());
    }

    let mut child = qemu.spawn().expect("failed to start qemu");

    if gdb {
        let status = Command::new("rust-gdb")
            .arg("-readnow")
            .arg(kernel)
            .args([
                "-ex",
                "set tcp auto-retry on",
                "-ex",
                "set tcp connect-timeout 5",
                "-ex",
                "target remote 127.0.0.1:3333",
            ])
            .status();

        let _ = child.kill();
        child.wait().expect("failed to wait on qemu");

        println!("GDB exit code: {}", status.expect("failed to start gdb"));
        return;
    }

    let status = child.wait().expect("failed to wait on qemu");

    println!("QEMU exit code: {status}");
}
