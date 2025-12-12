use eframe::{NativeOptions, icon_data};
use std::sync::Arc;

const ICON_BYTES: &[u8] = include_bytes!("../../assets/icons/shadcn-egui/icon.png");

pub fn app_icon() -> Option<egui::IconData> {
    icon_data::from_png_bytes(ICON_BYTES).ok()
}

pub fn native_options() -> NativeOptions {
    let mut viewport = egui::ViewportBuilder::default();
    if let Some(icon) = app_icon() {
        viewport = viewport.with_icon(Arc::new(icon));
    }

    NativeOptions {
        viewport,
        ..Default::default()
    }
}
