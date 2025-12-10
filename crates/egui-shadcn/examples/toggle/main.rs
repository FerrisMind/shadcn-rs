use eframe::{App, Frame, NativeOptions, egui};
use egui::{FontData, FontDefinitions, FontFamily, FontId, RichText};
use egui_shadcn::{ControlSize, Theme, ToggleVariant, toggle};
use log::{error, info};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

struct ToggleDemo {
    theme: Theme,
    default_on: bool,
    outline_on: bool,
    icon_on: bool,
    icon_lg_on: bool,
    disabled_on: bool,
}

impl ToggleDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            default_on: false,
            outline_on: true,
            icon_on: false,
            icon_lg_on: true,
            disabled_on: true,
        }
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

fn apply_dark_background(ctx: &egui::Context) {
    let mut style = ctx.style().as_ref().clone();
    let bg = egui::Color32::from_rgb(10, 10, 10);
    let input_bg = egui::Color32::from_rgb(21, 21, 21);
    style.visuals.window_fill = bg;
    style.visuals.panel_fill = bg;
    style.visuals.extreme_bg_color = input_bg;
    ctx.set_style(style);
}

impl App for ToggleDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ensure_lucide_font(ctx);
        apply_dark_background(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Toggle — Variants");
            toggle(
                ui,
                &self.theme,
                &mut self.default_on,
                "Default",
                ToggleVariant::Default,
                ControlSize::Md,
                true,
            );
            toggle(
                ui,
                &self.theme,
                &mut self.outline_on,
                "Outline",
                ToggleVariant::Outline,
                ControlSize::Md,
                true,
            );

            ui.add_space(12.0);
            ui.heading("Toggle — Icon Sizes");
            toggle(
                ui,
                &self.theme,
                &mut self.icon_on,
                lucide_icon(Icon::Star, 18.0),
                ToggleVariant::Outline,
                ControlSize::IconSm,
                true,
            );
            toggle(
                ui,
                &self.theme,
                &mut self.icon_lg_on,
                lucide_icon(Icon::LockKeyhole, 20.0),
                ToggleVariant::Default,
                ControlSize::IconLg,
                true,
            );

            ui.add_space(12.0);
            ui.heading("Toggle — Disabled");
            toggle(
                ui,
                &self.theme,
                &mut self.disabled_on,
                "Disabled",
                ToggleVariant::Outline,
                ControlSize::Md,
                false,
            );
        });
    }
}

fn main() {
    env_logger::init();
    info!("Starting toggle example");

    let native_options = NativeOptions::default();
    if let Err(err) = eframe::run_native(
        "egui-shadcn — toggle",
        native_options,
        Box::new(|_cc| Ok(Box::new(ToggleDemo::new()))),
    ) {
        error!("Failed to run toggle example: {err}");
    }
}
