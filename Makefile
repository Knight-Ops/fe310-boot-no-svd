# Default to the Hifive
ifndef BSP
	BSP = bsp_hifive
endif

# BSP-specific arguments
ifeq ($(BSP),bsp_hifive)
	TARGET = riscv32imac-unknown-none-elf
	OUTPUT = bootloader
	LINKER_FILE = src/bsp/hifive/link.ld
	RUSTC_MISC_ARGS = -C target-feature=-f -C target-cpu=sifive-e31
endif
# ifeq ($(BSP),bsp_rpi4)
# 	TARGET = aarch64-unknown-none
# 	OUTPUT = kernel8.img
# 	QEMU_BINARY = qemu-system-aarch64
# 	QEMU_MACHINE_TYPE = raspi3
# 	QEMU_MISC_ARGS = -serial stdio
# 	LINKER_FILE = src/bsp/rpi4/link.ld
# 	RUSTC_MISC_ARGS = -C target-feature=-fp-armv8 -C target-cpu=cortex-a72
# endif

SOURCES = $(wildcard **/*.rs) $(wildcard **/*.S) $(wildcard **/*.ld)

BUILD_CMD = cargo rustc \
		--target=$(TARGET) \
		--features $(BSP) \
		--release \
		-Z build-std=core \
		-- \
		-C link-args=-T$(LINKER_FILE) \
		$(RUSTC_MISC_ARGS)

CARGO_OUTPUT = target/$(TARGET)/release/riscv-rtos

OBJCOPY_CMD = llvm-objcopy.exe \
		--strip-all \
		-O binary


.PHONY: all clippy clean objdump nm expand server_riscv program_riscv

all: clean $(CARGO_OUTPUT)

$(CARGO_OUTPUT): $(SOURCES)
	$(BUILD_CMD)

doc:
	cargo doc --target=$(TARGET) --features $(BSP) --document-private-items
	xdg-open target/$(TARGET)/doc/riscv-rtos/index.html

clippy:
	cargo clippy --target=$(TARGET) --features $(BSP)

clean:
	rm -f bootloader
	cargo clean

expand:
	cargo expand --target=$(TARGET) \
		--features $(BSP) \
		--release \
		-Z build-std=core

objdump:
	llvm-objdump.exe -disassemble -print-imm-hex $(CARGO_OUTPUT)

nm:
	cargo nm --target $(TARGET) -- riscv-rtos | sort

server_riscv:
	"C:\Program Files (x86)\SEGGER\JLink\JLinkGDBServer.exe" -device FE310 -if JTAG -speed 4000 -port 3333 -nolocalhostonly

program_riscv:
	cp $(CARGO_OUTPUT) ./bootloader
	~/riscv64-unknown-elf-gcc-8.3.0-2020.04.0-x86_64-linux-ubuntu14/bin/riscv64-unknown-elf-gdb -x gdb_init