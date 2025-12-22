//! Build script for `egui-shadcn`.
//!
//! - On Windows: embeds the application icon from `assets/icons/shadcn-egui/icon.ico`.
//! - On other platforms: no-op.
//!
//! Warnings are emitted via `cargo:warning` when the icon is missing or cannot be set; the build
//! should continue to keep CI green.

#[cfg(windows)]
fn main() {
    if let Err(err) = set_windows_icon() {
        // Не прерываем сборку на CI, но подсвечиваем проблему.
        println!("cargo:warning=Не удалось проставить иконку для Windows: {err}");
    }
}

#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn set_windows_icon() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let icon_path = std::path::Path::new(&manifest_dir)
        .join("assets")
        .join("icons")
        .join("shadcn-egui")
        .join("icon.ico");

    if !icon_path.exists() {
        println!("cargo:warning=Иконка не найдена: {}", icon_path.display());
        return Ok(());
    }

    let mut res = winres::WindowsResource::new();
    res.set_icon(icon_path.to_str().unwrap());
    res.compile()?;
    Ok(())
}
