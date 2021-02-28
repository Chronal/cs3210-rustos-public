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
use pi::timer;
use pi::gpio::Gpio;
use pi::gpio::Output;
use core::time::Duration; 
use core::fmt::Write; 

fn kmain() -> ! {

    let mut led = Gpio::new(16).into_output();
    init_flash(&mut led);

    shell::shell("> ");
}

fn init_flash(led: &mut Gpio<Output>) {
    for _ in 0..5 {
        led.set();
        pi::timer::spin_sleep(Duration::from_millis(100));
        led.clear();
        pi::timer::spin_sleep(Duration::from_millis(100));
    }
}
