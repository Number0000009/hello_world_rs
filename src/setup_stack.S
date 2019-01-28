.section ".text.entry_point"

.global setup_stack
setup_stack:
    ldr     x0, =STACK_START
    mov     sp, x0
    msr     sp_el0, x0      // sp_el0 will be used as sp_elx
    b       _start