#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backup;
mod commands;
mod db;
mod models;

use db::Db;
use rusqlite::Connection;
use std::sync::Mutex;

fn main() {
    let conn = Connection::open(db::db_path()).expect("no se pudo abrir la db");
    db::init_db(&conn);

    tauri::Builder::default()
        .manage(Db(Mutex::new(conn)))
        .invoke_handler(tauri::generate_handler![
            commands::add_quote,
            commands::get_all,
            commands::get_random,
            commands::get_random_pair,
            commands::vote_pair,
            commands::like_quote,
            commands::delete_quote,
            backup::export_backup,
            backup::import_backup,
        ])
        .run(tauri::generate_context!())
        .expect("error al correr la app");
}
