use super::super::app::EguiPreviewApp;
use eframe::egui::{CornerRadius, Margin, RichText, Ui};
use egui_shadcn::{
    NavigationMenuContentProps, NavigationMenuLinkProps, NavigationMenuProps, navigation_menu,
    navigation_menu_content, navigation_menu_item, navigation_menu_link, navigation_menu_list,
    navigation_menu_trigger,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let row_width = if compact { 260.0 } else { 340.0 };
    let nav_id = ui.id().with("preview-navigation-menu");
    ui.horizontal(|row| {
        row.add_space(((row.available_width() - row_width) * 0.5).max(0.0));
        navigation_menu(
            row,
            &app.theme,
            NavigationMenuProps::new(nav_id),
            |ui, nav_ctx| {
                navigation_menu_list(ui, nav_ctx, |ui, nav_ctx| {
                    navigation_menu_item(ui, nav_ctx, "getting-started", |ui, item_ctx| {
                        navigation_menu_trigger(
                            ui,
                            &app.theme,
                            nav_ctx,
                            item_ctx,
                            "Getting started",
                        );
                        let _ = navigation_menu_content(
                            ui,
                            &app.theme,
                            nav_ctx,
                            item_ctx,
                            NavigationMenuContentProps::new().width(if compact {
                                240.0
                            } else {
                                420.0
                            }),
                            |content_ui| {
                                content_ui.label(RichText::new("Introduction").strong());
                                content_ui.label(
                                    RichText::new("Reusable components built with egui.")
                                        .size(12.0)
                                        .color(app.theme.palette.muted_foreground),
                                );
                            },
                        );
                    });

                    let _ = navigation_menu_link(
                        ui,
                        &app.theme,
                        NavigationMenuLinkProps::new()
                            .min_width(120.0)
                            .min_height(32.0)
                            .padding(Margin::symmetric(12, 6))
                            .rounding(CornerRadius::same(6)),
                        |link_ui, state| {
                            let color = if state.hovered {
                                app.theme.palette.accent_foreground
                            } else {
                                app.theme.palette.foreground
                            };
                            link_ui.label(RichText::new("Documentation").size(13.0).color(color));
                        },
                    );
                });
            },
        );
    });
}
