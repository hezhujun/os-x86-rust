APP_DIR := src/bin
TARGET_DIR := target/x86-unknown-bare-metal/release
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))

elf: $(APPS)
	@cargo build --release

build: elf

clean:
	@cargo clean

.PHONY: elf build clean
