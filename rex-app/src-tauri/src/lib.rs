mod collection;
mod database;
mod migrations;
mod plugin;
mod protocol;

#[cfg(dev)]
mod dev_tools;

use std::net::ToSocketAddrs;
use std::time::Duration;

use tauri::Manager;

use collection::scanner::scan_collection_dir;
use database::init_database;
use migrations::migrations;
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
        .append_invoke_initialization_script("
        ;(function() {
        // Inject only on child frames
        if (window.self === window.top) return;
        if (window.origin === 'null') return;
        if (window.location.href === 'about:blank') return;
        setTimeout(() => import(window.__TAURI__.core.convertFileSrc('', 'rex') + 'assets/init-script.js'), 0)
        })();
        ")
        .register_uri_scheme_protocol("rex", |ctx, req| {
            protocol::handle_rex_request(ctx.app_handle(), req)
        })
        .setup(|app| {
            let handle = app.handle().clone();

            #[cfg(dev)]
            dev_tools::init_watcher(handle.clone());

            let app_data_dir = app.path().app_data_dir().unwrap();
            // std::fs::create_dir_all(&app_data_dir).ok();

            tauri::async_runtime::block_on(async move {
                let pool = init_database(&app_data_dir).await.expect("DB init failed");
                handle.manage(pool);
            });

            Ok(())
        })
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations("sqlite:rex.db", migrations())
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            list_plugins,
            check_server_status,
            scan_collection_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
