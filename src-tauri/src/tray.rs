// 系统托盘：图标 + 双击呼出主界面 + 右键菜单。

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    AppHandle,
};

use crate::commands;

pub fn build(app: &AppHandle) -> tauri::Result<()> {
    let toggle_desktop = MenuItem::with_id(app, "toggle_desktop", "显示/隐藏桌面", true, None::<&str>)?;
    let toggle_lock = MenuItem::with_id(app, "toggle_lock", "锁定/解锁桌面", true, None::<&str>)?;
    let show_main = MenuItem::with_id(app, "show_main", "打开主界面", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&toggle_desktop, &toggle_lock, &show_main, &quit])?;

    let _tray = TrayIconBuilder::with_id("main-tray")
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("MyToDo")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "toggle_desktop" => commands::toggle_desktop_visibility(app),
            "toggle_lock" => commands::toggle_desktop_lock(app),
            "show_main" => commands::show_main(app),
            "quit" => commands::do_quit(app),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            // 双击呼出主界面
            if let TrayIconEvent::DoubleClick {
                button: MouseButton::Left,
                ..
            } = event
            {
                commands::show_main(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}
