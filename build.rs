fn main() {
    let elf = std::env::var("CARGO_BIN_FILE_FIRMWARE_firmware")
        .expect("Could not find firmware");

    println!("cargo:rustc-env=FIRMWARE_ELF={elf}");
}
