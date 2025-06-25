#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(Astrix::test_runner)]
#![reexport_test_harness_main = "test_main"]
use Astrix::serial::SERIAL1;
use core::panic::PanicInfo;
use Astrix::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
       println!("Hello World{}", "!");

    Astrix::init();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();
    loop {}
}

// Define StackPointer as a type alias for a pointer-sized unsigned integer
type StackPointer = usize;

struct InterruptStackTable {
    stack_pointers: [Option<StackPointer>; 7],
}


/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Astrix::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

// Define an enum for QEMU exit codes, used to signal test results to QEMU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10, // Indicates successful test execution.
    Failed = 0x11,  // Indicates test failure.
}

// Exits QEMU by writing the exit code to the special I/O port 0xf4.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// Internal print function that writes formatted arguments to the serial port.
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

// Trait for test cases, allowing them to be run and reported.
pub trait Testable {
    fn run(&self) -> ();
}

// Implement Testable for any function that takes no arguments and returns nothing.
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}