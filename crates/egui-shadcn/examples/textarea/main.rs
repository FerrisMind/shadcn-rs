use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::{ControlSize, TextareaProps, Theme, textarea};
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
            textarea(
                ui,
                &self.theme,
                TextareaProps {
                    value: &mut self.basic_text,
                    placeholder: "Enter text".into(),
                    size: ControlSize::Md,
                    is_invalid: false,
                    show_counter: false,
                    max_len: None,
                    enabled: true,
                },
            );
            ui.add_space(12.0);

            ui.heading("Textarea — Counter and Limit");
            textarea(
                ui,
                &self.theme,
                TextareaProps {
                    value: &mut self.limited_text,
                    placeholder: "Max 120 characters".into(),
                    size: ControlSize::Lg,
                    is_invalid: false,
                    show_counter: true,
                    max_len: Some(120),
                    enabled: true,
                },
            );
            ui.add_space(12.0);

            ui.heading("Textarea — invalid");
            textarea(
                ui,
                &self.theme,
                TextareaProps {
                    value: &mut self.invalid_text,
                    placeholder: "Required field".into(),
                    size: ControlSize::Md,
                    is_invalid: true,
                    show_counter: true,
                    max_len: Some(60),
                    enabled: true,
                },
            );
            ui.add_space(12.0);

            ui.heading("Textarea — No Counter (Sm)");
            textarea(
                ui,
                &self.theme,
                TextareaProps {
                    value: &mut self.no_counter_text,
                    placeholder: "Compact input".into(),
                    size: ControlSize::Sm,
                    is_invalid: false,
                    show_counter: false,
                    max_len: None,
                    enabled: true,
                },
            );
            ui.add_space(12.0);

            ui.heading("Textarea — Disabled");
            textarea(
                ui,
                &self.theme,
                TextareaProps {
                    value: &mut self.disabled_text,
                    placeholder: "Disabled".into(),
                    size: ControlSize::Md,
                    is_invalid: false,
                    show_counter: false,
                    max_len: None,
                    enabled: false,
                },
            );
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
