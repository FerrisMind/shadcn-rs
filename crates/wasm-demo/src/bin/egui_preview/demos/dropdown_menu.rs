use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{
    ControlSize, ControlVariant, DropdownMenuCheckboxItemProps, DropdownMenuItemProps,
    DropdownMenuProps, DropdownMenuTriggerProps, button, dropdown_menu,
    dropdown_menu_checkbox_item, dropdown_menu_item, dropdown_menu_separator,
    dropdown_menu_trigger,
};
use lucide_icons::Icon;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let checked_id = ui.make_persistent_id("preview-dropdown-checked");
    let mut checked = ui.data(|d| d.get_temp::<bool>(checked_id)).unwrap_or(true);

    let trigger = dropdown_menu_trigger(
        ui,
        DropdownMenuTriggerProps::new(ui.make_persistent_id("preview-dropdown-trigger")),
        |ui| {
            button(
                ui,
                &app.theme,
                "Open menu",
                ControlVariant::Outline,
                ControlSize::Md,
                true,
            )
        },
    );

    let _ = dropdown_menu(
        ui,
        &app.theme,
        DropdownMenuProps::new(&trigger.response),
        |menu_ui| {
            let icon_back = Icon::ArrowLeft.unicode().to_string();
            let icon_reload = Icon::RefreshCw.unicode().to_string();
            let icon_tools = Icon::Wrench.unicode().to_string();
            let _ = dropdown_menu_item(
                menu_ui,
                &app.theme,
                DropdownMenuItemProps::new("Back").icon(icon_back.as_str()),
            );
            let _ = dropdown_menu_item(
                menu_ui,
                &app.theme,
                DropdownMenuItemProps::new("Reload").icon(icon_reload.as_str()),
            );
            dropdown_menu_separator(menu_ui, &app.theme);
            let resp = dropdown_menu_checkbox_item(
                menu_ui,
                &app.theme,
                DropdownMenuCheckboxItemProps::new("Show status bar", checked),
            );
            if resp.clicked() {
                checked = !checked;
            }
            if !compact {
                let _ = dropdown_menu_item(
                    menu_ui,
                    &app.theme,
                    DropdownMenuItemProps::new("Dev Tools").icon(icon_tools.as_str()),
                );
            }
        },
    );

    ui.data_mut(|d| d.insert_temp(checked_id, checked));
}
