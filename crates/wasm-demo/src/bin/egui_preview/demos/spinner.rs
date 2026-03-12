use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{SpinnerProps, SpinnerSize, SpinnerVariant, spinner};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let _ = spinner(
        ui,
        &app.theme,
        SpinnerProps::default()
            .size(if compact {
                SpinnerSize::Size2
            } else {
                SpinnerSize::Size3
            })
            .variant(SpinnerVariant::RadixLeaf),
    );
}
