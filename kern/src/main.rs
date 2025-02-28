#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(optin_builtin_traits)]
#![feature(raw_vec_internals)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;

extern crate alloc;

pub mod allocator;
pub mod console;
pub mod fs;
pub mod mutex;
pub mod shell;

use pi::timer;
use pi::gpio::{Gpio, Output};
use console::kprintln;

use core::time::Duration; 
use core::fmt::Write; 
use allocator::Allocator;
use fs::FileSystem;

#[cfg_attr(not(test), global_allocator)]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();
//pub static FILESYSTEM: FileSystem = FileSystem::uninitialized();

fn kmain() -> ! {
    pi::timer::spin_sleep(Duration::from_secs(3));

    unsafe {
        //FILESYSTEM.initialize();
        ALLOCATOR.initialize();
    }

    shell::shell("> ");
}
