#![no_std]
#![no_main]

use indicator::Indicator;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use defmt_rtt as _;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;

use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;

use stm32g0::stm32g030::interrupt;
use stm32g0::stm32g030::Interrupt::EXTI0_1;

mod app;
mod indicator;
mod rs485_trigger_stm32g0;

static G_APP: Mutex<
    RefCell<Option<app::App<rs485_trigger_stm32g0::Led0, rs485_trigger_stm32g0::Led1>>>,
> = Mutex::new(RefCell::new(None));

#[interrupt]
fn EXTI0_1() {
    // SYSCFG_ITLINE5でステータスが見れそう

    defmt::error!("error from defmt");
    defmt::warn!("warn from defmt");

    rs485_trigger_stm32g0::clear_exti()

}



#[entry]
fn main() -> ! {
    use stm32g0::stm32g030;

    defmt::info!("Hello from STM32G0!");
    // stm32f401モジュールより、ペリフェラルの入り口となるオブジェクトを取得する。
    let perip = stm32g030::Peripherals::take().unwrap();
    let mut core_perip = stm32g030::CorePeripherals::take().unwrap();

    rs485_trigger_stm32g0::clock_init(&perip, &mut core_perip);
    rs485_trigger_stm32g0::exti_init(&perip, &mut core_perip);

    // init g peripheral
    rs485_trigger_stm32g0::init_g_peripheral(perip);

    let led0 = rs485_trigger_stm32g0::Led0::new();
    led0.init();
    led0.off();
    let led1 = rs485_trigger_stm32g0::Led1::new();
    led1.init();
    led1.off();


    let mut t = 0;
    free(
        |cs| match rs485_trigger_stm32g0::G_PERIPHERAL.borrow(cs).borrow().as_ref() {
            None => (),
            Some(perip) => {
                t = perip.TIM3.cnt.read().cnt_l().bits();
            }
        },
    );
    let mut prev = t;

    // rs485_trigger_stm32g0::set_swier();

    loop {
        free(
            |cs| match rs485_trigger_stm32g0::G_PERIPHERAL.borrow(cs).borrow().as_ref() {
                None => (),
                Some(perip) => {
                    t = perip.TIM3.cnt.read().cnt_l().bits();
                }
            },
        );

        if t.wrapping_sub(prev) > 500 {
            // defmt::info!("t: {}", t);
            // defmt::info!("prev: {}", prev);

            // led0.toggle();
            // led1.toggle();
            // rs485_trigger_stm32g0::check_status();

            prev = t;
        }
    }
}
