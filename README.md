# ri5cy_example

> Example project for RI5CY core of RV32M1 SoC

⚠ This project is under construction. ⚠

## Getting Started

### Prerequisites

This project requires Rust with `riscv32imc-unknown-none-elf` target. Install the target to your existing Rust toolchain with:

```
rustup target add riscv32imc-unknown-none-elf
```

This project is tested on Rust 1.37+, but may also work with older Rust versions. Support both GNU and MSVC toolchains on Windows. 

Running requires an OpenOCD provided by your hardware platform vendor. 

### Building

```
cargo build
```

Compiled ELF file can be found at: `target/riscv32imc-unknown-none-elf/debug/rv32m1_ri5cy_example`.

### Connecting to Target

We use VEGAboard from OpenISA for the following steps. (steps may vary if you have a different platform)

Download and install the RISC-V SDK and toolchain from [OpenISA Downloads page](https://open-isa.org/downloads/). Connect a J-Link external debugger to VEGAboard, and launch OpenOCD with:

```
C:\Vega\Toolchain_Windows\openocd\bin\openocd.exe -f C:\Vega\rv32m1_sdk_riscv\boards\rv32m1_vega\rv32m1_ri5cy.cfg
```

The following message will be shown on the console if the connection is up successfully:

```
Info : Listening on port 3333 for gdb connections
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
```

OpenOCD will expose multiple ports for interacting with the target.

### Programming with OpenOCD

Connect to `localhost:4444` with any preferred telnet client (like PuTTY for Windows). Use following commands to program and run the target:

```
program target/riscv32imc-unknown-none-elf/debug/rv32m1_ri5cy_example
reset
resume
```

### Debugging with GDB

Open the binary with GDB and connect to the remote targed provided by OpenOCD with:

```
C:\Vega\Toolchain_Windows\riscv32-unknown-elf-gcc\bin\riscv32-unknown-elf-gdb.exe target\riscv32imc-unknown-none-elf\debug\rv32m1_ri5cy_example --eval-command="target remote localhost:3333"
```

GDB can also program the target with `load` command.

## Contributing

Issues and pull requests are welcome, always. 

## License

Copyright (c) 2019 SHA Miao

This project is licensed under either of

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))
* [MIT License](http://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.
