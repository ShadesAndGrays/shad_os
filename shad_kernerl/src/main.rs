#![no_std]
#![no_main]

#![allow(dead_code)]


mod std;

use bootloader_api::config;
use x86_64::instructions::hlt;
use std::writer::*;

#[cfg_attr(not(test), panic_handler)]
fn panic(_info:&core::panic::PanicInfo) -> ! {

    loop{
        hlt();
    }
}

pub static BOOTLOADER_CONFIG:bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(config::Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

bootloader_api::entry_point!(kmain,config = &BOOTLOADER_CONFIG);

#[no_mangle]
fn kmain(bootinfo: &'static mut bootloader_api::BootInfo) -> !{

    // Get the info for a frame buffer the boot info api gives Thanks :)
    let frame_buffer_info = bootinfo.framebuffer.as_mut().unwrap().info();

    let buffer = bootinfo.framebuffer.as_mut().unwrap().buffer_mut();

    FRAME_BUFFER_WRITER.lock().init(buffer,frame_buffer_info);

    println!("Hello new world");


    loop {
        hlt()
    }

}
