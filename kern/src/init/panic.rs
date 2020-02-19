use core::panic::PanicInfo;
use crate::console::kprintln;
use crate::console::kprint;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kprintln!();
    kprintln!();
    kprintln!("     .-''''^'''^''^'''^''-.");
    kprintln!("    (//\\\\//\\\\//\\\\//\\\\//\\\\//)");
    kprintln!("     ~\\\\^^^^^^^^^^^^^^^^//~");
    kprintln!("       `================`");
    kprintln!();
    kprintln!("      Lamb sauce is missing");
    kprintln!();
    kprintln!("------------- PANIC -------------");
    kprintln!();
    kprintln!("FILE: {}", _info.location().expect("").file());
    kprintln!("LINE: {}", _info.location().expect("").line());
    kprintln!("COL: {}", _info.location().expect("").column());
    kprintln!();
    kprintln!("{:?}", _info.message());

    loop {}
}
