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
use egui_shadcn::{
    ControlSize, Label, SliderOrientation, SliderProps, SliderRadius, SliderSize, SliderVariant,
    Theme, slider_with_props,
};

const VARIANT_NAMES: [&str; 3] = ["Classic", "Surface", "Soft"];
const VARIANTS: [SliderVariant; 3] = [
    SliderVariant::Classic,
    SliderVariant::Surface,
    SliderVariant::Soft,
];

const SIZE_NAMES: [&str; 3] = ["Size 1", "Size 2", "Size 3"];
const SIZES: [SliderSize; 3] = [SliderSize::Size1, SliderSize::Size2, SliderSize::Size3];

const RADIUS_NAMES: [&str; 5] = ["None", "Small", "Medium", "Large", "Full"];
const RADII: [SliderRadius; 5] = [
    SliderRadius::None,
    SliderRadius::Small,
    SliderRadius::Medium,
    SliderRadius::Large,
    SliderRadius::Full,
];

const COLOR_NAMES: [&str; 6] = ["Blue", "Green", "Amber", "Red", "Purple", "Gray"];

struct SliderExample {
    theme: Theme,
    // Demo
    value: Vec<f32>,
    // Range
    price_range: Vec<f32>,
    // Variants: 3 variants × 3 (default, high_contrast, disabled)
    variant_values: [[Vec<f32>; 3]; 3],
    // Sizes: 3
    size_values: [Vec<f32>; 3],
    // Vertical: 3
    vertical_values: [Vec<f32>; 3],
    // Radius: 5
    radius_values: [Vec<f32>; 5],
    // Colors: 6
    color_values: [Vec<f32>; 6],
}

impl SliderExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            value: vec![50.0],
            price_range: vec![200.0, 800.0],
            variant_values: [
                [vec![30.0], vec![30.0], vec![30.0]],
                [vec![40.0], vec![40.0], vec![40.0]],
                [vec![50.0], vec![50.0], vec![50.0]],
            ],
            size_values: [vec![20.0], vec![35.0], vec![50.0]],
            vertical_values: [vec![40.0], vec![50.0], vec![60.0]],
            radius_values: [vec![25.0], vec![35.0], vec![45.0], vec![55.0], vec![65.0]],
            color_values: [
                vec![15.0],
                vec![27.0],
                vec![39.0],
                vec![51.0],
                vec![63.0],
                vec![75.0],
            ],
        }
    }
}

fn section_title(ui: &mut egui::Ui, title: &str) {
    ui.label(RichText::new(title).size(16.0).strong());
    ui.add_space(4.0);
}

