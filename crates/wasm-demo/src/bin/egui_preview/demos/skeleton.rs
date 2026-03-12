use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::skeleton_text;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    skeleton_text(ui, &app.theme, if compact { 2 } else { 3 }, 14.0);
}
