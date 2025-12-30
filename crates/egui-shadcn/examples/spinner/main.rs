#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{Direction, Layout};
use egui_shadcn::{
    CardProps, SpinnerProps, SpinnerSize, SpinnerVariant, Theme, card, spinner,
    spinner_with_content,
};

struct SpinnerExample {
    theme: Theme,
    loading: bool,
}

impl SpinnerExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            loading: true,
        }
    }
}

fn centered_card_content(
    ui: &mut egui::Ui,
    min_height: f32,
    content: impl FnOnce(&mut egui::Ui),
) {
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), min_height),
        Layout::centered_and_justified(Direction::LeftToRight),
        |ui| content(ui),
    );
}

impl App for SpinnerExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            let available = ui.available_size();
            ui.set_min_size(available);
            ui.vertical_centered(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(20.0, 20.0);

                ui.heading("Spinner");
                ui.label("Radix Themes leaf spinner with shadcn/ui defaults and loading overlay.");
                let grid_spacing = egui::vec2(20.0, 20.0);
                let card_width = 260.0;
                let card_height = 72.0;
                let block_width = card_width * 2.0 + grid_spacing.x;
                let left_pad = ((ui.available_width() - block_width) * 0.5).max(0.0);
                ui.horizontal(|ui| {
                    ui.add_space(left_pad);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.allocate_ui(egui::vec2(card_width, 0.0), |ui| {
                                ui.set_min_width(card_width);
                                ui.set_max_width(card_width);
                                let _ = card(
                                    ui,
                                    &self.theme,
                                    CardProps::default().with_heading("Sizes 1-3"),
                                    |ui| {
                                        centered_card_content(ui, card_height, |ui| {
                                            ui.horizontal_centered(|ui| {
                                                let _ = spinner(
                                                    ui,
                                                    &self.theme,
                                                    SpinnerProps::default()
                                                        .with_size(SpinnerSize::Size1),
                                                );
                                                let _ = spinner(
                                                    ui,
                                                    &self.theme,
                                                    SpinnerProps::default()
                                                        .with_size(SpinnerSize::Size2),
                                                );
                                                let _ = spinner(
                                                    ui,
                                                    &self.theme,
                                                    SpinnerProps::default()
                                                        .with_size(SpinnerSize::Size3),
                                                );
                                            });
                                        });
                                    },
                                );
                            });

                            ui.add_space(grid_spacing.x);

                            ui.allocate_ui(egui::vec2(card_width, 0.0), |ui| {
                                ui.set_min_width(card_width);
                                ui.set_max_width(card_width);
                                let _ = card(
                                    ui,
                                    &self.theme,
                                    CardProps::default().with_heading("Custom color"),
                                    |ui| {
                                        centered_card_content(ui, card_height, |ui| {
                                            let _ = spinner(
                                                ui,
                                                &self.theme,
                                                SpinnerProps::default().with_color(
                                                    egui::Color32::from_rgb(59, 130, 246),
                                                ),
                                            );
                                        });
                                    },
                                );
                            });
                        });

                        ui.add_space(grid_spacing.y);

                        ui.horizontal(|ui| {
                            ui.allocate_ui(egui::vec2(card_width, 0.0), |ui| {
                                ui.set_min_width(card_width);
                                ui.set_max_width(card_width);
                                let _ = card(
                                    ui,
                                    &self.theme,
                                    CardProps::default()
                                        .with_heading("Lucide loader-circle"),
                                    |ui| {
                                        centered_card_content(ui, card_height, |ui| {
                                            let _ = spinner(
                                                ui,
                                                &self.theme,
                                                SpinnerProps::default()
                                                    .with_variant(SpinnerVariant::LucideLoaderCircle)
                                                    .with_size(SpinnerSize::Size2),
                                            );
                                        });
                                    },
                                );
                            });

                            ui.add_space(grid_spacing.x);

                            ui.allocate_ui(egui::vec2(card_width, 0.0), |ui| {
                                ui.set_min_width(card_width);
                                ui.set_max_width(card_width);
                                let _ = card(
                                    ui,
                                    &self.theme,
                                    CardProps::default().with_heading("Overlay loading"),
                                    |ui| {
                                        centered_card_content(ui, card_height, |ui| {
                                            let overlay_props = SpinnerProps::default()
                                                .with_loading(self.loading)
                                                .with_size(SpinnerSize::Size2);

                                            let (_inner, response) = spinner_with_content(
                                                ui,
                                                &self.theme,
                                                overlay_props,
                                                |ui| {
                                                    ui.allocate_ui(egui::vec2(240.0, 44.0), |ui| {
                                                        ui.with_layout(
                                                            Layout::centered_and_justified(
                                                                Direction::LeftToRight,
                                                            ),
                                                            |ui| {
                                                                ui.label(if self.loading {
                                                                    "Saving changes:"
                                                                } else {
                                                                    "Saved"
                                                                });
                                                            },
                                                        );
                                                    });
                                                },
                                            );

                                            response.on_hover_text(
                                                "Spinner overlays content while loading",
                                            );
                                        });
                                    },
                                );
                            });
                        });
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Toggle loading");
                    ui.checkbox(&mut self.loading, "");
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Spinner example",
        options,
        Box::new(|_cc| Ok(Box::new(SpinnerExample::new()))),
    )
}
