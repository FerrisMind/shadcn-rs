#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::RichText;
use egui_shadcn::{
    ControlSize, ControlVariant, Label, LabelProps, LabelVariant, Theme, checkbox, label_with_props,
};

struct LabelDemo {
    theme: Theme,
    terms: bool,
}

impl LabelDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            terms: false,
        }
    }
}

impl App for LabelDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Label Component");
            ui.add_space(16.0);

            // -- With Checkbox --
            ui.label(RichText::new("With Checkbox").strong());
            ui.add_space(4.0);
            ui.horizontal(|row| {
                row.spacing_mut().item_spacing.x = 8.0;
                let _ = checkbox(
                    row,
                    &self.theme,
                    &mut self.terms,
                    "",
                    ControlVariant::Primary,
                    ControlSize::Md,
                    true,
                );
                let resp = Label::new("Accept terms and conditions")
                    .size(ControlSize::Md)
                    .show(row, &self.theme);
                if resp.clicked() {
                    self.terms = !self.terms;
                }
            });

            ui.add_space(16.0);

            // -- Variants --
            ui.label(RichText::new("Variants").strong());
            ui.add_space(4.0);
            label_with_props(
                ui,
                &self.theme,
                LabelProps::new("Default variant").variant(LabelVariant::Default),
            );
            label_with_props(
                ui,
                &self.theme,
                LabelProps::new("Secondary variant").variant(LabelVariant::Secondary),
            );
            label_with_props(
                ui,
                &self.theme,
                LabelProps::new("Muted variant").variant(LabelVariant::Muted),
            );
            label_with_props(
                ui,
                &self.theme,
                LabelProps::new("Destructive variant").variant(LabelVariant::Destructive),
            );

            ui.add_space(16.0);

            // -- Sizes --
            ui.label(RichText::new("Sizes").strong());
            ui.add_space(4.0);
            label_with_props(
                ui,
                &self.theme,
                LabelProps::new("Small label").size(ControlSize::Sm),
            );
            label_with_props(
                ui,
                &self.theme,
                LabelProps::new("Medium label (default)").size(ControlSize::Md),
            );
            label_with_props(
                ui,
                &self.theme,
                LabelProps::new("Large label").size(ControlSize::Lg),
            );

            ui.add_space(16.0);

            // -- Disabled --
            ui.label(RichText::new("Disabled").strong());
            ui.add_space(4.0);
            label_with_props(
                ui,
                &self.theme,
                LabelProps::new("This label is disabled").disabled(true),
            );

            ui.add_space(16.0);

            // -- Required --
            ui.label(RichText::new("Required").strong());
            ui.add_space(4.0);
            label_with_props(
                ui,
                &self.theme,
                LabelProps::new("Email address").required(true),
            );

            ui.add_space(16.0);

            // -- With Description --
            ui.label(RichText::new("With Description").strong());
            ui.add_space(4.0);
            label_with_props(
                ui,
                &self.theme,
                LabelProps::new("Username")
                    .required(true)
                    .description("This is your public display name."),
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Label example",
        options,
        Box::new(|_cc| Ok(Box::new(LabelDemo::new()))),
    )
}
