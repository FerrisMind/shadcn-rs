use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{AlertProps, AlertVariant, alert};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    if compact {
        ui.vertical_centered(|center| {
            center.set_min_width(230.0);
            center.set_max_width(230.0);
            alert(
                center,
                &app.theme,
                AlertProps::new("Component alert preview")
                    .title("Heads up")
                    .variant(AlertVariant::Info),
            );
        });
        return;
    }

    alert(
        ui,
        &app.theme,
        AlertProps::new("Component alert preview")
            .title("Heads up")
            .variant(AlertVariant::Info),
    );
}
