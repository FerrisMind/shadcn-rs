use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{
    ControlSize, ControlVariant, Toast, ToastPosition, ToastVariant, Toaster, button,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let toaster = Toaster::get_or_init(ui.ctx());
    toaster.set_position(ToastPosition::BottomRight);

    ui.horizontal_wrapped(|row| {
        if button(
            row,
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
            row,
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
                row,
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

    toaster.render(ui, &app.theme);
}
