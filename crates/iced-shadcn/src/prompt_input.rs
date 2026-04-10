use iced::advanced::text::Wrapping;
use iced::alignment::Horizontal;
use iced::widget::text as iced_text;
use iced::widget::text_editor;
use iced::widget::{column, container, row};
use iced::{Element, Length, Padding};
use lucide_icons::Icon as LucideIcon;

use crate::button::{ButtonProps, ButtonRadius, ButtonSize, ButtonVariant, icon_button};
use crate::card::{CardProps, CardSize, CardVariant, card};
use crate::input_group::{InputGroupButtonProps, InputGroupButtonSize, input_group_button};
use crate::scroll_area::{
    ScrollAreaProps, ScrollAreaScrollbarVisibility, ScrollAreaScrollbars, scroll_area,
};
use crate::textarea::{TextareaProps, TextareaResize, TextareaSize, TextareaVariant, textarea};
use crate::theme::Theme;
use crate::typography::{TextProps, TextSize, text};

#[derive(Clone, Debug)]
pub struct PromptInputFloatingProps<'a> {
    pub placeholder: &'a str,
    pub max_width: f32,
    pub horizontal_inset: f32,
    pub bottom_inset: f32,
    pub can_submit: bool,
    pub is_loading: bool,
    pub textarea: TextareaProps,
    pub root_radius: f32,
}

impl<'a> Default for PromptInputFloatingProps<'a> {
    fn default() -> Self {
        Self {
            placeholder: "Ask anything",
            max_width: 768.0,
            horizontal_inset: 12.0,
            bottom_inset: 12.0,
            can_submit: false,
            is_loading: false,
            textarea: TextareaProps::new()
                .size(TextareaSize::Size1)
                .variant(TextareaVariant::Surface)
                .resize(TextareaResize::Vertical)
                .wrapping(Wrapping::WordOrGlyph)
                .borderless(true)
                .rows(2)
                .max_rows(12),
            root_radius: 24.0,
        }
    }
}

impl<'a> PromptInputFloatingProps<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width.max(320.0);
        self
    }

    pub fn horizontal_inset(mut self, inset: f32) -> Self {
        self.horizontal_inset = inset.max(0.0);
        self
    }

    pub fn bottom_inset(mut self, inset: f32) -> Self {
        self.bottom_inset = inset.max(0.0);
        self
    }

    pub fn can_submit(mut self, can_submit: bool) -> Self {
        self.can_submit = can_submit;
        self
    }

    pub fn is_loading(mut self, is_loading: bool) -> Self {
        self.is_loading = is_loading;
        self
    }

    pub fn textarea(mut self, textarea: TextareaProps) -> Self {
        self.textarea = textarea;
        self
    }

    pub fn root_radius(mut self, radius: f32) -> Self {
        self.root_radius = radius.max(0.0);
        self
    }
}

#[derive(Clone, Debug)]
pub struct PromptInputFloatingActions<Message: Clone> {
    pub add: Option<Message>,
    pub search: Option<Message>,
    pub more: Option<Message>,
    pub mic: Option<Message>,
    pub submit: Option<Message>,
    pub stop: Option<Message>,
}

impl<Message: Clone> Default for PromptInputFloatingActions<Message> {
    fn default() -> Self {
        Self {
            add: None,
            search: None,
            more: None,
            mic: None,
            submit: None,
            stop: None,
        }
    }
}

impl<Message: Clone> PromptInputFloatingActions<Message> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_action(mut self, message: Option<Message>) -> Self {
        self.add = message;
        self
    }

    pub fn search_action(mut self, message: Option<Message>) -> Self {
        self.search = message;
        self
    }

    pub fn more_action(mut self, message: Option<Message>) -> Self {
        self.more = message;
        self
    }

    pub fn mic_action(mut self, message: Option<Message>) -> Self {
        self.mic = message;
        self
    }

    pub fn submit_action(mut self, message: Option<Message>) -> Self {
        self.submit = message;
        self
    }

    pub fn stop_action(mut self, message: Option<Message>) -> Self {
        self.stop = message;
        self
    }
}

