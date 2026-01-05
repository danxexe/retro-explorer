use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, BufReader, Read, Seek};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::collections::HashMap;
use std::ffi::{CStr, CString, c_char};

use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use globset::{Glob, GlobSetBuilder};
use zip::ZipArchive;
use md5::Context;
use rayon::prelude::*;
use sqlx::sqlite::SqlitePool;
use sqlx::Row;

use tauri::{Window, Emitter, AppHandle, Manager};

use rcheevos_hash_sys;

use crate::collection::persistence_manager::PersistenceManager;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScannedFile {
    pub fs_path: String,
    pub fs_size: u64,
    pub fs_mtime: Option<u64>,

    pub inner_path: Option<String>,
    pub inner_size: Option<u64>,

    pub fs_md5: Option<String>,
    pub inner_md5: Option<String>,
    pub rcheevos_hash: Option<String>,
}

const BUFFER_SIZE: usize = 128 * 1024;
const PERSISTENCE_BATCH_SIZE: usize = 20;
const PROGRESS_BATCH_SIZE: usize = 1;
const RCHEEVOS_SCAN_FILE_SIZE_LIMIT: u64 = 10 * 1024 * 1024;

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

fn generate_rcheevos_hash(path: &str, buffer: Option<&[u8]>) -> Option<String> {
    let path_c = CString::new(path).ok()?;
    let mut hash = [0 as c_char; 33];

    if let Some(buffer) = buffer {
        println!("buffer: {}", buffer.len());
    } else {
        println!("no buffer");
    }

    let (ptr, len) = buffer.map_or((std::ptr::null(), 0), |b| (b.as_ptr(), b.len()));
    let mut iter: std::mem::MaybeUninit<rcheevos_hash_sys::rc_hash_iterator> = std::mem::MaybeUninit::uninit();

    unsafe {
        rcheevos_hash_sys::rc_hash_initialize_iterator(
            iter.as_mut_ptr(),
            path_c.as_ptr(),
            ptr,
            len
        );

        let mut iter = iter.assume_init();

        // iter.consoles[0] = 78 as u8;
        // iter.index = 0;

        println!("before iterate: {} {:?}", len, path_c);
        let result = rcheevos_hash_sys::rc_hash_iterate(hash.as_mut_ptr(), &mut iter);
        println!("iterate result: {}", result);
        if result != 0 {
            let hash = CStr::from_ptr(hash.as_ptr()).to_string_lossy().into_owned();
            println!("hash: {}", hash);
            return Some(hash);
        }
    }
    None
}

fn process_single_file(
    path: PathBuf,
    source: ScannedFile,
) -> Option<Vec<ScannedFile>> {
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
                    generate_rcheevos_hash(member.name(), Some(buffer.as_slice()))
                } else {
                    None
                };

                zip_results.push(ScannedFile {
                    fs_path: source.fs_path.clone(),
                    inner_path: Some(name.replace('\\', "/")),
                    fs_size: source.fs_size,
                    fs_mtime: source.fs_mtime,
                    inner_size: Some(inner_size),
                    fs_md5: fs_md5.clone(),
                    inner_md5,
                    rcheevos_hash,
                });
            }
        }
        Some(zip_results)
    } else {
        let mut contents = Vec::new();
        file.rewind().ok()?;
        file.read_to_end(&mut contents).ok()?;

        let rcheevos_hash = if source.fs_size < RCHEEVOS_SCAN_FILE_SIZE_LIMIT {
            generate_rcheevos_hash(path.to_str()?, Some(contents.as_slice()))
            // let rcheevos_hash = generate_rcheevos_hash(path.to_str()?, None);
        } else {
            None
        };

        Some(vec![ScannedFile {
            fs_path: source.fs_path,
            inner_path: None,
            fs_size: source.fs_size,
            fs_mtime: source.fs_mtime,
            inner_size: None,
            fs_md5,
            inner_md5: None,
            rcheevos_hash,
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
            let path = e.path().to_owned();
            let rel_path = path.strip_prefix(root).ok()?;
            let fs_path = rel_path.to_string_lossy().replace('\\', "/");
            let metadata = e.metadata().ok()?;
            let fs_size = metadata.len();
            let fs_mtime = metadata
                .modified().ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as u64);

            Some((path, ScannedFile {
                fs_path,
                fs_size,
                fs_mtime,
                inner_path: None,
                inner_size: None,
                fs_md5: None,
                inner_md5: None,
                rcheevos_hash: None,
            }))
        })
        .filter(|(_, e)| !ignore_set.is_match(&e.fs_path));

    let files: Vec<_> = walker.collect();
    let total = files.len();
    let _ = window.emit("scan-started", total);

    files
        .into_iter()
        .par_bridge()
        .filter_map(|(path, e)| {
            {
                let count = counter.fetch_add(1, Ordering::Relaxed) + 1;
                if count % PROGRESS_BATCH_SIZE == 0 {
                    let _ = window.emit("scan-progress", (count, &e.fs_path));
                }
            }

            if let Some(&(cached_size, cached_mtime)) = skip_map.get(&e.fs_path) {
                if cached_size == e.fs_size && e.fs_mtime == cached_mtime {
                    return None;
                }
            }

            process_single_file(path, e)
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
