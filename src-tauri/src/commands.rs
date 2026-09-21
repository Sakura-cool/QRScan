use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use xcap::Monitor;

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

    let all_black = canvas
        .pixels()
        .all(|p| p[0] == 0 && p[1] == 0 && p[2] == 0);
    if all_black {
        return Err("屏幕录制权限未授权：请在 系统设置 → 隐私与安全性 → 屏幕录制 中允许 QRScan 后重试".into());
    }

    let mut png = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png);
    image::DynamicImage::ImageRgba8(canvas)
        .write_with_encoder(encoder)
        .map_err(|e| e.to_string())?;
    Ok(png)
}

/// 显示选区遮罩窗口并铺满主显示器（set_size+set_position，不切换 Space）。
/// 显示后定向通知 capture 窗口开始截屏。
#[tauri::command]
pub fn capture_show(app: AppHandle) -> Result<(), String> {
    let win = app
        .get_webview_window("capture")
        .ok_or("选区窗口不存在")?;

    let monitors = Monitor::all().map_err(|e| e.to_string())?;
    let primary = monitors
        .iter()
        .find(|m| m.is_primary().unwrap_or(false))
        .or_else(|| monitors.first())
        .ok_or("未检测到显示器")?;
    let px = primary.x().unwrap_or(0);
    let py = primary.y().unwrap_or(0);
    let pw = primary.width().unwrap_or(0);
    let ph = primary.height().unwrap_or(0);

    win.set_size(tauri::LogicalSize::new(pw as f64, ph as f64))
        .map_err(|e| e.to_string())?;
    win.set_position(tauri::LogicalPosition::new(px as f64, py as f64))
        .map_err(|e| e.to_string())?;
    win.show().map_err(|e| e.to_string())?;
    win.set_focus().map_err(|e| e.to_string())?;

    app.emit_to("capture", "capture:start", ())
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn capture_hide(app: AppHandle) -> Result<(), String> {
    let win = app
        .get_webview_window("capture")
        .ok_or("选区窗口不存在")?;
    win.hide().map_err(|e| e.to_string())?;
    Ok(())
}
