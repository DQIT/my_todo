// Tauri 命令 + 窗口/桌面层操作的共享逻辑。

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_autostart::ManagerExt;

use crate::desktop;
use crate::storage::{self, DesktopBounds, Settings, Store, Todo};

// ---------------- 状态读写命令 ----------------

#[tauri::command]
pub fn load_state(app: AppHandle) -> storage::AppState {
    let store = app.state::<Store>();
    let guard = store.0.lock().unwrap();
    guard.clone()
}

#[tauri::command]
pub fn save_todos(app: AppHandle, todos: Vec<Todo>) {
    {
        let store = app.state::<Store>();
        let mut guard = store.0.lock().unwrap();
        guard.todos = todos;
        storage::save(&app, &guard);
    }
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: Settings) {
    let store = app.state::<Store>();
    let mut guard = store.0.lock().unwrap();
    guard.settings = settings;
    storage::save(&app, &guard);
}

#[tauri::command]
pub fn save_desktop_bounds(app: AppHandle, bounds: DesktopBounds) {
    let store = app.state::<Store>();
    let mut guard = store.0.lock().unwrap();
    guard.desktop_bounds = Some(bounds);
    storage::save(&app, &guard);
}

/// 广播状态变更，让另一窗口刷新
#[tauri::command]
pub fn broadcast_state_changed(app: AppHandle) {
    let _ = app.emit("state-changed", ());
}

// ---------------- 桌面层控制命令 ----------------

#[tauri::command]
pub fn set_desktop_visible(app: AppHandle, enabled: bool) {
    apply_desktop_visibility(&app, enabled);
}

#[tauri::command]
pub fn set_desktop_locked(app: AppHandle, locked: bool) {
    apply_desktop_lock(&app, locked);
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, enabled: bool) {
    let mgr = app.autolaunch();
    let _ = if enabled {
        mgr.enable()
    } else {
        mgr.disable()
    };
}

#[tauri::command]
pub fn show_main_window(app: AppHandle) {
    show_main(&app);
}

#[tauri::command]
pub fn hide_main_window(app: AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
    }
}

#[tauri::command]
pub fn quit_app(app: AppHandle) {
    do_quit(&app);
}

// ---------------- 共享逻辑 ----------------

pub fn show_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

pub fn apply_desktop_visibility(app: &AppHandle, enabled: bool) {
    let Some(w) = app.get_webview_window("desktop") else {
        return;
    };
    if enabled {
        let _ = w.show();
        if let Ok(hwnd) = w.hwnd() {
            let raw = hwnd.0 as isize;
            // 尽力嵌入壁纸层；失败则退化为始终置底
            if !desktop::embed_in_wallpaper(raw) {
                desktop::pin_to_bottom(raw);
            }
            // 应用当前锁定状态
            let locked = current_locked(app);
            desktop::set_click_through(raw, locked);
        }
    } else {
        let _ = w.hide();
    }
    persist_setting_desktop_enabled(app, enabled);
}

pub fn apply_desktop_lock(app: &AppHandle, locked: bool) {
    if let Some(w) = app.get_webview_window("desktop") {
        if let Ok(hwnd) = w.hwnd() {
            desktop::set_click_through(hwnd.0 as isize, locked);
        }
    }
    let store = app.state::<Store>();
    let mut guard = store.0.lock().unwrap();
    guard.settings.desktop_locked = locked;
    storage::save(app, &guard);
}

fn current_locked(app: &AppHandle) -> bool {
    let store = app.state::<Store>();
    let guard = store.0.lock().unwrap();
    guard.settings.desktop_locked
}

fn persist_setting_desktop_enabled(app: &AppHandle, enabled: bool) {
    let store = app.state::<Store>();
    let mut guard = store.0.lock().unwrap();
    guard.settings.desktop_enabled = enabled;
    storage::save(app, &guard);
}

// 托盘快捷开关
pub fn toggle_desktop_visibility(app: &AppHandle) {
    let now = {
        let store = app.state::<Store>();
        let g = store.0.lock().unwrap();
        g.settings.desktop_enabled
    };
    apply_desktop_visibility(app, !now);
    let _ = app.emit("state-changed", ());
}

pub fn toggle_desktop_lock(app: &AppHandle) {
    let now = current_locked(app);
    apply_desktop_lock(app, !now);
    let _ = app.emit("state-changed", ());
}

pub fn do_quit(app: &AppHandle) {
    app.exit(0);
}
