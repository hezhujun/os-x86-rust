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
ARDS_COUNT: dd 0x0
ARDS_BASE:
; [0x200+4, 0x400) record memory segments
times 0x400-($-$$) db 0

gdt_ptr dw 511
        dd GDT_BASE
        dw 0
begin_loader:
  mov ax, 0x9000
  mov ds, ax

load_kernel:
  mov cx, 240        ; sectors count
  mov ebx, 0x500     ; address
  mov di, 5          ; begin sector
  mov word [ds:block_count], 128
  mov word [ds:buffer_addr_offset], 0
  mov word [ds:buffer_addr_segment], 0x50
  mov [ds:sector_start], di

.load_kernel_loop:
  mov ah, 42h
  mov dl, 0x80
  mov si, disk_address_packet 
  int 13h
  jc load_kernel

  mov ax, [ds:block_count]
  add di, ax
  mov [ds:sector_start], di
  sub cx, ax
  cmp cx, 0
  je .load_success
  cmp cx, 128     ; max number of sector to read one time
  jb .remaining_sectors_below_128
  mov word [ds:block_count], 128
  jmp .handle_address
.remaining_sectors_below_128:
  mov [ds:block_count], cx
.handle_address:
  mov dx, 512
  mul dx
  shl edx, 16
  and eax, 0xffff
  add eax, edx
  add eax, ebx          ; new address
  ; new address must be 0x***00
  ; transform to 0x***0:0x0
  ; after read data, end address is 0x***0:0xffff
  shr eax, 4
  mov word [ds:buffer_addr_offset], 0
  mov [ds:buffer_addr_segment], ax
  jmp .load_kernel_loop

.load_success
  jmp read_memory_info

disk_address_packet:
  db 0x10
  db 0x0
block_count:
  dw 128
buffer_addr_offset:
  dw 0x500
buffer_addr_segment:
  dw 0x0
sector_start:
  dq 0x05

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

  ; disable bios interrupt
  ; my screen keeps flickering after open protect mode
  ; disable bios interrupt can solve this problem
  cli
  cld
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
