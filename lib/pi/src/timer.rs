use crate::common::IO_BASE;
use core::time::Duration;

use volatile::prelude::*;
use volatile::{ReadVolatile, Volatile};

/// The base address for the ARM system timer registers.
const TIMER_REG_BASE: usize = IO_BASE + 0x3000;

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    CS: Volatile<u32>,
    CLO: ReadVolatile<u32>,
    CHI: ReadVolatile<u32>,
    COMPARE: [Volatile<u32>; 4],
}

/// The Raspberry Pi ARM system timer.
pub struct Timer {
    registers: &'static mut Registers,
}

impl Timer {
    /// Returns a new instance of `Timer`.
    pub fn new() -> Timer {
        Timer {
            registers: unsafe { &mut *(TIMER_REG_BASE as *mut Registers) },
        }
    }

    /// Reads the system timer's counter and returns Duration.
    /// `CLO` and `CHI` together can represent the number of elapsed microseconds.
    pub fn read(&self) -> Duration {
        let usecs = (self.registers.CLO.read() as u64) + ((self.registers.CHI.read() as u64) << 32);

        Duration::from_micros(usecs)
    }
}

/// Returns current time.
pub fn current_time() -> Duration {
    Timer::new().read()
}

/// Spins until `t` duration have passed.
pub fn spin_sleep(t: Duration) {
    let timer = Timer::new();
    let start_time = timer.read();

    while t.checked_sub(timer.read() - start_time).is_some() {}
}
