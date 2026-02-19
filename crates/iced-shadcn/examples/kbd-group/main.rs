use iced::widget::{column, container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{KbdGroupProps, KbdProps, KbdSize, Theme, kbd, kbd_group, kbd_shortcut};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
}

impl Example {
    fn update(&mut self, _message: ()) {}

    fn view(&self) -> Element<'_, ()> {
        let theme = &self.theme;

        let content = column![
            // Header
            iced_text("KbdGroup Component")
                .size(24)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(theme.palette.foreground),
                }),
            iced_text("Grouping keyboard shortcuts with various separators")
                .size(14)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                }),
            // Section: Basic Group
            section_header("Basic Group", theme),
            {
                let group_props = KbdGroupProps::new().gap(4.0);
                kbd_group(
                    vec![
                        kbd("Ctrl", KbdProps::new(), theme),
                        kbd("Shift", KbdProps::new(), theme),
                        kbd("T", KbdProps::new(), theme),
                    ],
                    &group_props,
                )
            },
            // Section: With Plus Separator
            section_header("With '+' Separator", theme),
            kbd_shortcut(vec!["Ctrl", "Shift", "P"], KbdProps::new(), theme),
            // Section: Custom Separators
            section_header("Custom Separators", theme),
            column![
                {
                    let group_props = KbdGroupProps::new().gap(4.0).separator(",");
                    row_with_label(
                        "Arrow keys:",
                        kbd_group(
                            vec![
                                kbd("↑", KbdProps::new(), theme),
                                kbd("↓", KbdProps::new(), theme),
                                kbd("←", KbdProps::new(), theme),
                                kbd("→", KbdProps::new(), theme),
                            ],
                            &group_props,
                        ),
                        theme,
                    )
                },
                {
                    let group_props = KbdGroupProps::new().gap(4.0).separator("|");
                    row_with_label(
                        "Function keys:",
                        kbd_group(
                            vec![
                                kbd("F1", KbdProps::new(), theme),
                                kbd("F2", KbdProps::new(), theme),
                                kbd("F3", KbdProps::new(), theme),
                            ],
                            &group_props,
                        ),
                        theme,
                    )
                },
                {
                    let group_props = KbdGroupProps::new().gap(2.0).separator("-");
                    row_with_label(
                        "Alphabet:",
                        kbd_group(
                            vec![
                                kbd("A", KbdProps::new(), theme),
                                kbd("B", KbdProps::new(), theme),
                                kbd("C", KbdProps::new(), theme),
                                kbd("D", KbdProps::new(), theme),
                            ],
                            &group_props,
                        ),
                        theme,
                    )
                },
            ]
            .spacing(12),
            // Section: Multiple Shortcuts
            section_header("Multiple Shortcuts", theme),
            row![
                kbd_shortcut(vec!["Ctrl", "B"], KbdProps::new(), theme),
                iced_text("or")
                    .size(14)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(theme.palette.muted_foreground),
                    }),
                kbd_shortcut(vec!["Ctrl", "K"], KbdProps::new(), theme),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            // Section: Nested Groups
            section_header("Command Palette Shortcuts", theme),
            column![
                row_with_label(
                    "Open command palette:",
                    kbd_shortcut(vec!["⌘", "K"], KbdProps::new(), theme),
                    theme,
                ),
                row_with_label(
                    "Quick search:",
                    kbd_shortcut(vec!["Ctrl", "P"], KbdProps::new(), theme),
                    theme,
                ),
                row_with_label(
                    "File explorer:",
                    kbd_shortcut(vec!["Ctrl", "Shift", "E"], KbdProps::new(), theme),
                    theme,
                ),
                row_with_label(
                    "Terminal:",
                    kbd_shortcut(vec!["Ctrl", "`"], KbdProps::new(), theme),
                    theme,
                ),
            ]
            .spacing(12),
            // Section: Mixed Sizes
            section_header("Mixed Sizes in Group", theme),
            row![
                kbd("Ctrl", KbdProps::new().size(KbdSize::Size2), theme),
                iced_text("+")
                    .size(12)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(theme.palette.muted_foreground),
                    }),
                kbd("Alt", KbdProps::new().size(KbdSize::Size2), theme),
                iced_text("+")
                    .size(12)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(theme.palette.muted_foreground),
                    }),
                kbd("Delete", KbdProps::new().size(KbdSize::Size3), theme),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
        ]
        .spacing(24);

        app(theme, content.into())
    }
}

fn section_header<'a>(text: &'a str, theme: &'a Theme) -> iced::widget::Text<'a> {
    iced_text(text)
        .size(16)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(theme.palette.foreground),
        })
}

fn row_with_label<'a>(
    label: &'a str,
    kbd_element: Element<'a, ()>,
    theme: &'a Theme,
) -> Element<'a, ()> {
    row![
        iced_text(label)
            .size(14)
            .width(Length::Fixed(160.0))
            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }),
        kbd_element,
    ]
    .spacing(16)
    .align_y(Alignment::Center)
    .into()
}

fn app<'a>(theme: &Theme, content: Element<'a, ()>) -> Element<'a, ()> {
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
