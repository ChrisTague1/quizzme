use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

use rusqlite::Connection;
use tauri::State;

use crate::db::{self, Card, CardInput, Deck};
use crate::media;

pub struct AppState {
    pub db: Mutex<Connection>,
    pub data_dir: PathBuf,
    pub media_dir: PathBuf,
}

type Result<T, E = String> = std::result::Result<T, E>;

fn conn<'a>(state: &'a State<'_, AppState>) -> Result<MutexGuard<'a, Connection>> {
    state.db.lock().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_decks(state: State<'_, AppState>) -> Result<Vec<Deck>> {
    db::list_decks(&*conn(&state)?)
}

#[tauri::command]
pub fn get_deck(state: State<'_, AppState>, id: i64) -> Result<Deck> {
    db::get_deck(&*conn(&state)?, id)
}

#[tauri::command]
pub fn create_deck(state: State<'_, AppState>, title: String, description: String) -> Result<i64> {
    db::create_deck(&*conn(&state)?, &title, &description)
}

#[tauri::command]
pub fn update_deck(
    state: State<'_, AppState>,
    id: i64,
    title: String,
    description: String,
) -> Result<()> {
    db::update_deck(&*conn(&state)?, id, &title, &description)
}

#[tauri::command]
pub fn delete_deck(state: State<'_, AppState>, id: i64) -> Result<()> {
    db::delete_deck(&*conn(&state)?, id)
}

#[tauri::command]
pub fn list_cards(state: State<'_, AppState>, deck_id: i64) -> Result<Vec<Card>> {
    db::list_cards(&*conn(&state)?, deck_id)
}

#[tauri::command]
pub fn get_card(state: State<'_, AppState>, id: i64) -> Result<Card> {
    db::get_card(&*conn(&state)?, id)
}

#[tauri::command]
pub fn save_card(state: State<'_, AppState>, card: CardInput) -> Result<i64> {
    db::save_card(&mut *conn(&state)?, &card)
}

#[tauri::command]
pub fn delete_card(state: State<'_, AppState>, id: i64) -> Result<()> {
    db::delete_card(&mut *conn(&state)?, id)
}

/// Raw-body IPC: image bytes arrive as the request body, extension in the `x-ext` header.
#[tauri::command]
pub fn save_media(request: tauri::ipc::Request<'_>, state: State<'_, AppState>) -> Result<String> {
    let tauri::ipc::InvokeBody::Raw(bytes) = request.body() else {
        return Err("Expected raw image bytes".into());
    };
    let ext = request
        .headers()
        .get("x-ext")
        .and_then(|v| v.to_str().ok())
        .ok_or("Missing x-ext header")?;
    media::save(&state.media_dir, bytes, ext)
}

#[tauri::command]
pub fn media_dir(state: State<'_, AppState>) -> String {
    format!("{}{}", state.media_dir.display(), std::path::MAIN_SEPARATOR)
}

#[tauri::command]
pub fn open_data_dir(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<()> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .open_path(state.data_dir.to_string_lossy(), None::<&str>)
        .map_err(|e| e.to_string())
}
