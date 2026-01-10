use std::{
    collections::HashMap,
    io::{self, BufReader, Read},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use anyhow::{Result, anyhow};
use globset::{Glob, GlobSetBuilder};
use md5::Context;
use once_cell::sync::OnceCell;
use rayon::prelude::*;
use sqlx::{Row, sqlite::SqlitePool};
use walkdir::WalkDir;
use zip::ZipArchive;

use tauri::{AppHandle, Emitter, Manager, Window};

use crate::collection::{
    NormalizePath, content_source::ContentSource, persistence_manager::PersistenceManager,
    rcheevos, scanned_file::ScannedFile,
};

const BUFFER_SIZE: usize = 128 * 1024;
const PERSISTENCE_BATCH_SIZE: usize = 20;
const PROGRESS_BATCH_SIZE: usize = 1;
const RCHEEVOS_SCAN_FILE_SIZE_LIMIT: u64 = 1024 * 1024 * 1024;
const SMALL_COMPRESSED_LIMIT: u64 = 64 * 1024 * 1024;

#[tauri::command]
pub async fn scan_collection_dir(
    window: Window,
    app_handle: AppHandle,
    base_path: String,
    ignore_patterns: Vec<String>,
) -> Result<(), String> {
    scan_collection_dir_(window, app_handle, base_path, ignore_patterns)
        .await
        .map_err(|e| e.to_string())
}

async fn scan_collection_dir_(
    window: Window,
    app_handle: AppHandle,
    base_path: String,
    ignore_patterns: Vec<String>,
) -> Result<()> {
    let root = Path::new(&base_path);

    let pool = app_handle.state::<SqlitePool>().inner().clone();

    let collection_id = upsert_collection(&pool, root).await?;

    let skip_map = build_skip_map(&pool, collection_id).await?;

    let ignore_set = {
        let mut builder = GlobSetBuilder::new();
        for pattern in ignore_patterns {
            builder.add(Glob::new(&pattern)?);
        }
        builder.build()?
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
            let result: Result<ScannedFile> = (collection_id, root, &e).try_into();
            result.ok()
        })
        .filter(|e| !ignore_set.is_match(&e.fs_path));

    let files: Vec<_> = walker.collect();
    let total = files.len();
    let _ = window.emit("scan-started", (total, root));

    files
        .into_iter()
        .par_bridge()
        .filter_map(|e| {
            let count = counter.fetch_add(1, Ordering::Relaxed) + 1;
            if count % PROGRESS_BATCH_SIZE == 0 {
                let _ = window.emit("scan-progress", (count, &e.root, &e.fs_path));
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
    db_thread
        .join()
        .map_err(|_| anyhow!("DB thread panicked"))??;

    let _ = window.emit("scan-finished", (counter.load(Ordering::Relaxed), root));

    Ok(())
}

fn process_single_file(scanned: ScannedFile) -> Option<Vec<ScannedFile>> {
    let source = Arc::new(ContentSource::File {
        path: scanned.path.clone(),
    });

    let md5_reader = source.get_reader().ok()?;

    let fs_md5 = calculate_md5_from_reader(md5_reader).ok();

    let extension = scanned
        .path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    if extension.to_lowercase() == "zip" {
        let mut zip_results = Vec::new();
        let zip_reader = source.get_reader().ok()?;
        if let Ok(mut archive) = ZipArchive::new(zip_reader) {
            for i in 0..archive.len() {
                let member = archive.by_index(i).ok()?;
                if !member.is_file() {
                    continue;
                }

                let zip_source = if member.size() <= SMALL_COMPRESSED_LIMIT {
                    Arc::new(ContentSource::CompressedSmall {
                        source: Arc::clone(&source),
                        member_name: member.name().to_string(),
                        cache: Arc::new(OnceCell::new()),
                    })
                } else {
                    Arc::new(ContentSource::CompressedLarge {
                        source: Arc::clone(&source),
                        member_name: member.name().to_string(),
                        temp_file: Arc::new(OnceCell::new()),
                    })
                };

                let (name, inner_size) = { (member.name().to_string(), member.size()) };

                let zip_md5_reader = zip_source.get_reader().ok()?;
                let inner_md5 = calculate_md5_from_reader(zip_md5_reader).ok();

                let zip_bytes = zip_source.get_bytes().ok()?;
                let rcheevos_hash = if scanned.fs_size < RCHEEVOS_SCAN_FILE_SIZE_LIMIT {
                    rcheevos::compute_hash(member.name(), Some(zip_bytes.as_slice()))
                } else {
                    None
                };

                zip_results.push(ScannedFile {
                    inner_path: Some(name.normalize_path()),
                    inner_size: Some(inner_size),
                    fs_md5: fs_md5.clone(),
                    inner_md5,
                    rcheevos_hash,
                    ..scanned.clone()
                });
            }
        }
        Some(zip_results)
    } else if ["bps", "ips", "ups"].contains(&extension.to_lowercase().as_str()) {
        let path = &scanned.path;
        if let Some(zip_path) = find_zip_base(path) {
            let zip_file_source = Arc::new(ContentSource::File { path: zip_path });

            let member_name = {
                let reader = zip_file_source.get_reader().ok()?;
                let mut archive = zip::ZipArchive::new(reader).ok()?;

                if archive.is_empty() {
                    return None;
                }
                archive.by_index(0).ok()?.name().to_string()
            };

            let rom_source = Arc::new(ContentSource::CompressedSmall {
                source: zip_file_source,
                member_name,
                cache: Arc::new(OnceCell::new()),
            });

            let patch_path = path.to_path_buf();

            let patched_source = ContentSource::Patched {
                source: rom_source,
                patch_path: patch_path.clone(),
                cache: Arc::new(OnceCell::new()),
            };

            process_patched_rom(
                scanned.collection_id,
                &scanned.root,
                patch_path,
                patched_source,
            )
            .ok()
            .map(|scanned| vec![scanned])
        } else {
            None
        }
    } else {
        let rcheevos_hash = if is_disk_format(extension) {
            rcheevos::compute_hash(scanned.path.to_str()?, None)
        } else if scanned.fs_size < RCHEEVOS_SCAN_FILE_SIZE_LIMIT {
            let mut contents = Vec::new();
            let mut hash_reader = source.get_reader().ok()?;
            hash_reader.read_to_end(&mut contents).ok()?;

            rcheevos::compute_hash(scanned.path.to_str()?, Some(contents.as_slice()))
        } else {
            None
        };

        Some(vec![ScannedFile {
            fs_md5,
            rcheevos_hash,
            ..scanned
        }])
    }
}

fn calculate_md5_from_reader<R: Read>(reader: R) -> io::Result<String> {
    let mut buffered_reader = BufReader::with_capacity(BUFFER_SIZE, reader);
    let mut context = Context::new();
    let mut buffer = vec![0u8; BUFFER_SIZE];

    loop {
        let count = buffered_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.consume(&buffer[..count]);
    }

    io::Result::Ok(format!("{:x}", context.finalize()))
}

type SkipMap = HashMap<String, (u64, Option<u64>)>;

const INSERT_REX_COLLECTION: &str = "
INSERT INTO rex_collection
(path)
VALUES ($1)
ON CONFLICT(path) DO UPDATE SET
    path = excluded.path
RETURNING ID
";

async fn upsert_collection(pool: &SqlitePool, path: &Path) -> Result<u64> {
    let collection_id: u64 = sqlx::query(INSERT_REX_COLLECTION)
        .bind(path.to_string_lossy())
        .fetch_one(pool)
        .await?
        .get(0);

    Ok(collection_id)
}

async fn build_skip_map(pool: &SqlitePool, collection_id: u64) -> Result<SkipMap> {
    let mut skip_map: HashMap<String, (u64, Option<u64>)> = HashMap::new();
    let rows = sqlx::query(
        "
        SELECT fs_path, fs_size, fs_mtime FROM rex_collection_files
        WHERE collection_id = $1
    ",
    )
    .bind(collection_id as i64)
    .fetch_all(pool)
    .await?;

    for row in rows {
        let path: String = row.get(0);
        let size: i64 = row.get(1);
        let mtime: Option<i64> = row.get(2);
        skip_map.insert(path, (size as u64, mtime.map(|v| v as u64)));
    }

    Ok(skip_map)
}

fn is_disk_format(ext: &str) -> bool {
    matches!(ext, "chd" | "rvz" | "iso" | "cue" | "gdi")
}

fn find_zip_base(patch_path: &Path) -> Option<PathBuf> {
    let mut zip_path = patch_path.to_path_buf();
    zip_path.set_extension("zip");

    if zip_path.exists() {
        Some(zip_path)
    } else {
        None
    }
}

pub fn process_patched_rom(
    collection_id: u64,
    root: &PathBuf,
    patch_path: PathBuf,
    source: ContentSource,
) -> Result<ScannedFile> {
    let patch_metadata = std::fs::metadata(&patch_path)?;
    let fs_mtime = patch_metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs());

    let mut reader = source.get_reader()?;
    let inner_md5 = calculate_md5_from_reader(&mut reader).ok();

    let patched_bytes = source.get_bytes()?;

    let display_name = patch_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("patched_rom");

    let ra_hash = rcheevos::compute_hash(display_name, Some(&patched_bytes));

    let inner_path = if let ContentSource::Patched { source: inner, .. } = &source {
        if let ContentSource::CompressedSmall { member_name, .. } = &**inner {
            Some(member_name.clone())
        } else {
            None
        }
    } else {
        None
    };

    let fs_path = patch_path
        .strip_prefix(root)?
        .to_string_lossy()
        .replace('\\', "/");

    Ok(ScannedFile {
        collection_id,
        root: root.clone(),
        path: patch_path.clone(),

        fs_path: fs_path,
        fs_size: patch_metadata.len(),
        fs_mtime,

        inner_path,
        inner_size: Some(patched_bytes.len() as u64),

        fs_md5: None,
        inner_md5,
        rcheevos_hash: ra_hash,
    })
}
