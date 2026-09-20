#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .unwrap_or_else(|e| {
            eprintln!("QRScan 启动失败: {e}");
            std::process::exit(1);
        });
    app.run(|_app_handle, _event| {});
}
