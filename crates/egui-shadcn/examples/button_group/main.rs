#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;

use eframe::{App, Frame, egui};
use egui::{FontData, FontDefinitions, FontFamily, FontId, RichText, vec2};
use egui_shadcn::{
    Button, ButtonGroup, ButtonGroupOrientation, ButtonSize, ButtonVariant, Theme, button_group,
};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

struct ButtonGroupDemo {
    theme: Theme,
}

impl ButtonGroupDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

fn ensure_lucide_font(ctx: &egui::Context) {
    let font_loaded_id = egui::Id::new("lucide_font_loaded");
    let already_set = ctx.data(|d| d.get_temp::<bool>(font_loaded_id).unwrap_or(false));
    if already_set {
        return;
    }

    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "lucide".into(),
        FontData::from_static(LUCIDE_FONT_BYTES).into(),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "lucide".into());
    ctx.set_fonts(fonts);
    ctx.data_mut(|d| d.insert_temp(font_loaded_id, true));
}

fn lucide_icon(icon: Icon, size: f32) -> RichText {
    RichText::new(icon.unicode().to_string()).font(FontId::new(size, FontFamily::Proportional))
}

fn example_card(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.vertical(|ui| {
        ui.label(RichText::new(title).strong());
        ui.add_space(6.0);
        content(ui);
    });
}

impl App for ButtonGroupDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ensure_lucide_font(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = vec2(16.0, 16.0);

            ui.heading("Button Group");
            ui.add_space(12.0);

            egui::Grid::new("button_groups_grid")
                .num_columns(3)
                .spacing(vec2(40.0, 40.0))
                .show(ui, |grid| {
                    // Demo 1: Default (Horizontal)
                    example_card(grid, "Horizontal (Outline)", |ui| {
                        button_group(
                            ui,
                            &self.theme,
                            vec![
                                Button::new("One").variant(ButtonVariant::Outline),
                                Button::new("Two").variant(ButtonVariant::Outline),
                                Button::new("Three").variant(ButtonVariant::Outline),
                            ],
                        );
                    });

                    // Demo 2: Secondary
                    example_card(grid, "Secondary", |ui| {
                        button_group(
                            ui,
                            &self.theme,
                            vec![
                                Button::new("One").variant(ButtonVariant::Secondary),
                                Button::new("Two").variant(ButtonVariant::Secondary),
                                Button::new("Three").variant(ButtonVariant::Secondary),
                            ],
                        );
                    });

                    // Demo 3: Vertical
                    example_card(grid, "Vertical", |ui| {
                        ButtonGroup::new(vec![
                            Button::new("Top").variant(ButtonVariant::Outline),
                            Button::new("Middle").variant(ButtonVariant::Outline),
                            Button::new("Bottom").variant(ButtonVariant::Outline),
                        ])
                        .orientation(ButtonGroupOrientation::Vertical)
                        .show(ui, &self.theme);
                    });

                    grid.end_row();

                    // Demo 4: Mixed (Icon + Text)
                    example_card(grid, "With Icons", |ui| {
                        button_group(
                            ui,
                            &self.theme,
                            vec![
                                Button::new(lucide_icon(Icon::Bold, 16.0))
                                    .variant(ButtonVariant::Outline),
                                Button::new(lucide_icon(Icon::Italic, 16.0))
                                    .variant(ButtonVariant::Outline),
                                Button::new(lucide_icon(Icon::Underline, 16.0))
                                    .variant(ButtonVariant::Outline),
                            ],
                        );
                    });

                    // Demo 5: Single Item
                    example_card(grid, "Single Item", |ui| {
                        button_group(
                            ui,
                            &self.theme,
                            vec![Button::new("Solo").variant(ButtonVariant::Outline)],
                        );
                    });

                    // Demo 6: Custom Radius
                    example_card(grid, "Custom Radius (0)", |ui| {
                        ButtonGroup::new(vec![
                            Button::new("Square").variant(ButtonVariant::Outline),
                            Button::new("Buttons").variant(ButtonVariant::Outline),
                        ])
                        .radius(0)
                        .show(ui, &self.theme);
                    });

                    // Demo 7: Sizes
                    example_card(grid, "Sizes (Sm, Default, Lg)", |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 24.0;

                            // Small
                            ButtonGroup::new(vec![
                                Button::new("Small")
                                    .size(ButtonSize::Sm)
                                    .variant(ButtonVariant::Outline),
                                Button::new(lucide_icon(Icon::ChevronDown, 14.0))
                                    .size(ButtonSize::Sm)
                                    .variant(ButtonVariant::Outline),
                            ])
                            .show(ui, &self.theme);

                            // Default
                            ButtonGroup::new(vec![
                                Button::new("Default").variant(ButtonVariant::Outline),
                                Button::new(lucide_icon(Icon::ChevronDown, 16.0))
                                    .variant(ButtonVariant::Outline),
                            ])
                            .show(ui, &self.theme);

                            // Large
                            ButtonGroup::new(vec![
                                Button::new("Large")
                                    .size(ButtonSize::Lg)
                                    .variant(ButtonVariant::Outline),
                                Button::new(lucide_icon(Icon::ChevronDown, 18.0))
                                    .size(ButtonSize::Lg)
                                    .variant(ButtonVariant::Outline),
                            ])
                            .show(ui, &self.theme);
                        });
                    });

                    grid.end_row();
                });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Button Group example",
        options,
        Box::new(|_cc| Ok(Box::new(ButtonGroupDemo::new()))),
    )
}
