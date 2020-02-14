#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(optin_builtin_traits)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;

pub mod console;
pub mod mutex;
pub mod shell;

use console::kprintln;

// FIXME: You need to add dependencies here to
// test your drivers (Phase 2). Add them as needed.
use core::time::Duration;
use pi::timer;
use pi::gpio::{Gpio, Function};
use pi::uart::*;

unsafe fn kmain() -> ! {
    // FIXME: STEP 1: Set GPIO Pin 16 as output.
    
    // let mut led = Gpio::new(16).into_output();
    // let mut led2 = Gpio::new(20).into_output();
    // let mut led3 = Gpio::new(21).into_output();
    let mut uart = MiniUart::new();

    // FIXME: STEP 2: Continuously set and clear GPIO 16.
    loop {
        // led.set();
        // timer::spin_sleep(Duration::from_millis(100));
        // led.clear();
        // timer::spin_sleep(Duration::from_millis(100));
        // led2.set();
        // led3.set();
        // timer::spin_sleep(Duration::from_millis(100));
        // led2.clear();
        // led3.clear();
        // timer::spin_sleep(Duration::from_millis(100));
        let b = uart.read_byte();
        uart.write_byte(b);
    }
}
