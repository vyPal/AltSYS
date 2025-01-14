pub mod vga_buffer;
pub mod console;

#[derive(Clone, Copy)]
pub enum Color {
    RGB(usize, usize, usize),
    BGR(usize, usize, usize),
    GRAYSCALE(usize),
}