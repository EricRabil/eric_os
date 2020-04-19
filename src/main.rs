// disable stdlib to make this a freestanding library
#![no_std]
// disable standard entry point in rust program, i am god.
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
// -> ! implies that a function never returns.
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
// again, ! implies this function never returns. that makes sense because this is a KERNEL, bitch.
pub extern "C" fn _start() -> ! {
    loop {}
}