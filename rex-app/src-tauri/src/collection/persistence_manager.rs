use sqlx::SqlitePool;

use crate::{
    collection::scanned_file::ScannedFile,
};

pub struct PersistenceManager {
    pool: SqlitePool,
    buffer: Vec<ScannedFile>,
    batch_size: usize,
}

const FLUSH_LARGE_FILE_LIMIT: u64 = 10 * 1024 * 1024;

impl PersistenceManager {
    pub fn new(pool: SqlitePool, batch_size: usize) -> Self {
        Self {
            pool,
            buffer: Vec::with_capacity(batch_size),
            batch_size,
        }
    }

    pub async fn add(&mut self, file: ScannedFile) -> Result<(), String> {
        let is_large_file = file.fs_size > FLUSH_LARGE_FILE_LIMIT;

        self.buffer.push(file);

        if is_large_file || self.buffer.len() >= self.batch_size {
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
                (fs_path, inner_path, fs_size, fs_mtime, inner_size, fs_md5, inner_md5, rcheevos_hash, last_scanned)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)"
            )
            .bind(&file.fs_path)
            .bind(file.inner_path.as_deref().unwrap_or(""))
            .bind(file.fs_size as i64)
            .bind(file.fs_mtime.map(|mtime| mtime as i64))
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
