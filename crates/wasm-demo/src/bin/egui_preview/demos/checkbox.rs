use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui) {
    let _ = egui_shadcn::checkbox(
        ui,
        &app.theme,
        &mut app.checkbox_enabled,
        "Accept terms",
        egui_shadcn::ControlVariant::Primary,
        egui_shadcn::ControlSize::Md,
        true,
    );
}
