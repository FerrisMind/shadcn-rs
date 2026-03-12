mod app;
mod catalog;
mod demos;
mod route;
mod ui_component;
mod ui_home;

pub use app::EguiPreviewApp;

use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use egui_extras::install_image_loaders;
use lucide_icons::LUCIDE_FONT_BYTES;

pub fn ensure_lucide_font(ctx: &egui::Context) {
    let font_loaded_id = egui::Id::new("lucide_font_loaded_preview");
    let already_set = ctx.data(|d| d.get_temp::<bool>(font_loaded_id).unwrap_or(false));
    if already_set {
        return;
    }

    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "lucide".into(),
        FontData::from_static(LUCIDE_FONT_BYTES).into(),
    );
    let lucide = "lucide".to_owned();
    fonts
        .families
        .entry(FontFamily::Name(lucide.clone().into()))
        .or_default()
        .push(lucide.clone());

    // Make lucide available as a fallback for standard text styles.
    // This keeps normal text rendering intact and enables icon glyphs in components
    // that resolve to proportional/monospace text.
    for family in [FontFamily::Proportional, FontFamily::Monospace] {
        let family_fonts = fonts.families.entry(family).or_default();
        if !family_fonts.iter().any(|name| name == &lucide) {
            family_fonts.push(lucide.clone());
        }
    }
    ctx.set_fonts(fonts);
    ctx.data_mut(|d| d.insert_temp(font_loaded_id, true));
}

pub fn ensure_image_loaders(ctx: &egui::Context) {
    let loaders_installed_id = egui::Id::new("egui_image_loaders_installed_preview");
    let already_set = ctx.data(|d| d.get_temp::<bool>(loaders_installed_id).unwrap_or(false));
    if already_set {
        return;
    }

    install_image_loaders(ctx);
    ctx.data_mut(|d| d.insert_temp(loaders_installed_id, true));
}
