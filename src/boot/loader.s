; 
; load kernel from sector 6-245

RPL0 equ 00b
RPL1 equ 01b
RPL2 equ 10b
RPL3 equ 01b
TI_GDT equ 000b
TI_LDT equ 100b
CODE_SELECTOR equ (1 << 3) | TI_GDT | RPL0
DATA_SELECTOR equ (2 << 3) | TI_GDT | RPL0

section mbr vstart=0x90000

GDT_BASE: dd 0x0
          dd 0x0

CODE_DESC: dd 0x0000FFFF
           dd 0x00CF9800

DATA_DESC: dd 0x0000FFFF
           dd 0x00CF9200

dd 61 dup(0)
dd 61 dup(0)
; 64 segment descriptor

load_kernel:
  mov ax, cs
  mov ds, ax
  mov ah, 02h             ; load sector
  mov al, 2             ; load 6-245 sectors
  mov ch, 0               ; in track 0
  mov cl, 6               ; from sector 2
  mov dh, 0
  mov dl, 0x80
  mov bx, 0
  mov es, bx
  mov bx, 0x500           ; save data in 
  int 13h
  jb load_kernel

  ; open A20
  in al, 0x92
  or al, 0000_0010B
  out 0x92, al

  ; load GDT
  lgdt [gdt_ptr]

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

gdt_ptr dw 511
        dd GDT_BASE
