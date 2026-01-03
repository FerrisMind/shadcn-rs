#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{Sense, vec2};
use egui_shadcn::{
    AvatarProps, AvatarSize, ControlSize, ControlVariant, HoverCardProps, Theme, avatar, button,
    hover_card, icon_calendar,
};

struct HoverCardDemo {
    theme: Theme,
}

impl HoverCardDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

impl App for HoverCardDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(80.0);

                let _ = hover_card(
                    ui,
                    &self.theme,
                    HoverCardProps::new(ui.make_persistent_id("hover-card-demo")).with_width(320.0),
                    |trigger_ui| {
                        button(
                            trigger_ui,
                            &self.theme,
                            "@nextjs",
                            ControlVariant::Link,
                            ControlSize::Md,
                            true,
                        )
                    },
                    |content_ui| {
                        content_ui.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 16.0;
                            avatar(
                                row,
                                &self.theme,
                                AvatarProps::new("VC").size(AvatarSize::Size5),
                            );

                            row.vertical(|text| {
                                text.spacing_mut().item_spacing.y = 4.0;
                                text.label(egui::RichText::new("@nextjs").size(14.0).strong());
                                text.label(
                                    egui::RichText::new(
                                        "The React Framework - created and maintained by @vercel.",
                                    )
                                    .size(12.0)
                                    .color(self.theme.palette.muted_foreground),
                                );

                                text.add_space(8.0);
                                text.horizontal(|meta| {
                                    meta.spacing_mut().item_spacing.x = 6.0;
                                    let icon_size = 14.0;
                                    let (rect, _resp) = meta.allocate_exact_size(
                                        vec2(icon_size, icon_size),
                                        Sense::hover(),
                                    );
                                    icon_calendar(
                                        meta.painter(),
                                        rect.center(),
                                        icon_size,
                                        self.theme.palette.muted_foreground,
                                    );
                                    meta.label(
                                        egui::RichText::new("Joined December 2021")
                                            .size(11.0)
                                            .color(self.theme.palette.muted_foreground),
                                    );
                                });
                            });
                        });
                    },
                );
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Hover Card example",
        options,
        Box::new(|_cc| Ok(Box::new(HoverCardDemo::new()))),
    )
}
