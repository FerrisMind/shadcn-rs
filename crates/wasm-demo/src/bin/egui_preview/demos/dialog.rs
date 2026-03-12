use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{
    ControlSize, ControlVariant, DialogAlign, DialogProps, Input, InputSize, button, dialog,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, _compact: bool) {
    let open_id = ui.make_persistent_id("preview-dialog-open");
    let mut open = ui.data(|d| d.get_temp::<bool>(open_id)).unwrap_or(false);

    if button(
        ui,
        &app.theme,
        "Open Dialog",
        ControlVariant::Outline,
        ControlSize::Md,
        true,
    )
    .clicked()
    {
        open = true;
    }

    let name_id = ui.make_persistent_id("preview-dialog-name");
    let mut name = ui
        .data(|d| d.get_temp::<String>(name_id))
        .unwrap_or_else(|| "Pedro Duarte".to_owned());

    let mut should_close = false;
    let _ = dialog(
        ui,
        &app.theme,
        DialogProps::new(ui.make_persistent_id("preview-dialog"), &mut open)
            .title("Edit profile")
            .description("Make changes and save.")
            .align(DialogAlign::Center)
            .max_width(420.0)
            .height(220.0)
            .scrollable(false),
        |body| {
            let input_id = body.make_persistent_id("preview-dialog-input");
            Input::new(input_id)
                .size(InputSize::Size2)
                .width(body.available_width())
                .show(body, &app.theme, &mut name);
            body.add_space(12.0);
            if button(
                body,
                &app.theme,
                "Save",
                ControlVariant::Primary,
                ControlSize::Sm,
                true,
            )
            .clicked()
            {
                should_close = true;
            }
        },
    );

    if should_close {
        open = false;
    }

    ui.data_mut(|d| {
        d.insert_temp(open_id, open);
        d.insert_temp(name_id, name);
    });
}
