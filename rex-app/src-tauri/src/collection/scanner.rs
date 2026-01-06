use std::{
    path::Path,
    fs::File,
    io::{self, BufReader, Read, Seek},
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
    collections::HashMap,
};

use walkdir::WalkDir;
use globset::{Glob, GlobSetBuilder};
use zip::ZipArchive;
use md5::Context;
use rayon::prelude::*;
use sqlx::{sqlite::SqlitePool, Row};

use tauri::{Window, Emitter, AppHandle, Manager};

use crate::{
    collection::persistence_manager::PersistenceManager,
    collection::scanned_file::ScannedFile,
    collection::NormalizePath,
    collection::rcheevos,
};

const BUFFER_SIZE: usize = 128 * 1024;
const PERSISTENCE_BATCH_SIZE: usize = 20;
const PROGRESS_BATCH_SIZE: usize = 1;
const RCHEEVOS_SCAN_FILE_SIZE_LIMIT: u64 = 1024 * 1024 * 1024;

fn calculate_md5_from_reader<R: Read>(reader: R) -> io::Result<String> {
    let mut buffered_reader = BufReader::with_capacity(BUFFER_SIZE, reader);
    let mut context = Context::new();
    let mut buffer = vec![0u8; BUFFER_SIZE];

    loop {
        let count = buffered_reader.read(&mut buffer)?;
        if count == 0 { break; }
        context.consume(&buffer[..count]);
    }

    Ok(format!("{:x}", context.finalize()))
}

fn process_single_file(
    source: ScannedFile,
) -> Option<Vec<ScannedFile>> {
    let path = source.path;
    let mut file = File::open(&path).ok()?;

    let fs_md5 = calculate_md5_from_reader(&file).ok();

    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    if extension.to_lowercase() == "zip" {
        let mut zip_results = Vec::new();
        if let Ok(mut archive) = ZipArchive::new(file) {
            for i in 0..archive.len() {

                let mut member = archive.by_index(i).ok()?;

                let (name, inner_size) = {
                    if !member.is_file() { continue; }
                    (member.name().to_string(), member.size())
                };

                let mut buffer = Vec::new();
                std::io::copy(&mut member, &mut buffer).ok()?;

                let inner_md5 = calculate_md5_from_reader(buffer.as_slice()).ok();

                let rcheevos_hash = if source.fs_size < RCHEEVOS_SCAN_FILE_SIZE_LIMIT {
                    rcheevos::compute_hash(member.name(), Some(buffer.as_slice()))
                } else {
                    None
                };

                zip_results.push(ScannedFile {
                    fs_path: source.fs_path.clone(),
                    inner_path: Some(name.normalize_path()),
                    fs_size: source.fs_size,
                    fs_mtime: source.fs_mtime,
                    inner_size: Some(inner_size),
                    fs_md5: fs_md5.clone(),
                    inner_md5,
                    rcheevos_hash,
                    ..Default::default()
                });
            }
        }
        Some(zip_results)
    } else {
        let rcheevos_hash = if is_disk_format(extension) {
            rcheevos::compute_hash(path.to_str()?, None)
        } else if source.fs_size < RCHEEVOS_SCAN_FILE_SIZE_LIMIT {
            let mut contents = Vec::new();
            file.rewind().ok()?;
            file.read_to_end(&mut contents).ok()?;

            rcheevos::compute_hash(path.to_str()?, Some(contents.as_slice()))
        } else {
            None
        };

        Some(vec![ScannedFile {
            fs_path: source.fs_path,
            fs_size: source.fs_size,
            fs_mtime: source.fs_mtime,
            fs_md5,
            rcheevos_hash,
            ..Default::default()
        }])
    }
}

#[tauri::command]
pub async fn scan_collection_dir(
    window: Window,
    app_handle: AppHandle,
    base_path: String,
    ignore_patterns: Vec<String>
) -> Result<(), String> {
    let root = Path::new(&base_path);

    let pool = app_handle.state::<SqlitePool>().inner().clone();

    let skip_map = {
        let mut skip_map: HashMap<String, (u64, Option<u64>)> = HashMap::new();
        let rows = sqlx::query("SELECT fs_path, fs_size, fs_mtime FROM rex_collection_files")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string() )?;

        for row in rows {
            let path: String = row.get(0);
            let size: i64 = row.get(1);
            let mtime: Option<i64> = row.get(2);
            skip_map.insert(path, (size as u64, mtime.map(|v| v as u64)));
        }

        Arc::new(skip_map)
    };

    let ignore_set = {
        let mut builder = GlobSetBuilder::new();
        for pattern in ignore_patterns {
            builder.add(Glob::new(&pattern).map_err(|e| e.to_string())?);
        }
        builder.build().map_err(|e| e.to_string())?
    };

    let counter = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = std::sync::mpsc::channel::<ScannedFile>();

    let db_thread = std::thread::spawn(move || {
        tauri::async_runtime::block_on(async move {
            let mut manager = PersistenceManager::new(pool, PERSISTENCE_BATCH_SIZE);
            while let Ok(file) = rx.recv() {
                let _ = manager.add(file).await;
            }
            manager.flush().await
        })
    });

    let walker = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let result: anyhow::Result<ScannedFile> = (root, &e).try_into();
            result.ok()
        })
        .filter(|e| !ignore_set.is_match(&e.fs_path));

    let files: Vec<_> = walker.collect();
    let total = files.len();
    let _ = window.emit("scan-started", total);

    files
        .into_iter()
        .par_bridge()
        .filter_map(|e| {
            let count = counter.fetch_add(1, Ordering::Relaxed) + 1;
            if count % PROGRESS_BATCH_SIZE == 0 {
                let _ = window.emit("scan-progress", (count, &e.fs_path));
            }

            if let Some(&(cached_size, cached_mtime)) = skip_map.get(&e.fs_path) {
                if cached_size == e.fs_size && e.fs_mtime == cached_mtime {
                    return None;
                }
            }

            process_single_file(e)
        })
        .flatten()
        .for_each(|file| {
            let _ = tx.send(file.clone());
        });

    drop(tx);
    db_thread.join().map_err(|_| "DB thread panicked")??;

    let _ = window.emit("scan-finished", counter.load(Ordering::Relaxed));

    Ok(())
}

fn is_disk_format(ext: &str) -> bool {
    matches!(ext, "chd" | "rvz" | "iso" | "cue" | "gdi")
}
