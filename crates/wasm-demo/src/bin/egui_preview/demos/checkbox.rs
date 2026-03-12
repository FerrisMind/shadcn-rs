use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui) {
    let row_width = 150.0;
    ui.horizontal(|row| {
        row.add_space(((row.available_width() - row_width) * 0.5).max(0.0));
        let _ = egui_shadcn::checkbox(
            row,
            &app.theme,
            &mut app.checkbox_enabled,
            "Accept terms",
            egui_shadcn::ControlVariant::Primary,
            egui_shadcn::ControlSize::Md,
            true,
        );
    });
}
