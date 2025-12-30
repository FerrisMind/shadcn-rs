#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::text::LayoutJob;
use egui::{FontFamily, FontId, RichText, TextFormat, TextStyle};
use egui_shadcn::{ControlSize, Label, SliderProps, Theme, slider_with_props};

struct SliderExample {
    theme: Theme,
    value: Vec<f32>,
    price_range: Vec<f32>,
}

impl SliderExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            value: vec![50.0],
            price_range: vec![200.0, 800.0],
        }
    }
}

fn example_card(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.vertical(|ui| {
        ui.label(RichText::new(title).strong());
        ui.add_space(6.0);
        content(ui);
    });
}

fn price_range_description(ui: &egui::Ui, theme: &Theme, values: &[f32]) -> LayoutJob {
    let min_value = values.first().copied().unwrap_or(0.0).round() as i32;
    let max_value = values.get(1).copied().unwrap_or(min_value as f32).round() as i32;

    let base_font = ui
        .style()
        .text_styles
        .get(&TextStyle::Small)
        .cloned()
        .unwrap_or_else(|| FontId::proportional(12.0));
    let number_font = FontId::new(base_font.size, FontFamily::Monospace);

    let base_format = TextFormat {
        font_id: base_font,
        color: theme.palette.muted_foreground,
        ..Default::default()
    };
    let number_format = TextFormat {
        font_id: number_font,
        color: theme.palette.foreground,
        ..Default::default()
    };

    let mut job = LayoutJob::default();
    job.append("Set your budget range ($", 0.0, base_format.clone());
    job.append(&min_value.to_string(), 0.0, number_format.clone());
    job.append(" - ", 0.0, base_format.clone());
    job.append(&max_value.to_string(), 0.0, number_format);
    job.append(").", 0.0, base_format);
    job
}

impl App for SliderExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
            let top_pad = (ui.available_height() * 0.36).max(92.0);
            ui.add_space(top_pad);
            ui.vertical_centered(|ui| {
                ui.set_width(ui.available_width().min(760.0));
                ui.heading("Slider");
                ui.add_space(12.0);

                egui::Grid::new("slider_examples_grid")
                    .num_columns(2)
                    .spacing(egui::vec2(24.0, 20.0))
                    .show(ui, |grid| {
                        example_card(grid, "Slider", |ui| {
                            ui.set_width(ui.available_width());
                            let width = (ui.available_width() * 0.6).clamp(200.0, 420.0);
                            slider_with_props(
                                ui,
                                &self.theme,
                                SliderProps::new("slider-demo", &mut self.value)
                                    .min(0.0)
                                    .max(100.0)
                                    .step(1.0)
                                    .width(width),
                            );
                        });

                        example_card(grid, "Field", |ui| {
                            let field_width = ui.available_width().min(448.0);
                            ui.set_min_width(field_width);
                            ui.set_max_width(field_width);
                            ui.spacing_mut().item_spacing.y = 6.0;

                            let slider_id = ui.make_persistent_id("slider-field");
                            Label::new("Price Range")
                                .for_id(slider_id)
                                .size(ControlSize::Sm)
                                .show(ui, &self.theme);
                            ui.label(price_range_description(ui, &self.theme, &self.price_range));
                            ui.add_space(8.0);

                            slider_with_props(
                                ui,
                                &self.theme,
                                SliderProps::new(slider_id, &mut self.price_range)
                                    .min(0.0)
                                    .max(1000.0)
                                    .step(10.0)
                                    .width(field_width),
                            );
                        });
                        grid.end_row();
                    });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Slider example",
        options,
        Box::new(|_cc| Ok(Box::new(SliderExample::new()))),
    )
}
