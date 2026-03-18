use iced::alignment::{Horizontal, Vertical};
use iced::widget::container::Style as ContainerStyle;
use iced::widget::scrollable::Viewport;
use iced::widget::{Id, container, stack, text};
use iced::{Background, Element, Length, Padding, Task};
use lucide_icons::Icon as LucideIcon;

use crate::button::{ButtonProps, ButtonRadius, ButtonSize, ButtonVariant, icon_button};
use crate::card::{CardProps, CardSize, CardVariant, card};
use crate::empty::{EmptyProps, empty};
use crate::scroll_area::{
    ScrollAreaProps, ScrollAreaScrollAnimation, ScrollAreaScrollAnimator,
    ScrollAreaScrollbarVisibility, ScrollAreaScrollbars, scroll_area, scroll_area_is_at_bottom,
    scroll_area_scroll_to_bottom,
};
use crate::theme::Theme;

#[derive(Clone, Debug)]
pub struct ConversationProps {
    pub width: Length,
    pub height: Length,
    pub padding: Padding,
}

impl Default for ConversationProps {
    fn default() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            padding: Padding::ZERO,
        }
    }
}

impl ConversationProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}

#[derive(Clone, Debug)]
pub struct ConversationContentProps {
    pub id: Option<Id>,
    pub content_padding: Padding,
    pub scrollbars: ScrollAreaScrollbars,
    pub scrollbar_visibility: ScrollAreaScrollbarVisibility,
    pub scrollbar_width: Option<f32>,
}

impl Default for ConversationContentProps {
    fn default() -> Self {
        Self {
            id: None,
            content_padding: Padding::from(16.0),
            scrollbars: ScrollAreaScrollbars::Vertical,
            scrollbar_visibility: ScrollAreaScrollbarVisibility::Auto,
            scrollbar_width: None,
        }
    }
}

impl ConversationContentProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    pub fn content_padding(mut self, padding: impl Into<Padding>) -> Self {
        self.content_padding = padding.into();
        self
    }

    pub fn scrollbars(mut self, scrollbars: ScrollAreaScrollbars) -> Self {
        self.scrollbars = scrollbars;
        self
    }

    pub fn scrollbar_visibility(mut self, visibility: ScrollAreaScrollbarVisibility) -> Self {
        self.scrollbar_visibility = visibility;
        self
    }

    pub fn scrollbar_width(mut self, width: f32) -> Self {
        self.scrollbar_width = Some(width);
        self
    }

    fn to_scroll_area_props(&self) -> ScrollAreaProps {
        let mut props = ScrollAreaProps::new()
            .bordered(false)
            .scrollbars(self.scrollbars)
            .scrollbar_visibility(self.scrollbar_visibility);

        if let Some(id) = self.id.clone() {
            props = props.id(id);
        }

        if let Some(width) = self.scrollbar_width {
            props = props.scrollbar_width(width);
        }

        props
    }
}

#[derive(Clone, Debug)]
pub struct ConversationEmptyStateProps<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub icon: Option<&'a str>,
}

impl<'a> ConversationEmptyStateProps<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            description: None,
            icon: None,
        }
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConversationBubbleRole {
    User,
    Assistant,
}

#[derive(Clone, Debug)]
pub struct ConversationBubbleProps {
    pub role: ConversationBubbleRole,
    pub max_width: f32,
    pub padding: Padding,
}

impl Default for ConversationBubbleProps {
    fn default() -> Self {
        Self {
            role: ConversationBubbleRole::Assistant,
            max_width: 620.0,
            padding: Padding {
                top: 10.0,
                right: 14.0,
                bottom: 10.0,
                left: 14.0,
            },
        }
    }
}

impl ConversationBubbleProps {
    pub fn new(role: ConversationBubbleRole) -> Self {
        Self {
            role,
            ..Self::default()
        }
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width.max(0.0);
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}

#[derive(Clone, Debug)]
pub struct ConversationScrollButtonProps<'a> {
    pub label: &'a str,
    pub icon: LucideIcon,
    pub button: ButtonProps,
}

impl Default for ConversationScrollButtonProps<'_> {
    fn default() -> Self {
        Self {
            label: "Jump to bottom",
            icon: LucideIcon::ArrowDown,
            button: ButtonProps::new()
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::Size1)
                .radius(ButtonRadius::Full)
                .opaque_outline(true),
        }
    }
}

impl<'a> ConversationScrollButtonProps<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    pub fn button(mut self, button: ButtonProps) -> Self {
        self.button = button;
        self
    }

    pub fn icon(mut self, icon: LucideIcon) -> Self {
        self.icon = icon;
        self
    }
}

pub fn conversation<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: ConversationProps,
    theme: &Theme,
) -> iced::widget::Container<'a, Message> {
    let palette = theme.palette;
    container(content)
        .width(props.width)
        .height(props.height)
        .padding(props.padding)
        .clip(true)
        .style(move |_| ContainerStyle {
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..ContainerStyle::default()
        })
}

pub fn conversation_content<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: ConversationContentProps,
    theme: &Theme,
) -> iced::widget::scrollable::Scrollable<'a, Message> {
    let padded = container(content)
        .width(Length::Fill)
        .height(Length::Shrink)
        .padding(props.content_padding);

    scroll_area(padded, props.to_scroll_area_props(), theme)
        .width(Length::Fill)
        .height(Length::Fill)
}

