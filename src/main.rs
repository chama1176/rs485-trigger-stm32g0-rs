#![no_std]
#![no_main]

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

use stm32g0::stm32g030::{interrupt};

mod app;
mod indicator;
mod rs485_trigger_stm32g0;

static G_APP: Mutex<
    RefCell<
        Option<
            app::App<
                rs485_trigger_stm32g0::Led0,
                rs485_trigger_stm32g0::Led1,
            >,
        >,
    >,
> = Mutex::new(RefCell::new(None));


#[entry]
fn main() -> ! {
    use stm32g0::stm32g030;

    defmt::info!("Hello from STM32G0!");
    // stm32f401モジュールより、ペリフェラルの入り口となるオブジェクトを取得する。
    let perip = stm32g030::Peripherals::take().unwrap();
    let mut core_perip = stm32g030::CorePeripherals::take().unwrap();

    // rs485_trigger_stm32g0::clock_init(&perip, &mut core_perip);

    // // init g peripheral
    // rs485_trigger_stm32g0::init_g_peripheral(perip);

    // let led0 = rs485_trigger_stm32g0::Led0::new();
    // led0.init();
    // let led1 = rs485_trigger_stm32g0::Led1::new();
    // led1.init();

    // let clock: rs485_trigger_stm32g0::LocalClock = rs485_trigger_stm32g0::LocalClock::new();
    // clock.init();

    // let app = app::App::new(led0, led1);
    // free(|cs| G_APP.borrow(cs).replace(Some(app)));

    loop {
        // free(|cs| match G_APP.borrow(cs).borrow_mut().deref_mut() {
        //     None => (),
        //     Some(app) => {
        //         app.periodic_task();
        //         defmt::info!("parse uart task finished.");
        //     }
        // });
    }


    // let mut t = 0;
    // free(
    //     |cs| match rs485_trigger_stm32g0::G_PERIPHERAL.borrow(cs).borrow().as_ref() {
    //         None => (),
    //         Some(perip) => {
    //             t = perip.TIM3.cnt.read().cnt().bits();
    //         }
    //     },
    // );
    // let mut prev = t;
    // let mut cnt = 0;
    // let mut adc_data_fir: [u16; 4] = [0; 4];
    // loop {
    //     free(
    //         |cs| match rs485_trigger_stm32g0::G_PERIPHERAL.borrow(cs).borrow().as_ref() {
    //             None => (),
    //             Some(perip) => {
    //                 t = perip.TIM3.cnt.read().cnt().bits();
    //             }
    //         },
    //     );

    //     if t.wrapping_sub(prev) > 50 {
    //         for i in 0..4 {
    //             adc_data_fir[i] = (adc_data_fir[i] as f32 * 0.9 + adc_data[i] as f32 * 0.1) as u16;
    //         }
    //         cnt += 1;
    //         if cnt > 100 {
    //             free(|cs| match G_APP.borrow(cs).borrow_mut().deref_mut() {
    //                 None => (),
    //                 Some(app) => {
    //                     app.periodic_task();
    //                     app.read_imu_task();
    //                 }
    //             });

    //             defmt::error!("error from defmt");
    //             defmt::warn!("warn from defmt");
    //             defmt::info!("info from defmt");

    //             defmt::info!(
    //                 "{{\"FSR\":[{}, {}, {}, {}]}}",
    //                 adc_data_fir[3],
    //                 adc_data_fir[0],
    //                 adc_data_fir[1],
    //                 adc_data_fir[2]
    //             );

    //             // uart.write_str("hello ");
    //             // write!(uart, "{} + {} = {}\r\n", 2, 4, 2+4);
    //             // unsafe {
    //             //     write!(
    //             //         uart,
    //             //         "{{\"FSR\":[{}, {}, {}, {}]}}\r\n",
    //             //         adc_data_fir[3], adc_data_fir[0], adc_data_fir[1], adc_data_fir[2]
    //             //     );
    //             // }
    //             cnt = 0;
    //         }
    //         prev = t;
    //     }
    // }
}
