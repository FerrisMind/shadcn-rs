use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{CalendarCaptionLayout, CalendarProps, calendar_with_props};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui) {
    calendar_with_props(
        ui,
        &app.theme,
        CalendarProps::new("preview-calendar").caption_layout(CalendarCaptionLayout::Dropdown),
    );
}
