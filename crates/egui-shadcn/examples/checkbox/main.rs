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
    CheckboxCycle, CheckboxProps, CheckboxSize, CheckboxState, CheckboxVariant, ControlSize,
    ControlVariant, Label, Theme, button, checkbox, checkbox_with_props,
};

struct CheckboxDemo {
    theme: Theme,
    // Demo
    terms: bool,
    // With text
    terms_with_text: bool,
    // Disabled
    disabled_checked: bool,
    disabled_unchecked: bool,
    // Indeterminate
    indeterminate_state: CheckboxState,
    // Card
    card_checked: bool,
    // Variants (3 variants × 3 states: default, high_contrast, disabled)
    variant_states: [[CheckboxState; 3]; 3],
    // Sizes (3 sizes)
    size_states: [CheckboxState; 3],
    // Colors (6 colors)
    color_states: [CheckboxState; 6],
    // Form
    form_terms: bool,
    form_submitted: bool,
}

impl CheckboxDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            terms: false,
            terms_with_text: true,
            disabled_checked: true,
            disabled_unchecked: false,
            indeterminate_state: CheckboxState::Indeterminate,
            card_checked: true,
            variant_states: [[
                CheckboxState::Unchecked,
                CheckboxState::Checked,
                CheckboxState::Unchecked,
            ]; 3],
            size_states: [
                CheckboxState::Unchecked,
                CheckboxState::Checked,
                CheckboxState::Unchecked,
            ],
            color_states: [CheckboxState::Checked; 6],
            form_terms: false,
            form_submitted: false,
        }
    }
}

fn section_title(ui: &mut egui::Ui, title: &str) {
    ui.label(RichText::new(title).size(16.0).strong());
    ui.add_space(4.0);
}

const VARIANT_NAMES: [&str; 3] = ["Surface", "Classic", "Soft"];
const VARIANTS: [CheckboxVariant; 3] = [
    CheckboxVariant::Surface,
    CheckboxVariant::Classic,
    CheckboxVariant::Soft,
];

const SIZE_NAMES: [&str; 3] = ["Size 1", "Size 2", "Size 3"];
const SIZES: [CheckboxSize; 3] = [
    CheckboxSize::Size1,
    CheckboxSize::Size2,
    CheckboxSize::Size3,
];

const COLOR_NAMES: [&str; 6] = ["Blue", "Green", "Amber", "Red", "Purple", "Gray"];

