use core::ptr::write_bytes;

use bootloader_api::info::FrameBuffer;
use spin::Mutex;

use super::Color;

pub struct VGABuffer {
    buffer: Mutex<&'static mut [u8]>,
    bytes_per_pixel: usize,
    pub width: usize,
    pub height: usize,
    length: usize,
}

impl VGABuffer {
    pub fn new(buf: &'static mut FrameBuffer) -> Self {
        let info = buf.info();
        VGABuffer {
            buffer: Mutex::new(buf.buffer_mut()),
            bytes_per_pixel: info.bytes_per_pixel,
            width: info.width,
            height: info.height,
            length: info.byte_len,
        }
    }

    pub fn clear(&self) {
        unsafe {
            core::ptr::write_bytes(self.buffer.lock().as_mut_ptr(), 0, self.length);
        }
    }

    pub fn fill(&self, color: Color) {
        let mut buffer = self.buffer.lock();
    
        match self.bytes_per_pixel {
            1 => {
                let color = match color {
                    Color::GRAYSCALE(val) => val as u8,
                    Color::RGB(r, g, b) => ((r + g + b) / 3) as u8,
                    Color::BGR(b, g, r) => ((r + g + b) / 3) as u8,
                };
                unsafe { write_bytes(buffer.as_mut_ptr(), color, self.length); }
            }
            3 => {
                let mut offset = 0;
                match color {
                    Color::RGB(r, g, b) => {
                        for _ in 0..self.height * self.width {
                            buffer[offset] = r as u8;
                            buffer[offset + 1] = g as u8;
                            buffer[offset + 2] = b as u8;
                            offset += 3;
                        }
                    }
                    Color::BGR(b, g, r) => {
                        for _ in 0..self.height * self.width {
                            buffer[offset] = b as u8;
                            buffer[offset + 1] = g as u8;
                            buffer[offset + 2] = r as u8;
                            offset += 3;
                        }
                    }
                    Color::GRAYSCALE(val) => {
                        for _ in 0..self.height * self.width {
                            buffer[offset] = val as u8;
                            buffer[offset + 1] = val as u8;
                            buffer[offset + 2] = val as u8;
                            offset += 3;
                        }
                    }
                }
            }
            4 => {
                let color = match color {
                    Color::RGB(r, g, b) => (r << 16) | (g << 8) | b,
                    Color::BGR(b, g, r) => (r << 16) | (g << 8) | b,
                    Color::GRAYSCALE(val) => (val << 16) | (val << 8) | val,
                };
                let color = color.to_le_bytes();
    
                for i in (0..self.length).step_by(4) {
                    buffer[i] = color[0];
                    buffer[i + 1] = color[1];
                    buffer[i + 2] = color[2];
                    buffer[i + 3] = color[3];
                }
            }
            _ => panic!("unsupported bytes per pixel"),
        }
    }

