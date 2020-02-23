#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(optin_builtin_traits)]
#![feature(raw_vec_internals)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(panic_info_message)]

#[cfg(not(test))]
mod init;

extern crate alloc;

pub mod allocator;
pub mod console;
pub mod fs;
pub mod mutex;
pub mod shell;

use console::kprintln;
use console::kprint;

use allocator::Allocator;
use fs::FileSystem;

#[cfg_attr(not(test), global_allocator)]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();
pub static FILESYSTEM: FileSystem = FileSystem::uninitialized();
use pi::atags::Atags;

fn kmain() -> ! {
    // unsafe {
    //     ALLOCATOR.initialize();
    //     FILESYSTEM.initialize();
    // }

    let mut a = Atags::get();
    loop {
        match a.next() {
            Some(b) => kprintln!("{:#?}", b),
            None => break,
        }
        
    }

    kprintln!("Welcome to cs3210!");
    shell::shell("> ");
}
