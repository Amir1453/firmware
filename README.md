# Assignment

Write a small bare-metal firmware for the Arm Cortex-M3 emulated by QEMU (qemu-system-arm -M mps2-an385). You don't need any hardware. Use any Arm toolchain (e.g. Arm Toolchain for Embedded or the GNU Arm toolchain), but no vendor SDK, HAL, IDE or RTOS.

The firmware must:

-use your own linker script and startup code (vector table, reset handler);
- print its output through the board's UART (not semihosting);
- determine whether it runs on an emulator or on real silicon, using at least two independent checks, and print the verdict and the raw values it observed.

# Deliverebles

Send us a zip containing:

-the sources;
- a Makefile (or CMake) where make run builds and runs everything;
- the ELF;
- a report of 2 pages at most that:
- explains the boot sequence of your firmware, from reset to main;
- for each check: what it reads, the value seen under QEMU, the value you expect on a real Cortex-M3 with a reference to the Arm documentation, and how an attacker could get around it;
- describes at least one check that did not work, and why;
- lists the tools you used, LLMs included, and what you checked yourself.

This should take 3 to 4 hours. We don't expect a perfect detector: a check that fails, with a good explanation, is worth more than a long list of checks you can't explain.
