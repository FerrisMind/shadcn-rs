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
    ControlSize, ControlVariant, SwitchProps, SwitchSize, SwitchVariant, Theme, button, switch,
    switch_with_props,
};

struct SwitchDemo {
    theme: Theme,
    // Demo
    airplane_mode: bool,
    email_notifications: bool,
    // Form
    marketing_emails: bool,
    security_emails: bool,
    // Variants (3 variants × 2 high_contrast × 4 states: off/on/disabled-off/disabled-on)
    variant_states: Vec<bool>,
    // Sizes (4 sizes)
    size_states: [bool; 3],
    // Colors (6)
    color_states: [(bool, bool); 6], // (off, on) pairs
    // Form submit
    form_mfa: bool,
    form_submitted: bool,
}

const SWITCH_SIZES: [SwitchSize; 3] = [SwitchSize::Size1, SwitchSize::Size2, SwitchSize::Size3];

const SIZE_NAMES: [&str; 3] = ["Size 1", "Size 2", "Size 3"];

const VARIANT_NAMES: [&str; 3] = ["Classic", "Surface", "Soft"];
const VARIANTS: [SwitchVariant; 3] = [
    SwitchVariant::Classic,
    SwitchVariant::Surface,
    SwitchVariant::Soft,
];

const COLOR_NAMES: [&str; 6] = ["Blue", "Green", "Amber", "Red", "Purple", "Gray"];

impl SwitchDemo {
    fn new() -> Self {
        // 3 variants × 2 high_contrast × 4 states = 24
        let mut variant_states = Vec::new();
        for _variant in &VARIANTS {
            for _high_contrast in [false, true] {
                variant_states.extend([false, true, false, true]);
            }
        }

        Self {
            theme: Theme::default(),
            airplane_mode: false,
            email_notifications: true,
            marketing_emails: false,
            security_emails: true,
            variant_states,
            size_states: [false, true, false],
            color_states: [
                (false, true),
                (false, true),
                (false, true),
                (false, true),
                (false, true),
                (false, true),
            ],
            form_mfa: false,
            form_submitted: false,
        }
    }
}

fn section_title(ui: &mut egui::Ui, title: &str) {
    ui.label(RichText::new(title).size(16.0).strong());
    ui.add_space(4.0);
}

