#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

// in src/main.rs
#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}

// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     vga_buffer::print_something();

//     loop {}
// }

//static HELLO: &[u8] = "Hello World!";
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     let vga_buffer = 0xb8000 as *mut u8;
//     for (i, &byte) in HELLO.iter().enumerate() {
//         unsafe {
//             *vga_buffer.offset(i as isize * 2) = byte;
//             *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
//         }
//     }

//     loop {}
// }

/// This function is called on panic.
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


mod vga_buffer;