pub fn conversation_empty_state<'a, Message: 'a>(
    props: ConversationEmptyStateProps<'a>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    empty(
        EmptyProps::new(props.title)
            .description(
                props
                    .description
                    .unwrap_or("Start a conversation to see messages here"),
            )
            .icon(props.icon.unwrap_or("◌")),
        theme,
    )
}

pub fn conversation_bubble<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    props: ConversationBubbleProps,
    theme: &Theme,
) -> iced::widget::Container<'a, Message> {
    let palette = theme.palette;
    let (background, foreground, border_color) = match props.role {
        ConversationBubbleRole::User => {
            (palette.primary, palette.primary_foreground, palette.primary)
        }
        ConversationBubbleRole::Assistant => (palette.muted, palette.foreground, palette.border),
    };

    card(
        content,
        CardProps::new()
            .variant(CardVariant::Surface)
            .size(CardSize::Size1)
            .show_shadow(false)
            .background(background)
            .text_color(foreground)
            .border_color(border_color),
        theme,
    )
    .max_width(props.max_width)
    .padding(props.padding)
}

pub fn conversation_scroll_button<'a, Message: Clone + 'a>(
    on_press: Option<Message>,
    props: ConversationScrollButtonProps<'a>,
    theme: &Theme,
) -> iced::widget::button::Button<'a, Message> {
    let icon = text(char::from(props.icon).to_string())
        .font(iced::Font::with_name("lucide"))
        .size(16);
    icon_button(icon, on_press, props.button, theme)
}

pub fn conversation_overlay_scroll_button<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    button: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let layers: Vec<Element<'a, Message>> = vec![
        content.into(),
        container(button)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Bottom)
            .padding(Padding {
                top: 0.0,
                right: 0.0,
                bottom: 16.0,
                left: 0.0,
            })
            .into(),
    ];
    stack(layers).into()
}

pub fn conversation_is_at_bottom(viewport: Viewport, threshold: f32) -> bool {
    scroll_area_is_at_bottom(viewport, threshold)
}

pub fn conversation_scroll_to_bottom<Message>(id: Id) -> Task<Message> {
    scroll_area_scroll_to_bottom(id)
}

pub type ConversationScrollAnimation = ScrollAreaScrollAnimation;
pub type ConversationScrollAnimator = ScrollAreaScrollAnimator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversation_props_defaults_to_fill() {
        let props = ConversationProps::new();
        assert_eq!(props.width, Length::Fill);
        assert_eq!(props.height, Length::Fill);
        assert_eq!(props.padding, Padding::ZERO);
    }

    #[test]
    fn conversation_content_props_builder_sets_values() {
        let id = Id::new("conversation-content");
        let props = ConversationContentProps::new()
            .id(id.clone())
            .scrollbar_width(12.0)
            .scrollbars(ScrollAreaScrollbars::Both)
            .scrollbar_visibility(ScrollAreaScrollbarVisibility::Visible)
            .content_padding(Padding::from(24.0));

        assert_eq!(props.id, Some(id));
        assert_eq!(props.scrollbar_width, Some(12.0));
        assert_eq!(props.scrollbars, ScrollAreaScrollbars::Both);
        assert_eq!(
            props.scrollbar_visibility,
            ScrollAreaScrollbarVisibility::Visible
        );
        assert_eq!(props.content_padding, Padding::from(24.0));
    }

    #[test]
    fn conversation_empty_state_builder_sets_values() {
        let props = ConversationEmptyStateProps::new("No messages")
            .description("Write your first prompt")
            .icon("✉");

        assert_eq!(props.title, "No messages");
        assert_eq!(props.description, Some("Write your first prompt"));
        assert_eq!(props.icon, Some("✉"));
    }

    #[test]
    fn conversation_scroll_button_defaults() {
        let props = ConversationScrollButtonProps::new();
        assert_eq!(props.label, "Jump to bottom");
        assert_eq!(char::from(props.icon), char::from(LucideIcon::ArrowDown));
        assert_eq!(props.button.variant, ButtonVariant::Outline);
        assert_eq!(props.button.size, ButtonSize::Size1);
        assert_eq!(props.button.radius, Some(ButtonRadius::Full));
        assert!(props.button.opaque_outline);
    }

    #[test]
    fn conversation_bubble_defaults() {
        let props = ConversationBubbleProps::new(ConversationBubbleRole::User);
        assert_eq!(props.role, ConversationBubbleRole::User);
        assert_eq!(props.max_width, 620.0);
        assert_eq!(
            props.padding,
            Padding {
                top: 10.0,
                right: 14.0,
                bottom: 10.0,
                left: 14.0,
            }
        );
    }

    #[test]
    fn conversation_scroll_animation_builder_sets_values() {
        let animation = ConversationScrollAnimation::new()
            .enabled(false)
            .speed_px_per_sec(3000.0)
            .tick_ms(24)
            .settle_distance_px(4.0);
        assert!(!animation.enabled);
        assert_eq!(animation.speed_px_per_sec, 3000.0);
        assert_eq!(animation.tick_ms, 24);
        assert_eq!(animation.settle_distance_px, 4.0);
    }

    #[test]
    fn conversation_scroll_animator_default_is_inactive() {
        let animator = ConversationScrollAnimator::default();
        assert!(!animator.is_active());
    }
}
