use windows_sys::Win32::UI::WindowsAndMessaging::*;

fn main() {
    // https://docs.microsoft.com/en-us/windows/win32/menurc/wm-syscommand
    // const MONITORPOWER_POWERING_ON: isize = -1;
    // const MONITORPOWER_GOING_TO_LOW_POWER: isize = 1;
    const MONITORPOWER_BEING_SHUT_OFF: isize = 2;

    unsafe {
        PostMessageA(
            HWND_TOPMOST,
            WM_SYSCOMMAND,
            SC_MONITORPOWER as usize,
            MONITORPOWER_BEING_SHUT_OFF,
        );
    }
}
