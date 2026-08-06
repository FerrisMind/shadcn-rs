//! Rendering for checkbox using iced's native checkbox primitive.

use crate::iced_compat::widget::checkbox;
use crate::iced_compat::widget::text::{LineHeight, Shaping};
use crate::iced_compat::{Font, Length, Pixels};

use super::geometry;
use super::style;
use super::types::{CheckboxConfig, CheckboxState};
use crate::theme::Theme;

/// Builds the checkbox widget with shadcn-svelte-compatible state and styling.
pub fn build_checkbox<'a, Message>(
    config: CheckboxConfig,
    theme: &'a Theme,
    width: Length,
    spacing: f32,
    text_size: Option<Pixels>,
    on_press: Option<Message>,
    on_toggle: Option<Box<dyn Fn(CheckboxState) -> Message + 'a>>,
) -> checkbox::Checkbox<'a, Message>
where
    Message: Clone + 'a,
{
    let CheckboxConfig {
        state,
        variant,
        size,
        label,
        disabled,
    } = config;

    // iced's primitive is binary, so an indeterminate value is rendered as
    // checked and receives a minus glyph. The callback still emits the
    // component's controlled tri-state cycle.
    let is_checked = !matches!(state, CheckboxState::Unchecked);
    let next_state = state.cycle();

    let on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>> = if disabled {
        None
    } else {
        match (on_toggle, on_press) {
            (Some(callback), _) => Some(Box::new(move |_| callback(next_state))),
            (None, Some(message)) => Some(Box::new(move |_| message.clone())),
            (None, None) => None,
        }
    };

    let icon = checkbox::Icon {
        font: Font::with_name("Iced-Icons"),
        code_point: match state {
            CheckboxState::Indeterminate => '\u{f068}',
            CheckboxState::Unchecked | CheckboxState::Checked => '\u{f00c}',
        },
        size: Some(Pixels(
            geometry::track_size(size) - geometry::track_padding(size) * 2.0,
        )),
        line_height: LineHeight::default(),
        shaping: Shaping::Basic,
    };

    let mut widget = crate::iced_compat::widget::checkbox(is_checked)
        .width(width)
        .size(geometry::track_size(size))
        .spacing(spacing)
        .font(crate::fonts::iced_font(theme.font_pack().sans))
        .icon(icon)
        .style(move |_iced_theme, status| style::resolve_style(theme, variant, size, status))
        .on_toggle_maybe(on_toggle);

    if let Some(text_size) = text_size {
        widget = widget.text_size(text_size);
    }

    if let Some(label) = label {
        widget = widget.label(label);
    }

    widget
}
