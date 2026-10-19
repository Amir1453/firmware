#set page(paper: "a4", margin: 17mm)
#set text(font: "Libertinus Serif", size: 9.5pt)
#set par(justify: true)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 15pt, weight: "bold")[Internship Application for Quarkslab \ Protections against dynamic attacks for firmware]
  #v(3pt)
  *Author:* Amir Sabbag  |  *Date:* 18 October 2026
]

= Introduction

Hello, I hope you enjoy the report. The assignment was quite enjoyable.

The basic goal of the assignment was to create a bare-metal (no standart library) firmware for the Arm Cortex-M3, which would be emulated in QEMU, with the goal of detecting a running emulator. Cortex-M3 is a 32-bit reduced instruction set chip made by Arm.

I chose the Rust language and the Cargo ecosystem for writing the firmware, since no specification for languages was provided in the assignment. Nevertheless, the firmware is \#!\[no_std\] and \#!\[no_main\], so there is no possibility of using a standart library feature. Furthermore, I did not use any crates that provide external features either, all of the code is self contained. 

= Project Layout

To compile the project, you will need a working rustup installation. The project pins the compiler version, namely nightly-2026-10-16, so it should be reproducable. We use the nightly compiler, since the binary dependencies feature of Cargo is currently unstable, although it is very useful for our case.

The project uses the Cargo workspaces feature, so the code in src/ is simply a QEMU runner, and the actual firmware code is in firmware/src/. When building the project, Cargo first builds the firmware (of course compiling to target thumbv7m-none-eabi) and creates the ELF, and passes its path along to src/main.rs. 

In src/main.rs, we simply call the QEMU command with the specified arguments and the ELF source. A Makefile is provided as well, which will compile and run the firmware in QEMU. Most of the project layout was taken from another project of mine, an x86_64 hypervisor, also written in Rust.

= Boot Sequence

The Arm Cortex-M3 chip constantly runs in a loop, and when it recieves a reset, it will load the stack pointer from 0x00000000, and the reset handler from 0x00000004 (we are in 32-bit after all). So, we will have to put the stack and the reset handler function in those respective places. We use a linker script for that.

The linker script is in the root of the project, at linker.ld. To know exactly what to put in the linker script, I had to go around and find the specific memory layout of the Arm Cortex-M3 chip. A nice script was in the chemuRTOS source code, which I have linked below. In the linker script, we use two memory regions, mainly the 4 MiB ZBT SSRAM1 starting at 0x00000000 (denoted CODE), and ZBT SSRAM2/3 starting at 0x20000000 (denoted RAM).

The CODE segment is denoted read and execute, and RAM is read, write and execute. We start our stack at the very limit of the RAM region, at 0x20400000, since the stack grows towards lesser memory regions. In our rust code, we have a \_start() function, which is the default name that linkers search for. That will be our entrypoint. We store the function pointer of \_start() in a static variable, and place it in the section "vector\_table.reset\_vector", which the linker script places right after the stack pointer.

We register the eight exception handlers via the static variable EXCEPTIONS, which is placed in "vector\_table.exceptions". For now, all of them point to a default handler dh(), which loops forever. Not much "handling" there. After that, we initialize `.text` and `.rodata` in CODE, since those are read-only, and `.bss` and `.data` in RAM. We also provide the start and end pointers of `.bss` and `.data`, which we will need later. That wraps it up for the linker script!

Our \_start() function will do the necessary setup and then call main. According to the Embedonomicon (linked below), it is not advised to write the start-up code in Rust, yet adding assembly in the middle was going to complicate everything too much, so I just did what they told me not to do (it seems to work so far!). Simply, with the section pointers that we get from the linker script (I had written about this last year on my website, actually), we zero out the `.bss`, and copy `.data` from CODE to RAM. That is about it, after that, we can finally call our Rust function main(). So far it was just dealing with extern "C" fn's.

Finally, in src/firmware/uart.rs, with the init() function, we can initialize the UART interface. We only use the UART0, and the source file pretty much explains how to set it up.

#pagebreak()

= Detection Checks

