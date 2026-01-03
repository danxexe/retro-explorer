mod protocol;
mod plugin;
mod dev_tools;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .register_uri_scheme_protocol("rex", |ctx, req| {
            protocol::handle_rex_request(ctx.app_handle(), req)
        })
        .setup(|app| {
            #[cfg(dev)]
            dev_tools::init_watcher(app.handle().clone());

            Ok(())
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
