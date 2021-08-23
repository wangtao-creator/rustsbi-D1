dir = target/riscv64imac-unknown-none-elf/debug/
all: clean
	cargo build
	rust-objcopy $(dir)rustsbi-D1 --binary-architecture=riscv64 --strip-all -O binary $(dir)rustsbi-D1.bin
	xfel ddr ddr3
	xfel write 0x40000000 $(dir)rustsbi-D1.bin
	xfel exec 0x40000000
clean:
	cargo clean