// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::NaiveDate;
use std::collections::HashMap;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
