#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Multiboot header constants
const MULTIBOOT_HEADER_MAGIC: u32 = 0x1BADB002;
const MULTIBOOT_HEADER_FLAGS: u32 = 0;
const MULTIBOOT_HEADER_CHECKSUM: u32 = 0u32.wrapping_sub(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS);

/// Multiboot header
#[repr(C)]
pub struct MultibootHeader {
    magic: u32,
    flags: u32,
    checksum: u32,
}

/// Multiboot header in the .multiboot_header section
#[link_section = ".multiboot_header"]
#[no_mangle]
pub static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: MULTIBOOT_HEADER_MAGIC,
    flags: MULTIBOOT_HEADER_FLAGS,
    checksum: MULTIBOOT_HEADER_CHECKSUM,
};

/// VGA text buffer start address
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

/// Write a string to VGA text buffer
fn vga_print(text: &str) {
    let vga_buffer = VGA_BUFFER;
    
    for (i, byte) in text.bytes().enumerate() {
        unsafe {
            *vga_buffer.add(i * 2) = byte;
            *vga_buffer.add(i * 2 + 1) = 0x07; // Light gray on black
        }
    }
}

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_print("Hello from EspressOS!");
    
    loop {}
}
