use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{ControlSize, ControlVariant, Input, InputSize, PopoverProps, button, popover};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let open_id = ui.make_persistent_id("preview-popover-open");
    let width_id = ui.make_persistent_id("preview-popover-width");
    let mut open = ui.data(|d| d.get_temp::<bool>(open_id)).unwrap_or(false);
    let mut width_value = ui
        .data(|d| d.get_temp::<String>(width_id))
        .unwrap_or_else(|| "100%".to_owned());

    let _ =
        popover(
            ui,
            &app.theme,
            PopoverProps::new(ui.make_persistent_id("preview-popover"), &mut open)
                .width(if compact { 220.0 } else { 320.0 }),
            |trigger_ui| {
                button(
                    trigger_ui,
                    &app.theme,
                    "Open popover",
                    ControlVariant::Outline,
                    ControlSize::Md,
                    true,
                )
            },
            |content_ui| {
                content_ui.label("Dimensions");
                let input_id = content_ui.make_persistent_id("preview-popover-input");
                Input::new(input_id)
                    .size(InputSize::Size2)
                    .width(content_ui.available_width())
                    .show(content_ui, &app.theme, &mut width_value);
            },
        );

    ui.data_mut(|d| {
        d.insert_temp(open_id, open);
        d.insert_temp(width_id, width_value);
    });
}
