use super::super::app::EguiPreviewApp;
use eframe::egui::{Align2, FontId, Sense, Stroke, StrokeKind, Ui, vec2};
use egui_shadcn::{
    ContextMenuCheckboxItemProps, ContextMenuItemProps, ContextMenuLabelProps,
    ContextMenuRadioItemProps, context_menu, context_menu_checkbox_item, context_menu_item,
    context_menu_label, context_menu_radio_item, context_menu_separator,
};
use lucide_icons::Icon;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let show_bookmarks_id = ui.make_persistent_id("preview-cm-bookmarks");
    let selected_person_id = ui.make_persistent_id("preview-cm-person");
    let mut show_bookmarks = ui
        .data(|d| d.get_temp::<bool>(show_bookmarks_id))
        .unwrap_or(true);
    let mut selected_person = ui
        .data(|d| d.get_temp::<String>(selected_person_id))
        .unwrap_or_else(|| "pedro".to_owned());

    let (rect, response) = ui.allocate_exact_size(
        if compact {
            vec2(220.0, 90.0)
        } else {
            vec2(300.0, 130.0)
        },
        Sense::click(),
    );
    ui.painter().rect_stroke(
        rect,
        6.0,
        Stroke::new(1.0_f32, app.theme.palette.border),
        StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        "Right click here",
        FontId::proportional(14.0),
        app.theme.palette.muted_foreground,
    );

    context_menu(&response, &app.theme, |menu_ui| {
        let icon_back = Icon::ArrowLeft.unicode().to_string();
        let icon_reload = Icon::RefreshCw.unicode().to_string();

        let _ = context_menu_item(
            menu_ui,
            &app.theme,
            ContextMenuItemProps::new("Back").icon(icon_back.as_str()),
        );
        let _ = context_menu_item(
            menu_ui,
            &app.theme,
            ContextMenuItemProps::new("Reload").icon(icon_reload.as_str()),
        );
        context_menu_separator(menu_ui, &app.theme);
        let checkbox = context_menu_checkbox_item(
            menu_ui,
            &app.theme,
            ContextMenuCheckboxItemProps::new("Show Bookmarks", show_bookmarks),
        );
        if checkbox.clicked() {
            show_bookmarks = !show_bookmarks;
        }
        if !compact {
            context_menu_separator(menu_ui, &app.theme);
            context_menu_label(menu_ui, &app.theme, ContextMenuLabelProps::new("People"));
            if context_menu_radio_item(
                menu_ui,
                &app.theme,
                ContextMenuRadioItemProps::new("Pedro Duarte", "pedro", &selected_person),
            )
            .clicked()
            {
                selected_person = "pedro".to_owned();
            }
            if context_menu_radio_item(
                menu_ui,
                &app.theme,
                ContextMenuRadioItemProps::new("Colm Tuite", "colm", &selected_person),
            )
            .clicked()
            {
                selected_person = "colm".to_owned();
            }
        }
    });

    ui.data_mut(|d| {
        d.insert_temp(show_bookmarks_id, show_bookmarks);
        d.insert_temp(selected_person_id, selected_person);
    });
}
