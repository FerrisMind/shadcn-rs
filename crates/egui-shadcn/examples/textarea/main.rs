use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::{ControlSize, Theme};
use log::{error, info};

struct TextareaDemo {
    theme: Theme,
    basic_text: String,
    invalid_text: String,
    limited_text: String,
    disabled_text: String,
    no_counter_text: String,
}

impl TextareaDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            basic_text: "Basic input".into(),
            invalid_text: String::new(),
            limited_text: "Character counter enabled".into(),
            disabled_text: "Field is locked".into(),
            no_counter_text: "No counter".into(),
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

impl App for TextareaDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        apply_dark_background(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Textarea — Basic Field");
            egui_shadcn::Textarea::new("basic_textarea")
                .placeholder("Enter text")
                .size(ControlSize::Md.into())
                .show(ui, &self.theme, &mut self.basic_text);

            ui.add_space(32.0);

            ui.heading("Textarea — Counter and Limit");
            egui_shadcn::Textarea::new("limited_textarea")
                .placeholder("Max 120 characters")
                .size(ControlSize::Lg.into())
                .show_counter(true)
                .max_len(120)
                .show(ui, &self.theme, &mut self.limited_text);

            ui.add_space(32.0);

            ui.heading("Textarea — invalid");
            egui_shadcn::Textarea::new("invalid_textarea")
                .placeholder("Required field")
                .size(ControlSize::Md.into())
                .invalid(true)
                .show_counter(true)
                .max_len(60)
                .show(ui, &self.theme, &mut self.invalid_text);

            ui.add_space(32.0);

            ui.heading("Textarea — No Counter (Sm)");
            egui_shadcn::Textarea::new("sm_textarea")
                .placeholder("Compact input")
                .size(ControlSize::Sm.into())
                .show(ui, &self.theme, &mut self.no_counter_text);

            ui.add_space(32.0);

            ui.heading("Textarea — Disabled");
            egui_shadcn::Textarea::new("disabled_textarea")
                .placeholder("Disabled")
                .size(ControlSize::Md.into())
                .enabled(false)
                .show(ui, &self.theme, &mut self.disabled_text);
        });
    }
}

fn main() {
    env_logger::init();
    info!("Starting textarea example");

    let native_options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "egui-shadcn — textarea",
        native_options,
        Box::new(|_cc| Ok(Box::new(TextareaDemo::new()))),
    ) {
        error!("Failed to run textarea example: {err}");
    }
}
