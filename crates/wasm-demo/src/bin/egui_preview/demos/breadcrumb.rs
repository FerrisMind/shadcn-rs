use super::super::app::EguiPreviewApp;
use eframe::egui::{Id, Ui};
use egui_shadcn::{
    BreadcrumbProps, DropdownMenuItemProps, DropdownMenuProps, DropdownMenuTriggerProps,
    breadcrumb, breadcrumb_ellipsis, breadcrumb_item, breadcrumb_link, breadcrumb_list,
    breadcrumb_page, breadcrumb_separator, dropdown_menu, dropdown_menu_item,
    dropdown_menu_trigger,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    breadcrumb(ui, &app.theme, BreadcrumbProps::new(), |ui, ctx| {
        breadcrumb_list(ui, ctx, |ui, ctx| {
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_link(ui, ctx, "Home");
            });
            breadcrumb_separator(ui, ctx, None);
            if compact {
                breadcrumb_item(ui, ctx, |ui| {
                    breadcrumb_ellipsis(ui, ctx);
                });
            } else {
                breadcrumb_item(ui, ctx, |ui| {
                    let trigger = dropdown_menu_trigger(
                        ui,
                        DropdownMenuTriggerProps::new(Id::new("preview-breadcrumb-menu")),
                        |ui| breadcrumb_link(ui, ctx, "Components"),
                    );
                    let _ = dropdown_menu(
                        ui,
                        &app.theme,
                        DropdownMenuProps::new(&trigger.response),
                        |menu_ui| {
                            let _ = dropdown_menu_item(
                                menu_ui,
                                &app.theme,
                                DropdownMenuItemProps::new("Navigation"),
                            );
                            let _ = dropdown_menu_item(
                                menu_ui,
                                &app.theme,
                                DropdownMenuItemProps::new("Data Display"),
                            );
                        },
                    );
                });
            }
            breadcrumb_separator(ui, ctx, None);
            breadcrumb_item(ui, ctx, |ui| {
                breadcrumb_page(ui, ctx, "Breadcrumb");
            });
        });
    });
}
