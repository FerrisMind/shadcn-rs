use iced::widget::button as iced_button;
use iced::widget::{column, container};
use iced::{Alignment, Element, Length};

use crate::button::{ButtonProps, ButtonSize, ButtonVariant, button_content};
use crate::theme::Theme;
use crate::tokens::mix;

#[derive(Clone, Copy, Debug, Default)]
pub struct CollapsibleProps {
    pub disabled: bool,
    pub compact: bool,
    pub trigger_hover_highlight: bool,
}

impl CollapsibleProps {
    pub fn new() -> Self {
        Self {
            disabled: false,
            compact: false,
            trigger_hover_highlight: true,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    pub fn trigger_hover_highlight(mut self, trigger_hover_highlight: bool) -> Self {
        self.trigger_hover_highlight = trigger_hover_highlight;
        self
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CollapsibleContentProps {
    pub force_mount: bool,
}

impl CollapsibleContentProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }
}

pub fn collapsible<'a, Message: Clone + 'a, F>(
    open: bool,
    trigger: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_open_change: Option<F>,
    content_props: CollapsibleContentProps,
    props: CollapsibleProps,
    theme: &Theme,
) -> Element<'a, Message>
where
    F: Fn(bool) -> Message + 'a,
{
    let padding_y = if props.compact { 8.0 } else { 12.0 };
    let trigger_size = if props.compact {
        ButtonSize::Size1
    } else {
        ButtonSize::Size2
    };

    let on_press = on_open_change.map(|f| (f)(!open));
    let trigger: Element<'a, Message> = if props.trigger_hover_highlight {
        button_content(
            trigger,
            on_press,
            ButtonProps::new()
                .variant(ButtonVariant::Ghost)
                .size(trigger_size)
                .disabled(props.disabled),
            theme,
        )
        .into()
    } else {
        let palette = theme.palette;
        let mut button = iced_button(trigger).width(Length::Fill).padding(0);

        if let Some(message) = on_press
            && !props.disabled
        {
            button = button.on_press(message);
        }

        button
            .style(move |_theme, status| {
                let idle_color = palette.muted_foreground;
                let hover_color = mix(idle_color, iced::Color::WHITE, 0.35);

                let text_color = if props.disabled {
                    palette.muted_foreground
                } else {
                    match status {
                        iced::widget::button::Status::Hovered
                        | iced::widget::button::Status::Pressed => hover_color,
                        _ => idle_color,
                    }
                };

                iced::widget::button::Style {
                    background: None,
                    text_color,
                    border: iced::border::Border::default(),
                    shadow: iced::Shadow::default(),
                    snap: true,
                }
            })
            .into()
    };

    let mut body = column![
        container(trigger)
            .width(Length::Fill)
            .padding([padding_y, 0.0])
    ]
    .spacing(0)
    .align_x(Alignment::Start);

    if open || content_props.force_mount {
        body = body.push(container(content).width(Length::Fill));
    }

    container(body.width(Length::Fill))
        .width(Length::Fill)
        .into()
}
