#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CentralPanel, RichText};
use egui_shadcn::{
    ControlSize, ControlVariant, FormControl, FormDescription, FormItem, FormLabel, FormMessage,
    FormState, InputOTPProps, Theme, ValidationMode, button, compose, input_otp, input_otp_group,
    input_otp_separator, input_otp_slot, input_otp_slot_last, min_length, required,
};
use regex::Regex;

struct InputOTPExample {
    theme: Theme,
    demo_value: String,
    pattern_value: String,
    separator_value: String,
    controlled_value: String,
    form_value: String,
    form_state: FormState,
    submit_message: Option<String>,
    digits_pattern: Regex,
}

impl InputOTPExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            demo_value: String::new(),
            pattern_value: String::new(),
            separator_value: String::new(),
            controlled_value: String::new(),
            form_value: String::new(),
            form_state: FormState::new(ValidationMode::OnSubmit),
            submit_message: None,
            digits_pattern: Regex::new(r"^[0-9]+$").unwrap(),
        }
    }
}

impl App for InputOTPExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Input OTP");
            ui.label("One-time password input with grouped slots.");
            ui.add_space(16.0);

            ui.spacing_mut().item_spacing.y = 20.0;

            render_section(
                ui,
                &self.theme,
                "Input OTP demo",
                "Grouped slots with separator.",
            );
            render_input_otp_demo(ui, &self.theme, &mut self.demo_value);

            render_section(
                ui,
                &self.theme,
                "Input OTP pattern",
                "Digits only with regex filtering.",
            );
            render_input_otp_pattern(
                ui,
                &self.theme,
                &mut self.pattern_value,
                &self.digits_pattern,
            );

            render_section(
                ui,
                &self.theme,
                "Input OTP separator",
                "Multiple groups with separators.",
            );
            render_input_otp_separator(ui, &self.theme, &mut self.separator_value);

            render_section(
                ui,
                &self.theme,
                "Input OTP controlled",
                "Controlled value with helper text.",
            );
            render_input_otp_controlled(ui, &self.theme, &mut self.controlled_value);

            render_section(
                ui,
                &self.theme,
                "Input OTP form",
                "Form validation on submit.",
            );
            render_input_otp_form(
                ui,
                &self.theme,
                &mut self.form_state,
                &mut self.form_value,
                &mut self.submit_message,
            );
        });
    }
}

fn render_section(ui: &mut egui::Ui, theme: &Theme, title: &str, description: &str) {
    ui.vertical(|ui| {
        ui.label(RichText::new(title).strong());
        ui.label(
            RichText::new(description)
                .color(theme.palette.muted_foreground)
                .size(12.0),
        );
        ui.add_space(8.0);
    });
}

fn render_input_otp_demo(ui: &mut egui::Ui, theme: &Theme, value: &mut String) {
    let props = InputOTPProps::new(6);
    let _ = input_otp(ui, theme, value, props, |ui, ctx| {
        input_otp_group(ui, |ui| {
            input_otp_slot(ui, ctx, 0);
            input_otp_slot(ui, ctx, 1);
            input_otp_slot_last(ui, ctx, 2);
        });
        input_otp_separator(ui, theme);
        input_otp_group(ui, |ui| {
            input_otp_slot(ui, ctx, 3);
            input_otp_slot(ui, ctx, 4);
            input_otp_slot_last(ui, ctx, 5);
        });
    });
}

fn render_input_otp_pattern(ui: &mut egui::Ui, theme: &Theme, value: &mut String, pattern: &Regex) {
    let props = InputOTPProps::new(6).pattern(pattern);
    let _ = input_otp(ui, theme, value, props, |ui, ctx| {
        input_otp_group(ui, |ui| {
            for index in 0..5 {
                input_otp_slot(ui, ctx, index);
            }
            input_otp_slot_last(ui, ctx, 5);
        });
    });
}

fn render_input_otp_separator(ui: &mut egui::Ui, theme: &Theme, value: &mut String) {
    let props = InputOTPProps::new(6);
    let _ = input_otp(ui, theme, value, props, |ui, ctx| {
        input_otp_group(ui, |ui| {
            input_otp_slot(ui, ctx, 0);
            input_otp_slot_last(ui, ctx, 1);
        });
        input_otp_separator(ui, theme);
        input_otp_group(ui, |ui| {
            input_otp_slot(ui, ctx, 2);
            input_otp_slot_last(ui, ctx, 3);
        });
        input_otp_separator(ui, theme);
        input_otp_group(ui, |ui| {
            input_otp_slot(ui, ctx, 4);
            input_otp_slot_last(ui, ctx, 5);
        });
    });
}

fn render_input_otp_controlled(ui: &mut egui::Ui, theme: &Theme, value: &mut String) {
    ui.vertical(|ui| {
        let props = InputOTPProps::new(6);
        let _ = input_otp(ui, theme, value, props, |ui, ctx| {
            input_otp_group(ui, |ui| {
                for index in 0..5 {
                    input_otp_slot(ui, ctx, index);
                }
                input_otp_slot_last(ui, ctx, 5);
            });
        });

        let text = if value.is_empty() {
            "Enter your one-time password.".to_string()
        } else {
            format!("You entered: {}", value)
        };
        let color = if value.is_empty() {
            theme.palette.muted_foreground
        } else {
            theme.palette.foreground
        };
        ui.label(RichText::new(text).size(12.0).color(color));
    });
}

fn render_input_otp_form(
    ui: &mut egui::Ui,
    theme: &Theme,
    form: &mut FormState,
    value: &mut String,
    submit_message: &mut Option<String>,
) {
    form.field(
        "pin",
        compose(vec![
            required("One-time password is required."),
            min_length(6, "Your one-time password must be 6 characters."),
        ]),
    );

    ui.set_max_width(360.0);
    ui.spacing_mut().item_spacing.y = 12.0;

    let error = form.error("pin").map(|err| err.to_string());
    FormItem::new(ui.make_persistent_id("input-otp-form-pin")).show(ui, |ui, ctx| {
        FormLabel::new("One-Time Password")
            .required(true)
            .error(error.is_some())
            .show(ui, theme, ctx);
        FormControl::new().show(ui, ctx, |ui, id| {
            ui.push_id(id, |ui| {
                let props = InputOTPProps::new(6);
                let _ = input_otp(ui, theme, value, props, |ui, ctx| {
                    input_otp_group(ui, |ui| {
                        for index in 0..5 {
                            input_otp_slot(ui, ctx, index);
                        }
                        input_otp_slot_last(ui, ctx, 5);
                    });
                });
            })
            .inner
        });
        form.set_text("pin", value.clone());
        FormDescription::new("Please enter the one-time password sent to your phone.")
            .show(ui, theme);
        FormMessage::from_error(error.as_deref()).show(ui, theme);
    });

    let submit = button(
        ui,
        theme,
        "Submit",
        ControlVariant::Primary,
        ControlSize::Md,
        true,
    );
    if submit.clicked() {
        if form.validate() {
            *submit_message = Some(value.clone());
        } else {
            *submit_message = None;
        }
    }

    if let Some(message) = submit_message.as_deref() {
        ui.label(
            RichText::new(format!("Submitted: {}", message))
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Input OTP example",
        options,
        Box::new(|_cc| Ok(Box::new(InputOTPExample::new()))),
    )
}
