use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::thread;
use std::io::BufRead;
use std::io::BufReader;
use std::time::{SystemTime, UNIX_EPOCH};

pub static mut RECORDING_CHILD: Option<Mutex<Child>> = None;

// 获取 ffmpeg 可执行文件的路径
pub fn get_ffmpeg_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        PathBuf::from("ffmpeg.exe")
    } else {
        PathBuf::from("ffmpeg")
    }
}

// 启动 ffmpeg 录屏进程
pub fn start_ffmpeg_recording() -> Result<(), String> {
    println!("临时目录: {:?}", std::env::temp_dir());
    println!("[start_recording] called");
    let ffmpeg_path = get_ffmpeg_path();
    println!("[start_recording] ffmpeg_path: {:?}", ffmpeg_path);
    let mut cmd = Command::new(ffmpeg_path);
    #[cfg(target_os = "macos")]
    {
        println!("[start_recording] using avfoundation for macOS");
        let mut output_path = std::env::temp_dir();
        output_path.push("output.mp4");
        let output_path_str = output_path.to_str().unwrap();
        cmd.args([
            "-y",
            "-f",
            "avfoundation",
            "-framerate",
            "30",
            "-i",
            "3:none",
            "-c:v",
            "libx264",
            "-crf",
            "30",
            "-preset",
            "slow",
            "-pix_fmt",
            "yuv420p",
            "-vf",
            "scale=1920:-2",
            "-an",
            output_path_str,
        ]);
    }
    #[cfg(target_os = "windows")]
    {
        println!("[start_recording] using gdigrab for Windows");
        let mut output_path = std::env::temp_dir();
        output_path.push("output.mp4");
        let output_path_str = output_path.to_str().unwrap();
        cmd.args([
            "-y",
            "-f",
            "gdigrab",
            "-framerate",
            "30",
            "-i",
            "desktop",
            "-video_size",
            "1920x1080",
            output_path_str,
        ]);
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| {
        let msg = format!("启动录屏失败: {}", e);
        println!("[start_recording] error: {}", msg);
        msg
    })?;

    // 监听 stderr
    let stderr = child.stderr.take().unwrap();
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            let line = line.unwrap_or_default();
            // 你可以根据实际 ffmpeg 输出内容调整关键字
            if line.contains("Press [q] to stop") || line.contains("frame=") {
                // 记录此刻时间
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64;
                // 更新 listener 的 start_time
                unsafe {
                    if let Some(listener_arc) = &crate::record::MOUSE_LISTENER {
                        let mut listener_opt = listener_arc.lock().unwrap();
                        if let Some(listener) = listener_opt.as_mut() {
                            listener.set_start_time(now);
                        }
                    }
                }
                break; // 只需要第一次
            }
        }
    });

    unsafe {
        RECORDING_CHILD = Some(Mutex::new(child));
    }
    println!("[start_recording] ffmpeg started");
    Ok(())
}

// 停止 ffmpeg 录屏进程
pub fn stop_ffmpeg_process() {
    unsafe {
        if let Some(lock) = &RECORDING_CHILD {
            let mut child = lock.lock().unwrap();
            println!("[stop_recording] got child process, pid={}", child.id());
            println!("[debug] 主进程 PID: {}", std::process::id());
            #[cfg(target_os = "macos")]
            {
                println!("[stop_recording] sending SIGINT to ffmpeg (macOS)");
                let _ = Command::new("kill")
                    .arg("-INT")
                    .arg(child.id().to_string())
                    .output();
            }
            #[cfg(target_os = "windows")]
            {
                println!("[stop_recording] killing ffmpeg (Windows)");
                let _ = child.kill();
            }
            let _ = child.wait();
            println!("[stop_recording] ffmpeg process stopped");
        } else {
            println!("[stop_recording] no ffmpeg process found");
        }
    }
}
