mod plugin;

use std::net::ToSocketAddrs;
use std::time::Duration;

#[cfg(not(dev))]
use tauri::{ipc::CapabilityBuilder, Manager, Url};

use plugin::discovery::list_plugins;

#[tauri::command]
async fn check_server_status(address: String) -> bool {
    let Ok(mut addrs) = address.to_socket_addrs() else {
        return false;
    };

    let Some(socket_addr) = addrs.next() else {
        return false;
    };

    std::net::TcpStream::connect_timeout(&socket_addr, Duration::from_millis(200)).is_ok()
}

use std::path::PathBuf;
use std::fs;

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
        Err(_) => tauri::http::Response::builder().status(404).body(vec![]).unwrap(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .register_uri_scheme_protocol("rex", move |ctx, request| {
            let app_handle = ctx.app_handle();
            let uri = request.uri();
            let path_str = uri.path();
            let clean_path = path_str.strip_prefix('/').unwrap_or(path_str);
            let path = PathBuf::from(clean_path);

            let mut components = path.components();
            let first_dir = components.next().and_then(|c| c.as_os_str().to_str());

            match first_dir {
                Some("game-scripts") if cfg!(debug_assertions) => {
                    let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                    abs_path.pop();
                    abs_path.pop();
                    abs_path.push("game-scripts");
                    abs_path.push(components.as_path());

                    let final_path = if abs_path.is_dir() {
                        abs_path.join("index.html")
                    } else {
                        abs_path
                    };

                    serve_from_disk(&final_path)
                }

                _ => {
                    let asset_key = if clean_path.is_empty() {
                        "index.html"
                    } else {
                        clean_path
                    };

                    match app_handle.asset_resolver().get(asset_key.to_string()) {
                        Some(asset) => tauri::http::Response::builder()
                            .header("Content-Type", asset.mime_type)
                            .header("Access-Control-Allow-Origin", "*")
                            .status(200)
                            .body(asset.bytes)
                            .unwrap(),
                        None => tauri::http::Response::builder()
                            .status(404)
                            .body(vec![])
                            .unwrap(),
                    }
                }
            }
        })
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_plugins,
            check_server_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
