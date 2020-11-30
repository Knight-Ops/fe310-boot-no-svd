MEMORY
{
    MASKROM     (rx) : ORIGIN = 0x00010000, LENGTH = 8K
    OTP         (rx) : ORIGIN = 0x00020000, LENGTH = 8K
    FLASH       (rx) : ORIGIN = 0x20000000, LENGTH = 512M
    /* APPFLASH (rx) : ORIGIN = ?, LENGTH = ? */
    RAM         (rwx): ORIGIN = 0x80000000, LENGTH = 16K 
}

__maskrom_start__ = ORIGIN(MASKROM);
__maskrom_size__ = LENGTH(MASKROM);
__otp_start__ = ORIGIN(OTP);
__otp_size__ = LENGTH(OTP);
__rom_start__ = ORIGIN(FLASH);
__rom_size__ = LENGTH(FLASH);
__ram_start__ = ORIGIN(RAM);
__ram_size__ = LENGTH(RAM);

__stack_start = ORIGIN(RAM) + LENGTH(RAM);

ENTRY(_start)

SECTIONS
{
    .text :
    {
        __text_start = .;
        *(.text._start) *(.text*)
        __text_end = .;
    } > FLASH

    .rodata ALIGN(4) : 
    {
        __rodata_start = .;
        *(.rodata*)
        __rodata_end = .;
    } > RAM AT > FLASH

    .got ALIGN(4) :
    {
        __got_start = .;
        *(.got*)
        __got_end = .;
    } > RAM AT > FLASH

    .data  ALIGN(4) :
    {
        __data_start = .;
        *(.data)
        __data_end = .;
    } > RAM AT > FLASH

    .bss ALIGN(4) (NOLOAD) : 
    {
        __bss_start = .;
        *(.bss*)
        __bss_end = .;
    } > RAM

    /DISCARD/ : { *(.comment*) }
}