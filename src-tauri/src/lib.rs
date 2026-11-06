mod commands;
mod db;
mod media;

use std::sync::Mutex;

use tauri::Manager;

use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            let media_dir = data_dir.join("media");
            std::fs::create_dir_all(&media_dir)?;
            let conn = db::open(&data_dir.join("quizzme.db"))?;
            // Startup is the only safe time to GC: no editor holds unsaved pasted images yet.
            media::gc(&media_dir, &db::referenced_media(&conn)?);
            app.manage(AppState {
                db: Mutex::new(conn),
                data_dir,
                media_dir,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_decks,
            commands::get_deck,
            commands::create_deck,
            commands::update_deck,
            commands::delete_deck,
            commands::list_cards,
            commands::get_card,
            commands::save_card,
            commands::delete_card,
            commands::save_media,
            commands::media_dir,
            commands::open_data_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
