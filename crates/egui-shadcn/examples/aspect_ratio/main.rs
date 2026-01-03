#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CentralPanel, CornerRadius, Frame as EguiFrame, RichText, Stroke};
use egui_shadcn::{AspectRatioProps, Theme, aspect_ratio};

struct AspectRatioExample {
    theme: Theme,
}

impl AspectRatioExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

impl App for AspectRatioExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Aspect Ratio");
            ui.label("Maintain a fixed ratio for media.");
            ui.add_space(16.0);

            ui.vertical(|ui| {
                ui.label(RichText::new("Aspect ratio demo").strong());
                ui.label(
                    RichText::new("16:9 container with muted background.")
                        .color(self.theme.palette.muted_foreground)
                        .size(12.0),
                );
                ui.add_space(8.0);

                let rounding = CornerRadius::same(self.theme.radius.r4.round() as u8);
                let _ = aspect_ratio(ui, AspectRatioProps::new(16.0 / 9.0), |content_ui| {
                    let frame = EguiFrame::default()
                        .fill(self.theme.palette.muted)
                        .stroke(Stroke::new(1.0, self.theme.palette.border))
                        .corner_radius(rounding);
                    frame.show(content_ui, |frame_ui| {
                        frame_ui.centered_and_justified(|inner| {
                            inner.label(
                                RichText::new("16:9").color(self.theme.palette.muted_foreground),
                            );
                        });
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
        "Aspect Ratio example",
        options,
        Box::new(|_cc| Ok(Box::new(AspectRatioExample::new()))),
    )
}
