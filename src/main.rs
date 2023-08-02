#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)] // make it run test framework
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}



// AUTOMATIC EXIT
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}



// // PANIC HANDLERS
// // our existing panic handler
// #[cfg(not(test))] // new attribute
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     println!("{}", info);
//     loop {}
// }

// // our panic handler in test mode
// #[cfg(test)]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     serial_println!("[failed]\n");
//     serial_println!("Error: {}\n", info);
//     exit_qemu(QemuExitCode::Failed);
//     loop {}
// }



// // TESTABLE TRAIT - automate printing messages about error
// pub trait Testable {
//     fn run(&self) -> ();
// }

// impl<T> Testable for T
// where
//     T: Fn(),
// {
//     fn run(&self) {
//         serial_print!("{}...\t", core::any::type_name::<T>());
//         self();
//         serial_println!("[ok]");
//     }
// }



// UNIT TESTING
// TEST RUNNER
#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    /// new
    exit_qemu(QemuExitCode::Success);
}

// UNIT TESTS
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
