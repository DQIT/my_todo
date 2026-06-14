// 桌面悬浮层的平台相关行为。
// Windows 上实现：鼠标穿透切换、始终置底、(尽力) 嵌入壁纸层 WorkerW。
// 非 Windows 平台为 no-op 占位，使工程在 mac/Linux 上也能编译（仅用于开发 UI）。

#[cfg(windows)]
mod imp {
    use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{
        EnumWindows, FindWindowExW, FindWindowW, GetWindowLongPtrW, SendMessageTimeoutW,
        SetParent, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, GWL_STYLE, HWND_BOTTOM,
        SMTO_NORMAL, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, WS_CHILD, WS_EX_LAYERED,
        WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT, WS_POPUP,
    };
    use windows::core::{w, PCWSTR};

    fn hwnd_from(raw: isize) -> HWND {
        HWND(raw as *mut core::ffi::c_void)
    }

    /// 设置/取消鼠标穿透。locked=true → 穿透（WS_EX_TRANSPARENT|LAYERED），融入桌面。
    pub fn set_click_through(raw: isize, locked: bool) {
        let hwnd = hwnd_from(raw);
        unsafe {
            let mut ex = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
            let through =
                WS_EX_TRANSPARENT.0 | WS_EX_LAYERED.0 | WS_EX_NOACTIVATE.0 | WS_EX_TOOLWINDOW.0;
            if locked {
                ex |= through;
            } else {
                // 解锁：仍保留 LAYERED（透明渲染需要）与 TOOLWINDOW（不进任务栏/Alt-Tab），
                // 仅去掉穿透与 NOACTIVATE，使窗口可被拖拽/缩放。
                ex &= !(WS_EX_TRANSPARENT.0 | WS_EX_NOACTIVATE.0);
                ex |= WS_EX_LAYERED.0 | WS_EX_TOOLWINDOW.0;
            }
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex as isize);
        }
    }

    /// 把窗口压到最底层（在所有普通窗口之下，但仍在壁纸之上）。
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

    // ---- WorkerW 嵌入：把窗口挂到桌面图标层之下、壁纸之上 ----
    // 原理：向 Progman 发送 0x052C 消息，触发系统创建一个位于桌面图标 SHELLDLL_DefView
    // 之后的 WorkerW 窗口；我们枚举找到那个 WorkerW，将自身 SetParent 进去。
    static mut WORKERW: HWND = HWND(std::ptr::null_mut());

    unsafe extern "system" fn enum_proc(top: HWND, _l: LPARAM) -> windows::core::BOOL {
        let shell = FindWindowExW(Some(top), None, w!("SHELLDLL_DefView"), PCWSTR::null());
        if let Ok(sh) = shell {
            if !sh.0.is_null() {
                // 紧跟在该 top 之后的 WorkerW 即为壁纸绘制层
                if let Ok(worker) =
                    FindWindowExW(None, Some(top), w!("WorkerW"), PCWSTR::null())
                {
                    WORKERW = worker;
                }
            }
        }
        true.into()
    }

    /// 尝试将窗口嵌入壁纸层。成功返回 true。
    pub fn embed_in_wallpaper(raw: isize) -> bool {
        let hwnd = hwnd_from(raw);
        unsafe {
            let progman = match FindWindowW(w!("Progman"), PCWSTR::null()) {
                Ok(h) if !h.0.is_null() => h,
                _ => return false,
            };
            // 让 Progman 生成 WorkerW
            let mut result: usize = 0;
            let _ = SendMessageTimeoutW(
                progman,
                0x052C,
                Some(WPARAM(0)),
                Some(LPARAM(0)),
                SMTO_NORMAL,
                1000,
                Some(&mut result as *mut usize),
            );

            WORKERW = HWND(std::ptr::null_mut());
            let _ = EnumWindows(Some(enum_proc), LPARAM(0));

            if WORKERW.0.is_null() {
                return false;
            }

            // 改为子窗口样式并挂入 WorkerW
            let style = GetWindowLongPtrW(hwnd, GWL_STYLE) as u32;
            let style = (style & !WS_POPUP.0) | WS_CHILD.0;
            SetWindowLongPtrW(hwnd, GWL_STYLE, style as isize);
            SetParent(hwnd, Some(WORKERW)).is_ok()
        }
    }
}

#[cfg(not(windows))]
mod imp {
    pub fn set_click_through(_raw: isize, _locked: bool) {}
    pub fn pin_to_bottom(_raw: isize) {}
    pub fn embed_in_wallpaper(_raw: isize) -> bool {
        false
    }
}

pub use imp::{embed_in_wallpaper, pin_to_bottom, set_click_through};
