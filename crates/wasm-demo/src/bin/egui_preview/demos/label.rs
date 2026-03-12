use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::Label;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui) {
    let _ = Label::new("Email")
        .description("Required field")
        .show(ui, &app.theme);
}
