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

    let _ = radio_group(ui, &app.theme, props);
}
