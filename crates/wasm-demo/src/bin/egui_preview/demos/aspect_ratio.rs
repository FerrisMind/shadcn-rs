use super::super::app::EguiPreviewApp;
use eframe::egui::{self, Ui};
use egui_shadcn::{AspectRatioProps, CardProps, CardVariant, TextProps, aspect_ratio, card, text};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let ratio = 16.0 / 9.0;
    let box_width = if compact { 208.0 } else { 360.0 };

    ui.vertical_centered(|center| {
        center.set_min_width(box_width);
        center.set_max_width(box_width);
        let _ = aspect_ratio(center, AspectRatioProps::new(ratio), |ratio_ui| {
            card(
                ratio_ui,
                &app.theme,
                CardProps::default()
                    .variant(CardVariant::Classic)
                    .padding(egui::vec2(8.0, 8.0)),
                |content| {
                    content.centered_and_justified(|inner| {
                        let _ = text(inner, &app.theme, TextProps::new("16:9"));
                    });
                },
            );
        });
    });
}
