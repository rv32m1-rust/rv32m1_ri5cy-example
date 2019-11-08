#![no_std]
#![no_main]
extern crate panic_halt;

use rv32m1_ri5cy_hal as hal;
use hal::prelude::*;
use hal::pac as pac;

mod delay;

fn main() -> Result<(), core::convert::Infallible> {
    #![inline(always)]

    let peripherals = pac::Peripherals::take().unwrap();
    peripherals.PCC0.pcc_porta.write(|w| { w.cgc().cgc_1() });
    peripherals.PORTA.pcr24.modify(|r, w| unsafe {
        w.bits(r.bits()).mux().mux_1()
    });

    use rv32m1_ri5cy_hal::gpio::GpioExt;
    let mut gpioa = peripherals.GPIOA.split();
    loop {
        use delay::{delay, DELAY_CYCLES};
        gpioa.p24.set_high()?;
        delay(DELAY_CYCLES * 3); // make logic 1 output lasts longer
        gpioa.p24.set_low()?;
        delay(DELAY_CYCLES);
    }
}

#[riscv_rt::entry]
fn entry_point() -> ! {
    main().unwrap();
    unreachable!();
}
