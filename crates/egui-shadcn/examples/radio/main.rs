#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;

use eframe::{App, Frame, egui};
use egui::RichText;
use egui_shadcn::radio::{GridLayout, RadioCardVariant, RadioGroup, RadioOption};
use egui_shadcn::{ControlSize, ControlVariant, Theme, button};

struct RadioDemo {
    theme: Theme,
    layout_value: String,
    notification_value: String,
    rhf_plan: String,
    rhf_error: Option<String>,
}

impl RadioDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            layout_value: "comfortable".to_string(),
            notification_value: "all".to_string(),
            rhf_plan: String::new(),
            rhf_error: None,
        }
    }

    fn layout_options() -> Vec<RadioOption<String>> {
        vec![
            RadioOption::new("default".to_string(), "Default").description("Standard density."),
            RadioOption::new("comfortable".to_string(), "Comfortable")
                .description("Cozy with extra padding."),
            RadioOption::new("compact".to_string(), "Compact").description("Fits more content."),
        ]
    }

    fn notification_options() -> Vec<RadioOption<String>> {
        vec![
            RadioOption::new("all".to_string(), "All new messages"),
            RadioOption::new("mentions".to_string(), "Direct messages and mentions"),
            RadioOption::new("none".to_string(), "Nothing"),
        ]
    }

    fn full_plans() -> Vec<RadioOption<String>> {
        vec![
            RadioOption::new("starter".to_string(), "Starter (100K tokens/month)")
                .description("For everyday use with basic features."),
            RadioOption::new("pro".to_string(), "Pro (1M tokens/month)")
                .description("For advanced AI usage with more features."),
            RadioOption::new("enterprise".to_string(), "Enterprise (Unlimited tokens)")
                .description("For large teams and heavy usage."),
        ]
    }
}

fn example_card(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.vertical(|ui| {
        ui.label(RichText::new(title).strong());
        ui.add_space(6.0);
        content(ui);
    });
}

fn muted(theme: &Theme, text: &str) -> RichText {
    RichText::new(text)
        .color(theme.palette.muted_foreground)
        .size(12.0)
}

fn error(theme: &Theme, text: &str) -> RichText {
    RichText::new(text)
        .color(theme.palette.destructive)
        .size(12.0)
}

impl App for RadioDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
            ui.heading("Radio Group");
            ui.add_space(12.0);

            ui.horizontal(|row| {
                row.spacing_mut().item_spacing.x = 24.0;

                let narrow = 260.0;
                let wide = 320.0;

                example_card(row, "Radio Group", |ui| {
                    ui.set_min_width(narrow);
                    ui.set_max_width(narrow);

                    ui.label(RichText::new("Display density").size(13.0));
                    ui.label(muted(&self.theme, "Choose how compact the UI is."));
                    RadioGroup::new(
                        "radio-group-layout",
                        &mut self.layout_value,
                        &RadioDemo::layout_options(),
                    )
                    .custom_spacing(10.0)
                    .card_variant(RadioCardVariant::Card)
                    .show(ui, &self.theme);
                });

                example_card(row, "Notifications", |ui| {
                    ui.set_min_width(narrow);
                    ui.set_max_width(narrow);

                    ui.label(RichText::new("Notify me about...").size(13.0));
                    RadioGroup::new(
                        "radio-group-notifications",
                        &mut self.notification_value,
                        &RadioDemo::notification_options(),
                    )
                    .custom_spacing(8.0)
                    .card_variant(RadioCardVariant::Button)
                    .show(ui, &self.theme);
                });

                example_card(row, "React Hook Form", |ui| {
                    ui.spacing_mut().item_spacing.y = 10.0;
                    ui.set_min_width(wide);
                    ui.set_max_width(wide);

                    ui.label(RichText::new("Plan").size(13.0));
                    ui.label(muted(
                        &self.theme,
                        "You can upgrade or downgrade your plan at any time.",
                    ));

                    RadioGroup::new(
                        "radio-rhf-plan",
                        &mut self.rhf_plan,
                        &RadioDemo::full_plans(),
                    )
                    .custom_spacing(8.0)
                    .card_variant(RadioCardVariant::Card)
                    .grid_layout(GridLayout::new(1).with_spacing(10.0))
                    .show(ui, &self.theme);

                    if let Some(err) = &self.rhf_error {
                        ui.label(error(&self.theme, err));
                    }

                    ui.add_space(8.0);
                    ui.horizontal(|row| {
                        row.spacing_mut().item_spacing.x = 8.0;

                        let save = button(
                            row,
                            &self.theme,
                            "Save",
                            ControlVariant::Primary,
                            ControlSize::Md,
                            true,
                        );
                        if save.clicked() {
                            self.rhf_error = match self.rhf_plan.as_str() {
                                "" => Some(
                                    "You must select a subscription plan to continue.".to_string(),
                                ),
                                _ => None,
                            };
                        }

                        let reset = button(
                            row,
                            &self.theme,
                            "Reset",
                            ControlVariant::Outline,
                            ControlSize::Md,
                            true,
                        );
                        if reset.clicked() {
                            self.rhf_plan.clear();
                            self.rhf_error = None;
                        }
                    });
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "RadioGroup example",
        options,
        Box::new(|_cc| Ok(Box::new(RadioDemo::new()))),
    )
}
