//! # EspressOS Kernel Binary
//! 
//! This is the main binary entry point for the EspressOS kernel.

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use espress_os::{vga_println};

/// Panic handler for the kernel.
/// 
/// This function is called when a panic occurs in the kernel. Since we're running
/// in a bare-metal environment without an operating system, we cannot unwind the
/// stack or perform complex error handling. Instead, we enter an infinite loop
/// to halt the system.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Kernel entry point.
/// 
/// This function serves as the entry point for the EspressOS kernel. The bootloader
/// transfers control to this function after setting up the basic execution environment.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_println!("Hello World!");
    vga_println!("Welcome to EspressOS!");

    loop {}
}
