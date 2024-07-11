; load kernel

RPL0 equ 00b
RPL1 equ 01b
RPL2 equ 10b
RPL3 equ 01b
TI_GDT equ 000b
TI_LDT equ 100b
CODE_SELECTOR equ (1 << 3) | TI_GDT | RPL0
DATA_SELECTOR equ (2 << 3) | TI_GDT | RPL0
PTE_ATTR_P equ 1
PTE_ATTR_RW equ 10b
PTE_ATTR_U equ 100b

section mbr vstart=0x90000

BASE_ADDRESS equ $
GDT_BASE: dd 0x0
          dd 0x0

CODE_DESC: dd 0x0000FFFF
           dd 0x00CF9800

DATA_DESC: dd 0x0000FFFF
           dd 0x00CF9200
dd 61 dup(0)
dd 61 dup(0)
; 64 segment descriptor

; memory segments count
ARDS_COUNT equ $ - BASE_ADDRESS
ards_count: dd 0x0
ARDS_BASE equ $ - BASE_ADDRESS
ards_base:
; [0x200+4, 0x400) record memory segments
times 0x400-($-$$) db 0

begin_loader:
  mov ax, 0x9000
  mov ds, ax
read_memory_info:
  ; read memory info
  mov ax, cs
  mov es, ax
  mov ebx, 0
  mov di, ARDS_BASE
  mov ecx, 20
  mov edx, 0x534d4150
  mov word [ds:ARDS_COUNT], 0
.read_memoy_info_loop:
  mov ecx, 20
  mov eax, 0xE820
  int 0x15
  jc read_memory_info
  add di, cx
  inc word [ds:ARDS_COUNT]
  cmp ebx, 0
  jnz .read_memoy_info_loop

  ; open A20
  in al, 0x92
  or al, 0000_0010B
  out 0x92, al

  ; load GDT
  lgdt [gdt_ptr_address]

  ; open protect mode
  mov eax, cr0
  or eax, 0x1
  mov cr0, eax
  jmp dword CODE_SELECTOR:p_mode_start

[bits 32]
p_mode_start:
  mov ax, DATA_SELECTOR
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax
  mov ss, ax

load_kernel:
  mov al, 240  ; number of sectors to read
  mov dx, 0x1f2
  out dx, al

  mov eax, 5   ; start sector  
  mov dx, 0x1f3
  out dx, al
  mov dx, 0x1f4
  shr eax, 8
  out dx, al
  mov dx, 0x1f5
  shr eax, 8
  out dx, al
  mov dx, 0x1f6
  shr eax, 8
  or al, 0xe0
  out dx, al

  mov dx, 0x1f7
  mov al, 0x20
  out dx, al

.wait:
  nop
  in al, dx
  and al, 0x88
  cmp al, 0x08
  jnz .wait

  mov eax, 240
  mov cx, 512
  mul cx
  shl edx, 16
  mov dx, ax
  mov ecx, edx
  shr ecx, 1
  
  mov ebx, 0x500
  mov dx, 0x1f0
.go_on_read:
  in ax, dx
  mov [ebx], ax
  add ebx, 2
  loop .go_on_read

  mov eax, 0x90000
  mov esp, eax

  ; setup memory page table
  ; root page 0x100000
  mov eax, 0x100000
  push eax
  call clean_page
  pop eax

  mov eax, 0x101000
  push eax
  call clean_page
  pop eax

  mov ebx, 0x100000
  or eax, PTE_ATTR_P | PTE_ATTR_RW ; pte attributes
  mov [ebx], eax         ; #0 pde
  mov [ebx + 768*4], eax   ; #768 pde

  ; map low 1M
  mov ebx, 0x101000
  mov ecx, 0x100
  mov edi, 0
  mov eax, 0
  or eax, PTE_ATTR_P | PTE_ATTR_RW
.map_low_1M_loop:
  mov [ebx + edi], eax
  add edi, 4
  add eax, 0x1000
  loop .map_low_1M_loop

  ; #769-#1022 pde
  mov ecx, 254
  mov edx, 0x102000
  mov ebx, 0x100000
  mov edi, 769 * 4
.map_high_pde_loop:
  push ecx
  push edx
  call clean_page
  pop edx
  mov eax, edx
  or eax, PTE_ATTR_P | PTE_ATTR_RW
  mov [ebx + edi], eax
  add edi, 4
  add edx, 0x1000
  pop ecx
  loop .map_high_pde_loop

  ; #1023 pde
  mov eax, 0x100000
  or eax, PTE_ATTR_P | PTE_ATTR_RW
  mov [ebx + 1023*4], eax

  add dword [gdt_ptr + 2], 0xc0000000

  mov eax, 0x100000
  mov cr3, eax
  mov eax, cr0
  or eax, 1 << 31
  mov cr0, eax

  lgdt [gdt_ptr]

  ; clean #0 pde
  mov ebx, 0xfffff000
  mov dword [ebx], 0

  mov eax, 0xc0090000
  mov esp, eax
  xor eax, eax
  mov ebx, eax
  mov ecx, eax
  mov edx, eax
  mov esi, eax
  mov edi, eax
  mov ebp, eax
  jmp 0xc0000500

clean_page:
  push ebp
  mov ebp, esp
  push ebx
  push edi
  
  mov ebx, [ebp + 8]  ; page address
  mov edi, 0
  mov ecx, 0x1000
  shr ecx, 2
  xor eax, eax
.clean_page_loop:
  mov [ebx + edi], eax
  add edi, 4
  loop .clean_page_loop

  pop edi
  pop ebx
  mov esp, ebp
  pop ebp
  ret

gdt_ptr_address equ $ - BASE_ADDRESS
gdt_ptr dw 511
        dd GDT_BASE
