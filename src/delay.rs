extern "C" { fn __nop(); }

pub fn delay(cycles: u32) {
    #![inline(always)]
    for _i in 0..cycles {
        unsafe { __nop(); }
    }
}

const fn delay_cycles_fn() -> u32 {
    #[cfg(debug_assertions)] return 10000;
    #[cfg(not(debug_assertions))] return 1000000;
}

pub const DELAY_CYCLES: u32 = delay_cycles_fn();
