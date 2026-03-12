use super::super::app::EguiPreviewApp;
use super::super::catalog::component_icon;
use super::super::ui_home::icon_text;
use eframe::egui::Ui;
use egui_shadcn::{BadgeProps, BadgeVariant, TextProps, badge, text};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, slug: &str, compact: bool) {
    ui.vertical_centered(|content| {
        let _ = text(
            content,
            &app.theme,
            TextProps::new(icon_text(
                component_icon(slug),
                if compact { 30.0 } else { 42.0 },
            )),
        );
        content.add_space(8.0);
        badge(
            content,
            &app.theme,
            BadgeProps::new("Source Included").variant(BadgeVariant::Outline),
        );
    });
}
