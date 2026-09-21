use xcap::Monitor;

/// 截取全部显示器画面，按逻辑坐标合成一张位图，编码 PNG 返回。
/// 前端直接将该截图作为输入，识别全部二维码（图片不出本地）。
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