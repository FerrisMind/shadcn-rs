use super::super::app::EguiPreviewApp;
use eframe::egui::{Align, Layout, Ui};
use egui_shadcn::{ControlSize, ControlVariant, TooltipProps, button, tooltip};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, _compact: bool) {
    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        let response = button(
            ui,
            &app.theme,
            "Hover",
            ControlVariant::Outline,
            ControlSize::Md,
            true,
        );
        let _ = tooltip(
            &response,
            ui,
            &app.theme,
            TooltipProps::new("Add to library")
                .delay_ms(0)
                .skip_delay_ms(0)
                .show_arrow(true)
                .side_offset(8.0),
        );
    });
}
