use sqlx::SqlitePool;

use crate::collection::scanner::ScannedFile;

pub struct PersistenceManager {
    pool: SqlitePool,
    buffer: Vec<ScannedFile>,
    batch_size: usize,
}

impl PersistenceManager {
    pub fn new(pool: SqlitePool, batch_size: usize) -> Self {
        Self {
            pool,
            buffer: Vec::with_capacity(batch_size),
            batch_size,
        }
    }

    /// Adds a file to the buffer. If buffer is full, it flushes to DB.
    pub async fn add(&mut self, file: ScannedFile) -> Result<(), String> {
        self.buffer.push(file);
        if self.buffer.len() >= self.batch_size {
            self.flush().await?;
        }
        Ok(())
    }

    pub async fn flush(&mut self) -> Result<(), String> {
        if self.buffer.is_empty() { return Ok(()); }

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        for file in self.buffer.drain(..) {
            sqlx::query(
                "INSERT OR REPLACE INTO rex_collection_files
                (fs_path, inner_path, fs_size, inner_size, fs_md5, inner_md5, rcheevos_hash, last_scanned)
                VALUES (?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)"
            )
            .bind(&file.fs_path)
            .bind(file.inner_path.as_deref().unwrap_or(""))
            .bind(file.fs_size as i64)
            .bind(file.inner_size.map(|s| s as i64))
            .bind(&file.fs_md5)
            .bind(&file.inner_md5)
            .bind(&file.rcheevos_hash)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
