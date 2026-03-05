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
use egui_shadcn::radio::{RadioCardVariant, RadioGroup, RadioOption};
use egui_shadcn::{ControlSize, ControlVariant, RadioDirection, Theme, button};

struct RadioDemo {
    theme: Theme,
    // Demo
    layout_value: String,
    // Notifications
    notification_value: String,
    // Horizontal
    horizontal_value: String,
    // Disabled
    disabled_value: String,
    // Sizes
    size_sm_value: String,
    size_md_value: String,
    size_lg_value: String,
    // Colors
    color_blue_value: String,
    color_green_value: String,
    color_amber_value: String,
    // High Contrast
    hc_value: String,
    // Form
    rhf_plan: String,
    rhf_error: Option<String>,
}

impl RadioDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            layout_value: "comfortable".to_string(),
            notification_value: "all".to_string(),
            horizontal_value: String::new(),
            disabled_value: "starter".to_string(),
            size_sm_value: String::new(),
            size_md_value: String::new(),
            size_lg_value: String::new(),
            color_blue_value: String::new(),
            color_green_value: String::new(),
            color_amber_value: String::new(),
            hc_value: String::new(),
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

    fn plan_options() -> Vec<RadioOption<String>> {
        vec![
            RadioOption::new("starter".to_string(), "Starter"),
            RadioOption::new("pro".to_string(), "Pro"),
            RadioOption::new("team".to_string(), "Team"),
        ]
    }

    fn plan_options_with_desc() -> Vec<RadioOption<String>> {
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

fn section_title(ui: &mut egui::Ui, title: &str) {
    ui.add_space(8.0);
    ui.label(RichText::new(title).size(15.0).strong());
    ui.add_space(4.0);
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
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
            ui.heading("Radio Group");
            ui.add_space(4.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Demo — card variant with descriptions
                section_title(ui, "Demo");
                ui.horizontal(|row| {
                    row.spacing_mut().item_spacing.x = 24.0;

                    let narrow = 260.0;
                    row.vertical(|col| {
                        col.set_min_width(narrow);
                        col.set_max_width(narrow);
                        col.label(RichText::new("Display density").size(13.0));
                        col.label(muted(&self.theme, "Choose how compact the UI is."));
                        RadioGroup::new(
                            "radio-group-layout",
                            &mut self.layout_value,
                            &RadioDemo::layout_options(),
                        )
                        .custom_spacing(10.0)
                        .card_variant(RadioCardVariant::Card)
                        .show(col, &self.theme);
                    });
                });

                // Notifications — simple list (button variant)
                section_title(ui, "Notifications");
                ui.vertical(|col| {
                    col.set_max_width(300.0);
                    col.label(RichText::new("Notify me about...").size(13.0));
                    RadioGroup::new(
                        "radio-group-notifications",
                        &mut self.notification_value,
                        &RadioDemo::notification_options(),
                    )
                    .custom_spacing(8.0)
                    .card_variant(RadioCardVariant::Button)
                    .show(col, &self.theme);
                });

                // Horizontal
                section_title(ui, "Horizontal");
                RadioGroup::new(
                    "radio-horizontal",
                    &mut self.horizontal_value,
                    &RadioDemo::plan_options(),
                )
                .direction(RadioDirection::Horizontal)
                .custom_spacing(8.0)
                .show(ui, &self.theme);

                // Disabled
                section_title(ui, "Disabled");
                RadioGroup::new(
                    "radio-disabled",
                    &mut self.disabled_value,
                    &RadioDemo::plan_options(),
                )
                .disabled(true)
                .custom_spacing(8.0)
                .show(ui, &self.theme);

                // Sizes
                section_title(ui, "Sizes");
                ui.vertical(|col| {
                    col.spacing_mut().item_spacing.y = 12.0;

                    col.horizontal(|row| {
                        row.spacing_mut().item_spacing.x = 16.0;
                        row.label(muted(&self.theme, "Sm"));
                        RadioGroup::new(
                            "radio-size-sm",
                            &mut self.size_sm_value,
                            &RadioDemo::plan_options(),
                        )
                        .size(ControlSize::Sm)
                        .direction(RadioDirection::Horizontal)
                        .custom_spacing(8.0)
                        .show(row, &self.theme);
                    });

                    col.horizontal(|row| {
                        row.spacing_mut().item_spacing.x = 16.0;
                        row.label(muted(&self.theme, "Md"));
                        RadioGroup::new(
                            "radio-size-md",
                            &mut self.size_md_value,
                            &RadioDemo::plan_options(),
                        )
                        .size(ControlSize::Md)
                        .direction(RadioDirection::Horizontal)
                        .custom_spacing(8.0)
                        .show(row, &self.theme);
                    });

                    col.horizontal(|row| {
                        row.spacing_mut().item_spacing.x = 16.0;
                        row.label(muted(&self.theme, "Lg"));
                        RadioGroup::new(
                            "radio-size-lg",
                            &mut self.size_lg_value,
                            &RadioDemo::plan_options(),
                        )
                        .size(ControlSize::Lg)
                        .direction(RadioDirection::Horizontal)
                        .custom_spacing(8.0)
                        .show(row, &self.theme);
                    });
                });

                // Colors
                section_title(ui, "Colors");
                ui.vertical(|col| {
                    col.spacing_mut().item_spacing.y = 12.0;

                    col.horizontal(|row| {
                        row.spacing_mut().item_spacing.x = 16.0;
                        row.label(muted(&self.theme, "Blue"));
                        RadioGroup::new(
                            "radio-color-blue",
                            &mut self.color_blue_value,
                            &RadioDemo::notification_options(),
                        )
                        .accent_color(egui::Color32::from_rgb(37, 99, 235))
                        .direction(RadioDirection::Horizontal)
                        .custom_spacing(8.0)
                        .show(row, &self.theme);
                    });

                    col.horizontal(|row| {
                        row.spacing_mut().item_spacing.x = 16.0;
                        row.label(muted(&self.theme, "Green"));
                        RadioGroup::new(
                            "radio-color-green",
                            &mut self.color_green_value,
                            &RadioDemo::notification_options(),
                        )
                        .accent_color(egui::Color32::from_rgb(34, 197, 94))
                        .direction(RadioDirection::Horizontal)
                        .custom_spacing(8.0)
                        .show(row, &self.theme);
                    });

                    col.horizontal(|row| {
                        row.spacing_mut().item_spacing.x = 16.0;
                        row.label(muted(&self.theme, "Amber"));
                        RadioGroup::new(
                            "radio-color-amber",
                            &mut self.color_amber_value,
                            &RadioDemo::notification_options(),
                        )
                        .accent_color(egui::Color32::from_rgb(245, 158, 11))
                        .direction(RadioDirection::Horizontal)
                        .custom_spacing(8.0)
                        .show(row, &self.theme);
                    });
                });

                // High Contrast
                section_title(ui, "High Contrast");
                RadioGroup::new("radio-hc", &mut self.hc_value, &RadioDemo::plan_options())
                    .high_contrast(true)
                    .custom_spacing(8.0)
                    .show(ui, &self.theme);

                // Form with validation
                section_title(ui, "Form");
                ui.vertical(|form| {
                    form.spacing_mut().item_spacing.y = 10.0;
                    form.set_max_width(320.0);

                    form.label(RichText::new("Plan").size(13.0));
                    form.label(muted(
                        &self.theme,
                        "You can upgrade or downgrade your plan at any time.",
                    ));

                    RadioGroup::new(
                        "radio-rhf-plan",
                        &mut self.rhf_plan,
                        &RadioDemo::plan_options_with_desc(),
                    )
                    .custom_spacing(8.0)
                    .card_variant(RadioCardVariant::Card)
                    .show(form, &self.theme);

                    if let Some(err) = &self.rhf_error {
                        form.label(error(&self.theme, err));
                    }

                    form.add_space(8.0);
                    form.horizontal(|row| {
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
