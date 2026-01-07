use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;

pub async fn init_database(app_path: &std::path::Path) -> Result<SqlitePool, String> {
    let db_path = app_path.join("rex.db");
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());

    let opts = SqliteConnectOptions::from_str(&db_url)
        .map_err(|e| e.to_string())?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await
        .map_err(|e| e.to_string())?;

    Ok(pool)
}
