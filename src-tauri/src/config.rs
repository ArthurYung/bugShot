// config.rs
// 应用配置管理模块

use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use once_cell::sync::OnceCell;
use dirs;

static CONFIG_PATH: OnceCell<PathBuf> = OnceCell::new();

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    // 抓包配置
    pub capture_enabled: bool,
    pub capture_rules: String,
    pub whistle_port: u16,
    pub whistle_subdomain: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            // 抓包默认配置
            capture_enabled: false,
            capture_rules: "log\nexport http://127.0.0.1:3000/whistle-capture".to_string(),
            whistle_port: 8900,
            whistle_subdomain: "bugshot".to_string(),
        }
    }
}

impl AppConfig {
    /// 获取配置文件路径
    fn get_config_path() -> PathBuf {
        if let Some(path) = CONFIG_PATH.get() {
            return path.clone();
        }
        
        let mut config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        config_dir.push("bugshot");
        
        // 确保配置目录存在
        if !config_dir.exists() {
            let _ = fs::create_dir_all(&config_dir);
        }
        
        let config_file = config_dir.join("config.json");
        CONFIG_PATH.set(config_file.clone()).ok();
        config_file
    }
    
    /// 加载配置文件
    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        
        if !config_path.exists() {
            // 配置文件不存在，创建默认配置
            let default_config = Self::default();
            if let Err(e) = default_config.save() {
                eprintln!("[config] 保存默认配置失败: {}", e);
            }
            return default_config;
        }
        
        match fs::read_to_string(&config_path) {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(config) => {
                        println!("[config] 配置加载成功: {:?}", config_path);
                        config
                    }
                    Err(e) => {
                        eprintln!("[config] 配置文件格式错误: {}", e);
                        eprintln!("[config] 使用默认配置");
                        Self::default()
                    }
                }
            }
            Err(e) => {
                eprintln!("[config] 读取配置文件失败: {}", e);
                eprintln!("[config] 使用默认配置");
                Self::default()
            }
        }
    }
    
    /// 保存配置文件
    pub fn save(&self) -> Result<(), String> {
        let config_path = Self::get_config_path();
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("序列化配置失败: {}", e))?;
        
        fs::write(&config_path, json)
            .map_err(|e| format!("写入配置文件失败: {}", e))?;
        
        println!("[config] 配置保存成功: {:?}", config_path);
        Ok(())
    }
    
    /// 更新配置并保存
    pub fn update<F>(&mut self, updater: F) -> Result<(), String>
    where
        F: FnOnce(&mut Self),
    {
        updater(self);
        self.save()
    }
    
    /// 重置为默认配置
    pub fn reset(&mut self) -> Result<(), String> {
        *self = Self::default();
        self.save()
    }
    
}

/// 全局配置实例
static mut GLOBAL_CONFIG: Option<AppConfig> = None;

/// 初始化全局配置
pub fn init_config() -> AppConfig {
    unsafe {
        if GLOBAL_CONFIG.is_none() {
            GLOBAL_CONFIG = Some(AppConfig::load());
        }
        GLOBAL_CONFIG.as_ref().unwrap().clone()
    }
}

/// 获取全局配置
pub fn get_config() -> AppConfig {
    unsafe {
        GLOBAL_CONFIG.as_ref().unwrap_or(&AppConfig::default()).clone()
    }
}

/// 更新全局配置
pub fn update_config<F>(updater: F) -> Result<(), String>
where
    F: FnOnce(&mut AppConfig),
{
    unsafe {
        if GLOBAL_CONFIG.is_none() {
            GLOBAL_CONFIG = Some(AppConfig::load());
        }
        
        if let Some(config) = &mut GLOBAL_CONFIG {
            config.update(updater)
        } else {
            Err("全局配置未初始化".to_string())
        }
    }
}

/// 保存全局配置
pub fn save_config() -> Result<(), String> {
    unsafe {
        if let Some(config) = &GLOBAL_CONFIG {
            config.save()
        } else {
            Err("全局配置未初始化".to_string())
        }
    }
} 