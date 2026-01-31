use iced::widget::{button, column, container, row, text as iced_text};
use iced::{Alignment, Background, Element, Length, Theme as IcedTheme};

use iced_shadcn::{KbdProps, Theme, kbd};

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
            iced_text("Kbd with Buttons")
                .size(24)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(theme.palette.foreground),
                }),
            iced_text("Keyboard shortcuts displayed inside buttons")
                .size(14)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                }),
            // Examples
            row![
                button_with_kbd("Accept", "⏎", theme),
                button_with_kbd("Cancel", "Esc", theme),
                button_with_kbd("Save", "Ctrl+S", theme),
            ]
            .spacing(12),
            row![
                outline_button_with_kbd("Copy", "⌘C", theme),
                outline_button_with_kbd("Paste", "⌘V", theme),
                outline_button_with_kbd("Cut", "⌘X", theme),
            ]
            .spacing(12),
            // Ghost buttons with shortcuts
            row![
                ghost_button_with_kbd("New Tab", "⌘T", theme),
                ghost_button_with_kbd("Close", "⌘W", theme),
                ghost_button_with_kbd("Reload", "⌘R", theme),
            ]
            .spacing(12),
        ]
        .spacing(24);

        app(theme, content.into())
    }
}

fn button_with_kbd<'a>(label: &'a str, shortcut: &'a str, theme: &'a Theme) -> Element<'a, ()> {
    let palette = theme.palette;

    button(
        row![
            iced_text(label).size(14),
            kbd(
                shortcut,
                KbdProps::new().size(iced_shadcn::KbdSize::One),
                theme
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .padding([8, 16])
    .style(
        move |_theme: &IcedTheme, status| iced::widget::button::Style {
            background: Some(Background::Color(palette.primary)),
            text_color: palette.primary_foreground,
            border: iced::border::Border {
                radius: theme.radius.md.into(),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .into()
}

fn outline_button_with_kbd<'a>(
    label: &'a str,
    shortcut: &'a str,
    theme: &'a Theme,
) -> Element<'a, ()> {
    let palette = theme.palette;

    button(
        row![
            iced_text(label).size(14),
            kbd(
                shortcut,
                KbdProps::new().size(iced_shadcn::KbdSize::One),
                theme
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .padding([8, 16])
    .style(
        move |_theme: &IcedTheme, status| iced::widget::button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: palette.foreground,
            border: iced::border::Border {
                color: palette.border,
                width: 1.0,
                radius: theme.radius.md.into(),
            },
            ..Default::default()
        },
    )
    .into()
}

fn ghost_button_with_kbd<'a>(
    label: &'a str,
    shortcut: &'a str,
    theme: &'a Theme,
) -> Element<'a, ()> {
    let palette = theme.palette;

    button(
        row![
            iced_text(label).size(14),
            kbd(
                shortcut,
                KbdProps::new().size(iced_shadcn::KbdSize::One),
                theme
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .padding([8, 16])
    .style(
        move |_theme: &IcedTheme, status| iced::widget::button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: palette.foreground,
            border: iced::border::Border::default(),
            ..Default::default()
        },
    )
    .into()
}

use iced::Color;

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
