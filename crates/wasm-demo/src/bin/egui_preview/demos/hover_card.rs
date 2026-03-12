use super::super::app::EguiPreviewApp;
use eframe::egui::{RichText, Sense, Ui, vec2};
use egui_shadcn::{
    AvatarProps, AvatarSize, ControlSize, ControlVariant, HoverCardProps, avatar, button,
    hover_card, icon_calendar,
};

pub fn render(app: &mut EguiPreviewApp, ui: &mut Ui, compact: bool) {
    let hover_id = ui.id().with("preview-hover-card");
    let _ = hover_card(
        ui,
        &app.theme,
        HoverCardProps::new(hover_id).width(if compact { 240.0 } else { 320.0 }),
        |trigger_ui| {
            button(
                trigger_ui,
                &app.theme,
                "@nextjs",
                ControlVariant::Link,
                ControlSize::Md,
                true,
            )
        },
        |content_ui| {
            content_ui.horizontal(|row| {
                row.spacing_mut().item_spacing.x = 12.0;
                avatar(
                    row,
                    &app.theme,
                    AvatarProps::new("VC").size(AvatarSize::Size5),
                );
                row.vertical(|text| {
                    text.label(RichText::new("@nextjs").strong());
                    if !compact {
                        text.label(
                            RichText::new("The React Framework by Vercel")
                                .size(12.0)
                                .color(app.theme.palette.muted_foreground),
                        );
                    }
                    text.horizontal(|meta| {
                        let size = 12.0;
                        let (rect, _resp) =
                            meta.allocate_exact_size(vec2(size, size), Sense::hover());
                        icon_calendar(
                            meta.painter(),
                            rect.center(),
                            size,
                            app.theme.palette.muted_foreground,
                        );
                        meta.label(
                            RichText::new("Joined December 2021")
                                .size(11.0)
                                .color(app.theme.palette.muted_foreground),
                        );
                    });
                });
            });
        },
    );
}
