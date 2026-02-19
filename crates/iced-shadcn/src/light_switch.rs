use iced::widget::{container, row, text};
use iced::{Background, Color, Element, Length};

use crate::theme::Theme;

/// Properties for the LightSwitch (theme toggle) component.
#[derive(Clone, Debug)]
pub struct LightSwitchProps {
    pub dark_mode: bool,
    pub disabled: bool,
}

impl LightSwitchProps {
    pub fn new(dark_mode: bool) -> Self {
        Self {
            dark_mode,
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Render a light/dark mode toggle switch.
pub fn light_switch<'a, Message: 'a + Clone>(
    props: LightSwitchProps,
    on_toggle: Message,
    theme: &Theme,
) -> Element<'a, Message> {
    let track_color = if props.dark_mode {
        theme.palette.primary
    } else {
        theme.palette.muted
    };

    let thumb_color = Color::WHITE;
    let icon = if props.dark_mode { "🌙" } else { "☀" };

    let thumb = container(text(icon).size(12))
        .width(Length::Fixed(20.0))
        .height(Length::Fixed(20.0))
        .center_x(Length::Fixed(20.0))
        .center_y(Length::Fixed(20.0))
        .style(move |_t: &iced::Theme| iced::widget::container::Style {
            background: Some(Background::Color(thumb_color)),
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        });

    let track = container(row![thumb].padding([2, 2]))
        .width(Length::Fixed(44.0))
        .height(Length::Fixed(24.0))
        .style(move |_t: &iced::Theme| iced::widget::container::Style {
            background: Some(Background::Color(track_color)),
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        });

    if props.disabled {
        return container(track)
            .style(|_t: &iced::Theme| iced::widget::container::Style {
                ..Default::default()
            })
            .into();
    }

    iced::widget::button(track)
        .on_press(on_toggle)
        .style(|_t, _s| iced::widget::button::Style {
            background: None,
            border: iced::border::Border::default(),
            ..Default::default()
        })
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_switch_props_builder() {
        let props = LightSwitchProps::new(true).disabled(false);
        assert!(props.dark_mode);
        assert!(!props.disabled);
    }

    #[test]
    fn light_switch_props_light_mode() {
        let props = LightSwitchProps::new(false);
        assert!(!props.dark_mode);
    }
}
