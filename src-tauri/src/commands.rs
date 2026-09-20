use serde::Serialize;
use tauri::{AppHandle, Manager};
use xcap::Monitor;

/// 单台显示器的信息（逻辑坐标 + 物理尺寸 + 缩放因子），
/// 供遮罩窗口按显示器坐标渲染截图与对齐选区。
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorInfo {
    pub id: u32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
    pub is_primary: bool,
}

/// 枚举所有显示器（xcap，跨平台 macOS/Windows/Linux）。
#[tauri::command]
pub fn capture_monitors() -> Result<Vec<MonitorInfo>, String> {
    let monitors = Monitor::all().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(monitors.len());
    for m in monitors {
        out.push(MonitorInfo {
            id: m.id().unwrap_or(0),
            name: m.friendly_name().unwrap_or_else(|_| m.name().unwrap_or_default()),
            x: m.x().unwrap_or(0),
            y: m.y().unwrap_or(0),
            width: m.width().unwrap_or(0),
            height: m.height().unwrap_or(0),
            scale_factor: m.scale_factor().unwrap_or(1.0),
            is_primary: m.is_primary().unwrap_or(false),
        });
    }
    Ok(out)
}

/// 截取全部显示器画面，按逻辑坐标合成一张位图，编码 PNG 返回。
/// 前端在该图上渲染遮罩、拖拽框选并裁剪，识别仍在 WebView 本地完成。
#[tauri::command]
pub fn capture_all() -> Result<Vec<u8>, String> {
    let monitors = Monitor::all().map_err(|e| e.to_string())?;
    if monitors.is_empty() {
        return Err("未检测到显示器".into());
    }

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    for m in &monitors {
        let x = m.x().unwrap_or(0);
        let y = m.y().unwrap_or(0);
        let w = m.width().unwrap_or(0) as i32;
        let h = m.height().unwrap_or(0) as i32;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x + w);
        max_y = max_y.max(y + h);
    }
    let canvas_w = (max_x - min_x) as u32;
    let canvas_h = (max_y - min_y) as u32;
    if canvas_w == 0 || canvas_h == 0 {
        return Err("显示器尺寸无效".into());
    }

    let mut canvas = image::RgbaImage::new(canvas_w, canvas_h);
    for m in &monitors {
        let shot = m.capture_image().map_err(|e| e.to_string())?;
        let x = (m.x().unwrap_or(0) - min_x) as u32;
        let y = (m.y().unwrap_or(0) - min_y) as u32;
        image::imageops::overlay(&mut canvas, &shot, x as i64, y as i64);
    }

    let mut png = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png);
    image::DynamicImage::ImageRgba8(canvas)
        .write_with_encoder(encoder)
        .map_err(|e| e.to_string())?;
    Ok(png)
}

/// 显示选区遮罩窗口（capture.html），并聚焦。
#[tauri::command]
pub fn capture_show(app: AppHandle) -> Result<(), String> {
    let win = app
        .get_webview_window("capture")
        .ok_or("选区窗口不存在")?;
    win.show().map_err(|e| e.to_string())?;
    win.set_focus().map_err(|e| e.to_string())?;
    Ok(())
}

/// 隐藏选区遮罩窗口（取消/完成后调用）。
#[tauri::command]
pub fn capture_hide(app: AppHandle) -> Result<(), String> {
    let win = app
        .get_webview_window("capture")
        .ok_or("选区窗口不存在")?;
    win.hide().map_err(|e| e.to_string())?;
    Ok(())
}
