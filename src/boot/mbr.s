; load bootloader from sector 2-5

section mbr vstart=0x7c00
load_loader:
  mov ah, 00h
  mov dx, 0x80
  int 13h
  mov ah, 02h             ; load sector
  mov al, 4               ; load 2-5 sectors
  mov ch, 0               ; in track 0
  mov cl, 2               ; from sector 2
  mov dh, 0
  mov dl, 0x80
  mov bx, 0x9000
  mov es, bx
  mov bx, 0x0               ; save data in 
  int 13h
  jc load_loader
  jmp 0x9000:0x408

times 510-($-$$) db 0
db 0x55, 0xaa
