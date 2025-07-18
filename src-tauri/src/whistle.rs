// whistle.rs
// Whistle 代理服务器管理模块

use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use std::thread;

pub static mut WHISTLE_PROCESS: Option<Mutex<Child>> = None;


#[derive(Debug, Serialize, Deserialize)]
pub struct WhistleConfig {
    pub port: u16,
    pub subdomain: String,
}


/// 检查 Node.js 是否已安装
pub fn check_nodejs() -> bool {
    Command::new("node").arg("-v").output().is_ok()
}

/// 检查 Whistle 是否已安装
pub fn check_whistle() -> bool {
    Command::new("w2").arg("-v").output().is_ok()
}

/// 安装 Whistle
pub fn install_whistle() -> Result<(), String> {
    println!("[whistle] 正在安装 whistle...");
    let status = Command::new("npm")
        .args(&["install", "-g", "whistle"])
        .status()
        .map_err(|e| format!("npm 命令执行失败: {}", e))?;
    
    if status.success() {
        println!("[whistle] whistle 安装成功");
        Ok(())
    } else {
        Err("whistle 安装失败，请检查 Node.js 和 npm 是否可用".to_string())
    }
}

/// 启动 Whistle 进程
pub fn start_whistle(port: &u16, subdomain: &String) -> Result<(), String> {
    // 检查 Node.js
    if !check_nodejs() {
        return Err("Node.js 未安装，请先安装 Node.js".to_string());
    }
    
    // 检查并安装 Whistle
    if !check_whistle() {
        install_whistle()?;
    }
    
    // 构建启动命令
    let mut cmd = Command::new("w2");
    cmd.arg("start");
    cmd.arg("-S");
    cmd.arg(&subdomain);
    cmd.arg("-C");
    cmd.arg("-p");
    cmd.arg(&port.to_string());
    
    
    // 从本地config读取用户配置的抓包规则
    // for rule in &config.rules {
    //     cmd.arg("-r");
    //     cmd.arg(rule);
    // }
    
    // 设置存储文件
    // if let Some(storage) = &config.storage {
    //     cmd.arg("-S");
    //     cmd.arg(storage);
    // }
    
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    
    println!("[whistle] 启动命令: {:?}", cmd);
    
    let child = cmd.spawn().map_err(|e| format!("启动 whistle 失败: {}", e))?;
    
    unsafe {
        WHISTLE_PROCESS = Some(Mutex::new(child));
    }
    
    println!("[whistle] whistle 进程已启动，端口: {}", port);
    Ok(())
}

/// 停止 Whistle 进程
pub fn stop_whistle() -> Result<(), String> {
    unsafe {
        if let Some(process_mutex) = &WHISTLE_PROCESS {
            let mut process = process_mutex.lock().unwrap();
            println!("[whistle] 正在停止 whistle 进程...");
            
            #[cfg(target_os = "macos")]
            {
                let _ = Command::new("kill")
                    .arg("-TERM")
                    .arg(process.id().to_string())
                    .output();
            }
            
            #[cfg(target_os = "windows")]
            {
                let _ = process.kill();
            }
            
            let _ = process.wait();
            println!("[whistle] whistle 进程已停止");
        }
        WHISTLE_PROCESS = None;
    }
    Ok(())
}

/// 设置系统代理
pub fn set_system_proxy(port: &u16) -> Result<(), String> {
    // 1. 启动代理（同步，立即返回）
    let _ = Command::new("w2")
        .args(["proxy", &port.to_string()])
        .output();
    println!("[whistle] 系统代理已设置");

    // 2. 异步获取 whistle 配置并合并规则
    let port = *port;
    thread::spawn(move || {
        if let Ok(resp) = reqwest::blocking::get(format!("http://127.0.0.1:{}/cgi-bin/rules/list", port)) {
            if let Ok(existing_rules) = resp.json::<Vec<String>>() {
                // 合并规则
                let mut all_rules = existing_rules;
                // all_rules.push(user_rules);
                // 这里可以发请求设置新规则，或保存到本地
                println!("[whistle] 合并后的规则: {:?}", all_rules);
            }
        }
    });

    Ok(())
}

/// 清除系统代理
pub fn clear_system_proxy() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("networksetup")
            .args(["-setwebproxystate", "Wi-Fi", "off"])
            .output();
        let _ = Command::new("networksetup")
            .args(["-setsecurewebproxystate", "Wi-Fi", "off"])
            .output();
        println!("[whistle] macOS 系统代理已清除");
    }
    
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("netsh")
            .args(["winhttp", "reset", "proxy"])
            .output();
        println!("[whistle] Windows 系统代理已清除");
    }
    
    Ok(())
}

/// 检查 Whistle 是否正在运行
pub fn is_whistle_running() -> bool {
    unsafe {
        WHISTLE_PROCESS.is_some()
    }
} 