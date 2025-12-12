#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;

use eframe::{App, Frame, egui};
use egui_shadcn::{
    ScrollAreaProps, ScrollAreaRadius, ScrollAreaSize, ScrollAreaType, ScrollDirection,
    SeparatorProps, Theme, scroll_area, separator,
};

struct ScrollAreaDemo {
    theme: Theme,
}

impl ScrollAreaDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

impl App for ScrollAreaDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 24.0;

                let tags: Vec<String> =
                    (1..=50).rev().map(|i| format!("v1.2.0-beta.{i}")).collect();

                egui::Frame::NONE
                    .fill(self.theme.palette.background)
                    .stroke(egui::Stroke::new(1.0, self.theme.palette.border))
                    .corner_radius(egui::CornerRadius::same(6))
                    .show(ui, |frame_ui| {
                        let props = ScrollAreaProps::default()
                            .with_id(frame_ui.make_persistent_id("scroll-area-demo"))
                            .with_direction(ScrollDirection::Vertical)
                            .with_type(ScrollAreaType::Auto)
                            .with_size(ScrollAreaSize::Size1)
                            .with_radius(ScrollAreaRadius::Medium)
                            .with_max_size(egui::vec2(192.0, 288.0))
                            .with_auto_shrink([false; 2]);

                        scroll_area(frame_ui, &self.theme, props, |scroll_ui| {
                            egui::Frame::NONE.inner_margin(egui::Margin::same(16)).show(
                                scroll_ui,
                                |content| {
                                    content.spacing_mut().item_spacing.y = 8.0;
                                    content.label(egui::RichText::new("Tags").size(14.0).strong());
                                    content.add_space(8.0);

                                    for tag in tags {
                                        content.label(egui::RichText::new(tag).size(12.0));
                                        separator(content, &self.theme, SeparatorProps::default());
                                    }
                                },
                            );
                        });
                    });

                egui::Frame::NONE
                    .fill(self.theme.palette.background)
                    .stroke(egui::Stroke::new(1.0, self.theme.palette.border))
                    .corner_radius(egui::CornerRadius::same(6))
                    .show(ui, |frame_ui| {
                        let props = ScrollAreaProps::default()
                            .with_id(frame_ui.make_persistent_id("scroll-area-horizontal-demo"))
                            .with_direction(ScrollDirection::Horizontal)
                            .with_type(ScrollAreaType::Auto)
                            .with_size(ScrollAreaSize::Size1)
                            .with_radius(ScrollAreaRadius::Medium)
                            .with_max_size(egui::vec2(384.0, 220.0))
                            .with_auto_shrink([false; 2]);

                        scroll_area(frame_ui, &self.theme, props, |scroll_ui| {
                            egui::Frame::NONE.inner_margin(egui::Margin::same(16)).show(
                                scroll_ui,
                                |content| {
                                    content.horizontal(|row| {
                                        row.spacing_mut().item_spacing.x = 16.0;
                                        for artist in
                                            ["Ornella Binni", "Tom Byrom", "Vladimir Malyavko"]
                                        {
                                            row.vertical(|col| {
                                                col.spacing_mut().item_spacing.y = 8.0;
                                                let (rect, _) = col.allocate_exact_size(
                                                    egui::vec2(90.0, 120.0),
                                                    egui::Sense::hover(),
                                                );
                                                col.painter().rect_filled(
                                                    rect,
                                                    egui::CornerRadius::same(6),
                                                    self.theme.palette.muted,
                                                );
                                                col.label(
                                                    egui::RichText::new(format!(
                                                        "Photo by {artist}"
                                                    ))
                                                    .size(11.0)
                                                    .color(self.theme.palette.muted_foreground),
                                                );
                                            });
                                        }
                                    });
                                },
                            );
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
        "ScrollArea example",
        options,
        Box::new(|_cc| Ok(Box::new(ScrollAreaDemo::new()))),
    )
}
