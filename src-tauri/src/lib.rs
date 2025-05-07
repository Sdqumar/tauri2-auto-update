// Bring `updater()` into scope
use tauri_plugin_updater::UpdaterExt;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Register the updater plugin
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Only check for updates in release builds
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
    // Note the dot before `expect`
    match app_handle
        .updater()                       // Provided by UpdaterExt
        .expect("Failed to initialize updater") // Unwrap the Result
        .check()                        // Returns a Result<Update>
        .await
    {
        Ok(update) => {
            if update.is_update_available() {
                println!("Update available: {}", update.version());
                // Download & install handles its own errors
                if let Err(e) = update.download_and_install().await {
                    eprintln!("Error installing update: {}", e);
                }
            }
        }
        Err(e) => eprintln!("Error checking for updates: {}", e),
    }
}
