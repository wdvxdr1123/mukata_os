#![no_std] // 不要导入标准库
#![no_main] // 禁用所有Rust entry point
use core::panic::PanicInfo;

#[panic_handler] // This function is called on panic.
fn panic(__info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // 禁止编译器修改函数名
pub extern "C" fn _start() -> ! {
    loop {}
}
