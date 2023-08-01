#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

// in src/main.rs
#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    //panic!("Some panic message");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


mod vga_buffer;

#[feature(custom_test_frameworks)]
#[test_runner(crate::test_runner)]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
