#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let mut _a: u32 = 0;
    loop {
        _a += 1;
    }
}
