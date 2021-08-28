# RustSBI D1 平台支持包



## 二进制包下载
这是融合了rustsbi-D1.bin和test-kernel.bin的二进制文件
https://github.com/wangtao-creator/rustsbi-D1-BinaryFile


## 使用说明

### 编译并烧录rustsbi-D1.bin
```
路径是rustsbi-D1/rustsbi-D1/

$ cargo clean

$ cargo build

$ rust-objcopy target/riscv64imac-unknown-none-elf/debug/rustsbi-D1 --binary-architecture=riscv64 --strip-all -O binary target/riscv64imac-unknown-none-elf/debug/rustsbi-D1.bin

$ xfel ddr ddr3

$ xfel write 0x40000000 $(dir)rustsbi-D1.bin

$ xfel exec 0x40000000
```

#### 烧录结果

```
[rustsbi] RustSBI version 0.2.0-alpha.4

.______       __    __      _______.___________.  _______..______   __
|   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
|  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
|      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
|  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
| _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|

[rustsbi] Implementation: RustSBI-D1 Version 0.0.1
[rustsbi] misa: RV64ACDFIMSUVX
[rustsbi] mideleg: ssoft, stimer (0x22)
[rustsbi] medeleg: ima, bkpt, uecall (0x109)
[rustsbi] enter supervisor 0x40200000

<< Test-kernel: Hart id = 0, opaque = 0x40012ac0
>> Test-kernel: Testing base extension
<< Test-kernel: Base extension version: 1
<< Test-kernel: SBI specification version: 2
<< Test-kernel: SBI implementation Id: 4
<< Test-kernel: SBI implementation version: 200
<< Test-kernel: Device mvendorid: 5b7
<< Test-kernel: Device marchid: 0
<< Test-kernel: Device mimpid: 0
>> Test-kernel: Trigger illegal exception
<< Test-kernel: Value of scause: Exception(IllegalInstruction)
<< Test-kernel: Illegal exception delegate success
>> Test-kernel: Testing SBI instruction emulation
<< Test-kernel: Current time: 8a0
>> Test-kernel: Testing emulated virtual memory unit
<< Test-kernel: Code memory page test success
<< Test-kernel: Multi mapping page test success
>> Test-kernel: Testing catch page fault
>> Test-kernel: Wrong sign extension
>> Test-kernel: Read from invalid entry
>> Test-kernel: Unaligned huge page
>> Test-kernel: Non existing page
>> Test-kernel: Level zero page cannot have leaves
<< Test-kernel: SBI test SUCCESS, shutdown
[rustsbi] reset triggered! todo: shutdown all harts on D1; program halt. Type: 0, reason: 0
```

### 编译test-kernel.bin
```
路径是rustsbi-D1/test-kernel/

$ cargo clean

$ cargo build

$ rust-objcopy target/riscv64imac-unknown-none-elf/debug/test-kernel --binary-architecture=riscv64 --strip-all -O binary target/riscv64imac-unknown-none-elf/debug/test-kernel.bin
```

### 融合rustsbi-D1.bin和test-kernel.bin
```
$ dd if=target/riscv64imac-unknown-none-elf/debug/rustsbi-D1.bin of=target/riscv64imac-unknown-none-elf/debug/test-kernel.bin bs=1 seek=2

生成的文件路径是target/riscv64imac-unknown-none-elf/debug/test-kernel.bin
```