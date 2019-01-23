STACK_START = 0x80010000;

MEMORY { .SRAM : ORIGIN = 0x2e000000, LENGTH = 0xffffff }

OUTPUT_FORMAT("elf64-littleaarch64")
OUTPUT_ARCH(aarch64)
ENTRY(setup_stack)
SECTIONS
{
       .text : {
                . = ALIGN(8);
                *(.text.entry_point)
                *(.text*)
        } >.SRAM

        .rodata : {
                . = ALIGN(8);
                __rodata_start = .;
                *(.rodata)
                __rodata_end = .;
        } >.SRAM

        .data : {
                . = ALIGN(8);
                __data_start = .;
                *(.data)
                __data_end = .;
        } >.SRAM

        .bss : {
                . = ALIGN(8);
                __bss_start = .;
                *(.bss)
                __bss_end = .;
        } >.SRAM

        /DISCARD/ : { *(.dynstr*) }
        /DISCARD/ : { *(.dynamic*) }
        /DISCARD/ : { *(.plt*) }
        /DISCARD/ : { *(.interp*) }
        /DISCARD/ : { *(.gnu*) }
        /DISCARD/ : { *(.note*) }

        init_end = .;
}
