use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::{ControlSize, Label, LabelVariant, Theme};
use egui_shadcn::{Input, InputSize};
use log::{error, info};

struct LabelDemo {
    theme: Theme,
    email: String,
    password: String,
    newsletter: bool,
}

impl LabelDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            email: String::new(),
            password: String::new(),
            newsletter: false,
        }
    }
}

impl App for LabelDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let mut style = ctx.style().as_ref().clone();
        let bg = egui::Color32::from_rgb(12, 12, 12);
        style.visuals.window_fill = bg;
        style.visuals.panel_fill = bg;
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.heading("Label Component - Comprehensive Demo");
                    ui.add_space(12.0);

                    ui.label(
                        egui::RichText::new("Section 1: Form Labels with Input Association")
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(8.0);

                    let email_id = ui.make_persistent_id("email_input");
                    Label::new("Email address")
                        .for_id(email_id)
                        .size(ControlSize::Md)
                        .required(true)
                        .description("We will never share your email with third parties.")
                        .show(ui, &self.theme);
                    Input::new(email_id)
                        .placeholder("you@example.com")
                        .size(InputSize::Size2)
                        .show(ui, &self.theme, &mut self.email);

                    ui.add_space(12.0);

                    let password_id = ui.make_persistent_id("password_input");
                    Label::new("Password")
                        .for_id(password_id)
                        .size(ControlSize::Md)
                        .required(true)
                        .description("Use at least 8 characters with mixed case and numbers.")
                        .show(ui, &self.theme);
                    Input::new(password_id)
                        .placeholder("••••••••")
                        .size(InputSize::Size2)
                        .show(ui, &self.theme, &mut self.password);

                    ui.add_space(16.0);

                    ui.label(
                        egui::RichText::new("Section 2: Size Variants").color(egui::Color32::WHITE),
                    );
                    ui.add_space(8.0);

                    Label::new("Small label")
                        .size(ControlSize::Sm)
                        .description("Smaller font size (Sm)")
                        .show(ui, &self.theme);

                    ui.add_space(8.0);

                    Label::new("Medium label (default)")
                        .size(ControlSize::Md)
                        .description("Standard font size (Md)")
                        .show(ui, &self.theme);

                    ui.add_space(8.0);

                    Label::new("Large label")
                        .size(ControlSize::Lg)
                        .description("Larger font size (Lg)")
                        .show(ui, &self.theme);

                    ui.add_space(16.0);

                    ui.label(
                        egui::RichText::new("Section 3: Semantic Variants")
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(8.0);

                    Label::new("Default variant")
                        .variant(LabelVariant::Default)
                        .size(ControlSize::Sm)
                        .description("Primary text color - main label emphasis")
                        .show(ui, &self.theme);

                    ui.add_space(8.0);

                    Label::new("Secondary variant")
                        .variant(LabelVariant::Secondary)
                        .size(ControlSize::Sm)
                        .description("Secondary text color - medium emphasis")
                        .show(ui, &self.theme);

                    ui.add_space(8.0);

                    Label::new("Muted variant")
                        .variant(LabelVariant::Muted)
                        .size(ControlSize::Sm)
                        .description("Muted text color - low emphasis for hints and helper text")
                        .show(ui, &self.theme);

                    ui.add_space(8.0);

                    Label::new("Destructive variant")
                        .variant(LabelVariant::Destructive)
                        .size(ControlSize::Sm)
                        .description(
                            "Destructive color - for errors, warnings, or dangerous actions",
                        )
                        .show(ui, &self.theme);

                    ui.add_space(16.0);

                    ui.label(
                        egui::RichText::new("Section 4: Label States and Indicators")
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(8.0);

                    Label::new("Required field")
                        .variant(LabelVariant::Default)
                        .required(true)
                        .description("Shows an asterisk (*) indicator in red")
                        .show(ui, &self.theme);

                    ui.add_space(8.0);

                    Label::new("Disabled label")
                        .variant(LabelVariant::Secondary)
                        .disabled(true)
                        .description("Grayed out and non-interactive")
                        .show(ui, &self.theme);

                    ui.add_space(8.0);

                    Label::new("Required and destructive")
                        .variant(LabelVariant::Destructive)
                        .required(true)
                        .description("Combines destructive color with required indicator")
                        .show(ui, &self.theme);

                    ui.add_space(16.0);

                    ui.label(
                        egui::RichText::new("Section 5: Combined Configurations")
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(8.0);

                    Label::new("Newsletter subscription")
                        .variant(LabelVariant::Secondary)
                        .size(ControlSize::Sm)
                        .description("Optional: Subscribe to our newsletter for updates")
                        .show(ui, &self.theme);

                    ui.add_space(4.0);
                    ui.checkbox(
                        &mut self.newsletter,
                        "Send me newsletters and product updates",
                    );

                    ui.add_space(16.0);

                    Label::new("Account deletion")
                .variant(LabelVariant::Destructive)
                .size(ControlSize::Md)
                .required(true)
                .description(
                    "This action is irreversible. All your data will be permanently deleted. \
                    Please ensure you have exported any important information first."
                )
                .show(ui, &self.theme);

                    ui.add_space(4.0);
                    if ui.button("Delete My Account").clicked() {
                        info!("Account deletion initiated");
                    }
                });
        });
    }
}

fn main() {
    env_logger::init();
    info!("Starting comprehensive label example");

    let native_options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "Label Component Demo",
        native_options,
        Box::new(|_cc| Ok(Box::new(LabelDemo::new()))),
    ) {
        error!("Failed to run example: {err}");
    }
}
