# RustSBI D1 平台支持包



## 二进制包下载
这是融合了rustsbi-D1.bin和test-kernel.bin的二进制文件
https://github.com/wangtao-creator/rustsbi-D1-BinaryFile


## 使用说明

### 编译并烧录rustsbi-D1.bin
路径是rustsbi-D1/rustsbi-D1/

$ cargo clean
$ cargo build
$ rust-objcopy target/riscv64imac-unknown-none-elf/debug/rustsbi-D1 --binary-architecture=riscv64 --strip-all -O binary target/riscv64imac-unknown-none-elf/debug/rustsbi-D1.bin
$ xfel ddr ddr3
$ xfel write 0x40000000 $(dir)rustsbi-D1.bin
$ xfel exec 0x40000000

### 编译test-kernel.bin
路径是rustsbi-D1/test-kernel/
$ cargo clean
$ cargo build
$ rust-objcopy target/riscv64imac-unknown-none-elf/debug/test-kernel --binary-architecture=riscv64 --strip-all -O binary target/riscv64imac-unknown-none-elf/debug/test-kernel.bin


### 融合rustsbi-D1.bin和test-kernel.bin
$ dd if=target/riscv64imac-unknown-none-elf/debug/rustsbi-D1.bin of=target/riscv64imac-unknown-none-elf/debug/test-kernel.bin bs=1 seek=2
生成的文件路径是target/riscv64imac-unknown-none-elf/debug/test-kernel.bin