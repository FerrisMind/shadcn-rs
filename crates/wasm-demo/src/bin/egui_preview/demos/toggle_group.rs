use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{
    ControlSize, ToggleGroupProps, ToggleVariant, toggle_group, toggle_group_item,
    toggle_group_item_last,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let left_id = ui.make_persistent_id("preview-toggle-group-left");
    let right_id = ui.make_persistent_id("preview-toggle-group-right");
    let mut left_on = ui.data_mut(|d| d.get_persisted::<bool>(left_id).unwrap_or(true));
    let mut right_on = ui.data_mut(|d| d.get_persisted::<bool>(right_id).unwrap_or(false));
    let row_width = if compact { 120.0 } else { 160.0 };
    ui.horizontal(|row| {
        row.add_space(((row.available_width() - row_width) * 0.5).max(0.0));
        let _ = row.allocate_ui_with_layout(
            eframe::egui::vec2(row_width, 0.0),
            eframe::egui::Layout::left_to_right(eframe::egui::Align::Center),
            |centered| {
                let _ = toggle_group(
                    centered,
                    ToggleGroupProps {
                        variant: ToggleVariant::Outline,
                        size: if compact {
                            ControlSize::Sm
                        } else {
                            ControlSize::Md
                        },
                    },
                    |group_ui, ctx| {
                        let _ = toggle_group_item(group_ui, &app.theme, ctx, &mut left_on, "Left");
                        let _ = toggle_group_item_last(
                            group_ui,
                            &app.theme,
                            ctx,
                            &mut right_on,
                            "Right",
                        );
                    },
                );
            },
        );
    });
    ui.data_mut(|d| {
        d.insert_persisted(left_id, left_on);
        d.insert_persisted(right_id, right_on);
    });
}
