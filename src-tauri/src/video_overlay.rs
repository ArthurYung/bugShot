use std::process::Command;
use crate::mouse_event::MouseEvent;
use crate::record::DESKTOP_RESOLUTION;

pub fn process_video_with_overlays(events: &Vec<MouseEvent>, start_time: &i64) {
    if events.is_empty() {
        println!("[process_video_with_overlays] no events to process");
        return;
    }
    // 1. 输入输出路径
    let mut input_path = std::env::temp_dir();
    input_path.push("output.mp4");
    let input_path_str = input_path.to_str().unwrap();
    let mut output_path = std::env::temp_dir();
    output_path.push("output_with_circle.mp4");
    let output_path_str = output_path.to_str().unwrap();
    // 2. 圆圈图片路径
    let mut circle_path = std::env::current_dir().unwrap();
    circle_path.push("assets");
    circle_path.push("circle_small.png");
    let circle_path_str = circle_path.to_str().unwrap();
    // 2.1 鼠标指针图片路径
    let mut arrow_path = std::env::current_dir().unwrap();
    arrow_path.push("assets");
    arrow_path.push("arrow.png");
    let arrow_path_str = arrow_path.to_str().unwrap();
    // 3. 读取桌面分辨率（优先全局变量）
    let (desktop_width, desktop_height) = DESKTOP_RESOLUTION.get().copied().map(|(w, h)| (w as f64, h as f64)).unwrap_or((1920.0, 1080.0));
    // 4. 用 ffprobe 获取 output.mp4 实际分辨率
    let ffprobe_output = Command::new("ffprobe")
        .args(["-v", "error", "-select_streams", "v:0", "-show_entries", "stream=width,height", "-of", "csv=s=x:p=0", input_path_str])
        .output();
    let (video_width, video_height) = if let Ok(out) = ffprobe_output {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout);
            let parts: Vec<&str> = s.trim().split('x').collect();
            if parts.len() == 2 {
                if let (Ok(w), Ok(h)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                    (w, h)
                } else {
                    (desktop_width, desktop_height)
                }
            } else {
                (desktop_width, desktop_height)
            }
        } else {
            (desktop_width, desktop_height)
        }
    } else {
        (desktop_width, desktop_height)
    };
    let scale_x = video_width / desktop_width;
    let scale_y = video_height / desktop_height;
    // 5. 生成 overlay filter，按桌面分辨率等比缩放
    let mut overlays = Vec::new();
    overlays.push("[0:v][1]overlay=x=0:y=0:enable='lt(t,0)'[v0]".to_string()); // dummy 起始流
    // 1. 叠加点击圆圈特效（只处理 click 事件）
    let click_events: Vec<_> = events.iter().filter(|e| e.r#type == "click").collect();
    let mut overlay_idx = 0;
    for (i, e) in click_events.iter().enumerate() {
        let t_start = (e.time - start_time) as f64 / 1000.0;
        let t_end = t_start + 0.1;
        let x = (e.x as f64 * scale_x).round() as i32 - 16;
        let y = (e.y as f64 * scale_y).round() as i32 - 16;
        overlays.push(format!(
            "[v{idx}][1]overlay=x={x}:y={y}:enable='between(t,{t_start},{t_end})'[v{next}]",
            idx = overlay_idx,
            next = overlay_idx + 1,
            x = x,
            y = y,
            t_start = t_start,
            t_end = t_end
        ));
        overlay_idx += 1;
    }
    // 2. 叠加鼠标指针（每一帧 move 事件）
    let move_events: Vec<_> = events.iter().filter(|e| e.r#type == "move").collect();
    for (j, e) in move_events.iter().enumerate() {
        let t = (e.time - start_time) as f64 / 1000.0;
        let t_next = if j + 1 < move_events.len() {
            (move_events[j + 1].time - start_time) as f64 / 1000.0
        } else {
            t + 1.0
        };
        println!("move event {}: e.time={}, start_time={}, t={}", j, e.time, start_time, t);
        let x = (e.x as f64 * scale_x).round() as i32 - 10;
        let y = (e.y as f64 * scale_y).round() as i32 - 10;
        overlays.push(format!(
            "[v{idx}][2]overlay=x={x}:y={y}:enable='between(t,{t},{t_next})'[v{next}]",
            idx = overlay_idx,
            next = overlay_idx + 1,
            x = x,
            y = y,
            t = t,
            t_next = t_next
        ));
        overlay_idx += 1;
    }
    let final_stream = format!("[v{}]", overlay_idx);
    let filter_complex = overlays.join(";");
    let ffmpeg_args = vec![
        "-y",
        "-i", input_path_str,
        "-i", circle_path_str,
        "-i", arrow_path_str,
        "-filter_complex", &filter_complex,
        "-map", &final_stream,
        "-codec:a", "copy",
        output_path_str
    ];
    let ffmpeg_status = Command::new("ffmpeg")
        .args(&ffmpeg_args)
        .status();
    match ffmpeg_status {
        Ok(status) if status.success() => {
            println!("[process_video_with_overlays] output_with_circle.mp4 written to temp dir: {}", output_path_str);
        }
        Ok(status) => {
            println!("[process_video_with_overlays] ffmpeg exited with status: {}", status);
        }
        Err(e) => {
            println!("[process_video_with_overlays] failed to run ffmpeg for circle: {}", e);
        }
    }
} 