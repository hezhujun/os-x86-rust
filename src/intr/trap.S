.global intr_handler

.altmacro
.macro handle_intr
  call intr_handler
  add esp, 8
  iretd
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

  .section .bss.intr_table
  .globl intr_table
intr_table:
  .space 256 * 8
