use iced::widget::{column, container, row, text as iced_text, text_input};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    InputGroupAddonAlign, InputGroupProps, KbdProps, KbdSize, Theme, input_group,
    input_group_addon, input_group_control, kbd,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    search_value: String,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::SearchChanged(value) => {
                self.search_value = value;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let content = column![
            // Header
            iced_text("Kbd with Input Groups")
                .size(24)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(theme.palette.foreground),
                }),
            iced_text("Keyboard shortcuts displayed alongside input fields")
                .size(14)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                }),
            // Section: Search with Keyboard Shortcut
            section_header("Search with Shortcut", theme),
            input_group(
                vec![
                    input_group_control(
                        text_input("Search...", &self.search_value)
                            .on_input(Message::SearchChanged)
                            .padding([8, 12])
                    ),
                    input_group_addon(
                        row![
                            kbd("⌘", KbdProps::new().size(KbdSize::One), theme),
                            kbd("K", KbdProps::new().size(KbdSize::One), theme),
                        ]
                        .spacing(2),
                        iced_shadcn::InputGroupAddonProps::new()
                            .align(InputGroupAddonAlign::InlineEnd),
                    ),
                ],
                InputGroupProps::new(),
                theme,
            ),
            // Section: Command Palette Style
            section_header("Command Palette Style", theme),
            input_group(
                vec![
                    input_group_control(text_input("Type a command...", "").padding([10, 14])),
                    input_group_addon(
                        row![
                            kbd("ESC", KbdProps::new().size(KbdSize::One), theme),
                            iced_text("to close")
                                .size(12)
                                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                                    color: Some(theme.palette.muted_foreground),
                                }),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                        iced_shadcn::InputGroupAddonProps::new()
                            .align(InputGroupAddonAlign::InlineEnd),
                    ),
                ],
                InputGroupProps::new(),
                theme,
            ),
            // Section: Multiple Actions
            section_header("Multiple Actions", theme),
            input_group(
                vec![
                    input_group_addon(
                        iced_text("⌘").size(14).style(|_theme: &iced::Theme| {
                            iced::widget::text::Style {
                                color: Some(theme.palette.muted_foreground),
                            }
                        }),
                        iced_shadcn::InputGroupAddonProps::new()
                            .align(InputGroupAddonAlign::InlineStart),
                    ),
                    input_group_control(text_input("Quick action...", "").padding([8, 12])),
                    input_group_addon(
                        row![
                            kbd("↵", KbdProps::new().size(KbdSize::One), theme),
                            iced_text("to run").size(12).style(|_theme: &iced::Theme| {
                                iced::widget::text::Style {
                                    color: Some(theme.palette.muted_foreground),
                                }
                            }),
                        ]
                        .spacing(6)
                        .align_y(Alignment::Center),
                        iced_shadcn::InputGroupAddonProps::new()
                            .align(InputGroupAddonAlign::InlineEnd),
                    ),
                ],
                InputGroupProps::new(),
                theme,
            ),
            // Section: Navigation Hints
            section_header("Navigation Hints", theme),
            column![
                input_group(
                    vec![
                        input_group_control(
                            text_input("Press ↑↓ to navigate", "").padding([8, 12])
                        ),
                        input_group_addon(
                            row![
                                kbd("↑", KbdProps::new().size(KbdSize::One), theme),
                                kbd("↓", KbdProps::new().size(KbdSize::One), theme),
                            ]
                            .spacing(2),
                            iced_shadcn::InputGroupAddonProps::new()
                                .align(InputGroupAddonAlign::InlineEnd),
                        ),
                    ],
                    InputGroupProps::new(),
                    theme,
                ),
                input_group(
                    vec![
                        input_group_control(text_input("Press ↵ to select", "").padding([8, 12])),
                        input_group_addon(
                            kbd("↵", KbdProps::new().size(KbdSize::Two), theme),
                            iced_shadcn::InputGroupAddonProps::new()
                                .align(InputGroupAddonAlign::InlineEnd),
                        ),
                    ],
                    InputGroupProps::new(),
                    theme,
                ),
            ]
            .spacing(12),
        ]
        .spacing(24)
        .max_width(500);

        app(theme, content.into())
    }
}

#[derive(Debug, Clone)]
enum Message {
    SearchChanged(String),
}

fn section_header<'a>(text: &'a str, theme: &'a Theme) -> iced::widget::Text<'a> {
    iced_text(text)
        .size(16)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(theme.palette.foreground),
        })
}

fn app<'a, Message: 'a>(theme: &Theme, content: Element<'a, Message>) -> Element<'a, Message> {
    let background = theme.palette.background;
    container(content)
        .padding(48)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
        .into()
}
