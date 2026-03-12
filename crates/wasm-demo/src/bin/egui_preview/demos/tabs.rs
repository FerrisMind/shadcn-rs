use super::super::app::EguiPreviewApp;
use eframe::egui::{self, Ui};
use egui_shadcn::{TabItem, TabsProps, TabsVariant, TextProps, tabs, text};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui) {
    let items = [
        TabItem::new("account", "Account"),
        TabItem::new("password", "Password"),
    ];
    let _ = tabs(
        ui,
        &app.theme,
        TabsProps::new(egui::Id::new("preview-tabs"), &items, &mut app.tabs_value)
            .variant(TabsVariant::Soft),
        |content, active| {
            if active.id == "account" {
                let _ = text(content, &app.theme, TextProps::new("Account details"));
            } else {
                let _ = text(content, &app.theme, TextProps::new("Password settings"));
            }
        },
    );
}
