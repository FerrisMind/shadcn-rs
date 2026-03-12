use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{
    ControlSize, ControlVariant, Input, InputSize, SheetProps, SheetSide, button, sheet,
    sheet_content, sheet_description, sheet_footer, sheet_header, sheet_title, sheet_trigger,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let open_id = ui.make_persistent_id("preview-sheet-open");
    let mut open = ui.data(|d| d.get_temp::<bool>(open_id)).unwrap_or(false);

    let name_id = ui.make_persistent_id("preview-sheet-name");
    let mut name = ui
        .data(|d| d.get_temp::<String>(name_id))
        .unwrap_or_else(|| "Pedro Duarte".to_owned());

    sheet(
        ui,
        SheetProps::new(ui.make_persistent_id("preview-sheet"), &mut open).side(if compact {
            SheetSide::Bottom
        } else {
            SheetSide::Right
        }),
        |ui, ctx| {
            let _ = sheet_trigger(ui, ctx, |ui| {
                button(
                    ui,
                    &app.theme,
                    "Open",
                    ControlVariant::Outline,
                    ControlSize::Md,
                    true,
                )
            });

            let mut should_close = false;
            let _ = sheet_content(ui, &app.theme, ctx, |content_ui| {
                sheet_header(content_ui, |header_ui| {
                    sheet_title(header_ui, &app.theme, "Edit profile");
                    sheet_description(header_ui, &app.theme, "Make changes and save.");
                });
                let input_id = content_ui.make_persistent_id("preview-sheet-input");
                Input::new(input_id)
                    .size(InputSize::Size2)
                    .width(content_ui.available_width())
                    .show(content_ui, &app.theme, &mut name);
                sheet_footer(content_ui, |footer_ui| {
                    if button(
                        footer_ui,
                        &app.theme,
                        "Save changes",
                        ControlVariant::Primary,
                        ControlSize::Sm,
                        true,
                    )
                    .clicked()
                    {
                        should_close = true;
                    }
                });
            });
            if should_close {
                *ctx.open = false;
            }
        },
    );

    ui.data_mut(|d| {
        d.insert_temp(open_id, open);
        d.insert_temp(name_id, name);
    });
}
