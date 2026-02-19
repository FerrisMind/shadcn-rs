use iced::widget::{column, container, row, scrollable, text as iced_text};
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
            iced_text("Kbd Component Demo")
                .size(24)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(theme.palette.foreground),
                }),
            iced_text("Keyboard shortcut indicators inspired by shadcn/ui and Radix UI Themes")
                .size(14)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                }),
            // Section: Sizes
            section_header("Sizes", theme),
            row((1..=5)
                .map(|i| {
                    let size = match i {
                        1 => KbdSize::Size1,
                        2 => KbdSize::Size2,
                        3 => KbdSize::Size3,
                        4 => KbdSize::Size4,
                        5 => KbdSize::Five,
                        _ => KbdSize::Size2,
                    };
                    kbd(format!("S{i}"), KbdProps::new().size(size), theme)
                })
                .collect::<Vec<_>>())
            .spacing(8)
            .align_y(Alignment::Center),
            // Section: Modifier Keys
            section_header("Modifier Keys", theme),
            {
                let group_props = KbdGroupProps::new().gap(4.0);
                kbd_group(
                    vec![
                        kbd("⌘", KbdProps::new(), theme),
                        kbd("⇧", KbdProps::new(), theme),
                        kbd("⌥", KbdProps::new(), theme),
                        kbd("⌃", KbdProps::new(), theme),
                    ],
                    &group_props,
                )
            },
            // Section: Common Shortcuts
            section_header("Common Shortcuts", theme),
            column![
                shortcut_row(
                    "Copy",
                    kbd_shortcut(vec!["Ctrl", "C"], KbdProps::new(), theme),
                    theme
                ),
                shortcut_row(
                    "Paste",
                    kbd_shortcut(vec!["Ctrl", "V"], KbdProps::new(), theme),
                    theme
                ),
                shortcut_row(
                    "Cut",
                    kbd_shortcut(vec!["Ctrl", "X"], KbdProps::new(), theme),
                    theme
                ),
                shortcut_row(
                    "Undo",
                    kbd_shortcut(vec!["Ctrl", "Z"], KbdProps::new(), theme),
                    theme
                ),
                shortcut_row(
                    "Save",
                    kbd_shortcut(vec!["Ctrl", "S"], KbdProps::new(), theme),
                    theme
                ),
                shortcut_row(
                    "Find",
                    kbd_shortcut(vec!["Ctrl", "K"], KbdProps::new(), theme),
                    theme
                ),
            ]
            .spacing(12),
            // Section: Custom Colors
            section_header("Custom Colors", theme),
            row![
                kbd(
                    "Primary",
                    KbdProps::new()
                        .background(theme.palette.primary)
                        .color(theme.palette.primary_foreground),
                    theme
                ),
                kbd(
                    "Secondary",
                    KbdProps::new()
                        .background(theme.palette.secondary)
                        .color(theme.palette.secondary_foreground),
                    theme
                ),
                kbd(
                    "Accent",
                    KbdProps::new()
                        .background(theme.palette.accent)
                        .color(theme.palette.accent_foreground),
                    theme
                ),
                kbd(
                    "Muted",
                    KbdProps::new()
                        .background(theme.palette.muted)
                        .color(theme.palette.muted_foreground),
                    theme
                ),
            ]
            .spacing(8),
            // Section: Without Shadow
            section_header("Without Shadow", theme),
            row![
                kbd("Flat", KbdProps::new().shadow(false), theme),
                kbd("3D", KbdProps::new().shadow(true), theme),
            ]
            .spacing(8),
        ]
        .spacing(24);

        app(theme, scrollable(content.padding(24)).into())
    }
}

fn section_header<'a>(text: &'a str, theme: &'a Theme) -> iced::widget::Text<'a> {
    iced_text(text)
        .size(16)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(theme.palette.foreground),
        })
}

fn shortcut_row<'a>(
    label: &'a str,
    kbd_element: Element<'a, ()>,
    theme: &'a Theme,
) -> Element<'a, ()> {
    row![
        iced_text(label)
            .size(14)
            .width(Length::Fixed(60.0))
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
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
        .into()
}
