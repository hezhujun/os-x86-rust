MEMORY_SIZE := 16M
SYSTEM_IMG := hd60M.img
BOCHS_HOME := ~/package/bochs
BOCHS_CONFIG_FILE := bochsrc

SRC_PATH := src

QEMU_ARGS := -machine pc \
			 -smp 1 \
			 -m $(MEMORY_SIZE) \
			 -drive format=raw,file=$(SYSTEM_IMG)

$(SYSTEM_IMG):
	dd if=/dev/zero of=$(SYSTEM_IMG) bs=1M count=512

mbr.bin: $(SRC_PATH)/boot/mbr.s
	nasm -f bin $(SRC_PATH)/boot/mbr.s -o mbr.bin

loader.bin: $(SRC_PATH)/boot/loader.s
	nasm $(SRC_PATH)/boot/loader.s -o loader.bin

env:
	cargo install cargo-binutils

kernel: env
	cargo build

kernel.bin: kernel
	~/.cargo/bin/rust-objcopy target/x86-unknown-bare-metal/debug/os --binary-architecture=i386 -O binary $@

build: $(SYSTEM_IMG) mbr.bin loader.bin kernel.bin
	python3 check_kernel_size.py
	dd if=mbr.bin of=$(SYSTEM_IMG) bs=512 count=1 conv=notrunc
	dd if=loader.bin of=$(SYSTEM_IMG) bs=512 count=4 seek=1 conv=notrunc
	dd if=kernel.bin of=$(SYSTEM_IMG) bs=512 count=16384 seek=5 conv=notrunc

run: build
	qemu-system-i386 $(QEMU_ARGS)

run_with_graphic: build
	qemu-system-i386 -monitor stdio $(QEMU_ARGS)

debug: system_img
	@tmux new-session -d \
		"qemu-system-i386 $(QEMU_ARGS) -nographic -s -S" && \
		tmux split-window -h "lldb --arch x86 -S lldb_debug.txt" && \
		tmux -2 attach-session -d

gdbserver: build
	qemu-system-i386 $(QEMU_ARGS) -s -S

lldbclient:
	lldb --arch i386 -S lldb_debug.txt

gdbclient:
	gdb -ex 'file $(SYSTEM_IMG)' -ex 'set arch i386:x86' -ex 'target remote localhost:1234'

clean:
	rm mbr.bin
	rm loader.bin
	rm kernel.bin
	rm $(SYSTEM_IMG)
	cargo clean

bochs_disk_file:
	$(BOCHS_HOME)/bin/bximage -func=create -hd=60M -imgmode=flat -q $(SYSTEM_IMG)

bochs_run: build
	$(BOCHS_HOME)/bin/bochs -f $(BOCHS_CONFIG_FILE) -q

disassemble: build
	rust-objdump --x86-asm-syntax=intel -d target/x86-unknown-bare-metal/debug/os > os.s
	