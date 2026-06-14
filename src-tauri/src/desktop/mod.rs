// 桌面悬浮层的平台相关行为。
// 鼠标穿透改用 Tauri 原生的 set_ignore_cursor_events（见 commands.rs），
// 不再手动设置 WS_EX_LAYERED —— 那会破坏 Tauri 透明窗口的 DirectComposition 合成，
// 导致窗口变黑、透明度异常。
//
// 本模块只保留一个 Win32 操作：把窗口压到最底层（在普通窗口之下、壁纸之上），
// 用于抵御「显示桌面 / 资源管理器刷新」导致窗口上浮。非 Windows 平台为 no-op。

#[cfg(windows)]
mod imp {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowPos, HWND_BOTTOM, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
    };

    fn hwnd_from(raw: isize) -> HWND {
        HWND(raw as *mut core::ffi::c_void)
    }

    /// 把窗口压到最底层（仍在壁纸之上）。
    pub fn pin_to_bottom(raw: isize) {
        let hwnd = hwnd_from(raw);
        unsafe {
            let _ = SetWindowPos(
                hwnd,
                Some(HWND_BOTTOM),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            );
        }
    }
}

#[cfg(not(windows))]
mod imp {
    pub fn pin_to_bottom(_raw: isize) {}
}

pub use imp::pin_to_bottom;
