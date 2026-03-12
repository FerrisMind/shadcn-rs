use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::Textarea;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let textarea_id = ui.make_persistent_id("preview-textarea");
    Textarea::new(textarea_id)
        .placeholder("Write something...")
        .rows(if compact { 2 } else { 3 })
        .width(if compact { 220.0 } else { 420.0 })
        .show(ui, &app.theme, &mut app.email);
}
