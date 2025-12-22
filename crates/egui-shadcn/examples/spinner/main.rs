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

impl App for SpinnerExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            let available = ui.available_size();
            ui.set_min_size(available);
            ui.vertical_centered_justified(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(20.0, 20.0);

                ui.heading("Spinner");
                ui.label("Radix Themes leaf spinner with shadcn/ui defaults and loading overlay.");

                egui::Grid::new("spinner-card-grid")
                    .spacing(egui::vec2(20.0, 20.0))
                    .min_col_width(220.0)
                    .show(ui, |ui| {
                        // Card: size variants 1–3
                        let _ = card(
                            ui,
                            &self.theme,
                            CardProps::default().with_heading("Sizes 1–3"),
                            |ui| {
                                ui.horizontal(|ui| {
                                    let _ = spinner(
                                        ui,
                                        &self.theme,
                                        SpinnerProps::default().with_size(SpinnerSize::Size1),
                                    );
                                    let _ = spinner(
                                        ui,
                                        &self.theme,
                                        SpinnerProps::default().with_size(SpinnerSize::Size2),
                                    );
                                    let _ = spinner(
                                        ui,
                                        &self.theme,
                                        SpinnerProps::default().with_size(SpinnerSize::Size3),
                                    );
                                });
                            },
                        );

                        // Card: color / theme override
                        let _ = card(
                            ui,
                            &self.theme,
                            CardProps::default().with_heading("Custom color"),
                            |ui| {
                                ui.horizontal_centered(|ui| {
                                    let _ = spinner(
                                        ui,
                                        &self.theme,
                                        SpinnerProps::default()
                                            .with_color(egui::Color32::from_rgb(59, 130, 246)),
                                    );
                                });
                            },
                        );
                        ui.end_row();

                        // Card: Lucide loader-circle
                        let _ = card(
                            ui,
                            &self.theme,
                            CardProps::default().with_heading("Lucide loader-circle"),
                            |ui| {
                                ui.horizontal_centered(|ui| {
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

                        // Card: overlay example
                        let _ = card(
                            ui,
                            &self.theme,
                            CardProps::default().with_heading("Overlay loading"),
                            |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.set_width(ui.available_width());
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
                                                            "Saving changes…"
                                                        } else {
                                                            "Saved"
                                                        });
                                                    },
                                                );
                                            });
                                        },
                                    );

                                    response
                                        .on_hover_text("Spinner overlays content while loading");
                                });
                            },
                        );
                        ui.end_row();
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
