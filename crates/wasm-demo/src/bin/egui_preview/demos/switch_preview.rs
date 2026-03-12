use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui) {
    let _ = egui_shadcn::switch(
        ui,
        &app.theme,
        &mut app.switch_enabled,
        "Enable feature",
        egui_shadcn::ControlVariant::Primary,
        egui_shadcn::ControlSize::Md,
        true,
    );
}
