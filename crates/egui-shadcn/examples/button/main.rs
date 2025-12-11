#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use eframe::{App, Frame, NativeOptions, egui};
use egui::{FontData, FontDefinitions, FontFamily, FontId, RichText};
use egui_shadcn::{ColorPalette, ControlSize, ControlVariant, Theme, button, switch};
use log::{error, info};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

struct ButtonDemo {
    theme: Theme,
    dark_mode: bool,
    primary_clicks: u32,
    destructive_clicks: u32,
}

impl ButtonDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            dark_mode: true,
            primary_clicks: 0,
            destructive_clicks: 0,
        }
    }

    fn update_theme(&mut self) {
        let palette = if self.dark_mode {
            ColorPalette::dark()
        } else {
            ColorPalette::light()
        };
        self.theme = Theme::new(palette);
    }
}

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
        .entry(FontFamily::Name("lucide".into()))
        .or_default()
        .insert(0, "lucide".into());
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "lucide".into());
    ctx.set_fonts(fonts);
    ctx.data_mut(|d| d.insert_temp(font_loaded_id, true));
}

fn lucide_icon(icon: Icon, size: f32) -> RichText {
    RichText::new(icon.unicode().to_string()).font(FontId::new(size, FontFamily::Proportional))
}

fn apply_background(ctx: &egui::Context, dark_mode: bool) {
    let mut style = ctx.style().as_ref().clone();
    if dark_mode {
        let bg = egui::Color32::from_rgb(10, 10, 10);
        let input_bg = egui::Color32::from_rgb(21, 21, 21);
        style.visuals.window_fill = bg;
        style.visuals.panel_fill = bg;
        style.visuals.extreme_bg_color = input_bg;
        style.visuals.override_text_color = Some(egui::Color32::from_rgb(249, 249, 249));
    } else {
        let bg = egui::Color32::from_rgb(255, 255, 255);
        let input_bg = egui::Color32::from_rgb(245, 245, 245);
        style.visuals.window_fill = bg;
        style.visuals.panel_fill = bg;
        style.visuals.extreme_bg_color = input_bg;
        style.visuals.override_text_color = Some(egui::Color32::from_rgb(37, 37, 37));
    }
    ctx.set_style(style);
}

impl App for ButtonDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ensure_lucide_font(ctx);
        apply_background(ctx, self.dark_mode);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("Theme:");
                        let prev_dark = self.dark_mode;
                        let icon = if self.dark_mode {
                            Icon::Moon
                        } else {
                            Icon::Sun
                        };
                        let label = icon.unicode().to_string();
                        switch(
                            ui,
                            &self.theme,
                            &mut self.dark_mode,
                            label,
                            ControlVariant::Secondary,
                            ControlSize::Sm,
                            true,
                        );
                        if prev_dark != self.dark_mode {
                            self.update_theme();
                        }
                    });
                    ui.add_space(16.0);

                    ui.heading("Button — Variants");
                    ui.label("Primary and Destructive increment click counters");
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        let primary = button(
                            ui,
                            &self.theme,
                            "Primary",
                            ControlVariant::Primary,
                            ControlSize::Md,
                            true,
                        );
                        let secondary = button(
                            ui,
                            &self.theme,
                            "Secondary",
                            ControlVariant::Secondary,
                            ControlSize::Md,
                            true,
                        );
                        let ghost = button(
                            ui,
                            &self.theme,
                            "Ghost",
                            ControlVariant::Ghost,
                            ControlSize::Md,
                            true,
                        );
                        let outline = button(
                            ui,
                            &self.theme,
                            "Outline",
                            ControlVariant::Outline,
                            ControlSize::Md,
                            true,
                        );
                        let destructive = button(
                            ui,
                            &self.theme,
                            "Destructive",
                            ControlVariant::Destructive,
                            ControlSize::Md,
                            true,
                        );
                        let link = button(
                            ui,
                            &self.theme,
                            "Link",
                            ControlVariant::Link,
                            ControlSize::Md,
                            true,
                        );

                        if primary.clicked() {
                            self.primary_clicks += 1;
                        }
                        if destructive.clicked() {
                            self.destructive_clicks += 1;
                        }

                        let _ = secondary;
                        let _ = ghost;
                        let _ = outline;
                        let _ = link;
                    });
                    ui.label(format!(
                        "Clicks: primary = {}, destructive = {}",
                        self.primary_clicks, self.destructive_clicks
                    ));
                    ui.add_space(12.0);

                    ui.heading("Button — Sizes");
                    ui.horizontal(|ui| {
                        button(
                            ui,
                            &self.theme,
                            "Small",
                            ControlVariant::Primary,
                            ControlSize::Sm,
                            true,
                        );
                        button(
                            ui,
                            &self.theme,
                            "Medium",
                            ControlVariant::Primary,
                            ControlSize::Md,
                            true,
                        );
                        button(
                            ui,
                            &self.theme,
                            "Large",
                            ControlVariant::Primary,
                            ControlSize::Lg,
                            true,
                        );
                    });
                    ui.add_space(8.0);
                    ui.label("Icon sizes (lucide):");
                    ui.horizontal(|ui| {
                        button(
                            ui,
                            &self.theme,
                            lucide_icon(Icon::Bell, 18.0),
                            ControlVariant::Secondary,
                            ControlSize::IconSm,
                            true,
                        );
                        button(
                            ui,
                            &self.theme,
                            lucide_icon(Icon::Star, 18.0),
                            ControlVariant::Secondary,
                            ControlSize::Icon,
                            true,
                        );
                        button(
                            ui,
                            &self.theme,
                            lucide_icon(Icon::Bolt, 18.0),
                            ControlVariant::Secondary,
                            ControlSize::IconLg,
                            true,
                        );
                    });
                    ui.add_space(12.0);

                    ui.heading("Button — Disabled States");
                    ui.horizontal(|ui| {
                        button(
                            ui,
                            &self.theme,
                            "Primary disabled",
                            ControlVariant::Primary,
                            ControlSize::Md,
                            false,
                        );
                        button(
                            ui,
                            &self.theme,
                            "Outline disabled",
                            ControlVariant::Outline,
                            ControlSize::Md,
                            false,
                        );
                        button(
                            ui,
                            &self.theme,
                            "Link disabled",
                            ControlVariant::Link,
                            ControlSize::Md,
                            false,
                        );
                    });
                });
        });
    }
}

fn main() {
    env_logger::init();
    info!("Starting buttons example");

    let native_options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "egui-shadcn — buttons",
        native_options,
        Box::new(|_cc| Ok(Box::new(ButtonDemo::new()))),
    ) {
        error!("Failed to run buttons example: {err}");
    }
}
