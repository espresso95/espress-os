//! # EspressOS Kernel
//! 
//! A minimal operating system kernel written in Rust for the x86_64 architecture.
//! This module provides VGA text mode output functionality for displaying text
//! to the screen in a no_std environment.
//!
//! ## Features
//! - VGA text mode output with 16 colors
//! - Thread-safe text writer using spin locks
//! - Print macros for easy text output
//! - Automatic line wrapping and scrolling

#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Entry point for the kernel.
/// 
/// This function is called by the bootloader and serves as the main entry point
/// for the EspressOS kernel. It initializes VGA output and displays welcome messages.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    vga_println!("Hello World{}", "!");
    vga_println!("Welcome to EspressOS!");

    loop {}
}

/// VGA text mode buffer dimensions.
/// Standard VGA text mode provides an 80x25 character display.
const BUFFER_HEIGHT: usize = 25;
/// Width of the VGA text buffer in characters.
const BUFFER_WIDTH: usize = 80;

/// Standard VGA color palette.
/// 
/// Represents the 16 standard VGA colors available in text mode.
/// Each color is represented by a 4-bit value (0-15) that can be used
/// for both foreground and background colors.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    /// Black (0x0)
    Black = 0,
    /// Blue (0x1)
    Blue = 1,
    /// Green (0x2)
    Green = 2,
    /// Cyan (0x3)
    Cyan = 3,
    /// Red (0x4)
    Red = 4,
    /// Magenta (0x5)
    Magenta = 5,
    /// Brown (0x6)
    Brown = 6,
    /// Light Gray (0x7)
    LightGray = 7,
    /// Dark Gray (0x8)
    DarkGray = 8,
    /// Light Blue (0x9)
    LightBlue = 9,
    /// Light Green (0xA)
    LightGreen = 10,
    /// Light Cyan (0xB)
    LightCyan = 11,
    /// Light Red (0xC)
    LightRed = 12,
    /// Pink (0xD)
    Pink = 13,
    /// Yellow (0xE)
    Yellow = 14,
    /// White (0xF)
    White = 15,
}

/// VGA color code combining foreground and background colors.
/// 
/// Represents a VGA color attribute byte that combines both foreground
/// and background colors. The format is:
/// - Bits 0-3: Foreground color (4 bits)
/// - Bits 4-7: Background color (4 bits)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    /// Creates a new color code from foreground and background colors.
    /// 
    /// # Arguments
    /// * `foreground` - The foreground text color
    /// * `background` - The background color behind the text
    /// 
    /// # Returns
    /// A `ColorCode` with the combined color information
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// A VGA screen character consisting of an ASCII character and color information.
/// 
/// Represents a single character cell in VGA text mode memory. Each character
/// occupies 2 bytes: one for the ASCII character and one for the color attributes.
/// This struct uses `#[repr(C)]` to ensure the memory layout matches VGA expectations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    /// The ASCII character to display (1 byte)
    ascii_character: u8,
    /// Color attributes for the character (1 byte)
    color_code: ColorCode,
}

