global pdirectory

align 4096
pdirectory resd 1024


global ptable

align 4096
ptable resd 1024


global paging_load

paging_load:
    mov eax, pdirectory
    mov cr3, eax
    ret


global paging_enable

paging_enable:
    mov eax, cr0
    or  eax, 0x80000000
    mov cr0, eax
    ret
