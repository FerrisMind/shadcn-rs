#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use eframe::{App, Frame, NativeOptions, egui};
use egui::{FontData, FontDefinitions, FontFamily};
use egui_shadcn::{ColorPalette, ControlSize, ControlVariant, Theme, switch};
use log::{error, info};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

struct TextareaDemo {
    theme: Theme,
    dark_mode: bool,
    basic_text: String,
    invalid_text: String,
    limited_text: String,
    disabled_text: String,
    no_counter_text: String,
}

impl TextareaDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            dark_mode: true,
            basic_text: "Basic input".into(),
            invalid_text: String::new(),
            limited_text: "Character counter enabled".into(),
            disabled_text: "Field is locked".into(),
            no_counter_text: "No counter".into(),
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
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "lucide".into());
    ctx.set_fonts(fonts);
    ctx.data_mut(|d| d.insert_temp(font_loaded_id, true));
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

impl App for TextareaDemo {
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

                    ui.heading("Textarea — Basic Field");
                    egui_shadcn::Textarea::new("basic_textarea")
                        .placeholder("Enter text")
                        .size(ControlSize::Md.into())
                        .show(ui, &self.theme, &mut self.basic_text);

                    ui.add_space(32.0);

                    ui.heading("Textarea — Counter and Limit");
                    egui_shadcn::Textarea::new("limited_textarea")
                        .placeholder("Max 120 characters")
                        .size(ControlSize::Lg.into())
                        .show_counter(true)
                        .max_len(120)
                        .show(ui, &self.theme, &mut self.limited_text);

                    ui.add_space(32.0);

                    ui.heading("Textarea — invalid");
                    egui_shadcn::Textarea::new("invalid_textarea")
                        .placeholder("Required field")
                        .size(ControlSize::Md.into())
                        .invalid(true)
                        .show_counter(true)
                        .max_len(60)
                        .show(ui, &self.theme, &mut self.invalid_text);

                    ui.add_space(32.0);

                    ui.heading("Textarea — No Counter (Sm)");
                    egui_shadcn::Textarea::new("sm_textarea")
                        .placeholder("Compact input")
                        .size(ControlSize::Sm.into())
                        .show(ui, &self.theme, &mut self.no_counter_text);

                    ui.add_space(32.0);

                    ui.heading("Textarea — Disabled");
                    egui_shadcn::Textarea::new("disabled_textarea")
                        .placeholder("Disabled")
                        .size(ControlSize::Md.into())
                        .enabled(false)
                        .show(ui, &self.theme, &mut self.disabled_text);
                });
        });
    }
}

fn main() {
    env_logger::init();
    info!("Starting textarea example");

    let native_options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "egui-shadcn — textarea",
        native_options,
        Box::new(|_cc| Ok(Box::new(TextareaDemo::new()))),
    ) {
        error!("Failed to run textarea example: {err}");
    }
}
