// mouse_listener.rs
// 全局鼠标事件监听模块，负责监听系统级鼠标点击和移动事件，并收集事件数据

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use rdev::{listen, Event as RdevEvent, EventType as RdevEventType, Button};
use crate::mouse_event::MouseEvent;

/// 鼠标监听器，包含事件 Vec、停止信号和监听线程句柄
pub struct MouseListener {
    pub events: Arc<Mutex<Vec<MouseEvent>>>,
    pub start_time: i64,
    stop_flag: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl MouseListener {
    /// 创建并启动监听线程
    pub fn start() -> Self {
        let events = Arc::new(Mutex::new(Vec::new()));
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let stop_flag = Arc::new(AtomicBool::new(false));
        let events_clone = events.clone();
        let stop_flag_clone = stop_flag.clone();
        let mut last_pos = (0, 0); // 记录最近一次鼠标位置
        let callback = move |event: RdevEvent| {
            if stop_flag_clone.load(Ordering::Relaxed) {
                return;
            }
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            match event.event_type {
                RdevEventType::MouseMove { x, y } => {
                    last_pos = (x as i32, y as i32);
                    let evt = MouseEvent {
                        r#type: "move".to_string(),
                        x: last_pos.0,
                        y: last_pos.1,
                        time: now,
                        button: None,
                    };
                    if let Ok(mut vec) = events_clone.lock() {
                        vec.push(evt);
                    }
                }
                RdevEventType::ButtonPress(button) => {
                    let (x, y) = last_pos;
                    let btn = match button {
                        Button::Left => Some("left".to_string()),
                        Button::Right => Some("right".to_string()),
                        Button::Middle => Some("middle".to_string()),
                        _ => Some("other".to_string()),
                    };
                    let evt = MouseEvent {
                        r#type: "click".to_string(),
                        x,
                        y,
                        time: now,
                        button: btn,
                    };
                    if let Ok(mut vec) = events_clone.lock() {
                        vec.push(evt);
                    }
                }
                _ => {}
            }
        };
        if let Err(e) = listen(callback) {
            println!("[mouse_listener] error: {:?}", e);
        }
        println!("[mouse_listener] thread exiting");
        let handle = thread::spawn(move || {
            // 线程体
        });
        Self {
            events,
            start_time,
            stop_flag,
            handle: Some(handle),
        }
    }

    pub fn set_start_time(&mut self, start_time: i64) {
        self.start_time = start_time;
    }

    /// 停止监听线程
    pub fn stop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
        println!("[mouse_listener] set stop flag");
        // rdev 的 listen 是阻塞的，无法优雅终止，只能依赖 stop_flag 跳过事件处理
    }

    /// 获取所有事件的只读副本
    pub fn get_events(&self) -> Vec<MouseEvent> {
        self.events.lock().unwrap().clone()
    }

    /// 获取开始时间
    pub fn get_start_time(&self) -> i64 {
        self.start_time.clone()
    }
} 