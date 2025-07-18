// whistle_commands.rs
// Whistle 相关的 Tauri 命令接口

use tauri::command;
use crate::whistle::{start_whistle, stop_whistle, clear_system_proxy, is_whistle_running, check_nodejs, check_whistle, install_whistle};
use crate::config;

#[command]
pub fn whistle_check_environment() -> Result<serde_json::Value, String> {
    let nodejs_installed = check_nodejs();
    let whistle_installed = check_whistle();
    let whistle_running = is_whistle_running();
    
    Ok(serde_json::json!({
        "nodejs_installed": nodejs_installed,
        "whistle_installed": whistle_installed,
        "whistle_running": whistle_running,
    }))
}

#[command]
pub fn whistle_install() -> Result<(), String> {
    install_whistle()
}

#[command]
pub fn whistle_start() -> Result<serde_json::Value, String> {
    let config = config::get_config();
    
    // 启动 whistle
    start_whistle(&config.whistle_port, &config.whistle_subdomain)?;
    
    // 设置系统代理
    // set_system_proxy(&config.port)?;
    
    Ok(serde_json::json!({
        "success": true,
        "port": config.whistle_port,
        "message": "Whistle 已启动并设置系统代理"
    }))
}

#[command]
pub fn whistle_stop() -> Result<serde_json::Value, String> {
    // 清除系统代理
    clear_system_proxy()?;
    
    // 停止 whistle
    stop_whistle()?;
    
    Ok(serde_json::json!({
        "success": true,
        "message": "Whistle 已停止并清除系统代理"
    }))
}

#[command]
pub fn whistle_get_status() -> Result<serde_json::Value, String> {
    let running = is_whistle_running();
    
    Ok(serde_json::json!({
        "running": running,
    }))
}

#[command]
pub fn whistle_open_web_ui() -> Result<(), String> {
    use std::process::Command;
    
    
    Ok(())
} 