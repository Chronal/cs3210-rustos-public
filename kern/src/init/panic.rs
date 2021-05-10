use core::time::Duration;
use core::panic::PanicInfo;

use crate::console::kprintln;
use crate::timer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("---------- PANIC ----------");
    loop {
        kprintln!("AAAAAAAAHHHHHHHHHHHHHHHHHHHHHHHHHHHHH");
        kprintln!("{}", info);
        timer::spin_sleep(Duration::new(1,0));
    }
}
