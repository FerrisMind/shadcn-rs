use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{
    AlertDialogProps, AlertDialogResult, Button, ButtonSize, ButtonVariant, alert_dialog,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui) {
    if Button::new("Show Alert Dialog")
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::Default)
        .show(ui, &app.theme)
        .clicked()
    {
        app.alert_open = true;
    }
    if app.alert_open {
        let result = alert_dialog(
            ui,
            &app.theme,
            AlertDialogProps::new(
                &mut app.alert_open,
                "Delete Item",
                "This action cannot be undone.",
            ),
        );
        if result == AlertDialogResult::Confirmed {
            app.alert_open = false;
        }
    }
}
