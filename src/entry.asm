  .section .text.entry
  .global _start
_start:
  call main
.dead_loop:
  jmp .dead_loop
