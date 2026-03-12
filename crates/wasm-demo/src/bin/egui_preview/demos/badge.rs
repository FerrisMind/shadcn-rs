use super::super::app::EguiPreviewApp;
use super::super::catalog::component_icon;
use eframe::egui::Ui;
use egui_shadcn::{BadgeProps, BadgeVariant, badge};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, slug: &str) {
    let row_width = 260.0;
    ui.horizontal(|container| {
        container.add_space(((container.available_width() - row_width) * 0.5).max(0.0));
        container.horizontal_wrapped(|row| {
            badge(
                row,
                &app.theme,
                BadgeProps::new("Default").variant(BadgeVariant::Default),
            );
            badge(
                row,
                &app.theme,
                BadgeProps::new("Secondary").variant(BadgeVariant::Secondary),
            );
            badge(
                row,
                &app.theme,
                BadgeProps::new("With Icon").icon(component_icon(slug)),
            );
        });
    });
}
