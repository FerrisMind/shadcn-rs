use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{
    ControlSize, ControlVariant, Toast, ToastPosition, ToastVariant, Toaster, button,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let toaster = Toaster::get_or_init(ui.ctx());
    toaster.set_position(ToastPosition::BottomRight);

    let row_width = if compact { 170.0 } else { 260.0 };
    ui.horizontal(|row| {
        row.add_space(((row.available_width() - row_width) * 0.5).max(0.0));
        let _ = row.allocate_ui_with_layout(
            eframe::egui::vec2(row_width, 0.0),
            eframe::egui::Layout::left_to_right(eframe::egui::Align::Center),
            |buttons| {
                buttons.horizontal_wrapped(|buttons| {
                    if button(
                        buttons,
                        &app.theme,
                        "Default",
                        ControlVariant::Outline,
                        ControlSize::Sm,
                        true,
                    )
                    .clicked()
                    {
                        toaster.show(Toast::new("Event has been created"));
                    }

                    if button(
                        buttons,
                        &app.theme,
                        "Success",
                        ControlVariant::Outline,
                        ControlSize::Sm,
                        true,
                    )
                    .clicked()
                    {
                        toaster.show(Toast::new("Saved").variant(ToastVariant::Success));
                    }

                    if !compact
                        && button(
                            buttons,
                            &app.theme,
                            "Error",
                            ControlVariant::Outline,
                            ControlSize::Sm,
                            true,
                        )
                        .clicked()
                    {
                        toaster.show(Toast::new("Failed").variant(ToastVariant::Error));
                    }
                });
            },
        );
    });

    toaster.render(ui, &app.theme);
}
