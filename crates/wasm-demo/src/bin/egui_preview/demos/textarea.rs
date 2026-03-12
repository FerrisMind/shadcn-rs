use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::Textarea;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let textarea_id = ui.make_persistent_id("preview-textarea");
    let width = if compact { 220.0 } else { 420.0 };

    ui.horizontal(|row| {
        row.add_space(((row.available_width() - width) * 0.5).max(0.0));
        Textarea::new(textarea_id)
            .placeholder("Write something...")
            .rows(if compact { 2 } else { 3 })
            .width(width)
            .resizable(false)
            .show(row, &app.theme, &mut app.email);
    });
}
