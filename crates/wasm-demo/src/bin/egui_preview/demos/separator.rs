use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{SeparatorOrientation, SeparatorProps, TextProps, separator, text};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui) {
    ui.vertical(|col| {
        let _ = text(col, &app.theme, TextProps::new("Before"));
        let _ = separator(
            col,
            &app.theme,
            SeparatorProps::default()
                .orientation(SeparatorOrientation::Horizontal)
                .thickness(1.0),
        );
        let _ = text(col, &app.theme, TextProps::new("After"));
    });
}
