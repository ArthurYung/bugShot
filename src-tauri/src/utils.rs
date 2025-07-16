// 工具相关功能模块
// 提供通用工具命令，如打招呼和检测 ffmpeg 是否安装

// Tauri 命令：打招呼，返回一段问候语
#[tauri::command]
pub fn greet(name: &str) -> String {
    println!("[greet] called with name: {}", name); // 打印收到的参数
    let msg = format!("Hello, {}! You've been greeted from Rust!", name); // 生成问候语
    println!("[greet] returning: {}", msg); // 打印返回内容
    msg
}

// Tauri 命令：检测 ffmpeg 是否已安装
// 通过运行 ffmpeg -version 判断命令是否可用
#[tauri::command]
pub fn check_ffmpeg_installed() -> bool {
    println!("[check_ffmpeg_installed] called"); // 打印调用日志
    let result = std::process::Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false); // 检查命令是否执行成功
    println!("[check_ffmpeg_installed] ffmpeg installed: {}", result); // 打印检测结果
    result
} 