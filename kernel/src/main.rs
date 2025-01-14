#![no_std]
#![no_main]

use core::panic::PanicInfo;

use graphics::vga_buffer::VGABuffer;

mod graphics;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let buffer = VGABuffer::new(boot_info.framebuffer.as_mut().unwrap());
    buffer.clear();
    buffer.fill(graphics::Color::RGB(255, 0, 0));
    buffer.clear();

    loop {}
}