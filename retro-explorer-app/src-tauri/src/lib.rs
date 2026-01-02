mod plugin;

#[cfg(not(dev))]
use tauri::{ipc::CapabilityBuilder, Manager, Url};

use tauri::{webview::WebviewWindowBuilder, WebviewUrl};

use plugin::discovery::list_plugins;
use plugin::protocol::handle_request;

use std::net::ToSocketAddrs;
use std::time::Duration;

#[tauri::command]
async fn check_server_status(address: String) -> bool {
    // 1. Safe DNS Resolution: Convert "127.0.0.1:3030" to a SocketAddr
    // This returns an error if the string is malformed or DNS fails.
    let Ok(mut addrs) = address.to_socket_addrs() else {
        return false;
    };

    // 2. Take the first resolved address (usually IPv4)
    let Some(socket_addr) = addrs.next() else {
        return false;
    };

    // 3. Perform a silent Layer 4 (TCP) handshake
    // The browser never sees this, so no ERR_CONNECTION_REFUSED is logged.
    // We use a short timeout since it's local.
    std::net::TcpStream::connect_timeout(&socket_addr, Duration::from_millis(200)).is_ok()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let port: u16 = 9527;

    tauri::Builder::default()
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .setup(move |app| {
            #[cfg(dev)]
            let url = WebviewUrl::App(std::path::PathBuf::from("/"));

            #[cfg(not(dev))]
            let url = {
                let url: Url = format!("http://localhost:{}/index.html", port).parse().unwrap();

                app.add_capability(
                    CapabilityBuilder::new("localhost")
                        .remote(url.to_string())
                        .window("main"),
                )?;

                WebviewUrl::External(url)
            };

            WebviewWindowBuilder::new(app, "main".to_string(), url)
                .title("REX")
                .build()?;
            Ok(())
        })
        .register_uri_scheme_protocol("plugin", |ctx, request| handle_request(ctx, request))
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_plugins,
            check_server_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
