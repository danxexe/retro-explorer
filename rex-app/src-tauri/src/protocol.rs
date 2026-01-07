use std::fs;
use tauri::{AppHandle, Manager, Runtime};

const CONTENT_404: [u8; 4] = [52, 48, 52, 10];

pub fn handle_rex_request<R: Runtime>(
    app_handle: &AppHandle<R>,
    request: tauri::http::Request<Vec<u8>>,
) -> tauri::http::Response<Vec<u8>> {
    let uri = request.uri();
    let path = uri.path().strip_prefix('/').unwrap_or(uri.path());
    let asset_key = if path.is_empty() { "index.html" } else { path };

    #[cfg(dev)]
    if asset_key.starts_with("game-scripts") {
        let mut dev_path = std::env::current_dir().unwrap();
        dev_path.pop();
        dev_path.pop();
        dev_path.push(asset_key);
        if dev_path.exists() {
            return serve_from_disk(&dev_path);
        }
    }

    if asset_key.starts_with("user-scripts") {
        if let Ok(mut user_path) = app_handle.path().app_data_dir() {
            user_path.push(asset_key);
            if user_path.exists() {
                return serve_from_disk(&user_path);
            }
        }
    }

    if asset_key == "favicon.ico" {
        return app_handle
            .asset_resolver()
            .get("favicon.ico".into())
            .map(|a| {
                tauri::http::Response::builder()
                    .header("Content-Type", "image/x-icon")
                    .body(a.bytes)
                    .unwrap()
            })
            .unwrap_or_else(|| serve_transparent_pixel());
    }

    // HACK: For some weird reason, Tauri defaults to index.html instead of None when a resource is not found.
    // We use a custom index.html containing the string "404\n" to simulate it.
    // Our "real" index.html is set in the tauri.conf.json window config.
    let asset = app_handle
        .asset_resolver()
        .get(asset_key.to_string())
        .and_then(|asset| (asset.bytes != CONTENT_404).then(|| asset));

    match asset {
        Some(asset) => tauri::http::Response::builder()
            .header("Content-Type", asset.mime_type)
            .header("Access-Control-Allow-Origin", "*")
            .body(asset.bytes)
            .unwrap(),
        None => tauri::http::Response::builder()
            .status(404)
            .body(vec![])
            .unwrap(),
    }
}

fn serve_from_disk(path: &std::path::Path) -> tauri::http::Response<Vec<u8>> {
    match fs::read(path) {
        Ok(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            tauri::http::Response::builder()
                .header("Content-Type", mime.as_ref())
                .header("Access-Control-Allow-Origin", "*")
                .status(200)
                .body(content)
                .unwrap()
        }
        Err(_) => tauri::http::Response::builder()
            .status(404)
            .body(vec![])
            .unwrap(),
    }
}

fn serve_transparent_pixel() -> tauri::http::Response<Vec<u8>> {
    let pixel = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F,
        0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x00,
        0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    tauri::http::Response::builder()
        .header("Content-Type", "image/png")
        .body(pixel)
        .unwrap()
}