fn caption(_ui: &egui::Ui, theme: &Theme, text: &str) -> egui::widget_text::WidgetText {
    RichText::new(text)
        .size(12.0)
        .color(theme.palette.muted_foreground)
        .into()
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
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_max_width(760.0);
                ui.spacing_mut().item_spacing.y = 24.0;

                ui.heading("Slider");
                ui.add_space(4.0);

                // Demo
                section_title(ui, "Demo");
                {
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
                }

                // Range
                section_title(ui, "Range");
                {
                    let field_width = ui.available_width().min(448.0);
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
                    ui.spacing_mut().item_spacing.y = 24.0;
                }

                // Variants
                section_title(ui, "Variants");
                egui::Grid::new("slider_variants_grid")
                    .num_columns(4)
                    .spacing(egui::vec2(16.0, 12.0))
                    .show(ui, |grid| {
                        // Header
                        grid.label(caption(grid, &self.theme, "Variant"));
                        grid.label(caption(grid, &self.theme, "Default"));
                        grid.label(caption(grid, &self.theme, "High Contrast"));
                        grid.label(caption(grid, &self.theme, "Disabled"));
                        grid.end_row();

                        for (v_idx, (variant, name)) in
                            VARIANTS.iter().zip(VARIANT_NAMES.iter()).enumerate()
                        {
                            grid.label(caption(grid, &self.theme, name));

                            slider_with_props(
                                grid,
                                &self.theme,
                                SliderProps::new(
                                    format!("v-{v_idx}-default"),
                                    &mut self.variant_values[v_idx][0],
                                )
                                .min(0.0)
                                .max(100.0)
                                .variant(*variant)
                                .width(180.0),
                            );

                            slider_with_props(
                                grid,
                                &self.theme,
                                SliderProps::new(
                                    format!("v-{v_idx}-hc"),
                                    &mut self.variant_values[v_idx][1],
                                )
                                .min(0.0)
                                .max(100.0)
                                .variant(*variant)
                                .high_contrast(true)
                                .width(180.0),
                            );

                            slider_with_props(
                                grid,
                                &self.theme,
                                SliderProps::new(
                                    format!("v-{v_idx}-dis"),
                                    &mut self.variant_values[v_idx][2],
                                )
                                .min(0.0)
                                .max(100.0)
                                .variant(*variant)
                                .disabled(true)
                                .width(180.0),
                            );

                            grid.end_row();
                        }
                    });

                // Sizes
                section_title(ui, "Sizes");
                egui::Grid::new("slider_sizes_grid")
                    .num_columns(2)
                    .spacing(egui::vec2(16.0, 12.0))
                    .show(ui, |grid| {
                        for (s_idx, (size, name)) in SIZES.iter().zip(SIZE_NAMES.iter()).enumerate()
                        {
                            grid.label(caption(grid, &self.theme, name));
                            slider_with_props(
                                grid,
                                &self.theme,
                                SliderProps::new(
                                    format!("s-{s_idx}"),
                                    &mut self.size_values[s_idx],
                                )
                                .min(0.0)
                                .max(100.0)
                                .size(*size)
                                .width(300.0),
                            );
                            grid.end_row();
                        }
                    });

                // Vertical
                section_title(ui, "Vertical");
                ui.horizontal(|row| {
                    row.spacing_mut().item_spacing.x = 16.0;
                    for (v_idx, (size, _name)) in SIZES.iter().zip(SIZE_NAMES.iter()).enumerate() {
                        slider_with_props(
                            row,
                            &self.theme,
                            SliderProps::new(
                                format!("vert-{v_idx}"),
                                &mut self.vertical_values[v_idx],
                            )
                            .min(0.0)
                            .max(100.0)
                            .size(*size)
                            .orientation(SliderOrientation::Vertical)
                            .height(160.0),
                        );
                    }
                });

                // Radius
                section_title(ui, "Radius");
                egui::Grid::new("slider_radius_grid")
                    .num_columns(2)
                    .spacing(egui::vec2(16.0, 12.0))
                    .show(ui, |grid| {
                        for (r_idx, (radius, name)) in
                            RADII.iter().zip(RADIUS_NAMES.iter()).enumerate()
                        {
                            grid.label(caption(grid, &self.theme, name));
                            slider_with_props(
                                grid,
                                &self.theme,
                                SliderProps::new(
                                    format!("r-{r_idx}"),
                                    &mut self.radius_values[r_idx],
                                )
                                .min(0.0)
                                .max(100.0)
                                .radius(*radius)
                                .width(300.0),
                            );
                            grid.end_row();
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
                    egui::Grid::new("slider_colors_grid")
                        .num_columns(2)
                        .spacing(egui::vec2(16.0, 12.0))
                        .show(ui, |grid| {
                            for (c_idx, (color, name)) in
                                colors.iter().zip(COLOR_NAMES.iter()).enumerate()
                            {
                                grid.label(caption(grid, &self.theme, name));
                                slider_with_props(
                                    grid,
                                    &self.theme,
                                    SliderProps::new(
                                        format!("c-{c_idx}"),
                                        &mut self.color_values[c_idx],
                                    )
                                    .min(0.0)
                                    .max(100.0)
                                    .color(*color)
                                    .width(200.0),
                                );
                                grid.end_row();
                            }
                        });
                }
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
