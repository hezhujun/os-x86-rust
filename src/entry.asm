  .section .text.entry
  .global _start
_start:
  mov sp, kernel_stack_top
  call main
.dead_loop:
  jmp .dead_loop

  .section .bss.stack
  .globl kernel_stack_lower_bound
kernel_stack_lower_bound:
  .space 4096 * 16
  .globl kernel_stack_top
kernel_stack_top:
