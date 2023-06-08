// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod utils;
use crate::utils::{read_file, update_file, write_file, Note};
use std::collections::HashMap;
use std::path::Path;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_all_notes() -> HashMap<String, Note> {
    let path = Path::new("./files/data.json");
    let data = read_file(path);

    let user_notes = match data {
        Ok(d) => d,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    return user_notes;
}

#[tauri::command]
fn save_note(id: String, title: String, description: String, tags: Vec<String>) {
    let path = Path::new("./files/data.json");
    let note = Note {
        id,
        title,
        description,
        tags,
    };

    let data = read_file(path);
    println!("{:?}", data);

    let mut user_notes = match data {
        Ok(d) => d,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    update_file(&mut user_notes, note);

    match write_file(path, &user_notes) {
        Ok(_) => (),
        Err(error) => panic!("Problem writting the file: {:?}", error),
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, save_note, get_all_notes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
