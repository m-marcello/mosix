//! My own OS written in Rust

// Don't link the standard library
#![no_std]
// Disable all Rust-level entry points
#![no_main]

// PanicInfo contains the file and line where the panic happened and the
// optional panic message.
use core::panic::PanicInfo;

mod vga_buffer;

// The panic function should cause the current thread to crash and thus it will
// never return. These functions are called 'diverging functions' and marked
// with the `!` as the return.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}

// We removed the `main` function because there will be no OS calling it.
// Instead, we are overwriting the operating system entry point with our own
// `_start` function.
// With the `no_mangle` attribute we ensure that the compiler really outputs a
// function with the name `_start` and not a unique sequence of generated
// cryptic symbol.
// We also have to mark the function as extern "C" to tell the compiler to use
// the C calling convention for this function. The function is not called by any
// function, but invoked directly by the bootloader or operating system. So
// instead of returning, the entry point should e.g. invoke the `exit` system
// call.
#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Hello World {}", 42);

	loop {}
}
