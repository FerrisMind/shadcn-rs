use super::super::app::EguiPreviewApp;
use eframe::egui::{self, Align2, FontFamily, FontId, Ui};
use egui_shadcn::{Input as ShadInput, InputSize, InputType};
use lucide_icons::Icon;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let input_id = ui.make_persistent_id("preview-email-input");
    let _ = ShadInput::new(input_id)
        .input_type(InputType::Email)
        .placeholder("you@example.com")
        .size(InputSize::Size2)
        .width(if compact { 220.0 } else { 420.0 })
        .left_slot(|painter, rect, color| {
            let icon_size = 14.0;
            painter.text(
                egui::pos2(rect.left() + 4.0 + icon_size * 0.5, rect.center().y),
                Align2::CENTER_CENTER,
                Icon::Search.unicode().to_string(),
                FontId::new(icon_size, FontFamily::Name("lucide".into())),
                color,
            );
        })
        .show(ui, &app.theme, &mut app.email);
}
