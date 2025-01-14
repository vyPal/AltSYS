#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let buffer = boot_info.framebuffer.as_mut().unwrap();
    let info = buffer.info();

    let mut_buffer = buffer.buffer_mut();
    for y in 0..info.height {
        for x in 0..info.width {
            let pixel_index = (y * info.stride + x) * info.bytes_per_pixel;
            mut_buffer[pixel_index] = 255;
            mut_buffer[pixel_index + 1] = 255;
            mut_buffer[pixel_index + 2] = 255;
        }
    }

    loop {}
}