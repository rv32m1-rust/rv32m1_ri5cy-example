
# ri5cy_example

> Example project for RI5CY core of RV32M1 SoC

## Prerequisites

### Rust Compiler

Rust 1.37+ with `riscv32imc-unknown-none-elf` target is required.  (note: may also work with older Rust versions)

Install the target to your existing Rust toolchain with:

```
rustup target add riscv32imc-unknown-none-elf
```

Both GNU and MSVC toolchains work for Windows users.

### Hardware and Toolchain

An RV32M1 development board with JTAG is required. This project is tested on:

 - VEGAboard from [OpenISA](https://open-isa.org/)
 - VEGA-Lite from [OpenISA China](https://open-isa.cn/)

An installation of RISC-V SDK and toolchain (including GDB and OpenOCD) from [OpenISA Downloads page](https://open-isa.org/downloads/) is required. 

### Hardware Settings

RV32M1 must be configured to boot from RI5CY core. 

Both boards configure RV32M1 to boot from RI5CY core by default, so there is no need to consider unless you have once changed (or not sure about) current boot configuration.

See 'Boot Configuration' from [RV32M1-VEGA Quick Start Guide](https://github.com/open-isa-org/open-isa.org/blob/master/RV32M1_VEGA_Quick_Start_Guide.pdf) for instructions to select which core to boot.

## Getting Started

### Building

Build this project as any other Cargo crate:

```
cargo build
```

Compiled ELF file will be found at `target/riscv32imc-unknown-none-elf/debug/rv32m1_ri5cy-example`.

### Connecting to Target

We use VEGAboard for the following steps. Details may vary if you have a different platform. 

Remove J11 jumper and connect a J-Link external debugger to VEGAboard, then launch OpenOCD with:

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

### Programming and debugging with GDB

Open the binary with GDB and connect to the target via OpenOCD with:

```
C:\Vega\Toolchain_Windows\riscv32-unknown-elf-gcc\bin\riscv32-unknown-elf-gdb.exe target\riscv32imc-unknown-none-elf\debug\rv32m1_ri5cy-example --eval-command="target remote localhost:3333"
```

Use following commands to program and run the target:

```
load
continue
```

Send SIGINT (`Control+C`) to terminal to halt the target and start debugging.

 > It's also possible to use OpenOCD without GDB to program the target. Connect to `localhost:4444` with any preferred telnet client (like [PuTTY](https://www.chiark.greenend.org.uk/~sgtatham/putty/)), and use following commands to program and run:
 > 
 >     program target/riscv32imc-unknown-none-elf/debug/rv32m1_ri5cy-example
 >     reset

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
