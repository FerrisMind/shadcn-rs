use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{RadioCardVariant, RadioDirection, RadioGroupProps, RadioOption, radio_group};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let options = vec![
        RadioOption::new("starter".to_owned(), "Starter"),
        RadioOption::new("pro".to_owned(), "Pro"),
        RadioOption::new("team".to_owned(), "Team"),
    ];

    let props = RadioGroupProps::new("preview-radio", &mut app.radio_value, &options)
        .direction(if compact {
            RadioDirection::Vertical
        } else {
            RadioDirection::Horizontal
        })
        .card_variant(if compact {
            RadioCardVariant::Button
        } else {
            RadioCardVariant::Card
        });

    let row_width = if compact { 170.0 } else { 300.0 };
    ui.horizontal(|row| {
        row.add_space(((row.available_width() - row_width) * 0.5).max(0.0));
        let _ = radio_group(row, &app.theme, props);
    });
}
