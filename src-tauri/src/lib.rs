// 主模块文件 lib.rs
// 负责声明和导入各个功能模块，并注册 Tauri 命令

pub mod recorder;
pub mod mouse_event;
pub mod mouse_listener;
pub mod video_overlay;
pub mod record;
mod utils;  // 工具相关功能模块


// 导入各模块中的命令和结构体，供 Tauri 注册和调用
use record::{start_recording, stop_recording};
use utils::{greet, check_ffmpeg_installed};


// Tauri 应用程序入口函数
// 负责初始化插件、注册命令并启动应用
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    std::panic::set_hook(Box::new(|info| {
        println!("[panic] {:?}", info);
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init()) // 初始化文件打开插件
        .invoke_handler(tauri::generate_handler![
            greet, start_recording, stop_recording, check_ffmpeg_installed // 注册所有命令
        ])
        .run(tauri::generate_context!()) // 启动 Tauri 应用
        .expect("error while running tauri application");
}
