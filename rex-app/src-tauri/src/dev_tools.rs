use notify::{Watcher, RecursiveMode};
use tauri::{AppHandle, Manager};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

pub fn init_watcher(app_handle: AppHandle) {
    let handle = app_handle.clone();
    let last_reload = Arc::new(Mutex::new(Instant::now()));

    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if let Ok(event) = res {
            let extensions = ["html", "js", "css", "json", "ts", "png", "svg"];
            let should_reload = event.paths.iter().any(|p| {
                p.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| extensions.contains(&ext))
                    .unwrap_or(false)
            });

            if should_reload {
                let mut last = last_reload.lock().unwrap();
                if last.elapsed() > Duration::from_millis(500) {
                    if let Some(window) = handle.get_webview_window("main") {
                        let _ = window.eval("window.location.reload()");
                        *last = Instant::now();
                    }
                }
            }
        }
    }).unwrap();

    let paths_to_watch = vec![
        "../src",
        "../../game-scripts",
    ];

    for p in paths_to_watch {
        let path = std::env::current_dir().unwrap().join(p);
        if path.exists() {
            let _ = watcher.watch(&path, RecursiveMode::Recursive);
        }
    }

    Box::leak(Box::new(watcher));
}
