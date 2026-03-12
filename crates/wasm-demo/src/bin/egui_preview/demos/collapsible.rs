use super::super::app::EguiPreviewApp;
use eframe::egui::{Id, RichText, Ui};
use egui_shadcn::{Button, ButtonSize, ButtonVariant, CollapsibleProps, collapsible};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let state_id = ui.make_persistent_id("preview-collapsible-open");
    let mut open = ui.data(|d| d.get_temp::<bool>(state_id)).unwrap_or(false);
    let is_open = open;

    collapsible(
        ui,
        &app.theme,
        CollapsibleProps::new(Id::new("preview-collapsible"), &mut open)
            .animation(true)
            .animation_ms(220.0),
        |ui, api| {
            let _ = api.trigger(ui, |ui| {
                Button::new(if is_open {
                    "Hide details"
                } else {
                    "Show details"
                })
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Sm)
                .show(ui, &app.theme)
            });

            let _ = api.content(ui, |ui| {
                ui.add_space(8.0);
                ui.label(RichText::new("@radix-ui/primitives").strong());
                if !compact {
                    ui.label("@radix-ui/colors");
                    ui.label("@stitches/react");
                }
            });
        },
    );

    ui.data_mut(|d| d.insert_temp(state_id, open));
}
