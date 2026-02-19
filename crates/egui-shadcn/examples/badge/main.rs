#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CentralPanel, Color32, FontData, FontDefinitions, FontFamily};
use egui_shadcn::{BadgeProps, BadgeSize, BadgeVariant, Theme, badge};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

fn ensure_lucide_font(ctx: &egui::Context) {
    let font_loaded_id = egui::Id::new("lucide_font_loaded");
    let already_set = ctx.data(|d| d.get_temp::<bool>(font_loaded_id).unwrap_or(false));
    if already_set {
        return;
    }
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "lucide".into(),
        FontData::from_static(LUCIDE_FONT_BYTES).into(),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "lucide".into());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "lucide".into());
    ctx.set_fonts(fonts);
    ctx.data_mut(|d| d.insert_temp(font_loaded_id, true));
}

struct BadgeExample {
    theme: Theme,
}

impl BadgeExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

impl App for BadgeExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ensure_lucide_font(ctx);
        screenshot::apply_screenshot_scale(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Badge Component");
            ui.add_space(16.0);

            // -- Sizes --
            ui.label(egui::RichText::new("Sizes").strong());
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Size 1").size(BadgeSize::Size1),
                );
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Size 2").size(BadgeSize::Size2),
                );
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Size 3").size(BadgeSize::Size3),
                );
            });

            ui.add_space(16.0);

            // -- Variants --
            ui.label(egui::RichText::new("Variants").strong());
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Default").variant(BadgeVariant::Default),
                );
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Secondary").variant(BadgeVariant::Secondary),
                );
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Outline").variant(BadgeVariant::Outline),
                );
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Destructive").variant(BadgeVariant::Destructive),
                );
            });

            ui.add_space(16.0);

            // -- Custom Colors --
            ui.label(egui::RichText::new("Custom Colors").strong());
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Error")
                        .variant(BadgeVariant::Default)
                        .color(Color32::from_rgb(239, 68, 68)),
                );
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Success")
                        .variant(BadgeVariant::Default)
                        .color(Color32::from_rgb(34, 197, 94)),
                );
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Warning")
                        .variant(BadgeVariant::Default)
                        .color(Color32::from_rgb(245, 158, 11)),
                );
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Info")
                        .variant(BadgeVariant::Default)
                        .color(Color32::from_rgb(59, 130, 246)),
                );
            });

            ui.add_space(16.0);

            // -- High Contrast --
            ui.label(egui::RichText::new("High Contrast").strong());
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Normal").variant(BadgeVariant::Default),
                );
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("High Contrast")
                        .variant(BadgeVariant::Default)
                        .high_contrast(true),
                );
            });

            ui.add_space(16.0);

            // -- Icons --
            ui.label(egui::RichText::new("With Icons").strong());
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Package").icon(Icon::Package),
                );
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Available")
                        .variant(BadgeVariant::Secondary)
                        .icon(Icon::Check),
                );
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Warning")
                        .variant(BadgeVariant::Outline)
                        .color(Color32::from_rgb(245, 158, 11))
                        .icon(Icon::TriangleAlert),
                );
            });

            ui.add_space(16.0);

            // -- Link Badge (href) --
            ui.label(egui::RichText::new("Link Badge (href)").strong());
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Visit shadcn-rs")
                        .variant(BadgeVariant::Default)
                        .href("https://github.com/nicepkg/shadcn-rs"),
                );
                badge(
                    ui,
                    &self.theme,
                    BadgeProps::new("Documentation")
                        .variant(BadgeVariant::Outline)
                        .href("https://docs.rs/egui-shadcn"),
                );
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Badge example",
        options,
        Box::new(|_cc| Ok(Box::new(BadgeExample::new()))),
    )
}
