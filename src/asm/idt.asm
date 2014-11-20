global idt_load         ; allow the Rust code to call idtload()

idt_load:
    mov eax, [esp+4]    ; get the pointer to the IDT, passed as a parameter
    lidt [eax]          ; load the IDT
    ret
