MEMORY
{
	CODE (rx)  : ORIGIN = 0x00000000, LENGTH = 4M
	RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 4M
}

SECTIONS
{
  .vector_table ORIGIN(CODE) :
  {
    /* First entry: initial Stack Pointer value */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* Second entry: reset vector */
    KEEP(*(.vector_table.reset_vector));
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
    *(.bss .bss.*);
  } > RAM

  .data :
  {
    *(.data .data.*);
  } > RAM

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}
