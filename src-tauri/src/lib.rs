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
                    check_update(app_handle).await;
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
async fn check_update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
  if let Some(update) = app.updater()?.check().await? {
    update
      .download_and_install(|c, t| println!("Downloaded {c} / {t:?}"), || println!("Done"))
      .await?;
    println!("Update installed successfully");
    app.restart();
  } else {
    println!("No update available");
  }
  Ok(())
}

