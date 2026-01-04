use std::path::Path;
use std::fs::File;
use std::io::{self, Read, BufReader};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use globset::{Glob, GlobSetBuilder};
use zip::ZipArchive;
use md5::Context;
use rayon::prelude::*;
use sqlx::sqlite::SqlitePool;

use tauri::{Window, Emitter, AppHandle, Manager};

use crate::collection::persistence_manager::PersistenceManager;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScannedFile {
    pub fs_path: String,
    pub fs_size: u64,

    pub inner_path: Option<String>,
    pub inner_size: Option<u64>,

    pub fs_md5: Option<String>,
    pub inner_md5: Option<String>,
    pub rcheevos_hash: Option<String>,
}

const BUFFER_SIZE: usize = 128 * 1024;
const PERSISTENCE_BATCH_SIZE: usize = 20;
const PROGRESS_BATCH_SIZE: usize = 20;

fn calculate_md5_from_reader<R: Read>(reader: R) -> io::Result<String> {
    let mut buffered_reader = BufReader::with_capacity(BUFFER_SIZE, reader);
    let mut context = Context::new();
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let count = buffered_reader.read(&mut buffer)?;
        if count == 0 { break; }
        context.consume(&buffer[..count]);
    }

    Ok(format!("{:x}", context.finalize()))
}

fn process_single_file(
    path: &Path,
    fs_path_norm: &str,
    fs_size: u64,
) -> Option<Vec<ScannedFile>> {
    let file = File::open(path).ok()?;

    let fs_md5 = calculate_md5_from_reader(&file).ok();

    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    if extension.to_lowercase() == "zip" {
        let mut zip_results = Vec::new();
        if let Ok(mut archive) = ZipArchive::new(file) {
            for i in 0..archive.len() {

                let (name, inner_size) = {
                    let member = archive.by_index(i).ok()?;
                    if !member.is_file() { continue; }
                    (member.name().to_string(), member.size())
                };

                if let Ok(member_reader) = archive.by_index(i) {
                    let inner_md5 = calculate_md5_from_reader(member_reader).ok();

                    zip_results.push(ScannedFile {
                        fs_path: fs_path_norm.to_string(),
                        inner_path: Some(name.replace('\\', "/")),
                        fs_size,
                        inner_size: Some(inner_size),
                        fs_md5: fs_md5.clone(),
                        inner_md5,
                        rcheevos_hash: None,
                    });
                }
            }
        }
        Some(zip_results)
    } else {
        Some(vec![ScannedFile {
            fs_path: fs_path_norm.to_string(),
            inner_path: None,
            fs_size,
            inner_size: None,
            fs_md5,
            inner_md5: None,
            rcheevos_hash: None,
        }])
    }
}

#[tauri::command]
pub async fn scan_collection_dir(
    window: Window,
    app_handle: AppHandle,
    base_path: String,
    ignore_patterns: Vec<String>
) -> Result<Vec<ScannedFile>, String> {
    let root = Path::new(&base_path);

    let mut builder = GlobSetBuilder::new();
    for pattern in ignore_patterns {
        builder.add(Glob::new(&pattern).map_err(|e| e.to_string())?);
    }
    let ignore_set = builder.build().map_err(|e| e.to_string())?;

    let counter = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = std::sync::mpsc::channel::<ScannedFile>();

    let pool = app_handle.state::<SqlitePool>().inner().clone();
    // let db_worker_handle = tauri::async_runtime::spawn(async move {
    //     let mut manager = PersistenceManager::new(pool, PERSISTENCE_BATCH_SIZE);
    //     while let Ok(file) = rx.recv() {
    //         let _ = manager.add(file).await;
    //     }
    //     manager.flush().await
    // });

    let db_thread = std::thread::spawn(move || {
        // We use block_on ONLY here to bridge to the async sqlx calls
        tauri::async_runtime::block_on(async move {
            let mut manager = PersistenceManager::new(pool, PERSISTENCE_BATCH_SIZE);
            while let Ok(file) = rx.recv() {
                let _ = manager.add(file).await;
            }
            manager.flush().await
        })
    });

    let files: Vec<ScannedFile> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .par_bridge()
        .filter_map(|entry| {
            let path = entry.path();
            let rel_path = path.strip_prefix(root).ok()?;
            let fs_path_norm = rel_path.to_string_lossy().replace('\\', "/");

            if ignore_set.is_match(&fs_path_norm) {
                return None;
            }

            if entry.file_type().is_file() {
                let metadata = entry.metadata().ok()?;
                let fs_size = metadata.len();

                return process_single_file(path, &fs_path_norm, fs_size).map(|files| {
                    let count = counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                    if count % PROGRESS_BATCH_SIZE == 0 {
                        let _ = window.emit("scan-progress", (count, &fs_path_norm));
                    }
                    files
                });
            }
            None
        })
        .flatten()
        .inspect(|file| {
            let _ = tx.send(file.clone());
        })
        .collect();

    drop(tx);
    db_thread.join().map_err(|_| "DB thread panicked")??;

    let _ = window.emit("scan-finished", counter.load(Ordering::Relaxed));

    Ok(files)
}