impl App for SwitchDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_max_width(700.0);
                ui.spacing_mut().item_spacing.y = 24.0;

                ui.heading("Switch");
                ui.add_space(4.0);

                // Demo
                section_title(ui, "Demo");
                ui.vertical(|col| {
                    col.spacing_mut().item_spacing.y = 12.0;
                    col.horizontal(|row| {
                        row.spacing_mut().item_spacing.x = 8.0;
                        let _ = switch(
                            row,
                            &self.theme,
                            &mut self.airplane_mode,
                            "Airplane Mode",
                            ControlVariant::Primary,
                            ControlSize::Md,
                            true,
                        );
                    });
                    col.horizontal(|row| {
                        row.spacing_mut().item_spacing.x = 8.0;
                        let _ = switch(
                            row,
                            &self.theme,
                            &mut self.email_notifications,
                            "",
                            ControlVariant::Primary,
                            ControlSize::Md,
                            true,
                        );
                        row.vertical(|text| {
                            text.spacing_mut().item_spacing.y = 4.0;
                            text.label(RichText::new("Email Notifications").size(14.0).strong());
                            text.label(
                                RichText::new(
                                    "Receive email updates about your account activity.",
                                )
                                .color(self.theme.palette.muted_foreground)
                                .size(12.0),
                            );
                        });
                    });
                });

                // Form
                section_title(ui, "Switch Form");
                ui.vertical(|form| {
                    form.spacing_mut().item_spacing.y = 12.0;
                    form.set_max_width(420.0);
                    form.label(
                        RichText::new("Email Notifications")
                            .text_style(egui::TextStyle::Button)
                            .size(16.0)
                            .strong(),
                    );

                    for (id_suffix, title, description, value, enabled) in [
                        (
                            "marketing",
                            "Marketing emails",
                            "Receive emails about new products, features, and more.",
                            &mut self.marketing_emails,
                            true,
                        ),
                        (
                            "security",
                            "Security emails",
                            "Receive emails about your account security.",
                            &mut self.security_emails,
                            false,
                        ),
                    ] {
                        egui::Frame::NONE
                            .fill(self.theme.palette.background)
                            .stroke(egui::Stroke::new(1.0_f32, self.theme.palette.border))
                            .corner_radius(egui::CornerRadius::same(8))
                            .inner_margin(egui::Margin::symmetric(12, 10))
                            .show(form, |item_ui| {
                                item_ui.push_id(id_suffix, |item_ui| {
                                    item_ui.horizontal(|row| {
                                        row.set_width(row.available_width());
                                        row.vertical(|text| {
                                            text.spacing_mut().item_spacing.y = 4.0;
                                            text.label(
                                                RichText::new(title).size(14.0).strong(),
                                            );
                                            text.label(
                                                RichText::new(description)
                                                    .color(self.theme.palette.muted_foreground)
                                                    .size(12.0),
                                            );
                                        });
                                        row.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |right| {
                                                let _ = switch(
                                                    right,
                                                    &self.theme,
                                                    value,
                                                    "",
                                                    ControlVariant::Primary,
                                                    ControlSize::Sm,
                                                    enabled,
                                                );
                                            },
                                        );
                                    });
                                });
                            });
                    }

                    let _ = button(
                        form,
                        &self.theme,
                        "Submit",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        true,
                    );
                });

                // Variants
                section_title(ui, "Variants");
                egui::Grid::new("switch_variants_grid")
                    .num_columns(5)
                    .spacing(egui::vec2(16.0, 8.0))
                    .show(ui, |grid| {
                        // Header
                        let muted = self.theme.palette.muted_foreground;
                        grid.label(RichText::new("Variant").size(12.0).color(muted));
                        grid.label(RichText::new("Off").size(12.0).color(muted));
                        grid.label(RichText::new("On").size(12.0).color(muted));
                        grid.label(RichText::new("Disabled Off").size(12.0).color(muted));
                        grid.label(RichText::new("Disabled On").size(12.0).color(muted));
                        grid.end_row();

                        let mut idx = 0;
                        for (variant, name) in VARIANTS.iter().zip(VARIANT_NAMES.iter()) {
                            for high_contrast in [false, true] {
                                let hc_label = if high_contrast {
                                    format!("{name} + HC")
                                } else {
                                    name.to_string()
                                };
                                grid.label(
                                    RichText::new(hc_label).size(12.0).color(muted),
                                );

                                let id = grid.make_persistent_id(format!("sw-v-{idx}"));
                                let _ = switch_with_props(
                                    grid,
                                    &self.theme,
                                    SwitchProps::new(
                                        id,
                                        &mut self.variant_states[idx],
                                        "",
                                    )
                                    .style(*variant)
                                    .high_contrast(high_contrast)
                                    .size(SwitchSize::Size2),
                                );
                                idx += 1;

                                let id = grid.make_persistent_id(format!("sw-v-{idx}"));
                                let _ = switch_with_props(
                                    grid,
                                    &self.theme,
                                    SwitchProps::new(
                                        id,
                                        &mut self.variant_states[idx],
                                        "",
                                    )
                                    .style(*variant)
                                    .high_contrast(high_contrast)
                                    .size(SwitchSize::Size2),
                                );
                                idx += 1;

                                let id = grid.make_persistent_id(format!("sw-v-{idx}"));
                                let _ = switch_with_props(
                                    grid,
                                    &self.theme,
                                    SwitchProps::new(
                                        id,
                                        &mut self.variant_states[idx],
                                        "",
                                    )
                                    .style(*variant)
                                    .high_contrast(high_contrast)
                                    .size(SwitchSize::Size2)
                                    .disabled(true),
                                );
                                idx += 1;

                                let id = grid.make_persistent_id(format!("sw-v-{idx}"));
                                let _ = switch_with_props(
                                    grid,
                                    &self.theme,
                                    SwitchProps::new(
                                        id,
                                        &mut self.variant_states[idx],
                                        "",
                                    )
                                    .style(*variant)
                                    .high_contrast(high_contrast)
                                    .size(SwitchSize::Size2)
                                    .disabled(true),
                                );
                                idx += 1;

                                grid.end_row();
                            }
                        }
                    });

                // Sizes
                section_title(ui, "Sizes");
                ui.vertical(|col| {
                    col.spacing_mut().item_spacing.y = 12.0;
                    for (s_idx, (size, name)) in
                        SWITCH_SIZES.iter().zip(SIZE_NAMES.iter()).enumerate()
                    {
                        col.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 12.0;
                            row.label(
                                RichText::new(*name)
                                    .size(12.0)
                                    .color(self.theme.palette.muted_foreground),
                            );
                            let id = row.make_persistent_id(format!("sw-size-{s_idx}"));
                            let _ = switch_with_props(
                                row,
                                &self.theme,
                                SwitchProps::new(id, &mut self.size_states[s_idx], "")
                                    .size(*size),
                            );
                        });
                    }
                });

                // Colors
                section_title(ui, "Colors");
                {
                    let colors: [egui::Color32; 6] = [
                        egui::Color32::from_rgb(37, 99, 235),   // Blue
                        egui::Color32::from_rgb(34, 197, 94),   // Green
                        egui::Color32::from_rgb(245, 158, 11),  // Amber
                        egui::Color32::from_rgb(239, 68, 68),   // Red
                        egui::Color32::from_rgb(168, 85, 247),  // Purple
                        egui::Color32::from_rgb(115, 115, 115), // Gray
                    ];
                    egui::Grid::new("switch_colors_grid")
                        .num_columns(3)
                        .spacing(egui::vec2(16.0, 8.0))
                        .show(ui, |grid| {
                            let muted = self.theme.palette.muted_foreground;
                            grid.label(RichText::new("Color").size(12.0).color(muted));
                            grid.label(RichText::new("Off").size(12.0).color(muted));
                            grid.label(RichText::new("On").size(12.0).color(muted));
                            grid.end_row();

                            for (c_idx, (color, name)) in
                                colors.iter().zip(COLOR_NAMES.iter()).enumerate()
                            {
                                grid.label(
                                    RichText::new(*name)
                                        .size(12.0)
                                        .color(muted),
                                );
                                let id = grid.make_persistent_id(format!("sw-c-{c_idx}-off"));
                                let _ = switch_with_props(
                                    grid,
                                    &self.theme,
                                    SwitchProps::new(
                                        id,
                                        &mut self.color_states[c_idx].0,
                                        "",
                                    )
                                    .size(SwitchSize::Size2)
                                    .accent(*color),
                                );
                                let id = grid.make_persistent_id(format!("sw-c-{c_idx}-on"));
                                let _ = switch_with_props(
                                    grid,
                                    &self.theme,
                                    SwitchProps::new(
                                        id,
                                        &mut self.color_states[c_idx].1,
                                        "",
                                    )
                                    .size(SwitchSize::Size2)
                                    .accent(*color),
                                );
                                grid.end_row();
                            }
                        });
                }

                // Form with validation
                section_title(ui, "Form (React Hook Form)");
                ui.vertical(|form| {
                    form.spacing_mut().item_spacing.y = 12.0;
                    form.set_max_width(420.0);

                    form.label(
                        RichText::new("Security Settings")
                            .size(16.0)
                            .strong(),
                    );
                    form.label(
                        RichText::new("Manage your account security preferences.")
                            .color(self.theme.palette.muted_foreground)
                            .size(12.0),
                    );

                    egui::Frame::NONE
                        .fill(self.theme.palette.background)
                        .stroke(egui::Stroke::new(1.0_f32, self.theme.palette.border))
                        .corner_radius(egui::CornerRadius::same(8))
                        .inner_margin(egui::Margin::symmetric(12, 10))
                        .show(form, |item_ui| {
                            item_ui.horizontal(|row| {
                                row.set_width(row.available_width());
                                row.vertical(|text| {
                                    text.spacing_mut().item_spacing.y = 4.0;
                                    text.label(
                                        RichText::new("Multi-factor authentication")
                                            .size(14.0)
                                            .strong(),
                                    );
                                    text.label(
                                        RichText::new(
                                            "Enable multi-factor authentication to secure your account.",
                                        )
                                        .color(self.theme.palette.muted_foreground)
                                        .size(12.0),
                                    );
                                });
                                row.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |right| {
                                        let _ = switch(
                                            right,
                                            &self.theme,
                                            &mut self.form_mfa,
                                            "",
                                            ControlVariant::Primary,
                                            ControlSize::Md,
                                            true,
                                        );
                                    },
                                );
                            });
                        });

                    form.horizontal(|row| {
                        row.spacing_mut().item_spacing.x = 8.0;
                        let _ = button(
                            row,
                            &self.theme,
                            "Reset",
                            ControlVariant::Secondary,
                            ControlSize::Md,
                            true,
                        );
                        let submit = button(
                            row,
                            &self.theme,
                            "Save",
                            ControlVariant::Primary,
                            ControlSize::Md,
                            true,
                        );
                        if submit.clicked() {
                            self.form_submitted = true;
                        }
                    });

                    if self.form_submitted {
                        let msg = format!(
                            "MFA: {}",
                            if self.form_mfa { "enabled" } else { "disabled" }
                        );
                        form.label(
                            RichText::new(msg)
                                .size(12.0)
                                .color(self.theme.palette.muted_foreground),
                        );
                    }
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Switch example",
        options,
        Box::new(|_cc| Ok(Box::new(SwitchDemo::new()))),
    )
}
