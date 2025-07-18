// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

pub mod mouse_event;
pub mod mouse_listener;
pub mod recorder;
pub mod record;
pub mod video_overlay;
pub mod utils;
pub mod get_element;
pub mod whistle;
pub mod whistle_commands;
pub mod config;
pub mod config_commands;
pub mod window;

pub fn main() {
    // 初始化配置
    let config = config::init_config();

    if config.capture_enabled {
        let _ = whistle_commands::whistle_start();
    }

    tauri::Builder::default()
        .setup(|app| {
            window::create_system_tray(&app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            utils::greet,
            record::start_recording,
            record::stop_recording,
            whistle_commands::whistle_check_environment,
            whistle_commands::whistle_install,
            whistle_commands::whistle_start,
            whistle_commands::whistle_stop,
            whistle_commands::whistle_get_status,
            whistle_commands::whistle_open_web_ui,
            config_commands::config_get,
            config_commands::config_update,
            config_commands::config_reset,
            config_commands::config_get_path,
            config_commands::config_init,
            utils::check_ffmpeg_installed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
