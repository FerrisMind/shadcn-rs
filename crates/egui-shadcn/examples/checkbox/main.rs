use eframe::{App, Frame, NativeOptions, egui};
use egui::{FontData, FontDefinitions, FontFamily};
use egui_shadcn::{
    CheckboxCycle, CheckboxOptions, CheckboxState, ColorPalette, ControlSize, ControlVariant,
    Theme, checkbox, checkbox_state, switch,
};
use log::{error, info};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

struct CheckboxDemo {
    theme: Theme,
    dark_mode: bool,
    primary_checked: bool,
    secondary_checked: bool,
    ghost_checked: bool,
    outline_checked: bool,
    destructive_checked: bool,
    sm_checked: bool,
    md_checked: bool,
    lg_checked: bool,
    disabled_checked: bool,
    tri_state: CheckboxState,
    invalid_state: CheckboxState,
}

impl CheckboxDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            dark_mode: true,
            primary_checked: false,
            secondary_checked: true,
            ghost_checked: false,
            outline_checked: true,
            destructive_checked: false,
            sm_checked: false,
            md_checked: true,
            lg_checked: false,
            disabled_checked: true,
            tri_state: CheckboxState::Indeterminate,
            invalid_state: CheckboxState::Checked,
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

impl App for CheckboxDemo {
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
                        let icon = if self.dark_mode { Icon::Moon } else { Icon::Sun };
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

                    ui.heading("Checkbox — Variants");
                    ui.label("Demonstration of all ControlVariant options.");
                    ui.add_space(8.0);
                    checkbox(
                        ui,
                        &self.theme,
                        &mut self.primary_checked,
                        "Primary",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        true,
                    );
                    checkbox(
                        ui,
                        &self.theme,
                        &mut self.secondary_checked,
                        "Secondary",
                        ControlVariant::Secondary,
                        ControlSize::Md,
                        true,
                    );
                    checkbox(
                        ui,
                        &self.theme,
                        &mut self.ghost_checked,
                        "Ghost",
                        ControlVariant::Ghost,
                        ControlSize::Md,
                        true,
                    );
                    checkbox(
                        ui,
                        &self.theme,
                        &mut self.outline_checked,
                        "Outline",
                        ControlVariant::Outline,
                        ControlSize::Md,
                        true,
                    );
                    checkbox(
                        ui,
                        &self.theme,
                        &mut self.destructive_checked,
                        "Destructive",
                        ControlVariant::Destructive,
                        ControlSize::Md,
                        true,
                    );
                    ui.add_space(12.0);
                    ui.heading("Checkbox — Tri-state & invalid");
                    checkbox_state(
                        ui,
                        &self.theme,
                        &mut self.tri_state,
                        "Tri-state (unchecked → checked → indeterminate)",
                        CheckboxOptions {
                            variant: ControlVariant::Secondary,
                            size: ControlSize::Md,
                            enabled: true,
                            cycle: CheckboxCycle::TriState,
                            ..CheckboxOptions::default()
                        },
                    );
                    checkbox_state(
                        ui,
                        &self.theme,
                        &mut self.tri_state,
                        "Tri-state (reusable state)",
                        CheckboxOptions {
                            variant: ControlVariant::Ghost,
                            size: ControlSize::Sm,
                            enabled: true,
                            cycle: CheckboxCycle::TriState,
                            ..CheckboxOptions::default()
                        },
                    );
                    checkbox_state(
                        ui,
                        &self.theme,
                        &mut self.invalid_state,
                        "Invalid ring",
                        CheckboxOptions {
                            variant: ControlVariant::Secondary,
                            size: ControlSize::Md,
                            enabled: true,
                            invalid: true,
                            ..CheckboxOptions::default()
                        },
                    )
                    .on_hover_text("Invalid state mirrors shadcn ring color");
                    ui.add_space(12.0);

                    ui.heading("Checkbox — Sizes");
                    checkbox(
                        ui,
                        &self.theme,
                        &mut self.sm_checked,
                        "Small",
                        ControlVariant::Primary,
                        ControlSize::Sm,
                        true,
                    );
                    checkbox(
                        ui,
                        &self.theme,
                        &mut self.md_checked,
                        "Medium",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        true,
                    );
                    checkbox(
                        ui,
                        &self.theme,
                        &mut self.lg_checked,
                        "Large",
                        ControlVariant::Primary,
                        ControlSize::Lg,
                        true,
                    );
                    ui.add_space(12.0);

                    ui.heading("Checkbox — Disabled");
                    checkbox(
                        ui,
                        &self.theme,
                        &mut self.disabled_checked,
                        "Disabled",
                        ControlVariant::Secondary,
                        ControlSize::Md,
                        false,
                    );
                });
        });
    }
}

fn main() {
    env_logger::init();
    info!("Starting checkbox example");

    let native_options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "egui-shadcn — checkbox",
        native_options,
        Box::new(|_cc| Ok(Box::new(CheckboxDemo::new()))),
    ) {
        error!("Failed to run checkbox example: {err}");
    }
}
