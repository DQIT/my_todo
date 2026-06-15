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

/// 用系统默认浏览器打开外部链接（关于页的开发者/仓库/下载链接）
#[tauri::command]
pub fn open_url(url: String) {
    open_external(&url);
}

#[cfg(target_os = "windows")]
fn open_external(url: &str) {
    // `start` 的第一个引号参数会被当作窗口标题，故传入空标题占位
    let _ = std::process::Command::new("cmd")
        .args(["/C", "start", "", url])
        .spawn();
}

#[cfg(target_os = "macos")]
fn open_external(url: &str) {
    let _ = std::process::Command::new("open").arg(url).spawn();
}

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
fn open_external(url: &str) {
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
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
        // 压到底层
        if let Ok(hwnd) = w.hwnd() {
            desktop::pin_to_bottom(hwnd.0 as isize);
        }
        // 应用当前锁定状态（穿透）
        let locked = current_locked(app);
        let _ = w.set_ignore_cursor_events(locked);
    } else {
        let _ = w.hide();
    }
    persist_setting_desktop_enabled(app, enabled);
}

pub fn apply_desktop_lock(app: &AppHandle, locked: bool) {
    if let Some(w) = app.get_webview_window("desktop") {
        // 锁定 = 鼠标穿透；解锁 = 接收鼠标事件（可拖拽/缩放）
        let _ = w.set_ignore_cursor_events(locked);
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
