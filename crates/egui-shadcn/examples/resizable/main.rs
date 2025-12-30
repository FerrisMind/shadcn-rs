#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/logging.rs"]
mod logging;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CentralPanel, Color32, RichText};
use egui_shadcn::{
    ResizableDirection, ResizableHandleProps, ResizablePanelGroupProps, ResizablePanelProps,
    Theme, resizable_handle, resizable_panel, resizable_panel_group,
};

struct ResizableDemo {
    theme: Theme,
    horizontal_sizes: Vec<f32>,
    vertical_sizes: Vec<f32>,
}

impl ResizableDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            horizontal_sizes: vec![50.0, 50.0],
            vertical_sizes: vec![30.0, 70.0],
        }
    }
}

impl App for ResizableDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        let theme = self.theme.clone();

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Resizable Panels Demo");
            ui.add_space(16.0);

            // Horizontal example
            ui.label(RichText::new("Horizontal Layout").size(14.0).strong());
            ui.add_space(8.0);

            let available_height = 150.0;
            ui.allocate_ui(egui::vec2(ui.available_width(), available_height), |ui| {
                resizable_panel_group(
                    ui,
                    &theme,
                    ResizablePanelGroupProps::new("horizontal-demo"),
                    &mut self.horizontal_sizes,
                    |ui, ctx| {
                        resizable_panel(ui, ctx, ResizablePanelProps::new(50.0).min_size(20.0), 0, |ui| {
                            panel_content(ui, "Left Panel", Color32::from_rgb(59, 130, 246));
                        });

                        resizable_handle(ui, &theme, ctx, ResizableHandleProps::new().with_handle(true), 0);

                        resizable_panel(ui, ctx, ResizablePanelProps::new(50.0).min_size(20.0), 1, |ui| {
                            panel_content(ui, "Right Panel", Color32::from_rgb(34, 197, 94));
                        });
                    },
                );
            });

            ui.add_space(24.0);

            // Vertical example
            ui.label(RichText::new("Vertical Layout").size(14.0).strong());
            ui.add_space(8.0);

            ui.allocate_ui(egui::vec2(ui.available_width(), 200.0), |ui| {
                resizable_panel_group(
                    ui,
                    &theme,
                    ResizablePanelGroupProps::new("vertical-demo")
                        .direction(ResizableDirection::Vertical),
                    &mut self.vertical_sizes,
                    |ui, ctx| {
                        resizable_panel(ui, ctx, ResizablePanelProps::new(30.0), 0, |ui| {
                            panel_content(ui, "Top Panel", Color32::from_rgb(168, 85, 247));
                        });

                        resizable_handle(ui, &theme, ctx, ResizableHandleProps::new().with_handle(true), 0);

                        resizable_panel(ui, ctx, ResizablePanelProps::new(70.0), 1, |ui| {
                            panel_content(ui, "Bottom Panel", Color32::from_rgb(236, 72, 153));
                        });
                    },
                );
            });

            ui.add_space(16.0);

            // Show current sizes
            ui.label(format!("Horizontal sizes: {:.1}% / {:.1}%", self.horizontal_sizes[0], self.horizontal_sizes[1]));
            ui.label(format!("Vertical sizes: {:.1}% / {:.1}%", self.vertical_sizes[0], self.vertical_sizes[1]));
        });
    }
}

fn panel_content(ui: &mut egui::Ui, label: &str, color: Color32) {
    let rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(rect, 4.0, color.gamma_multiply(0.2));
    
    ui.centered_and_justified(|ui| {
        ui.label(RichText::new(label).color(color).size(14.0).strong());
    });
}

fn main() -> eframe::Result<()> {
    let log_path = logging::init_file_logger("resizable")
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "<failed>".to_string());
    let options = icon::native_options();
    eframe::run_native(
        "Resizable example",
        options,
        Box::new(move |_cc| {
            log::info!("logging to {log_path}");
            Ok(Box::new(ResizableDemo::new()))
        }),
    )
}
