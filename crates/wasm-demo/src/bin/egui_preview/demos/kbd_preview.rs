use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{KbdProps, KbdSize, kbd};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    ui.horizontal(|row| {
        let row_width = 120.0;
        row.add_space(((row.available_width() - row_width) * 0.5).max(0.0));
        row.horizontal(|ui| {
            kbd(ui, &app.theme, "Ctrl", KbdProps::new());
            ui.label("+");
            kbd(ui, &app.theme, "K", KbdProps::new());
        });
    });

    if !compact {
        ui.add_space(8.0);
        ui.horizontal(|row| {
            let row_width = 170.0;
            row.add_space(((row.available_width() - row_width) * 0.5).max(0.0));
            row.horizontal(|ui| {
                kbd(ui, &app.theme, "Esc", KbdProps::new().size(KbdSize::Size2));
                kbd(ui, &app.theme, "F", KbdProps::new().size(KbdSize::Size2));
                kbd(ui, &app.theme, "Cmd", KbdProps::new().size(KbdSize::Size2));
            });
        });
    }
}
