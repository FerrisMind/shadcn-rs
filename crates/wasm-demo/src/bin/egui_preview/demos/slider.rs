use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::slider;

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui) {
    let slider_id = ui.make_persistent_id("preview-slider-value");
    let mut slider_value = ui
        .data_mut(|d| d.get_persisted::<f32>(slider_id).unwrap_or(42.0))
        .clamp(0.0, 100.0);
    let mut values = vec![slider_value];
    let _ = slider(ui, &app.theme, slider_id, &mut values, 0.0, 100.0);
    slider_value = values[0];
    ui.data_mut(|d| d.insert_persisted(slider_id, slider_value));
}
