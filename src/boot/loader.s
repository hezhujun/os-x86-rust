; load kernel

RPL0 equ 00b
RPL1 equ 01b
RPL2 equ 10b
RPL3 equ 01b
TI_GDT equ 000b
TI_LDT equ 100b
CODE_SELECTOR equ (1 << 3) | TI_GDT | RPL0
DATA_SELECTOR equ (2 << 3) | TI_GDT | RPL0

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
  xor eax, eax
  mov ebx, eax
  mov ecx, eax
  mov edx, eax
  mov esi, eax
  mov edi, eax
  mov ebp, eax
  jmp 0x500

gdt_ptr_address equ $ - BASE_ADDRESS
gdt_ptr dw 511
        dd GDT_BASE
