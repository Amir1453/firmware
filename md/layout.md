.
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── firmware
│   ├── Cargo.toml
│   └── src
│       ├── em_check.rs
│       ├── main.rs
│       └── uart.rs
├── linker.ld
├── Makefile
├── md
│   ├── layout.md
│   ├── report.pdf
│   ├── report.typ
│   └── sources.md
├── README.md
├── rust-toolchain.toml
└── src
    └── main.rs

build.rs: Passes on the ELF location to the QEMU runner.
Cargo.toml: Simple cargo configuration.
firmware/
    Cargo.toml: Simple cargo configuration 2.
    src/
       em_check: File that has tests on checking whether we are being emulated. 
       main.rs: Main entrypoint of the firmware, sets up a bunch of stuff.
       uart.rs: UART communication set up code.
linker.ld: Linker file for the firmware.
Makefile: A good old makefile.
md/
    layout.md: Describes the layout of the project.
    report.typ: Typst source of the report.
    sources.md: A bunch of useful links that I used.
README.md: Assignment specification.
rust-toolchain.toml: Toolchain configuration that downloads the necessary stuff.
src/
    main.rs: Gets the firmware ELF, and then runs QEMU.
