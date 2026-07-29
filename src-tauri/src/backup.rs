use crate::db::{backup_dir, Db};
use crate::models::Quote;
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub fn export_backup(db: State<Db>) -> Result<String, String> {
    let conn = db.0.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, text, source, rating, likes FROM quotes ORDER BY id")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| {
            Ok(Quote {
                id: r.get(0)?,
                text: r.get(1)?,
                source: r.get(2)?,
                rating: r.get(3)?,
                likes: r.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let all: Vec<Quote> = rows.filter_map(|r| r.ok()).collect();
    let json = serde_json::to_string_pretty(&all).map_err(|e| e.to_string())?;

    let timestamp = chrono_stamp();
    let mut path = backup_dir();
    path.push(format!("backup-{}.json", timestamp));
    std::fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn import_backup(db: State<Db>, json: String) -> Result<i64, String> {
    let items: Vec<Quote> = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    let conn = db.0.lock().unwrap();
    let mut count = 0;
    for q in items {
        let norm = crate::db::normalize(&q.text);
        let res = conn.execute(
            "INSERT OR IGNORE INTO quotes (text, text_norm, source, rating, likes) VALUES (?1,?2,?3,?4,?5)",
            params![q.text, norm, q.source, q.rating, q.likes],
        );
        if let Ok(n) = res {
            count += n as i64;
        }
    }
    Ok(count)
}

fn chrono_stamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    now.to_string()
}
