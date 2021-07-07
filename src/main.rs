// main.rs

#![no_std]  // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

#[no_mangle] // preserve the name of this function
pub extern "C" fn _start() -> ! {     // this function is the entry point, since the linker looks for a function
  loop {}                             // named `_start` by default
}

#[panic_handler] // Needed for, ya know, panicking
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}
