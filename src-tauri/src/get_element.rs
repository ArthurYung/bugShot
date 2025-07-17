// get_element.rs
// 跨平台获取指定坐标下控件类型和文字内容

#[cfg(target_os = "windows")]
pub fn get_element_info_at(x: i32, y: i32) -> Option<(String, String)> {
    use uiautomation::{UIAutomation, ControlType};
    let automation = UIAutomation::new().ok()?;
    let element = automation.element_from_point(x, y).ok()?;
    let control_type = element.get_control_type().ok()?;
    let name = element.get_name().unwrap_or_default();
    let control_type_str = match control_type {
        ControlType::Button => "Button",
        ControlType::Edit => "Edit",
        ControlType::Text => "Text",
        _ => "Other",
    }.to_string();
    Some((control_type_str, name))
}

#[cfg(target_os = "macos")]
pub fn get_element_info_at(x: i32, y: i32) -> Option<(String, String)> {
    use std::process::Command;
    use std::path::PathBuf;

    let mut ax_query_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    ax_query_path.push("src/bin/ax_query");
    println!("ax_query_path: {:?}", ax_query_path);

    let output = Command::new(ax_query_path)
        .arg(x.to_string())
        .arg(y.to_string())
        .output()
        .ok()?;
    println!("ax_query output: {:?}", output);
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).ok()?;
    let typ = v.get("type")?.as_str()?.to_string();
    let text = v.get("text")?.as_str()?.to_string();
    Some((typ, text))
} 