/// VGA color palette enumeration.
/// 
/// Represents the 16 standard VGA colors available in text mode. Each color
/// is assigned a specific 4-bit value that corresponds to the VGA hardware
/// color palette. Colors can be used for both foreground (text) and background.
/// 
/// # Examples
/// 
/// ```rust
/// let red_text = Color::Red;
/// let blue_background = Color::Blue;
/// ```
/// 
/// # VGA Color Values
/// 
/// The enum values correspond to the standard VGA color palette:
/// - 0-7: Normal intensity colors
/// - 8-15: High intensity/bright colors
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    /// Black color (RGB: 0, 0, 0)
    Black = 0,
    /// Blue color (RGB: 0, 0, 170)
    Blue = 1,
    /// Green color (RGB: 0, 170, 0)
    Green = 2,
    /// Cyan color (RGB: 0, 170, 170)
    Cyan = 3,
    /// Red color (RGB: 170, 0, 0)
    Red = 4,
    /// Magenta color (RGB: 170, 0, 170)
    Magenta = 5,
    /// Brown color (RGB: 170, 85, 0)
    Brown = 6,
    /// Light Gray color (RGB: 170, 170, 170)
    LightGray = 7,
    /// Dark Gray color (RGB: 85, 85, 85)
    DarkGray = 8,
    /// Light Blue color (RGB: 85, 85, 255)
    LightBlue = 9,
    /// Light Green color (RGB: 85, 255, 85)
    LightGreen = 10,
    /// Light Cyan color (RGB: 85, 255, 255)
    LightCyan = 11,
    /// Light Red color (RGB: 255, 85, 85)
    LightRed = 12,
    /// Pink color (RGB: 255, 85, 255)
    Pink = 13,
    /// Yellow color (RGB: 255, 255, 85)
    Yellow = 14,
    /// White color (RGB: 255, 255, 255)
    White = 15,
}

/// VGA color code representation.
/// 
/// Combines foreground and background colors into a single byte value
/// that can be written to VGA memory. The format follows the VGA standard:
/// - Bits 0-3: Foreground color
/// - Bits 4-7: Background color
/// 
/// This struct provides a type-safe wrapper around the raw color byte
/// while maintaining the exact memory layout required by VGA hardware.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    /// Creates a new color code from foreground and background colors.
    /// 
    /// Combines the two colors into the VGA color format where the background
    /// color occupies the upper 4 bits and the foreground color occupies the
    /// lower 4 bits.
    /// 
    /// # Arguments
    /// 
    /// * `foreground` - The color for text characters
    /// * `background` - The color for the background behind text
    /// 
    /// # Returns
    /// 
    /// A `ColorCode` containing the combined color information
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let color = ColorCode::new(Color::White, Color::Blue);
    /// // Creates white text on blue background
    /// ```
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// A single character cell in the VGA text buffer.
/// 
/// Represents one character position on the screen, containing both the
/// ASCII character to display and the color information. This structure
/// matches the exact format expected by VGA hardware.
/// 
/// # Memory Layout
/// 
/// The struct uses `#[repr(C)]` to ensure the fields are laid out in memory
/// exactly as expected by the VGA hardware:
/// - Byte 0: ASCII character code
/// - Byte 1: Color code (foreground + background)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    /// The ASCII character to display (0-127)
    ascii_character: u8,
    /// Combined foreground and background color information
    color_code: ColorCode,
}

/// VGA text mode buffer representation.
/// 
/// Represents the entire VGA text buffer as a 2D array of characters.
/// The buffer is located at physical memory address 0xb8000 and provides
/// direct access to the screen contents.
/// 
/// # Memory Layout
/// 
/// Uses `#[repr(transparent)]` to ensure this struct has the same memory
/// layout as the underlying array, allowing safe casting from raw memory
/// addresses.
/// 
/// # Safety
/// 
/// Direct access to this buffer requires careful handling since it represents
/// actual hardware memory. All writes should use volatile operations to
/// prevent compiler optimizations from interfering with hardware updates.
#[repr(transparent)]
struct Buffer {
    /// 2D array representing screen characters: [row][column]
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// VGA text mode writer interface.
/// 
/// Provides a safe, high-level interface for writing text to the VGA buffer.
/// Handles cursor management, line wrapping, scrolling, and color formatting.
/// 
/// # Features
/// 
/// - Automatic line wrapping when text exceeds screen width
/// - Vertical scrolling when the screen is full
/// - Configurable text colors
/// - Support for newline characters
/// - Safe volatile memory operations
/// 
/// # Safety
/// 
/// The writer maintains a reference to the VGA buffer memory and uses volatile
/// operations to ensure all writes are visible to the hardware immediately.
pub struct Writer {
    /// Current column position of the cursor (0-79)
    column_position: usize,
    /// Current color code for new text
    color_code: ColorCode,
    /// Reference to the VGA text buffer in memory
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Writes a single byte to the VGA buffer.
    /// 
    /// Handles special characters (like newlines) and regular ASCII characters.
    /// Automatically wraps to the next line when the current line is full.
    /// 
    /// # Arguments
    /// 
    /// * `byte` - The byte to write to the screen
    /// 
    /// # Behavior
    /// 
    /// - `\n` (newline): Moves to the next line
    /// - Regular bytes: Written to the current cursor position
    /// - Automatic line wrapping when line is full
    /// 
    /// # Safety
    /// 
    /// Uses volatile writes to ensure the hardware sees all updates immediately
    /// and prevent compiler optimizations from reordering memory operations.
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
    /// This creates a scrolling effect when the screen is full of text.
    /// 
    /// # Behavior
    /// 
    /// 1. Copies each line to the line above it
    /// 2. Clears the bottom line
    /// 3. Resets the cursor to the start of the bottom line
    /// 
    /// # Safety
    /// 
    /// Uses volatile operations for all memory access to ensure proper
    /// hardware synchronization.
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

