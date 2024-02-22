// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::NaiveDate;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::Manager;
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![open_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone, Debug)]
pub struct Person {
    pub name: String,
    pub surname: String,
    pub middle_name: String,
    pub date_of_birth: NaiveDate,
    pub gender: bool,
}

#[derive(Clone, Debug)]
pub struct PersonStorage {
    persons: HashMap<i32, Person>,
}
#[tauri::command]
fn open_db(state: tauri::State<AppState>, file_path: String) {
    println!("{file_path}");
    state.set_file_path(file_path);
    println!("{:?}", state.get_file_path());
}

struct AppState {
    file_path: Mutex<PathBuf>,
    person_storage: PersonStorage,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            file_path: Mutex::new(PathBuf::new()),
        }
    }
    pub fn set_file_path(&self, file_path: String) {
        {
            let mut lock = self.file_path.lock().unwrap();
            *lock = PathBuf::from(file_path);
        }
    }
    pub fn get_file_path(&self) -> PathBuf {
        let result = {
            let lock = self.file_path.lock().unwrap();
            lock.to_owned()
        };
        result
    }
}
