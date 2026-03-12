use super::super::app::EguiPreviewApp;
use eframe::egui::{self, Ui};
use egui_shadcn::{TabItem, TabsProps, TabsVariant, TextProps, tabs, text};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui) {
    let items = [
        TabItem::new("account", "Account"),
        TabItem::new("password", "Password"),
    ];

    let approx_tabs_width = 170.0;
    ui.horizontal(|row| {
        row.add_space(((row.available_width() - approx_tabs_width) * 0.5).max(0.0));
        row.vertical(|content| {
            let _ = tabs(
                content,
                &app.theme,
                TabsProps::new(egui::Id::new("preview-tabs"), &items, &mut app.tabs_value)
                    .variant(TabsVariant::Soft)
                    .scrollable(false),
                |tab_content, active| {
                    if active.id == "account" {
                        let _ = text(tab_content, &app.theme, TextProps::new("Account details"));
                    } else {
                        let _ = text(tab_content, &app.theme, TextProps::new("Password settings"));
                    }
                },
            );
        });
    });
}
