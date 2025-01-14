#![no_std]
#![no_main]

use core::panic::PanicInfo;

use graphics::{console::{init_console, CONSOLE}, vga_buffer::VGABuffer};

mod graphics;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let buffer = VGABuffer::new(boot_info.framebuffer.as_mut().unwrap());
    buffer.clear();
    buffer.fill(graphics::Color::RGB(0, 0, 255));

    init_console(buffer);
    CONSOLE.get().unwrap().lock().clear();
    println!("Sobity");
    println!("Skibek\n");

    println!("There are {} memory regions", boot_info.memory_regions.len());
    println!("Only {} regions are usable", boot_info.memory_regions.iter().filter(|region| region.kind == bootloader_api::info::MemoryRegionKind::Usable).count());
    print!("Total memory: ");
    bytes_to_pretty_print(boot_info.memory_regions.iter().map(|region| region.end - region.start).sum());
    println!();
    print!("Usable memory: ");
    bytes_to_pretty_print(boot_info.memory_regions.iter().filter(|region| region.kind == bootloader_api::info::MemoryRegionKind::Usable).map(|region| region.end - region.start).sum());
    println!();
    for (i, region) in boot_info.memory_regions.iter().enumerate() {
        if region.kind != bootloader_api::info::MemoryRegionKind::Usable {
            continue;
        }
        print!("Region #{i}: {:#013x} - {:#013x} ", region.start, region.end);
        bytes_to_pretty_print(region.end - region.start);
        println!(" | {:?}", region.kind);
    }

    loop {}
}

fn bytes_to_pretty_print(bytes: u64) {
    let mut bytes = bytes;
    let mut units = 0;
    while bytes >= 1024 {
        bytes /= 1024;
        units += 1;
    }
    let unit = match units {
        0 => "B",
        1 => "KB",
        2 => "MB",
        3 => "GB",
        4 => "TB",
        5 => "PB",
        6 => "EB",
        7 => "ZB",
        8 => "YB",
        _ => "??",
    };
    print!("{} {}", bytes, unit);
}