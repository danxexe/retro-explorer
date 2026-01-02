use std::{fs, path::PathBuf};
use serde::Serialize;
use tauri::{AppHandle, Manager, path::BaseDirectory};

use crate::plugin::metadata::{extract_plugin_meta, PluginMeta};

#[derive(Debug, Serialize)]
pub struct DiscoveredPlugin {
  pub id: String,
  pub entry: String,
  pub name: String,
  pub description: Option<String>,
  pub keywords: Vec<String>,
}

fn plugin_root(app: &AppHandle) -> PathBuf {
  // println!(
  //   "Repolved dir: {:?}",
  //   app.path().resolve("pages", BaseDirectory::Resource)
  // );

  app.path()
    .resolve("pages", BaseDirectory::Resource)
    .expect("pages dir")
}

fn read_limited(path: &PathBuf, limit: usize) -> Option<String> {
  let data = fs::read(path).ok()?;
  let slice = &data[..data.len().min(limit)];
  String::from_utf8(slice.to_vec()).ok()
}

fn build_plugin(id: String, entry: String, meta: PluginMeta) -> DiscoveredPlugin {
  DiscoveredPlugin {
    name: meta.name.clone().unwrap_or_else(|| id.clone()),
    description: meta.description,
    keywords: meta.keywords.unwrap_or_default(),
    id,
    entry,
  }
}

#[tauri::command]
pub fn list_plugins(app: AppHandle) -> Vec<DiscoveredPlugin> {
  let root = plugin_root(&app);
  let mut out = Vec::new();

  let Ok(entries) = fs::read_dir(&root) else {
    return out;
  };

  for entry in entries.flatten() {
    let path = entry.path();
    let file_name = entry.file_name().to_string_lossy().to_string();

    // Directory plugin: plugins/foo/index.html
    if path.is_dir() {
      let index = path.join("index.html");
      if !index.exists() {
        continue;
      }

      let html = read_limited(&index, 32 * 1024).unwrap_or_default();
      let meta = extract_plugin_meta(&html).unwrap_or_default();

      out.push(build_plugin(
        file_name.clone(),
        format!("{}/index.html", file_name),
        meta,
      ));
    }

    // Single-file plugin: plugins/foo.html
    if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("html") {
      let id = path.file_stem().unwrap().to_string_lossy().to_string();
      let html = read_limited(&path, 32 * 1024).unwrap_or_default();
      let meta = extract_plugin_meta(&html).unwrap_or_default();

      out.push(build_plugin(
        id,
        format!("{}", file_name),
        meta,
      ));
    }
  }

  out
}
