global paging_load

paging_load:
    push ebp
    mov  ebp, esp
    mov  eax, [esp+8]
    mov  cr3, eax
    mov  esp, ebp
    pop  ebp
    ret


global paging_enable

paging_enable:
    mov eax, cr0
    or  eax, 0x80000000
    mov cr0, eax
    ret
