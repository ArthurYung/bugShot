// config_commands.rs
// 配置相关的 Tauri 命令接口

use tauri::command;
use crate::config::{AppConfig, init_config, get_config, update_config};

#[command]
pub fn config_get() -> Result<AppConfig, String> {
    Ok(get_config())
}

#[command]
pub fn config_update(updates: serde_json::Value) -> Result<AppConfig, String> {
    let mut config = get_config();
    
    // 从 JSON 更新配置
    if let Some(capture_enabled) = updates.get("capture_enabled").and_then(|v| v.as_bool()) {
        config.capture_enabled = capture_enabled;
    }
    
    if let Some(capture_rules) = updates.get("capture_rules").and_then(|v| v.as_str()) {
        config.capture_rules = capture_rules.to_string();
    }
    
    // 保存配置
    config.save()?;
    
    // 更新全局配置
    update_config(|global_config| {
        *global_config = config.clone();
    })?;
    
    Ok(config)
}

#[command]
pub fn config_reset() -> Result<AppConfig, String> {
    let mut config = AppConfig::default();
    config.save()?;
    
    update_config(|global_config| {
        *global_config = config.clone();
    })?;
    
    Ok(config)
}

#[command]
pub fn config_get_path() -> Result<String, String> {
    let config = get_config();
    let _ = config.save().map_err(|_| "无法获取配置路径".to_string())?;
    Ok(format!("{:?}", config))
}

#[command]
pub fn config_init() -> Result<AppConfig, String> {
    Ok(init_config())
} 