impl App for CheckboxDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_max_width(700.0);
                ui.spacing_mut().item_spacing.y = 24.0;

                ui.heading("Checkbox");
                ui.add_space(4.0);

                // Demo
                section_title(ui, "Demo");
                ui.horizontal(|row| {
                    row.spacing_mut().item_spacing.x = 12.0;
                    let _ = checkbox(
                        row,
                        &self.theme,
                        &mut self.terms,
                        "Accept terms and conditions",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        true,
                    );
                });

                // With text
                section_title(ui, "With Text");
                ui.horizontal(|row| {
                    row.spacing_mut().item_spacing.x = 12.0;
                    let _ = checkbox(
                        row,
                        &self.theme,
                        &mut self.terms_with_text,
                        "",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        true,
                    );
                    row.vertical(|col| {
                        col.spacing_mut().item_spacing.y = 6.0;
                        Label::new("Accept terms and conditions")
                            .size(ControlSize::Sm)
                            .show(col, &self.theme);
                        col.label(
                            RichText::new(
                                "By clicking this checkbox, you agree to the terms and conditions.",
                            )
                            .color(self.theme.palette.muted_foreground)
                            .size(12.0),
                        );
                    });
                });

                // Disabled
                section_title(ui, "Disabled");
                ui.horizontal(|row| {
                    row.spacing_mut().item_spacing.x = 24.0;
                    let _ = checkbox(
                        row,
                        &self.theme,
                        &mut self.disabled_unchecked,
                        "Unchecked disabled",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        false,
                    );
                    let _ = checkbox(
                        row,
                        &self.theme,
                        &mut self.disabled_checked,
                        "Checked disabled",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        false,
                    );
                });

                // Indeterminate
                section_title(ui, "Indeterminate");
                ui.horizontal(|row| {
                    row.spacing_mut().item_spacing.x = 12.0;
                    let _ = checkbox_with_props(
                        row,
                        &self.theme,
                        &mut self.indeterminate_state,
                        "Select all items",
                        CheckboxProps::default().cycle(CheckboxCycle::TriState),
                    );
                });

                // Card
                section_title(ui, "Card");
                {
                    let checked_border = egui::Color32::from_rgb(37, 99, 235);
                    let checked_bg = egui::Color32::from_rgba_unmultiplied(37, 99, 235, 20);
                    let border_color = if self.card_checked {
                        checked_border
                    } else {
                        egui::Color32::from_gray(80)
                    };
                    let fill_color = if self.card_checked {
                        checked_bg
                    } else {
                        egui::Color32::TRANSPARENT
                    };

                    let mut checkbox_clicked = false;
                    let frame_response = egui::Frame::NONE
                        .fill(fill_color)
                        .stroke(egui::Stroke::new(1.0_f32, border_color))
                        .corner_radius(egui::CornerRadius::same(8))
                        .inner_margin(egui::Margin::same(12))
                        .show(ui, |frame_ui| {
                            frame_ui.horizontal(|row| {
                                row.spacing_mut().item_spacing.x = 12.0;
                                let resp = checkbox(
                                    row,
                                    &self.theme,
                                    &mut self.card_checked,
                                    "",
                                    ControlVariant::Primary,
                                    ControlSize::Md,
                                    true,
                                );
                                checkbox_clicked = resp.clicked();
                                row.vertical(|col| {
                                    col.spacing_mut().item_spacing.y = 6.0;
                                    col.label(
                                        RichText::new("Enable notifications").size(14.0).strong(),
                                    );
                                    col.label(
                                        RichText::new(
                                            "You can enable or disable notifications at any time.",
                                        )
                                        .color(self.theme.palette.muted_foreground)
                                        .size(12.0),
                                    );
                                });
                            });
                        });

                    if frame_response.response.clicked() && !checkbox_clicked {
                        self.card_checked = !self.card_checked;
                    }
                }

                // Variants
                section_title(ui, "Variants");
                egui::Grid::new("checkbox_variants_grid")
                    .num_columns(4)
                    .spacing(egui::vec2(24.0, 12.0))
                    .show(ui, |grid| {
                        // Header
                        grid.label(
                            RichText::new("Variant")
                                .size(12.0)
                                .color(self.theme.palette.muted_foreground),
                        );
                        grid.label(
                            RichText::new("Default")
                                .size(12.0)
                                .color(self.theme.palette.muted_foreground),
                        );
                        grid.label(
                            RichText::new("High Contrast")
                                .size(12.0)
                                .color(self.theme.palette.muted_foreground),
                        );
                        grid.label(
                            RichText::new("Disabled")
                                .size(12.0)
                                .color(self.theme.palette.muted_foreground),
                        );
                        grid.end_row();

                        for (v_idx, (variant, name)) in
                            VARIANTS.iter().zip(VARIANT_NAMES.iter()).enumerate()
                        {
                            grid.label(
                                RichText::new(*name)
                                    .size(12.0)
                                    .color(self.theme.palette.muted_foreground),
                            );

                            // Default
                            let _ = checkbox_with_props(
                                grid,
                                &self.theme,
                                &mut self.variant_states[v_idx][0],
                                "",
                                CheckboxProps::default().variant(*variant),
                            );

                            // High contrast
                            let _ = checkbox_with_props(
                                grid,
                                &self.theme,
                                &mut self.variant_states[v_idx][1],
                                "",
                                CheckboxProps::default()
                                    .variant(*variant)
                                    .high_contrast(true),
                            );

                            // Disabled
                            let _ = checkbox_with_props(
                                grid,
                                &self.theme,
                                &mut self.variant_states[v_idx][2],
                                "",
                                CheckboxProps::default().variant(*variant).enabled(false),
                            );

                            grid.end_row();
                        }
                    });

                // Sizes
                section_title(ui, "Sizes");
                ui.horizontal(|row| {
                    row.spacing_mut().item_spacing.x = 24.0;
                    for (s_idx, (size, name)) in SIZES.iter().zip(SIZE_NAMES.iter()).enumerate() {
                        let _ = checkbox_with_props(
                            row,
                            &self.theme,
                            &mut self.size_states[s_idx],
                            *name,
                            CheckboxProps::default().size(*size),
                        );
                    }
                });

                // Colors
                section_title(ui, "Colors");
                {
                    let colors: [(egui::Color32, &str); 6] = [
                        (egui::Color32::from_rgb(37, 99, 235), COLOR_NAMES[0]), // Blue
                        (egui::Color32::from_rgb(34, 197, 94), COLOR_NAMES[1]), // Green
                        (egui::Color32::from_rgb(245, 158, 11), COLOR_NAMES[2]), // Amber
                        (egui::Color32::from_rgb(239, 68, 68), COLOR_NAMES[3]), // Red
                        (egui::Color32::from_rgb(168, 85, 247), COLOR_NAMES[4]), // Purple
                        (egui::Color32::from_rgb(115, 115, 115), COLOR_NAMES[5]), // Gray
                    ];
                    ui.horizontal(|row| {
                        row.spacing_mut().item_spacing.x = 24.0;
                        for (c_idx, (color, name)) in colors.iter().enumerate() {
                            let _ = checkbox_with_props(
                                row,
                                &self.theme,
                                &mut self.color_states[c_idx],
                                *name,
                                CheckboxProps::default().color(*color),
                            );
                        }
                    });
                }

                // Form
                section_title(ui, "Form");
                ui.vertical(|form| {
                    form.spacing_mut().item_spacing.y = 12.0;
                    form.set_max_width(360.0);
                    form.horizontal(|row| {
                        row.spacing_mut().item_spacing.x = 12.0;
                        let _ = checkbox(
                            row,
                            &self.theme,
                            &mut self.form_terms,
                            "",
                            ControlVariant::Primary,
                            ControlSize::Md,
                            true,
                        );
                        row.vertical(|col| {
                            col.spacing_mut().item_spacing.y = 4.0;
                            Label::new("Accept terms and conditions")
                                .size(ControlSize::Sm)
                                .show(col, &self.theme);
                            col.label(
                                RichText::new(
                                    "You agree to our Terms of Service and Privacy Policy.",
                                )
                                .color(self.theme.palette.muted_foreground)
                                .size(12.0),
                            );
                        });
                    });

                    let submit = button(
                        form,
                        &self.theme,
                        "Submit",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        true,
                    );
                    if submit.clicked() {
                        self.form_submitted = true;
                    }
                    if self.form_submitted {
                        let msg = if self.form_terms {
                            "Form submitted successfully."
                        } else {
                            "You must accept the terms."
                        };
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
        "Checkbox example",
        options,
        Box::new(|_cc| Ok(Box::new(CheckboxDemo::new()))),
    )
}
