// interfaces
use crate::indicator::Indicator;

//
use core::cell::RefCell;
use core::fmt::{self, Write};
use core::time::Duration;

use stm32g0::stm32g030::CorePeripherals;
use stm32g0::stm32g030::Interrupt;
use stm32g0::stm32g030::Peripherals;
use stm32g0::stm32g030::NVIC;

use cortex_m::interrupt::{free, Mutex};

pub static G_PERIPHERAL: Mutex<RefCell<Option<stm32g0::stm32g030::Peripherals>>> =
    Mutex::new(RefCell::new(None));

pub fn init_g_peripheral(perip: Peripherals) {
    free(|cs| G_PERIPHERAL.borrow(cs).replace(Some(perip)));
}

// pub fn clock_init(perip: &Peripherals, core_perip: &mut CorePeripherals) {
//     perip.RCC.cr.modify(|_, w| w.hsebyp().bypassed());
//     perip.RCC.cr.modify(|_, w| w.hseon().on());
//     while perip.RCC.cr.read().hserdy().is_not_ready() {}

//     // Disable the PLL
//     perip.RCC.cr.modify(|_, w| w.pllon().off());
//     // Wait until PLL is fully stopped
//     while perip.RCC.cr.read().pllrdy().is_ready() {}
//     perip.RCC.pllcfgr.modify(|_, w| w.pllsrc().hse());
//     perip.RCC.pllcfgr.modify(|_, w| w.pllm().div12());
//     // perip.RCC.pllcfgr.modify(|_, w| w.plln().div85());
//     perip.RCC.pllcfgr.modify(|_, w| w.plln().div70());
//     perip.RCC.pllcfgr.modify(|_, w| w.pllr().div2());

//     perip.RCC.cr.modify(|_, w| w.pllon().on());
//     while perip.RCC.cr.read().pllrdy().is_not_ready() {}
//     perip.RCC.pllcfgr.modify(|_, w| w.pllren().set_bit());

//     perip
//         .FLASH
//         .acr
//         .modify(|_, w| unsafe { w.latency().bits(4) });
//     while perip.FLASH.acr.read().latency().bits() != 4 {
//         defmt::info!("latency bit: {}", perip.FLASH.acr.read().latency().bits());
//     }

//     perip.RCC.cfgr.modify(|_, w| w.sw().pll());
//     // perip.RCC.cfgr.modify(|_, w| w.sw().hse());
//     defmt::debug!("sw bit: {}", perip.RCC.cfgr.read().sw().bits());
//     while !perip.RCC.cfgr.read().sw().is_pll() {}
//     while !perip.RCC.cfgr.read().sws().is_pll() {
//         defmt::info!("sw bit: {}", perip.RCC.cfgr.read().sw().bits());
//         defmt::info!("sws bit: {}", perip.RCC.cfgr.read().sws().bits());
//     }
//     // while !perip.RCC.cfgr.read().sws().is_hse() {}

//     perip.RCC.apb1enr1.modify(|_, w| w.tim3en().enabled());
//     perip.RCC.apb1enr1.modify(|_, w| w.tim6en().enabled());

//     let tim3 = &perip.TIM3;
//     // tim3.psc.modify(|_, w| unsafe { w.bits(170 - 1) });
//     tim3.psc.modify(|_, w| unsafe { w.bits(15_000 - 1) }); // 14_000?
//                                                            // tim3.arr.modify(|_, w| unsafe { w.bits(1000 - 1) });    // 1kHz
//     tim3.dier.modify(|_, w| w.uie().set_bit());
//     tim3.cr1.modify(|_, w| w.cen().set_bit());

//     let tim6 = &perip.TIM6;
//     tim6.psc.modify(|_, w| unsafe { w.bits(15_000 - 1) }); // 14_000?
//     tim6.arr.modify(|_, w| unsafe { w.bits(1000 - 1) }); // 1kHz
//     tim6.dier.modify(|_, w| w.uie().set_bit());
//     tim6.cr2.modify(|_, w| unsafe { w.mms().bits(0b010) });

