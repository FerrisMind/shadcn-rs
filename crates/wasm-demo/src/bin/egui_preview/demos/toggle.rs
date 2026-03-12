use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{ControlSize, ToggleVariant, toggle};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let _ = toggle(
        ui,
        &app.theme,
        &mut app.switch_enabled,
        if compact { "T" } else { "Toggle" },
        ToggleVariant::Outline,
        if compact {
            ControlSize::Icon
        } else {
            ControlSize::Md
        },
        true,
    );
}
