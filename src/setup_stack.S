.section ".text.entry_point"

.global setup_stack
setup_stack:
    ldr x0, =0x80010000
    mov sp, x0
    b _start
