use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{ControlSize, ControlVariant, Input, InputSize, PopoverProps, button, popover};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let scope_id = ui.id().with("preview-popover");
    let open_id = scope_id.with("open");
    let width_id = scope_id.with("width");
    let mut open = ui.data(|d| d.get_temp::<bool>(open_id)).unwrap_or(false);
    let mut width_value = ui
        .data(|d| d.get_temp::<String>(width_id))
        .unwrap_or_else(|| "100%".to_owned());

    let _ = popover(
        ui,
        &app.theme,
        PopoverProps::new(scope_id.with("root"), &mut open).width(if compact {
            220.0
        } else {
            320.0
        }),
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
            let input_id = scope_id.with("input");
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
