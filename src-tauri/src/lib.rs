// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Check for updates when the app starts
            #[cfg(not(debug_assertions))]
            {
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    check_update(&app_handle).await;
                });
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[cfg(not(debug_assertions))]
async fn check_update(app_handle: &tauri::AppHandle) {
    use tauri_plugin_updater::UpdaterExt;
    
    // Check for updates
    match app_handle.updater().check().await {
        Ok(update) => {
            if update.is_update_available() {
                // Update is available, can prompt the user or automatically download
                println!("Update available: {}", update.version());
                
                // Download and install the update
                match update.download_and_install().await {
                    Ok(_) => println!("Update installed successfully"),
                    Err(e) => eprintln!("Error installing update: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error checking for updates: {}", e),
    }
}