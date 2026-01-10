use anyhow::{Context, Result, bail};
use once_cell::sync::OnceCell;
use std::fs::File;
use std::io::{BufWriter, Cursor, Read, Seek};
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

pub trait ReadSeek: Read + Seek + Send {}
impl<T: Read + Seek + Send> ReadSeek for T {}

#[derive(Clone)]
pub enum ContentSource {
    /// A raw file on disk. No caching; always re-reads from disk to save RAM.
    File { path: PathBuf },

    /// A small file inside a ZIP. Cached in memory as an Arc<Vec<u8>>.
    CompressedSmall {
        source: Arc<ContentSource>,
        member_name: String,
        cache: Arc<OnceCell<Arc<Vec<u8>>>>,
    },

    /// A large file inside a ZIP. Streamed to a temp file to allow lazy reading.
    CompressedLarge {
        source: Arc<ContentSource>,
        member_name: String,
        temp_file: Arc<OnceCell<TempFile>>,
    },

    /// A patched version of any other source.
    Patched {
        source: Arc<ContentSource>,
        patch_path: PathBuf,
        cache: Arc<OnceCell<Arc<Vec<u8>>>>,
    },
}

impl ContentSource {
    /// Returns the full bytes of the content.
    /// Only efficient for Small or Patched variants.
    pub fn get_bytes(&self) -> Result<Arc<Vec<u8>>> {
        match self {
            ContentSource::File { path } => {
                let data = std::fs::read(path)
                    .with_context(|| format!("Failed to read file: {:?}", path))?;
                Ok(Arc::new(data))
            }
            ContentSource::CompressedSmall {
                source,
                member_name,
                cache,
            } => {
                // Hint the return type of the closure to resolve ambiguity
                let bytes = cache.get_or_try_init(|| -> Result<Arc<Vec<u8>>> {
                    let zip_bytes = source.get_bytes()?;
                    let mut archive = zip::ZipArchive::new(Cursor::new(&**zip_bytes))?;
                    let mut member = archive.by_name(member_name)?;
                    let mut buffer = Vec::with_capacity(member.size() as usize);
                    member.read_to_end(&mut buffer)?;
                    Ok(Arc::new(buffer))
                })?;
                Ok(Arc::clone(bytes))
            }
            ContentSource::Patched {
                source,
                patch_path,
                cache,
            } => {
                let bytes = cache.get_or_try_init(|| -> anyhow::Result<Arc<Vec<u8>>> {
                    let base_bytes = source.get_bytes()?;
                    let patched_vec = apply_patch(&base_bytes, patch_path)?;
                    Ok(Arc::new(patched_vec))
                })?;
                Ok(Arc::clone(bytes))
            }
            ContentSource::CompressedLarge { .. } => {
                bail!("Cannot call get_bytes on a Large source. Use get_reader instead.")
            }
        }
    }

    /// Provides a seekable reader.
    /// For Files/Large, this is a file handle. For Small/Patched, it's a memory cursor.
    pub fn get_reader(&self) -> Result<Box<dyn ReadSeek>> {
        match self {
            ContentSource::File { path } => {
                let file = File::open(path)?;
                Ok(Box::new(file))
            }
            ContentSource::CompressedLarge { temp_file, .. } => {
                let temp =
                    temp_file.get_or_try_init(|| -> Result<TempFile> { self.extract_to_temp() })?;
                let file = File::open(&temp.path)?;
                Ok(Box::new(file))
            }
            _ => {
                let bytes = self.get_bytes()?;
                // Creating a Cursor over a cloned Vec to satisfy Send + ReadSeek easily
                Ok(Box::new(Cursor::new(bytes.to_vec())))
            }
        }
    }

    fn extract_to_temp(&self) -> Result<TempFile> {
        if let ContentSource::CompressedLarge {
            source,
            member_name,
            ..
        } = self
        {
            let zip_reader = source.get_reader()?;
            let mut archive = zip::ZipArchive::new(zip_reader)?;
            let mut member = archive.by_name(member_name)?;

            let temp_path = std::env::temp_dir().join(format!("rex_{}.tmp", Uuid::new_v4()));
            let file = File::create(&temp_path)?;
            let mut writer = BufWriter::new(file);

            std::io::copy(&mut member, &mut writer)?;
            Ok(TempFile { path: temp_path })
        } else {
            bail!("Not a CompressedLarge source")
        }
    }
}

pub struct TempFile {
    pub path: PathBuf,
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

use flips::{BpsPatch, IpsPatch, UpsPatch};

fn apply_patch(base: &[u8], patch_path: &std::path::PathBuf) -> anyhow::Result<Vec<u8>> {
    let patch_data = std::fs::read(patch_path)
        .with_context(|| format!("Failed to read patch file: {:?}", patch_path))?;

    let extension = patch_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "bps" => {
            let patch = BpsPatch::new(&patch_data);
            let output = patch
                .apply(base)
                .map_err(|e| anyhow::anyhow!("BPS Patch error: {:?}", e))?;
            // Convert BpsOutput to Vec<u8>
            Ok(output.to_vec())
        }
        "ips" => {
            let patch = IpsPatch::new(&patch_data);
            let output = patch
                .apply(base)
                .map_err(|e| anyhow::anyhow!("IPS Patch error: {:?}", e))?;
            // Convert IpsOutput to Vec<u8>
            Ok(output.to_vec())
        }
        "ups" => {
            let patch = UpsPatch::new(&patch_data);
            let output = patch
                .apply(base)
                .map_err(|e| anyhow::anyhow!("UPS Patch error: {:?}", e))?;
            // Convert UpsOutput to Vec<u8>
            Ok(output.to_vec())
        }
        _ => anyhow::bail!("Unsupported patch extension: .{}", extension),
    }
}
