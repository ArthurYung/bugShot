// mouse_listener.rs
// 全局鼠标事件监听模块，负责监听系统级鼠标点击和移动事件，并收集事件数据

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use rdev::{listen, Event as RdevEvent, EventType as RdevEventType, Button};
use crate::mouse_event::MouseEvent;
// use crate::get_element::get_element_info_at;

/// 鼠标监听器，包含事件 Vec、停止信号和监听线程句柄
pub struct MouseListener {
    pub events: Arc<Mutex<Vec<MouseEvent>>>,
    pub start_time: Arc<Mutex<i64>>,
    stop_flag: Arc<AtomicBool>,
}

impl MouseListener {
    /// 创建并启动监听线程
    pub fn start() -> Self {
        let events = Arc::new(Mutex::new(Vec::new()));
        let start_time = Arc::new(Mutex::new(0));
        let stop_flag = Arc::new(AtomicBool::new(false));
        let events_clone = events.clone();
        let start_time_clone = start_time.clone();
        let stop_flag_clone = stop_flag.clone();
        let mut last_pos = (0, 0); // 记录最近一次鼠标位置
        let callback = move |event: RdevEvent| {
            if stop_flag_clone.load(Ordering::Relaxed) {
                return;
            }
            // 用锁访问 start_time
            if *start_time_clone.lock().unwrap() == 0 {
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
                    // 这里调用 get_element_info_at
                    // let element_info = get_element_info_at(x, y);
                    // 你可以把 element_info 加到 MouseEvent 结构体里，或者打印出来
                    // let (element_type, element_text) = element_info.unwrap_or((String::new(), String::new()));
                    let evt = MouseEvent {
                        r#type: "click".to_string(),
                        x,
                        y,
                        time: now,
                        button: btn,
                        // 比如新增字段 element_type/element_text
                        // element_type: element_info.as_ref().map(|(typ, _)| typ.clone()),
                        // element_text: element_info.as_ref().map(|(_, text)| text.clone()),
                        // element_type: if element_type.is_empty() { None } else { Some(element_type) },
                        // element_text: if element_text.is_empty() { None } else { Some(element_text) },
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
        Self {
            events,
            start_time,
            stop_flag,
        }
    }

    pub fn set_start_time(&self, start_time: i64) {
        *self.start_time.lock().unwrap() = start_time;
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
        *self.start_time.lock().unwrap()
    }
} 