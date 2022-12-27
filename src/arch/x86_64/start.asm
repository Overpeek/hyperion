    global start
    extern kernel_main

    ;; ----------
    ;; Multiboot2
    ;; ----------

    section .multiboot2

header_start:
    dd 0xE85250D6                                             ;; multiboot2 magic
    dd 0                                                      ;; arch: 32 bit (protected mode)
    dd header_end - header_start                              ;; header length
    dd 0x100000000 - (0xE85250D6 + header_end - header_start) ;; checksum

    dd 0 ;; end tag
    dd 8
header_end:

    ;; ----------
    ;; Multiboot1
    ;; ----------

    section .multiboot1
    dd 0x1BADB002                     ;; multiboot1 magic
    dd 3                              ;; flags
    dd 0x100000000 - (0x1BADB002 + 3) ;; checksum

    section .boot
    global start
    bits 32

    ;; ----------
    ;; Boot entry
    ;; ----------

start:
    cli
    cld

    ;; init stack
    mov esp, stack_top

    ;; support checks
    call check_multiboot1
    call check_cpuid
    call check_long_mode

    ;; setup
    call setup_page_tables
    call enable_paging

    ;; enter long mode
    lgdt [gdt64.pointer]
    jmp gdt64.code_segment: long_mode_start
    jmp halt

error:
    ;; print 'ERR: <err>'
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al
    jmp halt

halt:
    ;; print ZZZ
	mov word [0xb8f00], 0x0f5a
	mov word [0xb8f02], 0x0f5a
	mov word [0xb8f04], 0x0f5a
    hlt
    jmp halt

    ;; ------
    ;; Checks
    ;; ------

    section .boot
    bits 32

check_multiboot2:
	cmp eax, 0x36D76289
	jne .no_multiboot2
	ret

.no_multiboot2:
	mov al, 'M'
	jmp error

check_multiboot1:
	cmp eax, 0x2BADB002
	jne .no_multiboot1
	ret

.no_multiboot1:
	mov al, 'M'
	jmp error

check_cpuid:
	pushfd
	pop eax
	mov ecx, eax
	xor eax, 1 << 21
	push eax
	popfd

	pushfd
	pop eax
	push ecx
	popfd

	cmp eax, ecx
	je .no_cpuid
	ret

.no_cpuid:
	mov al, 'C'
	jmp error

check_long_mode:
	mov eax, 0x80000000
	cpuid
	cmp eax, 0x80000001
	jb .no_long_mode

	mov eax, 0x80000001
	cpuid
	test edx, 1 << 29
	jz .no_long_mode

	ret

.no_long_mode:
	mov al, 'L'
	jmp error

    ;; ----------
    ;; Page setup
    ;; ----------

setup_page_tables:
	mov eax, page_table_l3
	or  eax, 0b11 ; present, writeable
	mov [page_table_l4], eax

	mov eax, page_table_l2
	or  eax, 0b11 ; present, writeable
	mov [page_table_l3], eax

	mov ecx, 0 ; counter

.loop:
	mov eax, 0x200000 ; 2MiB
	mul ecx,
	or  eax, 0b10000011 ; present, writeable, huge page
	mov [page_table_l2 + ecx * 8], eax

	inc ecx ; inc counter
	cmp ecx, 512 ; check if the whole table is mapped
	jne .loop ; if not: continue

	ret

enable_paging:
	; pass page table location to the cpu
	mov eax, page_table_l4
	mov cr3, eax

	; enable PAE
	mov eax, cr4
	or  eax, 1 << 5
	mov cr4, eax

	; enable long mode
	mov ecx, 0xC0000080
	rdmsr
	or  eax, 1 << 8
	wrmsr

	; enable paging
	mov eax, cr0
	or  eax, 1 << 31
	mov cr0, eax

	ret

    ;; ---------
    ;; Long mode
    ;; ---------

    section .text
    bits 64
long_mode_start:
    mov ax, 0
	mov ss, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	; print 'OK'
	mov dword [0xb8000], 0x2f4b2f4f

	call kernel_main
.halt:
	hlt
    jmp halt

    ;; ------
    ;; Memory
    ;; ------

    section .bss

page_table_l4:
    resb 4096
page_table_l3:
    resb 4096
page_table_l2:
    resb 4096

stack_bottom:
    ;; 16 KiB
    resb 4096 * 4
stack_top:

    section .rodata
gdt64:
    dq 0                        ; zero entry
.code_segment: equ $ - gdt64
    dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53)
.pointer:
    dw $ - gdt64 - 1
    dq gdt64
