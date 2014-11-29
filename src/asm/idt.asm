global idt_load         ; allow the Rust code to call idtload()

idt_load:
    mov eax, [esp+4]    ; get the pointer to the IDT, passed as a parameter
    lidt [eax]          ; load the IDT
    ret


%macro ISR_NOERRCODE 1          ; define a macro, taking one parameter
    global isr%1                ; %1 accesses the first parameter.
    isr%1:
        cli                     ; disable interrupts
        push 0                  ; push a dummy error code
        push %1                 ; push the interrupt number
        jmp interrupt_wrapper         ; go to isr wrapper
%endmacro

%macro ISR_ERRCODE 1
    global isr%1
    isr%1:
        cli
        push %1
        jmp interrupt_wrapper
%endmacro

ISR_NOERRCODE 0
ISR_NOERRCODE 1
ISR_NOERRCODE 2
ISR_NOERRCODE 3
ISR_NOERRCODE 4
ISR_NOERRCODE 5
ISR_NOERRCODE 6
ISR_NOERRCODE 7
ISR_ERRCODE 8
ISR_NOERRCODE 9
ISR_ERRCODE 10
ISR_ERRCODE 11
ISR_ERRCODE 12
ISR_ERRCODE 13
ISR_ERRCODE 14

%assign i 15
%rep 256 - 15
    ISR_NOERRCODE i
%assign i i + 1
%endrep


global idt_interrupt_handlers

idt_interrupt_handlers:

%macro ISR_HANDLER_ENTRY 1
    dd isr%1
%endmacro

%assign i 0
%rep 256
    ISR_HANDLER_ENTRY i
%assign i i + 1
%endrep


extern handle_interrupt

; This is our common ISR stub. It saves the processor state, sets
; up for kernel mode segments, calls the C-level fault handler,
; and finally restores the stack frame.
interrupt_wrapper:
    pushad                  ; push edi, esi, ebp, esp, ebx, edx, ecx, eax

    mov ax, ds              ; lower 16-bits of eax = ds
    push eax                ; save the data segment descriptor

    mov ax, 0x10            ; load the kernel data segment descriptor
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    call handle_interrupt

    pop eax                 ; reload the original data segment descriptor
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    popad                   ; pop edi, esi, ebp...
    add esp, 8              ; clean up the pushed error code and pushed ISR number
    sti
    iret                    ; pop 5 things at once: CS, EIP, EFLAGS, SS, and ESP
