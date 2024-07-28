.global intr_handler

.altmacro
.macro handle_intr
  push edi
  push esi
  push ebp
  push ebx
  push edx
  push ecx
  push eax
  xor eax, eax
  mov eax, gs
  push eax
  mov eax, fs
  push eax
  mov eax, ds
  push eax
  mov eax, es
  push eax
  mov eax, 0x1234
  push eax
  call intr_handler
  jmp intr_exit
.endm

.macro intr_without_error_code n
  .section .text.intr
  .global intr_entry_\n
intr_entry_\n:
  push 0
  push \n
  handle_intr
.endm

.macro intr n
  .section .text.intr
  .global intr_entry_\n
intr_entry_\n:
  push \n
  handle_intr
.endm

  .section .text.intr_exit
  .global intr_exit
intr_exit:
  pop eax
  pop eax
  mov es, ax
  pop eax
  mov ds, ax
  pop eax
  mov fs, ax
  pop eax
  mov gs, ax
  pop eax
  pop ecx
  pop edx
  pop ebx
  pop ebp
  pop esi
  pop edi
  add esp, 8
  iretd

intr_without_error_code 0x00
intr_without_error_code 0x01
intr_without_error_code 0x02
intr_without_error_code 0x03
intr_without_error_code 0x04
intr_without_error_code 0x05
intr_without_error_code 0x06
intr_without_error_code 0x07
intr 0x08
intr_without_error_code 0x09
intr 0x0a
intr 0x0b
intr 0x0c
intr 0x0d
intr 0x0e
intr_without_error_code 0x0f
intr_without_error_code 0x10
intr 0x11
intr_without_error_code 0x12
intr_without_error_code 0x13
intr_without_error_code 0x14
intr_without_error_code 0x15
intr_without_error_code 0x16
intr_without_error_code 0x17
intr_without_error_code 0x18
intr_without_error_code 0x19
intr_without_error_code 0x1a
intr_without_error_code 0x1b
intr_without_error_code 0x1c
intr_without_error_code 0x1d
intr_without_error_code 0x1e
intr_without_error_code 0x1f
intr_without_error_code 0x20
intr_without_error_code 0x21
intr_without_error_code 0x22
intr_without_error_code 0x23
intr_without_error_code 0x24
intr_without_error_code 0x25
intr_without_error_code 0x26
intr_without_error_code 0x27
intr_without_error_code 0x28
intr_without_error_code 0x29
intr_without_error_code 0x2a
intr_without_error_code 0x2b
intr_without_error_code 0x2c
intr_without_error_code 0x2d
intr_without_error_code 0x2e
intr_without_error_code 0x2f
intr_without_error_code 0x30
intr_without_error_code 0x31
intr_without_error_code 0x32
intr_without_error_code 0x33
intr_without_error_code 0x34
intr_without_error_code 0x35
intr_without_error_code 0x36
intr_without_error_code 0x37
intr_without_error_code 0x38
intr_without_error_code 0x39
intr_without_error_code 0x3a
intr_without_error_code 0x3b
intr_without_error_code 0x3c
intr_without_error_code 0x3d
intr_without_error_code 0x3e
intr_without_error_code 0x3f
intr_without_error_code 0x40
intr_without_error_code 0x41
intr_without_error_code 0x42
intr_without_error_code 0x43
intr_without_error_code 0x44
intr_without_error_code 0x45
intr_without_error_code 0x46
intr_without_error_code 0x47
intr_without_error_code 0x48
intr_without_error_code 0x49
intr_without_error_code 0x4a
intr_without_error_code 0x4b
intr_without_error_code 0x4c
intr_without_error_code 0x4d
intr_without_error_code 0x4e
intr_without_error_code 0x4f
intr_without_error_code 0x50
intr_without_error_code 0x51
intr_without_error_code 0x52
intr_without_error_code 0x53
intr_without_error_code 0x54
intr_without_error_code 0x55
intr_without_error_code 0x56
intr_without_error_code 0x57
intr_without_error_code 0x58
intr_without_error_code 0x59
intr_without_error_code 0x5a
intr_without_error_code 0x5b
intr_without_error_code 0x5c
intr_without_error_code 0x5d
intr_without_error_code 0x5e
intr_without_error_code 0x5f
intr_without_error_code 0x60
intr_without_error_code 0x61
intr_without_error_code 0x62
intr_without_error_code 0x63
intr_without_error_code 0x64
intr_without_error_code 0x65
intr_without_error_code 0x66
intr_without_error_code 0x67
intr_without_error_code 0x68
intr_without_error_code 0x69
intr_without_error_code 0x6a
intr_without_error_code 0x6b
intr_without_error_code 0x6c
intr_without_error_code 0x6d
intr_without_error_code 0x6e
intr_without_error_code 0x6f
intr_without_error_code 0x70
intr_without_error_code 0x71
intr_without_error_code 0x72
intr_without_error_code 0x73
intr_without_error_code 0x74
intr_without_error_code 0x75
intr_without_error_code 0x76
intr_without_error_code 0x77
intr_without_error_code 0x78
intr_without_error_code 0x79
intr_without_error_code 0x7a
intr_without_error_code 0x7b
intr_without_error_code 0x7c
intr_without_error_code 0x7d
intr_without_error_code 0x7e
intr_without_error_code 0x7f
intr_without_error_code 0x80
intr_without_error_code 0x81

  .section .bss.intr_table
  .globl intr_table
  .align 8
intr_table:
  .space 256 * 8
