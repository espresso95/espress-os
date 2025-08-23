use wasm_bindgen::prelude::*;

// Import the `console.log` function from the browser's console API
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to print to the browser's console
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// VGA Color definitions (simplified version from espress-os)
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// VGA Text Mode Emulator for the web
#[wasm_bindgen]
pub struct VgaEmulator {
    buffer: Vec<Vec<(char, u8, u8)>>, // character, foreground, background
    width: usize,
    height: usize,
    cursor_x: usize,
    cursor_y: usize,
}

#[wasm_bindgen]
impl VgaEmulator {
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

    #[wasm_bindgen]
    pub fn get_char_at(&self, x: usize, y: usize) -> String {
        if x < self.width && y < self.height {
            let (ch, fg, bg) = self.buffer[y][x];
            format!("{}:{}:{}", ch, fg, bg)
        } else {
            " :15:0".to_string()
        }
    }

    #[wasm_bindgen]
    pub fn get_cursor_position(&self) -> String {
        format!("{}:{}", self.cursor_x, self.cursor_y)
    }
}

// Initialize function called when the WASM module loads
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("EspressOS WASM module loaded!");
}

// Demo function that simulates the OS welcome message
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