Both of the detection checks implemented are in the source file firmware/src/em_check.rs. They both follow from the same logic, mainly finding gaps in between QEMU's implementation, and ARM's specification. The way this search was done was straightforward:

1. Find the source code for the QEMU emulator for Cortex-M3, it is at https://github.com/qemu/qemu/blob/master/hw/arm/mps2.c
2. Find all the calls of create_unimplemented_device(), which creates and maps a dummy device.
3. Find address regions where there are unimplemented devices.
4. Refer to the ARM documentation on whether there are reset value registers in that range, or other R/W registers.

Step 4 was done via ChatGPT, where I asked the LLM to search through documentation and find me possible candidates.

Therefore, this solution is *heavily* dependant on QEMU's implementation, and an attacker wishing to bypass this detection only has to implement the devices themselves. Normally, I would have a few ideas on Hypervisor detection, mainly simple stuff like checking CPUID hypervisor set bits, and trying to certain operations to see if a VMEXIT is present or not. Since this is fully emulated, it seems the only possible ways are for checking gaps in implemented emulators, or maybe by some kind of hardware device that generates keys ?

Other ideas I had were checking for instruction timings, but that would require having knowledge of different kinds of execution time of bare metal boards.

== CMSDK AHB GPIO #0 PIDN and CIDN reset value 

For our first check, we used the following information from the documentation: CMSDK AHB GPIO #0 lays between start address 0x4001000, and end address 0x40010FFF (Application Note AN385). It contains the registers PID0, PID1, PID2, and CID0, CID1, CID2, CID3, which according to the documentation, all advertise reset values (Arm Cortex-M3 System Design Kit).

Reset values are values that the registers have after booting, and since these registers are read-only, reading them should provide the values advertised in the documentation. However, QEMU does not map those regions, and simply returns 0 for everything.

Doing a volatile read on those registers, and then comparing them to the expected reset values should tell us whether we are in an emulated environment that does not map that region. However, this was not tested on a bare-metal target, and on QEMU emulator version 11.1.2.

== APB test slave register

For the second check, we used the APB test slave register, which lays between the start address 0x4000B000, and end address 0x4000BFFF (Application Note AN385). From the documentation: "A simple APB test slave is included for verification purposes. Arm uses the test slave to verify the handling of wait states and error responses in the AHB to APB bridge. The APB test slave contains a software-programmable register that supports word, halfword, and byte size accesses."

We will be writing to the register at the base offset 0, so directly to 0x4000B000. If we write a word sized (in our 32-bit case it is u32) integer to it (0xDEADBEEF), via a volatile write, when we read back from the same register, we should have the same value (0xDEADBEEF). When testing this on QEMU emulator version 11.1.2, we get back 0, which is inline with what we saw from the source code.

= Conclusion

Overall, it was quite fun to implement for an architecture which I was unfamiliar with, but prior experience with x86 definetly helped, especially since most of the concept are quite the same. However, I was quite surprised by the difficulty of having a reliable way of checking whether one is running in an emulator, and I would have expected there to be some very easy way to do so. I am guessing, with the addition of hardware devices, the surface area emulators have to cover to be undetected rises sharply, so it would be quite hard to have an emulator without any gaps. My other hopes lay in hardware execution time checking, which would check execution time and compare it to an expected value.

Thank you for taking your time to read this report, hoping to hear back from you !

= Links

- https://github.com/qemu/qemu/blob/master/hw/arm/mps2.c
- https://github.com/calebhultmann/chemuRTOS/blob/main/board/mps2-an385/linker.ld
- https://docs.rust-embedded.org/embedonomicon/memory-layout.html 
- https://github.com/qemu/qemu/blob/master/hw/char/cmsdk-apb-uart.c
- https://www.qemu.org/docs/master/system/arm/mps2.html
- https://documentation-service.arm.com/static/5e8f1c777100066a414f770b
- https://documentation-service.arm.com/static/5ea823e69931941038df1af5 
- Application Note AN385 ARM Cortex-M3 SMM on V2M-MPS2
- Arm Cortex-M3 System Design Kit
