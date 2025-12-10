use crate::theme::{Theme, widget_visuals};
use crate::tokens::{ControlSize, ControlVariant, checkbox_metrics, checkbox_tokens};
use egui::style::Widgets;
use egui::{CornerRadius, Response, TextStyle, Ui, WidgetText};
use log::trace;

pub fn checkbox(
    ui: &mut Ui,
    theme: &Theme,
    checked: &mut bool,
    label: impl Into<WidgetText>,
    variant: ControlVariant,
    size: ControlSize,
    enabled: bool,
) -> Response {
    trace!(
        "Rendering checkbox variant={:?} size={:?} enabled={}",
        variant, size, enabled
    );
    let visuals = theme.control(variant, size);
    let metrics = checkbox_metrics(size);
    let toggle_tokens = checkbox_tokens(&theme.palette, variant);
    let rounding = CornerRadius::same(4);
    let expansion = size.expansion();
    let widgets = Widgets {
        noninteractive: widget_visuals(&toggle_tokens.disabled, rounding, expansion),
        inactive: widget_visuals(&toggle_tokens.off.idle, rounding, expansion),
        hovered: widget_visuals(&toggle_tokens.off.hovered, rounding, expansion),
        active: widget_visuals(&toggle_tokens.on.active, rounding, expansion),
        open: widget_visuals(&toggle_tokens.off.hovered, rounding, expansion),
    };

    theme.scoped(ui, widgets, |scoped_ui| {
        let mut style = scoped_ui.style().as_ref().clone();
        style.spacing.icon_width = metrics.track_size.x;
        style.spacing.icon_width_inner = metrics.thumb_size.x;
        style.spacing.icon_spacing = visuals.padding.x * 0.35;
        style.spacing.item_spacing = visuals.padding * 0.25;
        style
            .text_styles
            .insert(TextStyle::Body, visuals.text_style.clone());
        style.visuals.selection.bg_fill = toggle_tokens.on.idle.bg_fill;
        style.visuals.selection.stroke = toggle_tokens.on.idle.fg_stroke;
        scoped_ui.set_style(style);

        scoped_ui.add_enabled(enabled, egui::Checkbox::new(checked, label))
    })
}
