use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::{ControlSize, Theme, text_input};
use log::{error, info};

struct InputDemo {
    theme: Theme,
    value_sm: String,
    value_md: String,
    value_lg: String,
    value_invalid: String,
    value_disabled: String,
}

impl InputDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            value_sm: String::from("sm"),
            value_md: String::from("middle"),
            value_lg: String::from("large input"),
            value_invalid: String::new(),
            value_disabled: String::from("read only"),
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

impl App for InputDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        apply_dark_background(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Text Input — Sizes");
            ui.add_space(8.0);
            text_input(
                ui,
                &self.theme,
                &mut self.value_sm,
                "Small input",
                ControlSize::Sm,
                false,
                true,
            );
            text_input(
                ui,
                &self.theme,
                &mut self.value_md,
                "Medium input",
                ControlSize::Md,
                false,
                true,
            );
            text_input(
                ui,
                &self.theme,
                &mut self.value_lg,
                "Large input",
                ControlSize::Lg,
                false,
                true,
            );

            ui.add_space(12.0);
            ui.heading("Text Input — States");
            text_input(
                ui,
                &self.theme,
                &mut self.value_invalid,
                "Required field (invalid)",
                ControlSize::Md,
                true,
                true,
            );
            text_input(
                ui,
                &self.theme,
                &mut self.value_disabled,
                "Disabled input",
                ControlSize::Md,
                false,
                false,
            );
        });
    }
}

fn main() {
    env_logger::init();
    info!("Starting text_input example");

    let native_options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "egui-shadcn — text input",
        native_options,
        Box::new(|_cc| Ok(Box::new(InputDemo::new()))),
    ) {
        error!("Failed to run text_input example: {err}");
    }
}
