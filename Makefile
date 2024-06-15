MEMORY_SIZE := 16M
SYSTEM_IMG := os.img

SRC_PATH := src

QEMU_ARGS := -machine pc \
			 -smp 1 \
			 -m $(MEMORY_SIZE) \
			 -hda $(SYSTEM_IMG) \
			#  -nographic \
			#  -drive file=$(SYSTEM_IMG),if=none,format=raw,id=x0

system_img:
	echo "create $(SYSTEM_IMG)"
	dd if=/dev/zero of=$(SYSTEM_IMG) bs=1m count=512

mbr.bin: $(SRC_PATH)/boot/mbr.s
	nasm -f bin $(SRC_PATH)/boot/mbr.s -o mbr.bin

loader.bin: $(SRC_PATH)/boot/loader.s
	nasm $(SRC_PATH)/boot/loader.s -o loader.bin

kernel.bin:
	cargo build
	rust-objcopy target/x86-unknown-bare-metal/debug/os --binary-architecture=i386 -O binary $@

build: system_img mbr.bin loader.bin kernel.bin
	python3 check_kernel_size.py
	dd if=mbr.bin of=$(SYSTEM_IMG) bs=512 count=1 conv=notrunc
	dd if=loader.bin of=$(SYSTEM_IMG) bs=512 count=4 seek=1 conv=notrunc
	dd if=kernel.bin of=$(SYSTEM_IMG) bs=512 count=100 seek=5 conv=notrunc

run: build
	qemu-system-i386 $(QEMU_ARGS)

debug: system_img
	@tmux new-session -d \
		"qemu-system-i386 $(QEMU_ARGS) -s -S" && \
		tmux split-window -h "lldb --arch x86 -S lldb_debug.txt" && \
		tmux -2 attach-session -d

gdbserver: build
	qemu-system-i386 $(QEMU_ARGS) -s -S

gdbclient:
	lldb --arch i386 -S lldb_debug.txt

clean:
	rm mbr.bin
	rm loader.bin
	rm kernel.bin
	rm os.img
