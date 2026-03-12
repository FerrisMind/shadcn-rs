use super::super::app::EguiPreviewApp;
use super::super::ui_home::icon_text;
use eframe::egui::Ui;
use egui_shadcn::{Button, ButtonSize, ButtonVariant};
use lucide_icons::Icon;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui) {
    let row_width = 220.0;
    ui.horizontal(|container| {
        container.add_space(((container.available_width() - row_width) * 0.5).max(0.0));
        container.horizontal_wrapped(|row| {
            let _ = Button::new("Primary")
                .variant(ButtonVariant::Default)
                .size(ButtonSize::Default)
                .show(row, &app.theme);
            let _ = Button::new("Outline")
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Default)
                .show(row, &app.theme);
            let _ = Button::new(icon_text(Icon::ArrowUpRight, 16.0))
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Icon)
                .show(row, &app.theme);
        });
    });
}
