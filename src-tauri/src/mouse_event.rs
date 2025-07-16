use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SimpleEvent {
    pub r#type: String,
    pub x: i32,
    pub y: i32,
    pub time: i64,
    pub button: Option<String>,
}

// MouseEvent 结构体只做类型转换用，不负责监听
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MouseEvent {
    pub r#type: String,
    pub x: i32,
    pub y: i32,
    pub time: i64,
    pub button: Option<String>,
}

// MouseEvent 转 SimpleEvent
impl From<MouseEvent> for SimpleEvent {
    fn from(e: MouseEvent) -> Self {
        SimpleEvent {
            r#type: e.r#type,
            x: e.x,
            y: e.y,
            time: e.time,
            button: e.button,
        }
    }
}

// 批量转换
pub fn mouse_events_to_simple(events: &[MouseEvent]) -> Vec<SimpleEvent> {
    events.iter().cloned().map(SimpleEvent::from).collect()
}

// 保存事件到文件
pub fn save_events_to_file(events: &Vec<SimpleEvent>, path: &str) -> Result<(), String> {
    let json = serde_json::to_string_pretty(events).map_err(|e| {
        let msg = e.to_string();
        println!("[save_events_to_file] event serialization error: {}", msg);
        msg
    })?;
    match fs::write(path, json) {
        Ok(_) => {
            println!("[save_events_to_file] events.json written to: {}", path);
            Ok(())
        },
        Err(e) => {
            println!("[save_events_to_file] failed to write events.json: {}", e);
            Err(e.to_string())
        }
    }
} 