#![no_std]
#![no_main]

use bootloader_api::config;
use x86_64::instructions::hlt;

#[cfg_attr(not(test), panic_handler)]
fn panic(_info:&core::panic::PanicInfo) -> ! {

    loop{
        hlt();
    }
}
pub static BOOTLOADER_CONFIG:bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(config::Mapping::Dynamic);
    config
};
const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};
bootloader_api::entry_point!(kernel_start,config = &BOOTLOADER_CONFIG);



#[no_mangle]
fn kernel_start(bootinfo: &'static mut bootloader_api::BootInfo) -> !{

    let vga = 0xb8000 as *mut u8;

    for (i, b) in b"Hello world".iter().enumerate() {
        unsafe {
            *vga.offset(i as isize * 2) = *b;
            *vga.offset(i as isize * 2 + 1) = 0x0f;
        }
    }

    loop {
        hlt()
    }

}
