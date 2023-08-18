#![no_std]

#[cfg(not(test))]
use core::panic::PanicInfo;

/// When using the alloc crate it seems like it does some unwinding. Adding this
/// export satisfies the compiler but may introduce undefined behaviour when a
/// panic occurs.
#[no_mangle]
pub extern "system" fn __CxxFrameHandler3(_: *mut u8, _: *mut u8, _: *mut u8, _: *mut u8) -> i32 {
    unimplemented!()
}

/// Explanation can be found here: https://github.com/Trantect/win_driver_example/issues/4
#[export_name = "_fltused"]
static _FLTUSED: i32 = 0;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link(name = "ntoskrnl")]
extern "C" {
    fn DbgPrint(format: *const u8, ...) -> u64;
}

#[no_mangle]
pub extern "system" fn driver_entry(_: &mut u64, _: &mut u64) -> u64 {
    unsafe { DbgPrint("Hello World\n!".as_ptr()); }
    0
}
