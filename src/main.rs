#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::println;
use cortex_m_rt::entry;
use stm32f4xx_hal::pac::{self};
use {defmt_rtt as _, panic_probe as _};

fn delay() {
    for _ in 0..10000 {
        //do nothing
    }
}

#[entry]
fn main() -> ! {
    println!("Hello!");

    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC;
    let gpioa = dp.GPIOA;
    let tim  = dp.TIM3;

    let mut duty_cycle : u32 = 0;

    // clock
    rcc.ahb1enr.write(|w| w.gpioaen().set_bit());
    rcc.apb1enr.write(|w| w.tim3en().set_bit());

    // GPIOA6
    // alternate function mode
    gpioa.moder.write(|w| unsafe { w.moder6().bits(0b10) });
    // no pull
    gpioa.pupdr.write(|w| unsafe { w.pupdr6().bits(0b00)});
    // push pull output type
    gpioa.otyper.write(|w| unsafe { w.ot6().push_pull()});
    // set concrete alternate function
    // af02 is timer3 chanel1
    gpioa.afrl.write(|w| unsafe { w.afrl6().bits(0b10) });

    unsafe {
        tim.arr.write(|w| w.bits(0x8000)); // frequency
        tim.ccr1().write(|w| w.bits(0x0));// duty cycle
    }

    // clear enable to zero just to be sure
    tim.ccmr1_output().write(|w| w.oc1ce().clear_bit());
    //enable preload
    tim.ccmr1_output().write(|w| w.oc1pe().enabled());
    // set pwm mode 1
    tim.ccmr1_output().write(|w| w.oc1m().pwm_mode1());
    // enable output
    tim.ccer.write(|w| w.cc1e().set_bit());
    // enable auto-reload
    tim.cr1.write(|w| w.arpe().set_bit());
    // enable update generation - needed at first start
    tim.egr.write(|w| w.ug().set_bit());
    // start pwm
    tim.cr1.write(|w| w.cen().set_bit());

    loop {
        unsafe {
            tim.ccr1().write(|w| w.bits(duty_cycle));
        }
        duty_cycle = (duty_cycle + 0xff) % 0x8000;
        delay();
    }
}
