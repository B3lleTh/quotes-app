use rusqlite::Connection;
use std::sync::Mutex;

pub struct Db(pub Mutex<Connection>);

pub fn db_path() -> std::path::PathBuf {
    let mut p = dirs::data_dir().unwrap();
    p.push("quotes-app");
    std::fs::create_dir_all(&p).ok();
    p.push("quotes.db");
    p
}

pub fn backup_dir() -> std::path::PathBuf {
    let mut p = dirs::data_dir().unwrap();
    p.push("quotes-app");
    p.push("backups");
    std::fs::create_dir_all(&p).ok();
    p
}

pub fn init_db(conn: &Connection) {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS quotes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL,
            text_norm TEXT NOT NULL,
            source TEXT,
            rating REAL NOT NULL DEFAULT 1000,
            likes INTEGER NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );
        CREATE UNIQUE INDEX IF NOT EXISTS idx_text_norm ON quotes(text_norm);",
    )
    .unwrap();
}

pub fn normalize(text: &str) -> String {
    text.trim().to_lowercase().split_whitespace().collect::<Vec<_>>().join(" ")
}
