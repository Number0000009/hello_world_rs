.section ".text.entry_point"

.global setup_stack
setup_stack:
    ldr x0, =STACK_START
    mov sp, x0
    b _start
