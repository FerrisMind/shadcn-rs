use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::{ControlSize, ControlVariant, Theme, switch};
use log::{error, info};

struct SwitchDemo {
    theme: Theme,
    primary_on: bool,
    secondary_on: bool,
    outline_on: bool,
    large_on: bool,
    disabled_on: bool,
}

impl SwitchDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            primary_on: true,
            secondary_on: false,
            outline_on: false,
            large_on: true,
            disabled_on: true,
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

impl App for SwitchDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        apply_dark_background(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Switch — Variants");
            switch(
                ui,
                &self.theme,
                &mut self.primary_on,
                "Primary",
                ControlVariant::Primary,
                ControlSize::Sm,
                true,
            );
            switch(
                ui,
                &self.theme,
                &mut self.secondary_on,
                "Secondary",
                ControlVariant::Secondary,
                ControlSize::Sm,
                true,
            );
            switch(
                ui,
                &self.theme,
                &mut self.outline_on,
                "Outline",
                ControlVariant::Outline,
                ControlSize::Sm,
                true,
            );

            ui.add_space(12.0);
            ui.heading("Switch — Sizes");
            switch(
                ui,
                &self.theme,
                &mut self.large_on,
                "Large (Lg)",
                ControlVariant::Primary,
                ControlSize::Lg,
                true,
            );

            ui.add_space(12.0);
            ui.heading("Switch — Disabled");
            switch(
                ui,
                &self.theme,
                &mut self.disabled_on,
                "Disabled",
                ControlVariant::Ghost,
                ControlSize::Md,
                false,
            );
        });
    }
}

fn main() {
    env_logger::init();
    info!("Starting switch example");

    let native_options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "egui-shadcn — switch",
        native_options,
        Box::new(|_cc| Ok(Box::new(SwitchDemo::new()))),
    ) {
        error!("Failed to run switch example: {err}");
    }
}