pub fn prompt_input_floating<'a, Message: Clone + 'a, F>(
    content: &'a text_editor::Content,
    on_action: Option<F>,
    actions: PromptInputFloatingActions<Message>,
    props: PromptInputFloatingProps<'a>,
    theme: &Theme,
) -> Element<'a, Message>
where
    F: Fn(text_editor::Action) -> Message + 'a,
{
    let max_rows = props.textarea.max_rows.unwrap_or(12).max(1);
    let mut textarea_props = props.textarea;
    textarea_props.max_rows = None;
    let show_scrollbar = content.line_count() > max_rows;
    let textarea_height = textarea_visible_height(content, &textarea_props, max_rows);
    let textarea_widget = container(
        scroll_area(
            prompt_textarea(content, props.placeholder, on_action, textarea_props, theme),
            ScrollAreaProps::new()
                .bordered(false)
                .scrollbars(ScrollAreaScrollbars::Vertical)
                .scrollbar_visibility(if show_scrollbar {
                    ScrollAreaScrollbarVisibility::Visible
                } else {
                    ScrollAreaScrollbarVisibility::Hidden
                }),
            theme,
        )
        .height(Length::Fixed(textarea_height)),
    )
    .padding(Padding {
        top: 6.0,
        right: 6.0,
        bottom: 0.0,
        left: 6.0,
    })
    .width(Length::Fill);

    let left_actions = row![
        action_icon_button(LucideIcon::Plus, actions.add, theme),
        search_button(actions.search, theme),
        action_icon_button(LucideIcon::Ellipsis, actions.more, theme),
    ]
    .spacing(8.0)
    .align_y(iced::Alignment::Center);

    let submit_message = if props.is_loading {
        actions.stop
    } else if props.can_submit {
        actions.submit
    } else {
        None
    };
    let right_actions = row![
        action_icon_button(LucideIcon::Mic, actions.mic, theme),
        submit_button(submit_message, props.can_submit, props.is_loading, theme),
    ]
    .spacing(8.0)
    .align_y(iced::Alignment::Center);

    let toolbar = row![
        left_actions,
        container(right_actions)
            .width(Length::Fill)
            .align_x(Horizontal::Right),
    ]
    .align_y(iced::Alignment::Center);

    let body = column![textarea_widget, container(toolbar).padding([8.0, 8.0]),]
        .spacing(0.0)
        .width(Length::Fill);

    let palette = theme.palette;
    let root = card(
        body,
        CardProps::new()
            .variant(CardVariant::Surface)
            .size(CardSize::Size1)
            .padding(0.0)
            .radius(props.root_radius)
            .background(palette.popover)
            .text_color(palette.foreground)
            .border_color(palette.input),
        theme,
    );
    let floating = container(root)
        .width(Length::Fill)
        .max_width(props.max_width + props.horizontal_inset * 2.0)
        .align_x(Horizontal::Center);

    column![
        container(floating)
            .padding([0.0, props.horizontal_inset])
            .width(Length::Fill)
            .align_x(Horizontal::Center),
        container(iced_text(""))
            .height(Length::Fixed(props.bottom_inset))
            .width(Length::Fill),
    ]
    .width(Length::Fill)
    .into()
}

fn action_icon_button<'a, Message: Clone + 'a>(
    icon: LucideIcon,
    message: Option<Message>,
    theme: &Theme,
) -> Element<'a, Message> {
    input_group_button(
        lucide_icon(icon, 18.0),
        message,
        InputGroupButtonProps::new()
            .variant(ButtonVariant::Ghost)
            .size(InputGroupButtonSize::IconSm)
            .radius(ButtonRadius::Full),
        theme,
    )
}

