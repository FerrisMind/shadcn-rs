//! Пример Popover, повторяющий shadcn/ui `popover-demo`.
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::{
    ControlSize, ControlVariant, Input, InputSize, Label, PopoverProps, Theme, button, popover,
};

struct PopoverDemo {
    theme: Theme,
    open: bool,
    width_value: String,
    max_width_value: String,
    height_value: String,
    max_height_value: String,
}

struct DimensionRowConfig<'a> {
    id_suffix: &'a str,
    label: &'a str,
    value: &'a mut String,
}

impl PopoverDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            open: false,
            width_value: "100%".to_string(),
            max_width_value: "300px".to_string(),
            height_value: "25px".to_string(),
            max_height_value: "none".to_string(),
        }
    }
}

impl App for PopoverDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut open = self.open;
            let _ = popover(
                ui,
                &self.theme,
                PopoverProps::new(ui.make_persistent_id("popover-demo"), &mut open)
                    .with_width(320.0),
                |trigger_ui| {
                    button(
                        trigger_ui,
                        &self.theme,
                        "Open popover",
                        ControlVariant::Outline,
                        ControlSize::Md,
                        true,
                    )
                },
                |content_ui| {
                    content_ui.vertical(|content| {
                        content.spacing_mut().item_spacing.y = 16.0;

                        content.vertical(|header| {
                            header.spacing_mut().item_spacing.y = 8.0;
                            header.label(egui::RichText::new("Dimensions").size(14.0).strong());
                            header.label(
                                egui::RichText::new("Set the dimensions for the layer.")
                                    .color(self.theme.palette.muted_foreground)
                                    .size(12.0),
                            );
                        });

                        content.vertical(|fields| {
                            fields.spacing_mut().item_spacing.y = 8.0;
                            let input_size = InputSize::Size2;
                            let row_height = input_size.height();
                            let label_width = 90.0;

                            render_dimension_row(
                                fields,
                                &self.theme,
                                input_size,
                                label_width,
                                row_height,
                                DimensionRowConfig {
                                    id_suffix: "popover-width",
                                    label: "Width",
                                    value: &mut self.width_value,
                                },
                            );
                            render_dimension_row(
                                fields,
                                &self.theme,
                                input_size,
                                label_width,
                                row_height,
                                DimensionRowConfig {
                                    id_suffix: "popover-max-width",
                                    label: "Max. width",
                                    value: &mut self.max_width_value,
                                },
                            );
                            render_dimension_row(
                                fields,
                                &self.theme,
                                input_size,
                                label_width,
                                row_height,
                                DimensionRowConfig {
                                    id_suffix: "popover-height",
                                    label: "Height",
                                    value: &mut self.height_value,
                                },
                            );
                            render_dimension_row(
                                fields,
                                &self.theme,
                                input_size,
                                label_width,
                                row_height,
                                DimensionRowConfig {
                                    id_suffix: "popover-max-height",
                                    label: "Max. height",
                                    value: &mut self.max_height_value,
                                },
                            );
                        });
                    });
                },
            );
            self.open = open;
        });
    }
}

fn render_dimension_row(
    ui: &mut egui::Ui,
    theme: &Theme,
    input_size: InputSize,
    label_width: f32,
    row_height: f32,
    config: DimensionRowConfig<'_>,
) {
    let DimensionRowConfig {
        id_suffix,
        label,
        value,
    } = config;
    ui.horizontal(|row| {
        row.spacing_mut().item_spacing.x = 16.0;
        let input_id = row.make_persistent_id(id_suffix);
        row.allocate_ui_with_layout(
            egui::vec2(label_width, row_height),
            egui::Layout::right_to_left(egui::Align::Center),
            |label_ui| {
                Label::new(label)
                    .for_id(input_id)
                    .size(ControlSize::Sm)
                    .show(label_ui, theme);
            },
        );
        Input::new(input_id)
            .size(input_size)
            .width(row.available_width())
            .show(row, theme, value);
    });
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = NativeOptions::default();
    eframe::run_native(
        "Popover example",
        options,
        Box::new(|_cc| Ok(Box::new(PopoverDemo::new()))),
    )
}
