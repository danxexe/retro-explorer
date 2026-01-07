use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use walkdir::DirEntry;

use crate::collection::NormalizePath;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScannedFile {
    pub root: PathBuf,
    pub path: PathBuf,

    pub fs_path: String,
    pub fs_size: u64,

    pub fs_mtime: Option<u64>,
    pub inner_path: Option<String>,
    pub inner_size: Option<u64>,
    pub fs_md5: Option<String>,
    pub inner_md5: Option<String>,
    pub rcheevos_hash: Option<String>,
}

impl TryFrom<(&Path, &DirEntry)> for ScannedFile {
    type Error = anyhow::Error;

    fn try_from(value: (&Path, &DirEntry)) -> anyhow::Result<Self> {
        let (root, e) = value;

        let path = e.path().to_owned();
        let rel_path = path.strip_prefix(root)?;
        let fs_path = rel_path.to_string_lossy().as_ref().normalize_path();
        let metadata = e.metadata()?;
        let fs_size = metadata.len();
        let fs_mtime = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as u64);

        Ok(Self {
            root: root.to_owned(),
            path,
            fs_path,
            fs_size,
            fs_mtime,
            ..Default::default()
        })
    }
}
