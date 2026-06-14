// 阻止 release 模式弹出多余的控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    mytodo_lib::run()
}
