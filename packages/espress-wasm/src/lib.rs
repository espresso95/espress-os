//! # EspressOS WebAssembly Components
//! 
//! This module provides WebAssembly bindings for EspressOS components,
//! allowing the operating system functionality to be demonstrated in web browsers.
//! It includes a VGA text mode emulator that mimics the behavior of the real
//! hardware VGA buffer used in the bare-metal kernel.
//!
//! ## Features
//! - Web-based VGA text mode emulation
//! - Full 16-color VGA palette support
//! - Browser console integration for debugging
//! - OS boot sequence simulation

use wasm_bindgen::prelude::*;

// Import the `console.log` function from the browser's console API
#[wasm_bindgen]
extern "C" {
    /// JavaScript console.log function binding.
    /// 
    /// Allows printing debug messages to the browser's developer console
    /// from within WebAssembly code.
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Macro for logging to the browser console.
/// 
/// Provides a convenient way to print formatted debug messages to the browser's
/// console, similar to Rust's `println!` macro but targeting the web environment.
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// VGA color palette for WebAssembly emulation.
/// 
/// Represents the same 16-color VGA palette used in the bare-metal kernel,
/// enabling consistent color representation between the real hardware and
/// web-based emulation. Each color corresponds to a 4-bit value (0-15).
#[wasm_bindgen]
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

/// Web-based VGA text mode emulator.
/// 
/// Emulates the behavior of VGA text mode hardware in a web browser environment.
/// Maintains a 2D character buffer similar to the real VGA hardware and provides
/// the same functionality including scrolling, color support, and cursor tracking.
/// 
/// This allows demonstrating EspressOS features in a browser without requiring
/// actual hardware or emulators like QEMU.
#[wasm_bindgen]
pub struct VgaEmulator {
    /// 2D buffer storing characters with their color information: (char, fg_color, bg_color)
    buffer: Vec<Vec<(char, u8, u8)>>,
    /// Width of the emulated screen in characters
    width: usize,
    /// Height of the emulated screen in characters
    height: usize,
    /// Current cursor X position (column)
    cursor_x: usize,
    /// Current cursor Y position (row)
    cursor_y: usize,
}

#[wasm_bindgen]
impl VgaEmulator {
    /// Creates a new VGA emulator with the specified dimensions.
    /// 
    /// Initializes a text buffer with the given width and height, filled with
    /// space characters using white foreground on black background (the default).
    /// The cursor is positioned at the top-left corner (0, 0).
    /// 
    /// # Arguments
    /// * `width` - Screen width in characters
    /// * `height` - Screen height in characters
    /// 
    /// # Returns
    /// A new `VgaEmulator` instance ready for text output
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> VgaEmulator {
        console_log!("Initializing VGA Emulator {}x{}", width, height);
        
        let mut buffer = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push((' ', Color::White as u8, Color::Black as u8));
            }
            buffer.push(row);
        }
        
        VgaEmulator {
            buffer,
            width,
            height,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    /// Writes a string to the emulated VGA buffer.
    /// 
    /// Processes each character in the string, handling newlines by advancing
    /// to the next line and filtering out non-printable characters. Only ASCII
    /// characters in the printable range (32-126) are displayed.
    /// 
    /// # Arguments
    /// * `s` - The string to write to the screen
    /// * `fg_color` - Foreground color code (0-15)
    /// * `bg_color` - Background color code (0-15)
    #[wasm_bindgen]
    pub fn write_string(&mut self, s: &str, fg_color: u8, bg_color: u8) {
        for ch in s.chars() {
            if ch == '\n' {
                self.new_line();
            } else if ch as u32 >= 32 && (ch as u32) < 127 {
                self.write_char(ch, fg_color, bg_color);
            }
        }
    }

    /// Writes a single character to the current cursor position.
    /// 
    /// Places the character at the current cursor position with the specified colors,
    /// then advances the cursor to the next position. If the cursor reaches the end
    /// of a line, it automatically wraps to the next line.
    /// 
    /// # Arguments
    /// * `ch` - The character to write
    /// * `fg_color` - Foreground color code (0-15)
    /// * `bg_color` - Background color code (0-15)
    #[wasm_bindgen]
    pub fn write_char(&mut self, ch: char, fg_color: u8, bg_color: u8) {
        if self.cursor_x >= self.width {
            self.new_line();
        }
        
        if self.cursor_y < self.height {
            self.buffer[self.cursor_y][self.cursor_x] = (ch, fg_color, bg_color);
            self.cursor_x += 1;
        }
    }

    /// Advances to a new line and scrolls if necessary.
    /// 
    /// Moves the cursor to the beginning of the next line. If the cursor is already
    /// at the bottom of the screen, scrolls all content up by one line and clears
    /// the bottom line for new content.
    #[wasm_bindgen]
    pub fn new_line(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += 1;
        
        if self.cursor_y >= self.height {
            // Scroll up
            for y in 1..self.height {
                for x in 0..self.width {
                    self.buffer[y - 1][x] = self.buffer[y][x];
                }
            }
            
            // Clear last line
            for x in 0..self.width {
                self.buffer[self.height - 1][x] = (' ', Color::White as u8, Color::Black as u8);
            }
            
            self.cursor_y = self.height - 1;
        }
    }

    /// Clears the entire screen buffer.
    /// 
    /// Fills the entire screen with space characters using white foreground
    /// on black background, and resets the cursor to the top-left position.
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.buffer[y][x] = (' ', Color::White as u8, Color::Black as u8);
            }
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    /// Returns the entire buffer content as a string.
    /// 
    /// Converts the internal character buffer to a multi-line string representation,
    /// with each row on a separate line. Useful for debugging or displaying the
    /// buffer content in web interfaces.
    /// 
    /// # Returns
    /// A string containing all buffer content with newlines between rows
    #[wasm_bindgen]
    pub fn get_buffer_as_string(&self) -> String {
        let mut result = String::new();
        for row in &self.buffer {
            for &(ch, _fg, _bg) in row {
                result.push(ch);
            }
            result.push('\n');
        }
        result
    }

    /// Gets the character and color information at a specific position.
    /// 
    /// Returns a formatted string containing the character and its color codes
    /// at the specified coordinates. Useful for inspecting buffer contents.
    /// 
    /// # Arguments
    /// * `x` - Column position (0-based)
    /// * `y` - Row position (0-based)
    /// 
    /// # Returns
    /// A string in format "char:fg_color:bg_color", or " :15:0" for invalid positions
    #[wasm_bindgen]
    pub fn get_char_at(&self, x: usize, y: usize) -> String {
        if x < self.width && y < self.height {
            let (ch, fg, bg) = self.buffer[y][x];
            format!("{}:{}:{}", ch, fg, bg)
        } else {
            " :15:0".to_string()
        }
    }

    /// Gets the current cursor position.
    /// 
    /// Returns the current cursor coordinates as a formatted string.
    /// 
    /// # Returns
    /// A string in format "x:y" representing the cursor position
    #[wasm_bindgen]
    pub fn get_cursor_position(&self) -> String {
        format!("{}:{}", self.cursor_x, self.cursor_y)
    }
}

/// WASM module initialization function.
/// 
/// Called automatically when the WebAssembly module is loaded in the browser.
/// Logs a message to the console to confirm the module has been successfully
/// initialized and is ready for use.
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("EspressOS WASM module loaded!");
}

/// Simulates the EspressOS boot sequence for web demonstration.
/// 
/// Generates a string containing the same welcome messages that would appear
/// when the actual EspressOS kernel boots on real hardware. This provides
/// a web-friendly way to showcase the kernel's initial output.
/// 
/// # Returns
/// A multi-line string containing the simulated boot messages
#[wasm_bindgen]
pub fn simulate_os_boot() -> String {
    console_log!("Simulating OS boot sequence...");
    
    let mut output = String::new();
    output.push_str("Hello World!\n");
    output.push_str("Welcome to EspressOS!\n");
    output.push_str("\n");
    output.push_str("EspressOS WebAssembly Demo\n");
    output.push_str("==========================\n");
    output.push_str("This is a web version of the EspressOS kernel output.\n");
    output.push_str("The actual kernel runs on bare metal x86_64 hardware.\n");
    
    output
}