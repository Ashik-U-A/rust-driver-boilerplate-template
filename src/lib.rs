#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {
    
}

#[no_mangle]
pub extern "system" fn driver_entry() -> u32 {
    0
}