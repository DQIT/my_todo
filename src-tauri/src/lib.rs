mod commands;
mod desktop;
mod storage;
mod tray;

use storage::Store;
use std::sync::Mutex;
use tauri::{Manager, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 单实例：再次启动时唤出已运行实例的主界面
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            commands::show_main(app);
        }))
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .invoke_handler(tauri::generate_handler![
            commands::load_state,
            commands::save_todos,
            commands::save_settings,
            commands::save_desktop_bounds,
            commands::broadcast_state_changed,
            commands::set_desktop_visible,
            commands::set_desktop_locked,
            commands::set_autostart,
            commands::show_main_window,
            commands::hide_main_window,
            commands::quit_app,
        ])
        .setup(|app| {
            let handle = app.handle().clone();

            // 加载持久化状态
            let state = storage::load(&handle);
            let start_minimized = state.settings.start_minimized;
            let desktop_enabled = state.settings.desktop_enabled;
            let bounds = state.desktop_bounds;
            app.manage(Store(Mutex::new(state)));

            // 托盘
            tray::build(&handle)?;

            // 还原桌面层位置/大小（用物理像素，与保存时一致，避免 DPI 缩放错位）。
            // 首次无记忆、或记忆值异常（屏幕外/旧版本坏值）时，放到主显示器右上角。
            if let Some(w) = app.get_webview_window("desktop") {
                use tauri::{PhysicalPosition, PhysicalSize};
                let valid = bounds.filter(|b| {
                    b.x > -10000 && b.y > -10000 && b.width >= 120 && b.height >= 80
                });
                match valid {
                    Some(b) => {
                        let _ = w.set_size(PhysicalSize::new(b.width, b.height));
                        let _ = w.set_position(PhysicalPosition::new(b.x, b.y));
                    }
                    None => {
                        if let Ok(Some(monitor)) = w.primary_monitor() {
                            let scale = monitor.scale_factor();
                            let area = monitor.size();
                            let margin = (24.0 * scale) as i32;
                            let win_w = (340.0 * scale) as u32;
                            let win_h = (420.0 * scale) as u32;
                            let x = area.width as i32 - win_w as i32 - margin;
                            let y = margin;
                            let _ = w.set_size(PhysicalSize::new(win_w, win_h));
                            let _ = w.set_position(PhysicalPosition::new(x.max(0), y));
                        }
                    }
                }
            }

            // 主界面：按"启动后最小化"决定是否显示
            if let Some(main) = app.get_webview_window("main") {
                if !start_minimized {
                    let _ = main.show();
                    let _ = main.set_focus();
                }
            }

            // 桌面层：若上次开启则恢复显示并应用穿透/置底
            if desktop_enabled {
                commands::apply_desktop_visibility(&handle, true);
                // 周期性重新置底，抵御"显示桌面"/资源管理器重启导致的失效
                spawn_repin_guard(handle.clone());
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                // 主界面关闭 = 隐藏到托盘，不退出
                WindowEvent::CloseRequested { api, .. } if window.label() == "main" => {
                    api.prevent_close();
                    let _ = window.hide();
                }
                // 桌面层移动/缩放 → 记忆位置大小
                WindowEvent::Moved(_) | WindowEvent::Resized(_)
                    if window.label() == "desktop" =>
                {
                    persist_desktop_bounds(window);
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("启动 MyToDo 失败");
}

fn persist_desktop_bounds(window: &tauri::Window) {
    let app = window.app_handle();
    if let Some(w) = app.get_webview_window("desktop") {
        // 隐藏的窗口位置是哨兵值（约 -32000），不要保存，否则下次还原会把窗口移出屏幕。
        if !w.is_visible().unwrap_or(false) {
            return;
        }
        if let (Ok(pos), Ok(size)) = (w.outer_position(), w.inner_size()) {
            // 物理像素 < 0 的异常值同样跳过
            if pos.x < -10000 || pos.y < -10000 || size.width == 0 || size.height == 0 {
                return;
            }
            let store = app.state::<Store>();
            let mut guard = store.0.lock().unwrap();
            guard.desktop_bounds = Some(storage::DesktopBounds {
                x: pos.x,
                y: pos.y,
                width: size.width,
                height: size.height,
            });
            storage::save(app, &guard);
        }
    }
}

/// 每隔几秒把桌面层重新压到底层，保证不因系统刷新而消失/上浮。
fn spawn_repin_guard(handle: tauri::AppHandle) {
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(3));
        let enabled = {
            match handle.try_state::<Store>() {
                Some(store) => store.0.lock().unwrap().settings.desktop_enabled,
                None => return,
            }
        };
        if !enabled {
            continue;
        }
        if let Some(w) = handle.get_webview_window("desktop") {
            if let Ok(hwnd) = w.hwnd() {
                desktop::pin_to_bottom(hwnd.0 as isize);
            }
        }
    });
}
