#![feature(asm)]
#![feature(global_asm)]

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;
const MMIO_BASE: usize = 0x3F000000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

const RNG_CTRL: *mut u32 = (MMIO_BASE+0x00104000) as *mut u32;
const RNG_STATUS: *mut u32 = (MMIO_BASE+0x00104004) as *mut u32;
const RNG_DATA: *mut u64 = (MMIO_BASE+0x00104008) as *mut u64;
const RNG_INT_MASK: *mut u32 = (MMIO_BASE+0x00104010) as *mut u32;

use rand::Rng;
use rand_core::{RngCore, Error, impls};

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 6000) {
        unsafe { asm!("nop" :::: "volatile"); }
    }
}

// Ref. https://rust-random.github.io/rand/rand_core/trait.RngCore.html#example
#[derive(Default)]
struct RdRand;
impl RngCore for RdRand {
    fn next_u32(&mut self) -> u32 {
        unsafe { RNG_DATA.read_volatile() as u32 }
    }

    fn next_u64(&mut self) -> u64 {
        unsafe { RNG_DATA.read_volatile() }
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}

unsafe fn kmain() -> ! {
    GPIO_FSEL1.write_volatile(0b001 << 18);
    let mut x = 0;
    let mut rng: RdRand = Default::default();

    RNG_STATUS.write_volatile(0x40000);
    RNG_INT_MASK.write_volatile(RNG_INT_MASK.read_volatile()|1);
    RNG_CTRL.write_volatile(RNG_CTRL.read_volatile()|1);
    while (RNG_STATUS.read_volatile()>>24) == 0 {
        asm!("nop" :::: "volatile");
    }

    loop {
        GPIO_SET0.write_volatile(x << 16);
        GPIO_CLR0.write_volatile(!x << 16);
        spin_sleep_ms(rng.gen_range(0, 1000));
        x = !x;
    }
}
