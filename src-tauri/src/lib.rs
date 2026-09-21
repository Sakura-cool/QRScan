mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![commands::capture_all])
        .build(tauri::generate_context!())
        .unwrap_or_else(|e| {
            eprintln!("QRScan 启动失败: {e}");
            std::process::exit(1);
        });
    app.run(|_app_handle, _event| {});
}