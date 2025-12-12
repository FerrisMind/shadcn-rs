#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;

use eframe::{App, Frame, egui};
use egui::{FontData, FontDefinitions, FontFamily, FontId, RichText};
use egui_shadcn::{ControlSize, SeparatorProps, Theme, ToggleVariant, separator, toggle};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

struct ToggleDemo {
    theme: Theme,
    bookmark_on: bool,
    italic_on: bool,
    italic_lg_on: bool,
    italic_outline_on: bool,
    italic_with_text_on: bool,
    italic_disabled_on: bool,
}

impl ToggleDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            bookmark_on: false,
            italic_on: false,
            italic_lg_on: false,
            italic_outline_on: false,
            italic_with_text_on: false,
            italic_disabled_on: true,
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
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "lucide".into());
    ctx.set_fonts(fonts);
    ctx.data_mut(|d| d.insert_temp(font_loaded_id, true));
}

fn lucide_icon(icon: Icon, size: f32) -> RichText {
    RichText::new(icon.unicode().to_string()).font(FontId::new(size, FontFamily::Proportional))
}

impl App for ToggleDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ensure_lucide_font(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 16.0;

                let label = RichText::new(format!("{} Bookmark", Icon::Bookmark.unicode()))
                    .font(FontId::proportional(14.0));
                let _ = toggle(
                    ui,
                    &self.theme,
                    &mut self.bookmark_on,
                    label,
                    ToggleVariant::Outline,
                    ControlSize::Sm,
                    true,
                )
                .on_hover_text("Toggle bookmark");

                ui.add_space(8.0);
                separator(ui, &self.theme, SeparatorProps::default());
                ui.add_space(8.0);

                let _ = toggle(
                    ui,
                    &self.theme,
                    &mut self.italic_on,
                    lucide_icon(Icon::Italic, 16.0),
                    ToggleVariant::Default,
                    ControlSize::Sm,
                    true,
                )
                .on_hover_text("Toggle italic");

                let _ = toggle(
                    ui,
                    &self.theme,
                    &mut self.italic_lg_on,
                    lucide_icon(Icon::Italic, 18.0),
                    ToggleVariant::Default,
                    ControlSize::Lg,
                    true,
                )
                .on_hover_text("Toggle italic");

                let _ = toggle(
                    ui,
                    &self.theme,
                    &mut self.italic_outline_on,
                    lucide_icon(Icon::Italic, 16.0),
                    ToggleVariant::Outline,
                    ControlSize::Md,
                    true,
                )
                .on_hover_text("Toggle italic");

                let with_text = RichText::new(format!("{} Italic", Icon::Italic.unicode()))
                    .font(FontId::proportional(14.0));
                let _ = toggle(
                    ui,
                    &self.theme,
                    &mut self.italic_with_text_on,
                    with_text,
                    ToggleVariant::Default,
                    ControlSize::Md,
                    true,
                )
                .on_hover_text("Toggle italic");

                let _ = toggle(
                    ui,
                    &self.theme,
                    &mut self.italic_disabled_on,
                    lucide_icon(Icon::Underline, 16.0),
                    ToggleVariant::Default,
                    ControlSize::Md,
                    false,
                )
                .on_hover_text("Toggle italic");
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Toggle example",
        options,
        Box::new(|_cc| Ok(Box::new(ToggleDemo::new()))),
    )
}
