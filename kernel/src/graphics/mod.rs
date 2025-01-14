pub mod vga_buffer;

#[derive(Clone, Copy)]
pub enum Color {
    RGB(usize, usize, usize),
    BGR(usize, usize, usize),
    GRAYSCALE(usize),
}