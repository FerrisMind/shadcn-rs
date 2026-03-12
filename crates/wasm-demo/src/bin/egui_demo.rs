#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "egui_preview/mod.rs"]
mod egui_preview;

use egui_preview::EguiPreviewApp;
#[cfg(target_arch = "wasm32")]
use egui_preview::ensure_lucide_font;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Egui Components Preview",
        options,
        Box::new(|_cc| Ok(Box::new(EguiPreviewApp::new()))),
    )
}

#[cfg(target_arch = "wasm32")]
pub fn main() {
    use wasm_bindgen::JsCast;

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async move {
        let window = web_sys::window().expect("window is not available");
        let document = window.document().expect("document is not available");
        let canvas = document
            .get_element_by_id("egui-canvas")
            .expect("egui-canvas element was not found")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("egui-canvas is not an HtmlCanvasElement");

        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    ensure_lucide_font(&cc.egui_ctx);
                    Ok(Box::new(EguiPreviewApp::new()))
                }),
            )
            .await
            .expect("failed to start eframe web runner");
    });
}
