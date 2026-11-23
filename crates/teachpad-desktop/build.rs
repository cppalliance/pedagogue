//! The build script Tauri requires: it reads `tauri.conf.json` and sets up
//! what `tauri::generate_context!` reads, the app icon included.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri_build::try_build(tauri_build::Attributes::new())?;
    Ok(())
}