    /// Clears a single row of the screen buffer.
    /// 
    /// Fills the specified row with blank characters using the current color code.
    /// This is typically used to clear the bottom line after scrolling.
    /// 
    /// # Arguments
    /// 
    /// * `row` - The row index to clear (0-24)
    /// 
    /// # Safety
    /// 
    /// Uses volatile writes to ensure the hardware sees the changes immediately.
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
    /// Processes each byte of the string and writes it to the screen.
    /// Handles UTF-8 by replacing non-ASCII characters with a replacement character.
    /// 
    /// # Arguments
    /// 
    /// * `s` - The string to write to the screen
    /// 
    /// # Character Handling
    /// 
    /// - ASCII printable characters (0x20-0x7E): Written as-is
    /// - Newline character (`\n`): Triggers line advance
    /// - Other characters: Replaced with `■` (0xfe) symbol
    /// 
    /// This approach ensures compatibility with VGA text mode which only
    /// supports ASCII characters.
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

/// Implementation of the `Write` trait for formatted output.
/// 
/// This allows the `Writer` to be used with Rust's formatting macros
/// like `write!` and `writeln!`. The implementation delegates to the
/// `write_string` method to maintain consistency with the VGA output behavior.
/// 
/// # Returns
/// 
/// Always returns `Ok(())` since VGA output cannot fail in this context.
impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// Global VGA writer instance.
/// 
/// Provides thread-safe access to the VGA text buffer through a mutex-protected
/// writer instance. This static variable is initialized once and provides the
/// primary interface for kernel text output.
/// 
/// # Configuration
/// 
/// - **Colors**: Yellow text on black background
/// - **Buffer**: Points to VGA memory at address 0xb8000
/// - **Thread Safety**: Protected by a spin lock for concurrent access
/// 
/// # Usage
/// 
/// This writer is primarily accessed through the `vga_print!` and `vga_println!`
/// macros rather than direct access. The mutex ensures safe access even in
/// interrupt contexts.
/// 
/// # Safety
/// 
/// The buffer pointer is created by casting the VGA memory address (0xb8000)
/// to a mutable reference. This is safe because:
/// - VGA memory is always present on x86 systems
/// - The address is a standard hardware location
/// - We never deallocate or move this memory
lazy_static::lazy_static! {
    pub static ref WRITER: spin::Mutex<Writer> = spin::Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// Prints formatted text to the VGA buffer without a newline.
/// 
/// This macro provides `print!`-like functionality for the kernel's VGA output.
/// It supports all of Rust's standard formatting features including positional
/// arguments, formatting specifiers, and type conversions.
/// 
/// # Examples
/// 
/// ```rust
/// vga_print!("Hello");
/// vga_print!("Number: {}", 42);
/// vga_print!("Hex: 0x{:x}", 255);
/// ```
/// 
/// # Implementation
/// 
/// The macro converts the arguments into `format_args!` and passes them to
/// the internal `_vga_print` function for actual output.
#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => ($crate::_vga_print(format_args!($($arg)*)));
}

/// Prints formatted text to the VGA buffer with a newline.
/// 
/// This macro provides `println!`-like functionality for the kernel's VGA output.
/// It automatically appends a newline character after the formatted text.
/// 
/// # Examples
/// 
/// ```rust
/// vga_println!("Hello, World!");
/// vga_println!("System initialized: {}", true);
/// vga_println!(); // Print just a newline
/// ```
/// 
/// # Variants
/// 
/// - `vga_println!()`: Prints just a newline
/// - `vga_println!("text")`: Prints text followed by newline
/// - `vga_println!("format {}", args)`: Prints formatted text followed by newline
#[macro_export]
macro_rules! vga_println {
    () => ($crate::vga_print!("\n"));
    ($($arg:tt)*) => ($crate::vga_print!("{}\n", format_args!($($arg)*)));
}

/// Internal function for VGA text output.
/// 
/// This function provides the actual implementation for the `vga_print!` and
/// `vga_println!` macros. It acquires the global writer lock and performs
/// the formatted write operation.
/// 
/// # Arguments
/// 
/// * `args` - Formatted arguments created by `format_args!` macro
/// 
/// # Panics
/// 
/// Panics if the write operation fails, though this should never happen
/// in practice since VGA writes cannot fail.
/// 
/// # Thread Safety
/// 
/// Uses a spin lock to ensure thread-safe access to the global VGA writer.
/// The function will spin-wait if another thread is currently writing to
/// the VGA buffer.
/// 
/// # Note
/// 
/// This function is marked as `#[doc(hidden)]` because it's an internal
/// implementation detail not intended for direct use by kernel code.
#[doc(hidden)]
pub fn _vga_print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}