use super::super::app::EguiPreviewApp;
use eframe::egui::{Align, Layout, RichText, Sense, Ui, vec2};
use egui_shadcn::{
    SidebarGroupLabelProps, SidebarGroupProps, SidebarMenuButtonProps, SidebarProps,
    SidebarProviderProps,
    icons::{icon_calendar, icon_check},
    sidebar, sidebar_content, sidebar_group, sidebar_group_content, sidebar_group_label,
    sidebar_header, sidebar_menu, sidebar_menu_button, sidebar_menu_item, sidebar_provider,
    sidebar_trigger,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let open_id = ui.make_persistent_id("preview-sidebar-open");
    let provider_id = ui.make_persistent_id("preview-sidebar-provider");
    let mut sidebar_open = ui.data(|d| d.get_temp::<bool>(open_id)).unwrap_or(!compact);
    let row_width = if compact { 300.0 } else { 520.0 };

    ui.horizontal(|row| {
        row.add_space(((row.available_width() - row_width) * 0.5).max(0.0));
        sidebar_provider(
            row,
            SidebarProviderProps::new(provider_id, &mut sidebar_open)
                .expanded_width(if compact { 180.0 } else { 240.0 })
                .collapsed_width(56.0),
            |ui, sidebar_ctx| {
                ui.horizontal(|layout| {
                    let _ = sidebar(
                        layout,
                        &app.theme,
                        sidebar_ctx,
                        SidebarProps::new(),
                        |sidebar_ui, sidebar_ctx| {
                            sidebar_header(sidebar_ui, sidebar_ctx, |header, ctx| {
                                header.horizontal(|row| {
                                    let icon_size = 14.0;
                                    let (rect, _resp) = row.allocate_exact_size(
                                        vec2(icon_size, icon_size),
                                        Sense::hover(),
                                    );
                                    icon_check(
                                        row.painter(),
                                        rect.center(),
                                        icon_size,
                                        app.theme.palette.sidebar_foreground,
                                    );
                                    if !ctx.is_collapsed() {
                                        row.add_space(8.0);
                                        row.label(RichText::new("Acme Inc").strong());
                                    }
                                });
                            });
                            sidebar_content(sidebar_ui, sidebar_ctx, |content_ui, ctx| {
                                sidebar_group(
                                    content_ui,
                                    ctx,
                                    SidebarGroupProps::new(),
                                    |group_ui| {
                                        sidebar_group_label(
                                            group_ui,
                                            &app.theme,
                                            ctx,
                                            SidebarGroupLabelProps::new("Navigation"),
                                        );
                                        sidebar_group_content(group_ui, ctx, |group_ui| {
                                            sidebar_menu(group_ui, |menu_ui| {
                                                sidebar_menu_item(menu_ui, |item_ui| {
                                                    let _ = sidebar_menu_button(
                                                        item_ui,
                                                        &app.theme,
                                                        ctx,
                                                        SidebarMenuButtonProps::new("Overview")
                                                            .icon(icon_check)
                                                            .show_label_when_collapsed(false)
                                                            .active(true),
                                                    );
                                                });
                                                sidebar_menu_item(menu_ui, |item_ui| {
                                                    let _ = sidebar_menu_button(
                                                        item_ui,
                                                        &app.theme,
                                                        ctx,
                                                        SidebarMenuButtonProps::new("Projects")
                                                            .icon(icon_calendar)
                                                            .show_label_when_collapsed(false),
                                                    );
                                                });
                                            });
                                        });
                                    },
                                );
                            });
                        },
                    );

                    layout.add_space(12.0);
                    layout.vertical(|content_ui| {
                        content_ui.with_layout(Layout::top_down(Align::Min), |content_ui| {
                            sidebar_trigger(content_ui, &app.theme, sidebar_ctx, "Toggle sidebar");
                            content_ui.add_space(8.0);
                            content_ui.label("Main content");
                        });
                    });
                });
            },
        );
    });

    ui.data_mut(|d| d.insert_temp(open_id, sidebar_open));
}
