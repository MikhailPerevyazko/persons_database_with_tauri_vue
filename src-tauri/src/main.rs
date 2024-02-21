// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::NaiveDate;
use std::collections::HashMap;
use std::path::PathBuf;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![])
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
