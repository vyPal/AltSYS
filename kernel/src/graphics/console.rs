use core::fmt;

use font8x8::UnicodeFonts;
use spin::{Mutex, Once};

use super::vga_buffer::VGABuffer;

const FONT_SIZE: (usize, usize) = (8, 8);

pub static CONSOLE: Once<Mutex<Console>> = Once::new();

pub fn init_console(buf: VGABuffer) {
    CONSOLE.call_once(|| Mutex::new(Console::new(buf)));
}

pub struct Console {
    buf: VGABuffer,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl Console {
    pub fn new(buf: VGABuffer) -> Self {
        let w = buf.width;
        let h = buf.height;
        Console {
            buf,
            width: w,
            height: h,
            x: 0,
            y: 0,
        }
    }

    pub fn nl(&mut self) {
        self.y += FONT_SIZE.1;
        self.x = 0;
    }

    pub fn cr(&mut self) {
        self.x = 0;
    }

    pub fn clear(&mut self) {
        self.buf.clear();
        self.x = 0;
        self.y = 0;
    }

    pub fn scroll(&mut self) {
        self.buf.shift_up(FONT_SIZE.1);
        self.y -= FONT_SIZE.1;
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.nl(),
            '\r' => self.cr(),
            c => {
                if self.x >= self.width {
                    self.nl();
                }
                while self.y >= (self.height - 8) {
                    self.scroll();
                }
                let rendered = font8x8::BASIC_FONTS
                    .get(c)
                    .expect("character not found in basic font");
                self.write_rendered_char(rendered);
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: [u8; 8]) {
        for (y, byte) in rendered_char.iter().enumerate() {
            for (x, bit) in (0..8).enumerate() {
                let on = *byte & (1 << bit) != 0;
                let c = if on { 255 } else { 0 };
                self.buf.set_pixel(self.x + x, self.y + y, super::Color::GRAYSCALE(c as usize));
            }
        }
        self.x += 8;
    }

    fn write_string(&mut self, s: &str) {
        for char in s.chars() {
            self.write_char(char);
        }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::graphics::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    CONSOLE
        .get()
        .unwrap()
        .lock()
        .write_fmt(args)
        .unwrap();
}