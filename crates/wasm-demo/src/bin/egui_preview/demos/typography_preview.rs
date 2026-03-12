use super::super::app::EguiPreviewApp;
use eframe::egui::Ui;
use egui_shadcn::{ShadcnTypographyVariant, TypographyProps, blockquote, typography};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    typography(
        ui,
        &app.theme,
        TypographyProps::new("Typography").variant(ShadcnTypographyVariant::H3),
    );
    ui.add_space(6.0);
    typography(
        ui,
        &app.theme,
        TypographyProps::new("Build polished interfaces with accessible primitives.")
            .variant(ShadcnTypographyVariant::Lead),
    );
    if !compact {
        ui.add_space(10.0);
        blockquote(
            ui,
            &app.theme,
            egui_shadcn::BlockquoteProps::new(
                "\"Small components, consistent tokens, real production UX.\"",
            ),
        );
    }
}
