// 录屏相关功能模块
// 提供录屏的开始、停止命令，以及事件数据结构

use tauri::command;
use std::path::PathBuf;

use crate::recorder::*;
use crate::mouse_event::*;
use crate::mouse_listener::*;
use crate::video_overlay::*;

use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;

pub static mut MOUSE_LISTENER: Option<Arc<Mutex<Option<MouseListener>>>> = None;

pub static DESKTOP_RESOLUTION: OnceCell<(u32, u32)> = OnceCell::new();

#[command]
pub fn start_recording() -> Result<(), String> {
    start_ffmpeg_recording()?;
    // 启动全局鼠标监听器
    let listener = MouseListener::start();
    unsafe {
        MOUSE_LISTENER = Some(Arc::new(Mutex::new(Some(listener))));
    }
    #[cfg(target_os = "macos")]
    {
        use core_graphics::display::{CGMainDisplayID, CGDisplay};
        let display_id = unsafe { CGMainDisplayID() };
        let display = CGDisplay::new(display_id);
        let width = display.pixels_wide() as u32;
        let height = display.pixels_high() as u32;
       
        DESKTOP_RESOLUTION.set((width, height)).ok();
        println!("[start_recording] 桌面分辨率: {}x{} 已写入全局变量", width, height);
    }
    Ok(())
}

#[command]
pub fn stop_recording() -> Result<(), String> {
    tauri::async_runtime::spawn(async move {
        stop_ffmpeg_process();
        println!("临时目录: {:?}", std::env::temp_dir());
        let mut events_path = std::env::temp_dir();
    
        events_path.push("events.json");
        let events_path_str = events_path.to_str().unwrap().to_string();
        let (events, start_time) = unsafe {
            if let Some(listener_arc) = &MOUSE_LISTENER {
                let mut listener_opt = listener_arc.lock().unwrap();
                if let Some(listener) = listener_opt.as_mut() {
                    listener.stop();
                    let mouse_events = listener.get_events();
                    let start_time = listener.get_start_time();
                    *listener_opt = None;
                    (mouse_events, start_time)
                } else {
                    (vec![], 0)
                }
            } else {
                (vec![], 0)
            }
        };
    
        let _ = save_events_to_file(&events, &events_path_str);
        if !events.is_empty() {
            process_video_with_overlays(&events, &start_time);
        }
    });
 
    Ok(())
} 
