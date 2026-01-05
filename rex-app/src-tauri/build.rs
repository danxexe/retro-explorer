use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let src_scripts = manifest_dir.join("../../game-scripts");
    let dst_scripts = manifest_dir.join("../src/game-scripts");

    if dst_scripts.exists() {
        fs::remove_dir_all(&dst_scripts).expect("Failed to clean old game-scripts");
    }

    if profile == "release" {
        copy_dir_all(src_scripts, dst_scripts).expect("Failed to bundle game-scripts");
    }

    tauri_build::build()
}

fn copy_dir_all(src: impl AsRef<std::path::Path>, dst: impl AsRef<std::path::Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
