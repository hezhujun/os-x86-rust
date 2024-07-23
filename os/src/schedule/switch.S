  .section .text
  .global __switch
__switch:
  # __switch(
  #  current_task_cx_ptr: *mut TaskContext,
  #  next_task_cx_ptr: *const TaskContext,
  #)
  push ebp
  mov ebp, esp
  push ebx
  mov ebx, [ebp+8]
  mov eax, [ebp+4]
  mov [ebx], eax  # return_address
  add esp, 8
  mov [ebx+4], esp     # esp
  xor eax, eax
  mov ax, es
  mov [ebx+8], eax
  mov ax, ds
  mov [ebx+12], eax
  mov ax, fs
  mov [ebx+16], eax
  mov ax, gs
  mov [ebx+20], eax
  mov eax, [ebp-4]
  mov [ebx+24], eax
  mov eax, [ebp]
  mov [ebx+28], eax
  mov [ebx+32], esi
  mov [ebx+36], edi

  # next task
  mov ebx, [ebp+12]
  # change to next task stack
  mov esp, [ebx+4]
  mov ebp, esp
  mov eax, [ebx]
  mov [ebp], eax  # return_address
  mov eax, [ebx+8]
  mov es, ax
  mov eax, [ebx+12]
  mov ds, ax
  mov eax, [ebx+16]
  mov fs, ax
  mov eax, [ebx+20]
  mov gs, ax
  mov edi, [ebx+36]
  mov esi, [ebx+32]
  mov eax, [ebx+24]  # ebx
  push eax
  mov eax, [ebx+28]  # ebp
  push eax
  pop ebp
  pop ebx

  ret
  