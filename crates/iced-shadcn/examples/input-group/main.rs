use iced::advanced::text::Wrapping;
use iced::border::Border;
use iced::widget::{Space, column, container, row, text, text_editor};
use iced::{Alignment, Background, Color, Element, Font, Length, Task};

use iced_shadcn::{
    ButtonRadius, ButtonVariant, InputGroupAddonAlign, InputGroupAddonProps, InputGroupButtonProps,
    InputGroupButtonSize, InputGroupInputProps, InputGroupProps, InputGroupTextareaProps,
    InputSize, SeparatorOrientation, SeparatorProps, SeparatorSize, TextareaSize, Theme,
    input_group, input_group_addon, input_group_button, input_group_input, input_group_text,
    input_group_textarea, input_group_textarea_apply_action, separator,
};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

struct Example {
    theme: Theme,
    search_query: String,
    website: String,
    handle: String,
    note: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    SearchChanged(String),
    WebsiteChanged(String),
    HandleChanged(String),
    NoteAction(text_editor::Action),
    Noop,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            search_query: String::new(),
            website: "example.com".to_string(),
            handle: String::new(),
            note: text_editor::Content::new(),
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchChanged(value) => self.search_query = value,
            Message::WebsiteChanged(value) => self.website = value,
            Message::HandleChanged(value) => self.handle = value,
            Message::NoteAction(action) => {
                let props = InputGroupTextareaProps::new()
                    .size(TextareaSize::Size1)
                    .wrapping(Wrapping::WordOrGlyph)
                    .padding([10.0, 14.0])
                    .rows(2);
                let _ = input_group_textarea_apply_action(&mut self.note, action, props);
            }
            Message::Noop => {}
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let demos = column![
            input_group(
                vec![
                    input_group_addon(
                        lucide_icon(Icon::Search, 16.0, theme.palette.muted_foreground),
                        InputGroupAddonProps::new().align(InputGroupAddonAlign::InlineStart),
                    ),
                    input_group_input(
                        &self.search_query,
                        "Search...",
                        Some(Message::SearchChanged),
                        InputGroupInputProps::new().size(InputSize::Size2),
                        theme,
                    ),
                    input_group_addon(
                        input_group_text("12 results", theme),
                        InputGroupAddonProps::new().align(InputGroupAddonAlign::InlineEnd),
                    ),
                ],
                InputGroupProps::new(),
                theme,
            ),
            input_group(
                vec![
                    input_group_addon(
                        text("https://").size(14.0).style(move |_theme| {
                            iced::widget::text::Style {
                                color: Some(theme.palette.muted_foreground),
                            }
                        }),
                        InputGroupAddonProps::new().align(InputGroupAddonAlign::InlineStart),
                    ),
                    input_group_input(
                        &self.website,
                        "example.com",
                        Some(Message::WebsiteChanged),
                        InputGroupInputProps::new().size(InputSize::Size1),
                        theme,
                    ),
                    input_group_addon(
                        input_group_button(
                            lucide_icon(Icon::Info, 14.0, theme.palette.foreground),
                            Some(Message::Noop),
                            InputGroupButtonProps::new()
                                .size(InputGroupButtonSize::IconXs)
                                .radius(ButtonRadius::Full)
                                .variant(ButtonVariant::Soft),
                            theme,
                        ),
                        InputGroupAddonProps::new().align(InputGroupAddonAlign::InlineEnd),
                    ),
                ],
                InputGroupProps::new(),
                theme,
            ),
            input_group(
                vec![
                    input_group_textarea(
                        &self.note,
                        "Ask, Search or Chat...",
                        Some(Message::NoteAction),
                        InputGroupTextareaProps::new()
                            .size(TextareaSize::Size1)
                            .wrapping(Wrapping::WordOrGlyph)
                            .padding([10.0, 14.0])
                            .rows(2),
                        theme,
                    ),
                    input_group_addon(
                        row![
                            input_group_button(
                                lucide_icon(Icon::Plus, 14.0, theme.palette.foreground),
                                Some(Message::Noop),
                                InputGroupButtonProps::new()
                                    .size(InputGroupButtonSize::IconXs)
                                    .radius(ButtonRadius::Full)
                                    .variant(ButtonVariant::Soft),
                                theme,
                            ),
                            input_group_button(
                                input_group_text("Auto", theme),
                                Some(Message::Noop),
                                InputGroupButtonProps::new()
                                    .size(InputGroupButtonSize::Xs)
                                    .variant(ButtonVariant::Ghost),
                                theme,
                            ),
                            Space::new().width(Length::Fill),
                            input_group_text("52% used", theme),
                            separator(
                                SeparatorProps::new()
                                    .orientation(SeparatorOrientation::Vertical)
                                    .size(SeparatorSize::Size1)
                                    .length(16.0),
                                theme,
                            ),
                            input_group_button(
                                row![lucide_icon(
                                    Icon::CornerDownLeft,
                                    14.0,
                                    theme.palette.primary_foreground
                                )]
                                .align_y(Alignment::Center),
                                Some(Message::Noop),
                                InputGroupButtonProps::new()
                                    .size(InputGroupButtonSize::IconXs)
                                    .radius(ButtonRadius::Full)
                                    .variant(ButtonVariant::Default),
                                theme,
                            ),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                        InputGroupAddonProps::new().align(InputGroupAddonAlign::BlockEnd),
                    ),
                ],
                InputGroupProps::new(),
                theme,
            ),
            input_group(
                vec![
                    input_group_input(
                        &self.handle,
                        "@shadcn",
                        Some(Message::HandleChanged),
                        InputGroupInputProps::new().size(InputSize::Size2),
                        theme,
                    ),
                    input_group_addon(
                        circle_check(theme),
                        InputGroupAddonProps::new().align(InputGroupAddonAlign::InlineEnd),
                    ),
                ],
                InputGroupProps::new(),
                theme,
            ),
        ]
        .spacing(24)
        .width(Length::Fill)
        .max_width(384);

        let content = column![
            heading(theme),
            container(demos).width(Length::Fill).center_x(Length::Fill),
        ]
        .spacing(32)
        .width(Length::Fill)
        .align_x(Alignment::Start);

        container(content)
            .padding(32)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(theme.palette.background)),
                ..iced::widget::container::Style::default()
            })
            .into()
    }
}

fn heading<'a, Message: 'a>(theme: &'a Theme) -> iced::widget::Column<'a, Message> {
    column![
        text("Input Group").size(34),
        text("Display additional information or actions to an input or textarea.")
            .size(15)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }),
    ]
    .spacing(12)
}

fn circle_check<'a, Message: 'a>(theme: &'a Theme) -> iced::widget::Container<'a, Message> {
    let primary = theme.palette.primary;
    let foreground = theme.palette.primary_foreground;

    container(
        text(char::from(Icon::Check).to_string())
            .font(Font::with_name("lucide"))
            .size(12.0)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(foreground),
            }),
    )
    .width(16.0)
    .height(16.0)
    .center_x(Length::Fixed(16.0))
    .center_y(Length::Fixed(16.0))
    .style(move |_theme| iced::widget::container::Style {
        background: Some(Background::Color(primary)),
        border: Border {
            radius: 9999.0.into(),
            width: 0.0,
            color: primary,
        },
        ..iced::widget::container::Style::default()
    })
}

fn lucide_icon<'a>(icon: Icon, size: f32, color: Color) -> iced::widget::Text<'a> {
    text(char::from(icon).to_string())
        .font(Font::with_name("lucide"))
        .size(size)
        .style(move |_theme| iced::widget::text::Style { color: Some(color) })
}
