use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::tooltip::{TooltipAlign, TooltipProps, TooltipSide};
use egui_shadcn::{ControlSize, ControlVariant, Theme, button, tooltip};
use log::{error, info};

struct TooltipDemo {
    theme: Theme,
    clicks: usize,
    show_all_sides: bool,
    show_animations: bool,
}

impl TooltipDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            clicks: 0,
            show_all_sides: false,
            show_animations: false,
        }
    }
}

impl App for TooltipDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let mut style = ctx.style().as_ref().clone();
        let bg = egui::Color32::from_rgb(14, 14, 14);
        style.visuals.window_fill = bg;
        style.visuals.panel_fill = bg;
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
            ui.heading("Tooltip Examples (Radix UI Compatible)");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_all_sides, "Show All Sides");
                ui.checkbox(&mut self.show_animations, "Animation Demos");
            });
            ui.add_space(16.0);

            ui.separator();
            ui.label(egui::RichText::new("Basic Examples").strong());
            ui.add_space(8.0);

            ui.horizontal(|ui| {

                ui.label("Basic tooltip (700ms delay):");
                let basic_response = button(
                    ui,
                    &self.theme,
                    "Hover me",
                    ControlVariant::Secondary,
                    ControlSize::Md,
                    true,
                );
                let _ = tooltip(
                    &basic_response,
                    ui,
                    &self.theme,
                    TooltipProps::new("Default tooltip with 700ms delay"),
                );
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {

                ui.label("Instant tooltip (0ms):");
                let instant_response = button(
                    ui,
                    &self.theme,
                    "Instant",
                    ControlVariant::Primary,
                    ControlSize::Sm,
                    true,
                );
                let _ = tooltip(
                    &instant_response,
                    ui,
                    &self.theme,
                    TooltipProps::new("This appears instantly!").delay_ms(0),
                );
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {

                ui.label("Custom delay (200ms):");
                let action_response = button(
                    ui,
                    &self.theme,
                    "200ms delay",
                    ControlVariant::Primary,
                    ControlSize::Sm,
                    true,
                );
                if action_response.clicked() {
                    self.clicks += 1;
                }
                let _ = tooltip(
                    &action_response,
                    ui,
                    &self.theme,
                    TooltipProps::new("Custom 200ms delay tooltip").delay_ms(200),
                );
            });

            ui.add_space(16.0);
            ui.separator();
            ui.label(egui::RichText::new("Arrow Examples").strong());
            ui.add_space(8.0);

            ui.horizontal(|ui| {

                ui.label("With arrow:");
                let arrow_response = button(
                    ui,
                    &self.theme,
                    "Arrow tooltip",
                    ControlVariant::Ghost,
                    ControlSize::Md,
                    true,
                );
                let _ = tooltip(
                    &arrow_response,
                    ui,
                    &self.theme,
                    TooltipProps::new("Tooltip with default arrow (11x5)")
                        .show_arrow(true)
                        .delay_ms(200),
                );
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {

                ui.label("Large arrow:");
                let large_arrow_response = button(
                    ui,
                    &self.theme,
                    "Large arrow",
                    ControlVariant::Outline,
                    ControlSize::Md,
                    true,
                );
                let _ = tooltip(
                    &large_arrow_response,
                    ui,
                    &self.theme,
                    TooltipProps::new("Tooltip with large arrow (16x8)")
                        .show_arrow(true)
                        .arrow_size(16.0, 8.0)
                        .delay_ms(200),
                );
            });

            if self.show_all_sides {
                ui.add_space(16.0);
                ui.separator();
                ui.label(egui::RichText::new("All Sides").strong());
                ui.add_space(8.0);

                ui.horizontal(|ui| {

                    let top_response = button(
                        ui,
                        &self.theme,
                        "Top",
                        ControlVariant::Secondary,
                        ControlSize::Sm,
                        true,
                    );
                    let _ = tooltip(
                        &top_response,
                        ui,
                        &self.theme,
                        TooltipProps::new("Positioned on top")
                            .side(TooltipSide::Top)
                            .show_arrow(true)
                            .delay_ms(100),
                    );

                    let right_response = button(
                        ui,
                        &self.theme,
                        "Right",
                        ControlVariant::Secondary,
                        ControlSize::Sm,
                        true,
                    );
                    let _ = tooltip(
                        &right_response,
                        ui,
                        &self.theme,
                        TooltipProps::new("Positioned on right")
                            .side(TooltipSide::Right)
                            .show_arrow(true)
                            .delay_ms(100),
                    );

                    let bottom_response = button(
                        ui,
                        &self.theme,
                        "Bottom",
                        ControlVariant::Secondary,
                        ControlSize::Sm,
                        true,
                    );
                    let _ = tooltip(
                        &bottom_response,
                        ui,
                        &self.theme,
                        TooltipProps::new("Positioned on bottom")
                            .side(TooltipSide::Bottom)
                            .show_arrow(true)
                            .delay_ms(100),
                    );

                    let left_response = button(
                        ui,
                        &self.theme,
                        "Left",
                        ControlVariant::Secondary,
                        ControlSize::Sm,
                        true,
                    );
                    let _ = tooltip(
                        &left_response,
                        ui,
                        &self.theme,
                        TooltipProps::new("Positioned on left")
                            .side(TooltipSide::Left)
                            .show_arrow(true)
                            .delay_ms(100),
                    );
                });
            }

            ui.add_space(16.0);
            ui.separator();
            ui.label(egui::RichText::new("Alignment Examples").strong());
            ui.add_space(8.0);

            ui.horizontal(|ui| {

                let start_response = button(
                    ui,
                    &self.theme,
                    "Align Start",
                    ControlVariant::Outline,
                    ControlSize::Md,
                    true,
                );
                let _ = tooltip(
                    &start_response,
                    ui,
                    &self.theme,
                    TooltipProps::new("Aligned to start of anchor")
                        .align(TooltipAlign::Start)
                        .show_arrow(true)
                        .delay_ms(200),
                );

                let center_response = button(
                    ui,
                    &self.theme,
                    "Align Center",
                    ControlVariant::Outline,
                    ControlSize::Md,
                    true,
                );
                let _ = tooltip(
                    &center_response,
                    ui,
                    &self.theme,
                    TooltipProps::new("Aligned to center (default)")
                        .align(TooltipAlign::Center)
                        .show_arrow(true)
                        .delay_ms(200),
                );

                let end_response = button(
                    ui,
                    &self.theme,
                    "Align End",
                    ControlVariant::Outline,
                    ControlSize::Md,
                    true,
                );
                let _ = tooltip(
                    &end_response,
                    ui,
                    &self.theme,
                    TooltipProps::new("Aligned to end of anchor")
                        .align(TooltipAlign::End)
                        .show_arrow(true)
                        .delay_ms(200),
                );
            });

            if self.show_animations {
                ui.add_space(16.0);
                ui.separator();
                ui.label(egui::RichText::new("Animation Examples").strong());
                ui.add_space(8.0);

                ui.horizontal(|ui| {

                    let fast_response = button(
                        ui,
                        &self.theme,
                        "Fast (50ms)",
                        ControlVariant::Primary,
                        ControlSize::Sm,
                        true,
                    );
                    let _ = tooltip(
                        &fast_response,
                        ui,
                        &self.theme,
                        TooltipProps::new("Fast 50ms animation")
                            .animation_duration_ms(50)
                            .delay_ms(0),
                    );

                    let default_response = button(
                        ui,
                        &self.theme,
                        "Default (140ms)",
                        ControlVariant::Primary,
                        ControlSize::Sm,
                        true,
                    );
                    let _ = tooltip(
                        &default_response,
                        ui,
                        &self.theme,
                        TooltipProps::new("Default 140ms animation (Radix default)")
                            .animation_duration_ms(140)
                            .delay_ms(0),
                    );

                    let slow_response = button(
                        ui,
                        &self.theme,
                        "Slow (300ms)",
                        ControlVariant::Primary,
                        ControlSize::Sm,
                        true,
                    );
                    let _ = tooltip(
                        &slow_response,
                        ui,
                        &self.theme,
                        TooltipProps::new("Slow 300ms animation")
                            .animation_duration_ms(300)
                            .delay_ms(0),
                    );
                });
            }

            ui.add_space(16.0);
            ui.separator();
            ui.label(egui::RichText::new("Advanced Options").strong());
            ui.add_space(8.0);

            ui.horizontal(|ui| {

                let offset_response = button(
                    ui,
                    &self.theme,
                    "Side Offset 16px",
                    ControlVariant::Ghost,
                    ControlSize::Sm,
                    true,
                );
                let _ = tooltip(
                    &offset_response,
                    ui,
                    &self.theme,
                    TooltipProps::new("16px gap from anchor")
                        .side_offset(16.0)
                        .show_arrow(true)
                        .delay_ms(100),
                );

                let wide_response = button(
                    ui,
                    &self.theme,
                    "Max Width 200px",
                    ControlVariant::Ghost,
                    ControlSize::Sm,
                    true,
                );
                let _ = tooltip(
                    &wide_response,
                    ui,
                    &self.theme,
                    TooltipProps::new(
                        "This is a longer tooltip text that will wrap to multiple lines when it reaches the maximum width of 200 pixels. This demonstrates the max_width option."
                    )
                    .max_width(200.0)
                    .show_arrow(true)
                    .delay_ms(100),
                );

                let contrast_response = button(
                    ui,
                    &self.theme,
                    "High Contrast",
                    ControlVariant::Ghost,
                    ControlSize::Sm,
                    true,
                );
                let _ = tooltip(
                    &contrast_response,
                    ui,
                    &self.theme,
                    TooltipProps::new("High contrast styling")
                        .high_contrast(true)
                        .show_arrow(true)
                        .delay_ms(100),
                );
            });

            ui.add_space(16.0);
            ui.separator();
            ui.label(egui::RichText::new("Skip Delay Demo").strong());
            ui.label("Hover one tooltip, then quickly move to another to see instant appearance");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                for i in 1..=4 {
                    let response = button(
                        ui,
                        &self.theme,
                        &format!("Button {}", i),
                        ControlVariant::Secondary,
                        ControlSize::Sm,
                        true,
                    );
                    let _ = tooltip(
                        &response,
                        ui,
                        &self.theme,
                        TooltipProps::new(format!("Tooltip for button {}", i))
                            .delay_ms(700)
                            .skip_delay_ms(300)
                            .show_arrow(true),
                    );
                }
            });

            ui.add_space(16.0);
            ui.separator();
            ui.label(format!("Button clicks: {}", self.clicks));
            });
        });
    }
}

fn main() {
    env_logger::init();
    info!("Starting tooltip example (Radix UI compatible)");

    let native_options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "Tooltip examples (Radix UI compatible)",
        native_options,
        Box::new(|_cc| Ok(Box::new(TooltipDemo::new()))),
    ) {
        error!("Failed to run example: {err}");
    }
}
