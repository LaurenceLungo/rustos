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

unsafe fn kmain() -> ! {
    // FIXME: STEP 1: Set GPIO Pin 16 as output.
    let mut led = Gpio::new(16).into_output();

    // FIXME: STEP 2: Continuously set and clear GPIO 16.
    loop {
        led.set();
        timer::spin_sleep(Duration::from_secs(2));
        led.clear();
        timer::spin_sleep(Duration::from_secs(1));
    }
}