    pub fn set_pixel(&self, x: usize, y: usize, color: Color) {
        let mut buffer = self.buffer.lock();
        let offset = y * self.width + x;
        let offset = offset * self.bytes_per_pixel;
        match self.bytes_per_pixel {
            1 => buffer[offset] = match color {
                Color::GRAYSCALE(val) => val as u8,
                Color::RGB(r, g, b) => ((r + g + b) / 3) as u8,
                Color::BGR(b, g, r) => ((r + g + b) / 3) as u8,
            },
            3 => match color {
                Color::RGB(r, g, b) => {
                    buffer[offset] = r as u8;
                    buffer[offset + 1] = g as u8;
                    buffer[offset + 2] = b as u8;
                },
                Color::BGR(b, g, r) => {
                    buffer[offset] = r as u8;
                    buffer[offset + 1] = g as u8;
                    buffer[offset + 2] = b as u8;
                },
                Color::GRAYSCALE(val) => {
                    buffer[offset] = val as u8;
                    buffer[offset + 1] = val as u8;
                    buffer[offset + 2] = val as u8;
                }
            },
            4 => {
                let color = match color {
                    Color::RGB(r, g, b) => (r << 16) | (g << 8) | b,
                    Color::BGR(b, g, r) => (r << 16) | (g << 8) | b,
                    Color::GRAYSCALE(val) => (val << 16) | (val << 8) | val,
                };
                let color = color.to_le_bytes();
                buffer[offset] = color[0];
                buffer[offset + 1] = color[1];
                buffer[offset + 2] = color[2];
                buffer[offset + 3] = color[3];
            }
            _ => panic!("unsupported bytes per pixel"),
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let buffer = self.buffer.lock();
        let offset = y * self.width + x;
        let offset = offset * self.bytes_per_pixel;
        match self.bytes_per_pixel {
            1 => Color::GRAYSCALE(buffer[offset] as usize),
            3 => Color::RGB(
                buffer[offset] as usize,
                buffer[offset + 1] as usize,
                buffer[offset + 2] as usize,
            ),
            _ => panic!("unsupported bytes per pixel"),
        }
    }

    pub fn shift_up(&self, offset: usize) {
        self.buffer.lock().copy_within((self.width * offset * self.bytes_per_pixel).., 0);
    }

    pub fn write_pixels(&self, pixels: &[&[Color]]) {
        let mut buffer = self.buffer.lock();
        for (y, row) in pixels.iter().enumerate() {
            let offset = y * self.width;
            for (x, color) in row.iter().enumerate() {
                let offset = offset + x;
                let offset = offset * self.bytes_per_pixel;
                match self.bytes_per_pixel {
                    1 => buffer[offset] = match color {
                        Color::GRAYSCALE(val) => *val as u8,
                        Color::RGB(r, g, b) => ((*r + *g + *b) / 3) as u8,
                        Color::BGR(b, g, r) => ((*r + *g + *b) / 3) as u8,
                    },
                    3 => match color {
                        Color::RGB(r, g, b) => {
                            buffer[offset] = *r as u8;
                            buffer[offset + 1] = *g as u8;
                            buffer[offset + 2] = *b as u8;
                        },
                        Color::BGR(b, g, r) => {
                            buffer[offset] = *r as u8;
                            buffer[offset + 1] = *g as u8;
                            buffer[offset + 2] = *b as u8;
                        },
                        Color::GRAYSCALE(val) => {
                            buffer[offset] = *val as u8;
                            buffer[offset + 1] = *val as u8;
                            buffer[offset + 2] = *val as u8;
                        }
                    },
                    4 => {
                        let color = match color {
                            Color::RGB(r, g, b) => (r << 16) | (g << 8) | b,
                            Color::BGR(b, g, r) => (r << 16) | (g << 8) | b,
                            Color::GRAYSCALE(val) => (val << 16) | (val << 8) | val,
                        };
                        let color = color.to_le_bytes();
                        buffer[offset] = color[0];
                        buffer[offset + 1] = color[1];
                        buffer[offset + 2] = color[2];
                        buffer[offset + 3] = color[3];
                    }
                    _ => panic!("unsupported bytes per pixel"),
                }
            }
        }
    }

    pub fn write_pixels_at(&self, x: usize, y: usize, pixels: &[&[Color]]) {
        let mut buffer = self.buffer.lock();
        for (y_offset, row) in pixels.iter().enumerate() {
            let offset = (y + y_offset) * self.width + x;
            for (x_offset, color) in row.iter().enumerate() {
                let offset = offset + x_offset;
                let offset = offset * self.bytes_per_pixel;
                match self.bytes_per_pixel {
                    1 => buffer[offset] = match color {
                        Color::GRAYSCALE(val) => *val as u8,
                        Color::RGB(r, g, b) => ((*r + *g + *b) / 3) as u8,
                        Color::BGR(b, g, r) => ((*r + *g + *b) / 3) as u8,
                    },
                    3 => match color {
                        Color::RGB(r, g, b) => {
                            buffer[offset] = *r as u8;
                            buffer[offset + 1] = *g as u8;
                            buffer[offset + 2] = *b as u8;
                        },
                        Color::BGR(b, g, r) => {
                            buffer[offset] = *r as u8;
                            buffer[offset + 1] = *g as u8;
                            buffer[offset + 2] = *b as u8;
                        },
                        Color::GRAYSCALE(val) => {
                            buffer[offset] = *val as u8;
                            buffer[offset + 1] = *val as u8;
                            buffer[offset + 2] = *val as u8;
                        }
                    },
                    4 => {
                        let color = match color {
                            Color::RGB(r, g, b) => (r << 24) | (g << 16) | (b << 8),
                            Color::BGR(b, g, r) => (r << 24) | (g << 16) | (b << 8),
                            Color::GRAYSCALE(val) => (val << 24) | (val << 16) | (val << 8),
                        };
                        let color = color.to_le_bytes();
                        buffer[offset] = color[0];
                        buffer[offset + 1] = color[1];
                        buffer[offset + 2] = color[2];
                        buffer[offset + 3] = color[3];
                    }
                    _ => panic!("unsupported bytes per pixel"),
                }
            }
        }
    }
}