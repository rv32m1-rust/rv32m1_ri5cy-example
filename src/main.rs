#![no_std]
#![no_main]
extern crate panic_halt;
mod delay;

const BOARD_LED_GPIO_PIN: u32 = 24;

#[riscv_rt::entry]
fn main() -> ! {
    use delay::{delay, DELAY_CYCLES};
    let peripherals = rv32m1_ri5cy_pac::Peripherals::take().unwrap();
    peripherals.PCC0.pcc_porta.write(|w| { w.cgc().cgc_1() });
    peripherals.PORTA.pcr24.modify(|r, w| unsafe {
        w.bits(r.bits()).mux().mux_1()
    });
    peripherals.GPIOA.pddr.modify(|r, w| unsafe {
        w.pdd().bits(r.pdd().bits() | (1u32 << BOARD_LED_GPIO_PIN))
    });
    loop {
        peripherals.GPIOA.psor.write(
            |w| unsafe { w.ptso().bits(1u32 << BOARD_LED_GPIO_PIN) }
        );
        delay(DELAY_CYCLES * 3); // make logic 1 output lasts longer
        peripherals.GPIOA.pcor.write(
            |w| unsafe { w.ptco().bits(1u32 << BOARD_LED_GPIO_PIN) }
        );
        delay(DELAY_CYCLES);
    }
}
