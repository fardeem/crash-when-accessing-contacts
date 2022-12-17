#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rust_macios::{
    contacts::{CNContactStore, CNEntityType},
    foundation::NSError,
    objective_c_runtime::traits::PNSObject,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn request_permission() {
    let store = CNContactStore::m_new();

    store.request_access_for_entity_type_completion_handler(
        CNEntityType::Contacts,
        move |granted: bool, _error: NSError| {
            if granted {
                println!("Access granted");
            } else {
                println!("Access denied");
            }
        },
    );
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![request_permission])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
