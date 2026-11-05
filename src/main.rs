#![no_std]
#![no_main]

use windows_sys::Win32::UI::WindowsAndMessaging::{
    SendMessageW, HWND_BROADCAST, SC_MONITORPOWER, WM_SYSCOMMAND,
};
use windows_sys::Win32::System::Threading::ExitProcess;

const MONITOR_OFF: isize = 2;

#[unsafe(no_mangle)]
pub extern "C" fn main_crt_startup() -> ! {
    unsafe {
        SendMessageW(
            HWND_BROADCAST,
            WM_SYSCOMMAND,
            SC_MONITORPOWER as usize,
            MONITOR_OFF,
        );
        ExitProcess(0);
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { ExitProcess(1); }
}