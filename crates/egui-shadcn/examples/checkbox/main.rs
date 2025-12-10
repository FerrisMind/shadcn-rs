use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::{ControlSize, ControlVariant, Theme, checkbox};
use log::{error, info};

struct CheckboxDemo {
    theme: Theme,
    primary_checked: bool,
    secondary_checked: bool,
    ghost_checked: bool,
    outline_checked: bool,
    destructive_checked: bool,
    sm_checked: bool,
    md_checked: bool,
    lg_checked: bool,
    disabled_checked: bool,
}

impl CheckboxDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            primary_checked: false,
            secondary_checked: true,
            ghost_checked: false,
            outline_checked: true,
            destructive_checked: false,
            sm_checked: false,
            md_checked: true,
            lg_checked: false,
            disabled_checked: true,
        }
    }
}

fn apply_dark_background(ctx: &egui::Context) {
    let mut style = ctx.style().as_ref().clone();
    let bg = egui::Color32::from_rgb(10, 10, 10);
    let input_bg = egui::Color32::from_rgb(21, 21, 21);
    style.visuals.window_fill = bg;
    style.visuals.panel_fill = bg;
    style.visuals.extreme_bg_color = input_bg;
    ctx.set_style(style);
}

impl App for CheckboxDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        apply_dark_background(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Checkbox — Variants");
            ui.label("Demonstration of all ControlVariant options.");
            ui.add_space(8.0);
            checkbox(
                ui,
                &self.theme,
                &mut self.primary_checked,
                "Primary",
                ControlVariant::Primary,
                ControlSize::Md,
                true,
            );
            checkbox(
                ui,
                &self.theme,
                &mut self.secondary_checked,
                "Secondary",
                ControlVariant::Secondary,
                ControlSize::Md,
                true,
            );
            checkbox(
                ui,
                &self.theme,
                &mut self.ghost_checked,
                "Ghost",
                ControlVariant::Ghost,
                ControlSize::Md,
                true,
            );
            checkbox(
                ui,
                &self.theme,
                &mut self.outline_checked,
                "Outline",
                ControlVariant::Outline,
                ControlSize::Md,
                true,
            );
            checkbox(
                ui,
                &self.theme,
                &mut self.destructive_checked,
                "Destructive",
                ControlVariant::Destructive,
                ControlSize::Md,
                true,
            );
            ui.add_space(12.0);

            ui.heading("Checkbox — Sizes");
            checkbox(
                ui,
                &self.theme,
                &mut self.sm_checked,
                "Small",
                ControlVariant::Primary,
                ControlSize::Sm,
                true,
            );
            checkbox(
                ui,
                &self.theme,
                &mut self.md_checked,
                "Medium",
                ControlVariant::Primary,
                ControlSize::Md,
                true,
            );
            checkbox(
                ui,
                &self.theme,
                &mut self.lg_checked,
                "Large",
                ControlVariant::Primary,
                ControlSize::Lg,
                true,
            );
            ui.add_space(12.0);

            ui.heading("Checkbox — Disabled");
            checkbox(
                ui,
                &self.theme,
                &mut self.disabled_checked,
                "Disabled",
                ControlVariant::Secondary,
                ControlSize::Md,
                false,
            );
        });
    }
}

fn main() {
    env_logger::init();
    info!("Starting checkbox example");

    let native_options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "egui-shadcn — checkbox",
        native_options,
        Box::new(|_cc| Ok(Box::new(CheckboxDemo::new()))),
    ) {
        error!("Failed to run checkbox example: {err}");
    }
}
