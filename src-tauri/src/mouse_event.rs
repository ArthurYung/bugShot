use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MouseEvent {
    pub r#type: String,
    pub x: i32,
    pub y: i32,
    pub time: i64,
    pub button: Option<String>,
}

// 保存事件到文件
pub fn save_events_to_file(events: &Vec<MouseEvent>, path: &str) -> Result<(), String> {
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