use crate::theme::{Theme, widget_visuals, widgets_from_input};
use crate::tokens::{ControlSize, input_tokens};
use egui::{Response, Stroke, TextEdit, TextStyle, Ui, WidgetText};
use log::trace;

pub struct TextareaProps<'a> {
    pub value: &'a mut String,
    pub placeholder: WidgetText,
    pub size: ControlSize,
    pub is_invalid: bool,
    pub show_counter: bool,
    pub max_len: Option<usize>,
    pub enabled: bool,
}

pub fn textarea(ui: &mut Ui, theme: &Theme, props: TextareaProps<'_>) -> Response {
    trace!(
        "Rendering textarea size={:?} invalid={} counter={} enabled={}",
        props.size, props.is_invalid, props.show_counter, props.enabled
    );

    let tokens = input_tokens(&theme.palette);
    let rounding = props.size.rounding();
    let expansion = props.size.expansion();

    let mut widgets = widgets_from_input(&tokens, rounding, expansion);
    if props.is_invalid {
        let invalid = widget_visuals(&tokens.invalid, rounding, expansion);
        widgets.inactive = invalid;
        widgets.hovered = invalid;
        widgets.active = invalid;
    }

    let visuals = theme.input(props.size);

    let response = theme.scoped(ui, widgets, |scoped_ui| {
        let mut style = scoped_ui.style().as_ref().clone();
        style
            .text_styles
            .insert(TextStyle::Body, visuals.text_style.clone());
        style.visuals.selection.bg_fill = visuals.selection_bg;
        style.visuals.selection.stroke = Stroke::new(1.0, visuals.selection_fg);
        style.visuals.override_text_color = Some(visuals.text_color);

        style.visuals.extreme_bg_color = tokens.idle.bg_fill;
        scoped_ui.set_style(style);

        let placeholder_colored = props.placeholder.clone().color(visuals.placeholder);
        let mut edit = TextEdit::multiline(props.value)
            .hint_text(placeholder_colored)
            .text_color(visuals.text_color)
            .margin(visuals.padding);
        if let Some(limit) = props.max_len {
            edit = edit.char_limit(limit);
        }
        scoped_ui.add_enabled(props.enabled, edit)
    });

    if props.show_counter {
        let count = props.value.chars().count();
        if let Some(limit) = props.max_len {
            ui.label(format!("{}/{}", count, limit));
        } else {
            ui.label(format!("{}", count));
        }
    }

    response
}
