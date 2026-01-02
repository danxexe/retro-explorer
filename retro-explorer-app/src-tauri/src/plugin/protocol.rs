use mime_guess::from_path;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tauri::{
    http::{Request, Response},
    Manager, UriSchemeContext,
};

fn plugin_root<R: tauri::Runtime>(ctx: &UriSchemeContext<R>) -> PathBuf {
    ctx.app_handle()
        .path()
        .app_data_dir()
        .expect("app data dir")
        .join("plugins")
}

fn resolve_plugin_path(root: &Path, uri_path: &str) -> PathBuf {
    let mut result = root.to_path_buf();

    for segment in uri_path.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." {
            continue;
        }
        result.push(segment);
    }

    result
}

pub fn handle_request<R: tauri::Runtime>(
    ctx: UriSchemeContext<R>,
    request: Request<Vec<u8>>,
) -> Response<Vec<u8>> {
    let uri_path = request.uri().path().trim_start_matches('/');

    let root = plugin_root(&ctx);
    let full_path = resolve_plugin_path(&root, uri_path);

    match fs::read(&full_path) {
        Ok(bytes) => {
            let mime = from_path(&full_path).first_or_octet_stream();

            Response::builder()
                .status(200)
                .header("Access-Control-Allow-Origin", "*")
                .header("Content-Type", mime.as_ref())
                .body(bytes)
                .unwrap()
        }
        Err(_) => Response::builder()
            .header("Access-Control-Allow-Origin", "*")
            .status(404)
            .body(Vec::new())
            .unwrap(),
    }
}
