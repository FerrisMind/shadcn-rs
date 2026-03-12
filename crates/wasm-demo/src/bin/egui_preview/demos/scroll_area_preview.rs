use super::super::app::EguiPreviewApp;
use eframe::egui::{self, Ui};
use egui_shadcn::{
    ScrollAreaProps, ScrollAreaRadius, ScrollAreaSize, ScrollAreaType, ScrollDirection,
    SeparatorProps, scroll_area, separator,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let max = if compact {
        egui::vec2(220.0, 120.0)
    } else {
        egui::vec2(420.0, 180.0)
    };
    let props = ScrollAreaProps::default()
        .id(ui.make_persistent_id("preview-scroll-area"))
        .direction(ScrollDirection::Vertical)
        .scroll_type(ScrollAreaType::Auto)
        .size(ScrollAreaSize::Size2)
        .radius(ScrollAreaRadius::Medium)
        .max_size(max)
        .auto_shrink([false; 2]);

    ui.horizontal(|row| {
        row.add_space(((row.available_width() - max.x) * 0.5).max(0.0));
        row.vertical(|content| {
            content.set_width(max.x);
            scroll_area(content, &app.theme, props, |ui| {
                ui.spacing_mut().item_spacing.y = 6.0;
                for item in [
                    "Account settings",
                    "Billing & invoices",
                    "Team members",
                    "Integrations",
                    "Webhooks",
                    "API keys",
                    "Security logs",
                    "Backups",
                    "Notifications",
                    "Support",
                ] {
                    ui.label(item);
                    separator(ui, &app.theme, SeparatorProps::default());
                }
            });
        });
    });
}