//     // 割り込み設定
//     unsafe {
//         core_perip.NVIC.set_priority(Interrupt::USART1, 0);
//         NVIC::unmask(Interrupt::USART1);
//     }

// }


pub struct Led0 {}

impl Indicator for Led0 {
    fn on(&self) {
        // free(|cs| match G_PERIPHERAL.borrow(cs).borrow().as_ref() {
        //     None => (),
        //     Some(perip) => {
        //         let gpioc = &perip.GPIOC;
        //         gpioc.bsrr.write(|w| w.bs13().set());
        //     }
        // });
    }
    fn off(&self) {
        // free(|cs| match G_PERIPHERAL.borrow(cs).borrow().as_ref() {
        //     None => (),
        //     Some(perip) => {
        //         let gpioc = &perip.GPIOC;
        //         gpioc.bsrr.write(|w| w.br13().reset());
        //     }
        // });
    }
    fn toggle(&self) {
        // free(|cs| match G_PERIPHERAL.borrow(cs).borrow().as_ref() {
        //     None => (),
        //     Some(perip) => {
        //         let gpioc = &perip.GPIOC;
        //         if gpioc.odr.read().odr13().is_low() {
        //             gpioc.bsrr.write(|w| w.bs13().set());
        //         } else {
        //             gpioc.bsrr.write(|w| w.br13().reset());
        //         }
        //     }
        // });
    }
}

// impl Led0 {
//     pub fn new() -> Self {
//         Self {}
//     }

//     pub fn init(&self) {
//         free(|cs| {
//             match G_PERIPHERAL.borrow(cs).borrow().as_ref() {
//                 None => (),
//                 Some(perip) => {
//                     // GPIOポートの電源投入(クロックの有効化)
//                     perip.RCC.ahb2enr.modify(|_, w| w.gpiocen().set_bit());

//                     // gpioモード変更
//                     let gpioc = &perip.GPIOC;
//                     gpioc.moder.modify(|_, w| w.moder13().output());
//                 }
//             }
//         });
//     }
// }

pub struct Led1 {}

impl Indicator for Led1 {
    fn on(&self) {
        // free(|cs| match G_PERIPHERAL.borrow(cs).borrow().as_ref() {
        //     None => (),
        //     Some(perip) => {
        //         let gpioc = &perip.GPIOC;
        //         gpioc.bsrr.write(|w| w.bs14().set());
        //     }
        // });
    }
    fn off(&self) {
        // free(|cs| match G_PERIPHERAL.borrow(cs).borrow().as_ref() {
        //     None => (),
        //     Some(perip) => {
        //         let gpioc = &perip.GPIOC;
        //         gpioc.bsrr.write(|w| w.br14().reset());
        //     }
        // });
    }
    fn toggle(&self) {
        // free(|cs| match G_PERIPHERAL.borrow(cs).borrow().as_ref() {
        //     None => (),
        //     Some(perip) => {
        //         let gpioc = &perip.GPIOC;
        //         if gpioc.odr.read().odr14().is_low() {
        //             gpioc.bsrr.write(|w| w.bs14().set());
        //         } else {
        //             gpioc.bsrr.write(|w| w.br14().reset());
        //         }
        //     }
        // });
    }
}

// impl Led1 {
//     pub fn new() -> Self {
//         Self {}
//     }

//     pub fn init(&self) {
//         free(|cs| {
//             match G_PERIPHERAL.borrow(cs).borrow().as_ref() {
//                 None => (),
//                 Some(perip) => {
//                     // GPIOポートの電源投入(クロックの有効化)
//                     perip.RCC.ahb2enr.modify(|_, w| w.gpiocen().set_bit());

//                     // gpioモード変更
//                     let gpioc = &perip.GPIOC;
//                     gpioc.moder.modify(|_, w| w.moder14().output());
//                 }
//             }
//         });
//     }
// }