fn search_button<'a, Message: Clone + 'a>(
    message: Option<Message>,
    theme: &Theme,
) -> Element<'a, Message> {
    input_group_button(
        row![
            lucide_icon(LucideIcon::Globe, 18.0),
            text("Search", TextProps::new().size(TextSize::Size2), theme),
        ]
        .spacing(6.0)
        .align_y(iced::Alignment::Center),
        message,
        InputGroupButtonProps::new()
            .variant(ButtonVariant::Ghost)
            .size(InputGroupButtonSize::Sm)
            .radius(ButtonRadius::Full),
        theme,
    )
}

fn prompt_textarea<'a, Message: Clone + 'a, F>(
    content: &'a text_editor::Content,
    placeholder: &'a str,
    on_action: Option<F>,
    textarea_props: TextareaProps,
    theme: &Theme,
) -> Element<'a, Message>
where
    F: Fn(text_editor::Action) -> Message + 'a,
{
    textarea(content, placeholder, on_action, textarea_props, theme).into()
}

fn textarea_visible_height(
    content: &text_editor::Content,
    props: &TextareaProps,
    max_rows: usize,
) -> f32 {
    let text_size = match props.size {
        TextareaSize::Size1 | TextareaSize::Size2 => 14.0,
        TextareaSize::Size3 => 16.0,
    };
    let padding = props.padding.unwrap_or(match props.size {
        TextareaSize::Size1 => [6.0, 10.0],
        TextareaSize::Size2 => [8.0, 12.0],
        TextareaSize::Size3 => [10.0, 14.0],
    });
    let min_rows = props.rows.unwrap_or(1).max(1);
    let visible_rows = content
        .line_count()
        .max(min_rows)
        .min(max_rows.max(min_rows)) as f32;
    let line_height = text_size * 1.4;
    line_height * visible_rows + padding[0] * 2.0
}

fn submit_button<'a, Message: Clone + 'a>(
    message: Option<Message>,
    can_submit: bool,
    is_loading: bool,
    theme: &Theme,
) -> iced::widget::button::Button<'a, Message> {
    let content: Element<'a, Message> = if is_loading {
        lucide_icon(LucideIcon::Square, 14.0)
    } else {
        lucide_icon(LucideIcon::ArrowUp, 18.0)
    };

    icon_button(
        content,
        message,
        ButtonProps::new()
            .variant(ButtonVariant::Default)
            .size(ButtonSize::Size1)
            .radius(ButtonRadius::Full)
            .disabled(!is_loading && !can_submit),
        theme,
    )
}

fn lucide_icon<'a, Message: 'a>(icon: LucideIcon, size: f32) -> Element<'a, Message> {
    iced_text(char::from(icon).to_string())
        .font(iced::Font::with_name("lucide"))
        .size(size)
        .line_height(iced::widget::text::LineHeight::Absolute(size.into()))
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floating_props_defaults_match_prompt_input_shape() {
        let props = PromptInputFloatingProps::new();
        assert_eq!(props.placeholder, "Ask anything");
        assert!(props.max_width >= 768.0);
        assert!(!props.can_submit);
        assert!(!props.is_loading);
        assert_eq!(props.textarea.rows, Some(2));
        assert_eq!(props.root_radius, 24.0);
    }

    #[test]
    fn floating_actions_builder_sets_messages() {
        let actions = PromptInputFloatingActions::new()
            .add_action(Some(1))
            .search_action(Some(2))
            .more_action(Some(3))
            .mic_action(Some(4))
            .submit_action(Some(5))
            .stop_action(Some(6));

        assert_eq!(actions.add, Some(1));
        assert_eq!(actions.search, Some(2));
        assert_eq!(actions.more, Some(3));
        assert_eq!(actions.mic, Some(4));
        assert_eq!(actions.submit, Some(5));
        assert_eq!(actions.stop, Some(6));
    }
}
