use crate::theme::{Theme, widget_visuals, widgets_from_input};
use crate::tokens::{ControlSize, input_tokens};
use egui::{Response, Stroke, TextEdit, TextStyle, Ui, WidgetText};
use log::trace;

pub fn text_input(
    ui: &mut Ui,
    theme: &Theme,
    value: &mut String,
    placeholder: impl Into<WidgetText>,
    size: ControlSize,
    is_invalid: bool,
    enabled: bool,
) -> Response {
    trace!(
        "Rendering input size={:?} invalid={} enabled={}",
        size, is_invalid, enabled
    );

    let tokens = input_tokens(&theme.palette);
    let rounding = size.rounding();
    let expansion = size.expansion();

    let mut widgets = widgets_from_input(&tokens, rounding, expansion);
    if is_invalid {
        let invalid = widget_visuals(&tokens.invalid, rounding, expansion);
        widgets.inactive = invalid;
        widgets.hovered = invalid;
        widgets.active = invalid;
    }

    let visuals = theme.input(size);

    theme.scoped(ui, widgets, |scoped_ui| {
        let mut style = scoped_ui.style().as_ref().clone();
        style
            .text_styles
            .insert(TextStyle::Body, visuals.text_style.clone());
        style.visuals.selection.bg_fill = visuals.selection_bg;
        style.visuals.selection.stroke = Stroke::new(1.0, visuals.selection_fg);
        style.visuals.override_text_color = Some(visuals.text_color);

        style.visuals.extreme_bg_color = tokens.idle.bg_fill;
        scoped_ui.set_style(style);

        let hint = placeholder.into().color(visuals.placeholder);
        let edit = TextEdit::singleline(value)
            .frame(true)
            .hint_text(hint)
            .text_color(visuals.text_color)
            .margin(visuals.padding);
        scoped_ui.add_enabled(enabled, edit)
    })
}
