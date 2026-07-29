use crate::db::{normalize, Db};
use crate::models::Quote;
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub fn add_quote(db: State<Db>, text: String, source: Option<String>) -> Result<Quote, String> {
    let text = text.trim().to_string();
    if text.is_empty() {
        return Err("La frase está vacía".into());
    }
    let norm = normalize(&text);
    let conn = db.0.lock().unwrap();

    let exists: bool = conn
        .query_row("SELECT 1 FROM quotes WHERE text_norm=?1", [&norm], |_| Ok(true))
        .unwrap_or(false);
    if exists {
        return Err("DUPLICATE".into());
    }

    conn.execute(
        "INSERT INTO quotes (text, text_norm, source) VALUES (?1, ?2, ?3)",
        params![text, norm, source],
    )
    .map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(Quote { id, text, source, rating: 1000.0, likes: 0 })
}

#[tauri::command]
pub fn get_all(db: State<Db>) -> Result<Vec<Quote>, String> {
    let conn = db.0.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, text, source, rating, likes FROM quotes ORDER BY rating DESC")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], row_to_quote)
        .map_err(|e| e.to_string())?;
    Ok(rows.filter_map(|r| r.ok()).collect())
}

#[tauri::command]
pub fn get_random(db: State<Db>) -> Result<Option<Quote>, String> {
    let conn = db.0.lock().unwrap();
    conn.query_row(
        "SELECT id, text, source, rating, likes FROM quotes ORDER BY RANDOM() LIMIT 1",
        [],
        row_to_quote,
    )
    .map(Some)
    .or_else(|e| if e == rusqlite::Error::QueryReturnedNoRows { Ok(None) } else { Err(e.to_string()) })
}

#[tauri::command]
pub fn get_random_pair(db: State<Db>) -> Result<Vec<Quote>, String> {
    let conn = db.0.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, text, source, rating, likes FROM quotes ORDER BY RANDOM() LIMIT 2")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], row_to_quote)
        .map_err(|e| e.to_string())?;
    Ok(rows.filter_map(|r| r.ok()).collect())
}

fn row_to_quote(r: &rusqlite::Row) -> rusqlite::Result<Quote> {
    Ok(Quote {
        id: r.get(0)?,
        text: r.get(1)?,
        source: r.get(2)?,
        rating: r.get(3)?,
        likes: r.get(4)?,
    })
}

fn elo_update(winner: f64, loser: f64) -> (f64, f64) {
    let k = 32.0;
    let expected_w = 1.0 / (1.0 + 10f64.powf((loser - winner) / 400.0));
    let expected_l = 1.0 - expected_w;
    (winner + k * (1.0 - expected_w), loser + k * (0.0 - expected_l))
}

#[tauri::command]
pub fn vote_pair(db: State<Db>, winner_id: i64, loser_id: i64) -> Result<(), String> {
    let conn = db.0.lock().unwrap();
    let winner_r: f64 = conn
        .query_row("SELECT rating FROM quotes WHERE id=?1", [winner_id], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    let loser_r: f64 = conn
        .query_row("SELECT rating FROM quotes WHERE id=?1", [loser_id], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    let (nw, nl) = elo_update(winner_r, loser_r);
    conn.execute("UPDATE quotes SET rating=?1 WHERE id=?2", params![nw, winner_id])
        .map_err(|e| e.to_string())?;
    conn.execute("UPDATE quotes SET rating=?1 WHERE id=?2", params![nl, loser_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn like_quote(db: State<Db>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().unwrap();
    conn.execute(
        "UPDATE quotes SET likes = likes + 1, rating = rating + 15 WHERE id=?1",
        [id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_quote(db: State<Db>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().unwrap();
    conn.execute("DELETE FROM quotes WHERE id=?1", [id]).map_err(|e| e.to_string())?;
    Ok(())
}
