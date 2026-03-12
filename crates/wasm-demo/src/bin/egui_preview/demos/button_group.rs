use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{Button, ButtonGroup, ButtonGroupOrientation, ButtonVariant, button_group};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    if compact {
        let row_width = 220.0;
        ui.horizontal(|row| {
            row.add_space(((row.available_width() - row_width) * 0.5).max(0.0));
            button_group(
                row,
                &app.theme,
                vec![
                    Button::new("Left").variant(ButtonVariant::Outline),
                    Button::new("Center").variant(ButtonVariant::Outline),
                    Button::new("Right").variant(ButtonVariant::Outline),
                ],
            );
        });
        return;
    }

    let row_width = 240.0;
    ui.horizontal(|row| {
        row.add_space(((row.available_width() - row_width) * 0.5).max(0.0));
        row.vertical(|ui| {
            button_group(
                ui,
                &app.theme,
                vec![
                    Button::new("Bold").variant(ButtonVariant::Outline),
                    Button::new("Italic").variant(ButtonVariant::Outline),
                    Button::new("Underline").variant(ButtonVariant::Outline),
                ],
            );
            ui.add_space(8.0);
            ButtonGroup::new(vec![
                Button::new("Top").variant(ButtonVariant::Secondary),
                Button::new("Middle").variant(ButtonVariant::Secondary),
                Button::new("Bottom").variant(ButtonVariant::Secondary),
            ])
            .orientation(ButtonGroupOrientation::Vertical)
            .show(ui, &app.theme);
        });
    });
}
