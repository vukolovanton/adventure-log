// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod utils;
use utils::CanvasElement;

use crate::utils::{read_file, update_file, write_file, Note};
use std::collections::HashMap;
use std::path::{PathBuf};

#[tauri::command]
fn get_path(handle: &tauri::AppHandle) -> PathBuf {
    let resource_path = handle
        .path_resolver()
        .resolve_resource("data.json")
        .expect("failed to resolve resource");
    return resource_path;
}

#[tauri::command]
fn get_all_notes(handle: tauri::AppHandle) -> HashMap<String, Note> {
    let path = get_path(&handle);
    let data = read_file(&path);

    let user_notes = match data {
        Ok(d) => d,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    return user_notes;
}

#[tauri::command]
fn save_note(
    handle: tauri::AppHandle,
    id: String,
    title: String,
    description: String,
    tags: Vec<String>,
    canvas: Option<CanvasElement>,
) {
    let path = get_path(&handle);
    let note = Note {
        id,
        title,
        description,
        tags,
        canvas,
    };

    let data = read_file(&path);

    let mut user_notes = match data {
        Ok(d) => d,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    update_file(&mut user_notes, note);

    match write_file(&path, &user_notes) {
        Ok(_) => (),
        Err(error) => panic!("Problem writting the file: {:?}", error),
    };
}

#[tauri::command]
fn delete_note(handle: tauri::AppHandle, id: String) {
    let path = get_path(&handle);
    let mut notes = get_all_notes(handle);
    notes.remove(&id);

    match write_file(&path, &notes) {
        Ok(_) => (),
        Err(error) => panic!("Problem writting the file: {:?}", error),
    };
}

#[tauri::command]
fn import_notes(handle: tauri::AppHandle, notes: HashMap<String, Note>) {
    let path = get_path(&handle);
    match write_file(&path, &notes) {
        Ok(_) => (),
        Err(error) => panic!("Problem writting the file: {:?}", error),
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_note,
            get_all_notes,
            delete_note,
            import_notes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
