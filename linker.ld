MEMORY
{
	CODE (rx)  : ORIGIN = 0x00000000, LENGTH = 4M
	RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 4M
}

SECTIONS
{
  .vector_table ORIGIN(CODE) :
  {
    /* Register the stack pointer */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* Register the reset handler function */
    KEEP(*(.vector_table.reset_vector));

    /* Register the Exception Handlers */
    KEEP(*(.vector_table.exceptions));
  } > CODE

  .text :
  {
    *(.text .text.*);
  } > CODE

  .rodata :
  {
    *(.rodata .rodata.*);
  } > CODE

  .bss :
  {
    _start_bss = .;
    *(.bss .bss.*);
    _end_bss = .;
  } > RAM

  _init_data = LOADADDR(.data);
  .data :
  {
    _start_data = .;
    *(.data .data.*);
    _end_data = .;
  } > RAM AT > CODE

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}
