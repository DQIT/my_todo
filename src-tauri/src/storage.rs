// 数据模型与持久化：JSON 文件存于 app data 目录，容错回退默认值。

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub time: String,
    pub content: String,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "completedAt")]
    pub completed_at: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "desktopEnabled")]
    pub desktop_enabled: bool,
    #[serde(rename = "desktopLocked")]
    pub desktop_locked: bool,
    #[serde(rename = "backgroundColor", default = "default_bg_color")]
    pub background_color: String,
    #[serde(rename = "backgroundOpacity")]
    pub background_opacity: u32,
    pub autostart: bool,
    #[serde(rename = "startMinimized")]
    pub start_minimized: bool,
    #[serde(rename = "fontSize")]
    pub font_size: u32,
    #[serde(rename = "textColor")]
    pub text_color: String,
    #[serde(rename = "textShadow")]
    pub text_shadow: bool,
    #[serde(rename = "displayFormat")]
    pub display_format: String,
    #[serde(rename = "expiredHighlight")]
    pub expired_highlight: bool,
    pub theme: String,
}

fn default_bg_color() -> String {
    "#14161c".into()
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            desktop_enabled: false,
            desktop_locked: true,
            background_color: default_bg_color(),
            background_opacity: 0,
            autostart: false,
            start_minimized: false,
            font_size: 16,
            text_color: "#ffffff".into(),
            text_shadow: true,
            display_format: "time-content".into(),
            expired_highlight: true,
            theme: "system".into(),
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct DesktopBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    #[serde(default)]
    pub todos: Vec<Todo>,
    #[serde(default)]
    pub settings: Settings,
    #[serde(rename = "desktopBounds", default)]
    pub desktop_bounds: Option<DesktopBounds>,
}

/// 进程内持有的状态，受 Mutex 保护
pub struct Store(pub Mutex<AppState>);

fn data_file(app: &AppHandle) -> PathBuf {
    let dir = app
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    let _ = fs::create_dir_all(&dir);
    dir.join("mytodo-data.json")
}

/// 启动时加载；文件缺失或损坏则回退默认并重建
pub fn load(app: &AppHandle) -> AppState {
    let path = data_file(app);
    match fs::read_to_string(&path) {
        Ok(raw) => match serde_json::from_str::<AppState>(&raw) {
            Ok(state) => state,
            Err(_) => {
                // 损坏：备份后回退默认
                let _ = fs::rename(&path, path.with_extension("json.bak"));
                AppState::default()
            }
        },
        Err(_) => AppState::default(),
    }
}

pub fn save(app: &AppHandle, state: &AppState) {
    let path = data_file(app);
    if let Ok(json) = serde_json::to_string_pretty(state) {
        // 原子写：先写临时文件再替换
        let tmp = path.with_extension("json.tmp");
        if fs::write(&tmp, json).is_ok() {
            let _ = fs::rename(&tmp, &path);
        }
    }
}
