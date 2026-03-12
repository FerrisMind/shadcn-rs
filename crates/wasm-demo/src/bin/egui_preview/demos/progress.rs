use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{ProgressProps, ProgressSize, ProgressVariant, progress, slider};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    if !compact {
        let mut values = vec![app.progress_value];
        let _ = slider(
            ui,
            &app.theme,
            "preview-progress-slider",
            &mut values,
            0.0,
            100.0,
        );
        app.progress_value = values[0];
    }
    progress(
        ui,
        &app.theme,
        ProgressProps::new(Some(app.progress_value))
            .size(ProgressSize::Size2)
            .variant(ProgressVariant::Classic),
    );
}