/// VGA text mode buffer.
/// 
/// Represents the VGA text mode framebuffer as a 2D array of screen characters.
/// The buffer is mapped to VGA memory at address 0xb8000 and provides direct
/// access to what appears on screen. Uses `#[repr(transparent)]` to ensure
/// the memory layout matches the hardware expectations.
#[repr(transparent)]
struct Buffer {
    /// 2D array representing the screen: [row][column]
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// VGA text mode writer.
/// 
/// Provides functionality for writing text to the VGA text mode buffer.
/// Handles automatic line wrapping, scrolling, and maintains the current
/// cursor position. All output appears at the bottom line of the screen.
pub struct Writer {
    /// Current column position (0-based)
    column_position: usize,
    /// Current color code for new characters
    color_code: ColorCode,
    /// Reference to the VGA buffer in memory
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Writes a single byte to the VGA buffer.
    /// 
    /// Handles newline characters by advancing to the next line, and wraps
    /// to a new line automatically when reaching the end of the current line.
    /// Uses volatile writes to prevent compiler optimizations that might
    /// interfere with memory-mapped I/O.
    /// 
    /// # Arguments
    /// * `byte` - The ASCII byte to write to the screen
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                unsafe {
                    core::ptr::write_volatile(
                        &mut self.buffer.chars[row][col],
                        ScreenChar {
                            ascii_character: byte,
                            color_code,
                        },
                    );
                }
                self.column_position += 1;
            }
        }
    }

    /// Advances to a new line and scrolls the screen if necessary.
    /// 
    /// Moves all existing lines up by one position and clears the bottom line.
    /// This creates a scrolling effect when the screen is full. The cursor
    /// position is reset to the beginning of the line.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = unsafe { core::ptr::read_volatile(&self.buffer.chars[row][col]) };
                unsafe {
                    core::ptr::write_volatile(&mut self.buffer.chars[row - 1][col], character);
                }
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Clears a specific row by filling it with space characters.
    /// 
    /// Overwrites all characters in the specified row with spaces using
    /// the current color code. Uses volatile writes to ensure the operation
    /// is not optimized away by the compiler.
    /// 
    /// # Arguments
    /// * `row` - The row index to clear (0-based)
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            unsafe {
                core::ptr::write_volatile(&mut self.buffer.chars[row][col], blank);
            }
        }
    }

    /// Writes a string to the VGA buffer.
    /// 
    /// Iterates through each byte in the string and writes it to the screen.
    /// Only printable ASCII characters (0x20-0x7E) and newlines are supported.
    /// Unsupported characters are replaced with the ■ symbol (0xFE).
    /// 
    /// # Arguments
    /// * `s` - The string slice to write to the screen
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
}

/// Implementation of the `core::fmt::Write` trait for `Writer`.
/// 
/// This allows the `Writer` to be used with Rust's formatting macros
/// like `write!` and `writeln!`, enabling formatted output to the VGA buffer.
impl core::fmt::Write for Writer {
    /// Writes a string slice to the VGA buffer.
    /// 
    /// # Arguments
    /// * `s` - The string slice to write
    /// 
    /// # Returns
    /// Always returns `Ok(())` since VGA writes cannot fail
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static::lazy_static! {
    /// Global VGA writer instance.
    /// 
    /// A static `Writer` instance wrapped in a spin lock for thread-safe access.
    /// The writer is initialized to output yellow text on a black background,
    /// pointing to the VGA text mode buffer at physical address 0xb8000.
    /// 
    /// This global instance allows printing from anywhere in the kernel without
    /// needing to pass writer instances around.
    pub static ref WRITER: spin::Mutex<Writer> = spin::Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// Macro for printing to the VGA text buffer.
/// 
/// This macro provides a convenient way to print formatted text to the VGA buffer
/// using the global `WRITER` instance. It works similarly to the standard `print!`
/// macro but outputs to the VGA screen instead of stdout.
/// 
/// # Examples
/// ```
/// vga_print!("Hello, world!");
/// vga_print!("The answer is {}", 42);
/// ```
#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => ($crate::_vga_print(format_args!($($arg)*)));
}

/// Macro for printing to the VGA text buffer with a newline.
/// 
/// This macro works like `vga_print!` but automatically appends a newline character.
/// It's the VGA equivalent of the standard `println!` macro.
/// 
/// # Examples
/// ```
/// vga_println!("Hello, world!");
/// vga_println!("The answer is {}", 42);
/// vga_println!(); // Just prints a newline
/// ```
#[macro_export]
macro_rules! vga_println {
    () => ($crate::vga_print!("\n"));
    ($($arg:tt)*) => ($crate::vga_print!("{}\n", format_args!($($arg)*)));
}

/// Internal function for VGA printing.
/// 
/// This function is used internally by the `vga_print!` and `vga_println!` macros
/// to perform the actual printing to the VGA buffer. It locks the global writer
/// and uses the formatting infrastructure to write the provided arguments.
/// 
/// # Arguments
/// * `args` - Formatted arguments to print
/// 
/// # Note
/// This function is marked as `#[doc(hidden)]` because it's an implementation
/// detail and should not be called directly by user code.
#[doc(hidden)]
pub fn _vga_print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
