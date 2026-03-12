use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::Label;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui) {
    let row_width = 160.0;
    ui.horizontal(|row| {
        row.add_space(((row.available_width() - row_width) * 0.5).max(0.0));
        let _ = Label::new("Email")
            .description("Required field")
            .show(row, &app.theme);
    });
}
