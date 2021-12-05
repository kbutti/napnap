use windows_sys::{Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        PostMessageA(HWND_TOPMOST, WM_SYSCOMMAND, SC_MONITORPOWER as usize, 2);
    }
}
