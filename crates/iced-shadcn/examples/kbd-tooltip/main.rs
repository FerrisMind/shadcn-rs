use iced::widget::{button, column, container, row, text as iced_text, tooltip};
use iced::{Alignment, Background, Color, Element, Length, Theme as IcedTheme};

use iced_shadcn::{KbdGroupProps, KbdProps, KbdSize, Theme, kbd, kbd_group};

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
            iced_text("Kbd with Tooltips")
                .size(24)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(theme.palette.foreground),
                }),
            iced_text("Keyboard shortcuts displayed in tooltip content")
                .size(14)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                }),
            // Section: Button with shortcut tooltip
            section_header("Buttons with Shortcut Tooltips", theme),
            row![
                tooltip_button("Save", "S", theme),
                tooltip_button("Print", "Ctrl+P", theme),
                tooltip_button("Copy", "Ctrl+C", theme),
                tooltip_button("Paste", "Ctrl+V", theme),
            ]
            .spacing(12),
            // Section: Tooltip with KbdGroup
            section_header("Tooltip with Multiple Keys", theme),
            row![
                tooltip_with_kbd_group("New Tab", vec!["Ctrl", "T"], theme),
                tooltip_with_kbd_group("Close Tab", vec!["Ctrl", "W"], theme),
                tooltip_with_kbd_group("Reopen Tab", vec!["Ctrl", "Shift", "T"], theme),
            ]
            .spacing(12),
            // Section: Action descriptions with shortcuts
            section_header("Action Descriptions", theme),
            row![
                action_button_with_tooltip("Undo", "Undo last action", vec!["Ctrl", "Z"], theme,),
                action_button_with_tooltip("Redo", "Redo last action", vec!["Ctrl", "Y"], theme,),
            ]
            .spacing(12),
            // Section: Command Palette Style
            section_header("Command Palette", theme),
            row![
                action_button_with_tooltip(
                    "Command Palette",
                    "Open command palette",
                    vec!["Ctrl", "K"],
                    theme,
                ),
                action_button_with_tooltip(
                    "Quick Open",
                    "Quickly open a file",
                    vec!["Ctrl", "P"],
                    theme,
                ),
            ]
            .spacing(12),
            // Section: Navigation
            section_header("Navigation", theme),
            row![
                tooltip_simple("Go to File", "Ctrl+G", theme),
                tooltip_simple("Find in Files", "Ctrl+Shift+F", theme),
                tooltip_simple("Go to Line", "Ctrl+G", theme),
            ]
            .spacing(12),
        ]
        .spacing(24);

        app(theme, content.into())
    }
}

fn tooltip_button<'a>(label: &'a str, shortcut: &'a str, theme: &'a Theme) -> Element<'a, ()> {
    let palette = theme.palette;
    let muted_fg = palette.muted_foreground;
    let primary_fg = palette.primary_foreground;

    let tooltip_content = row![
        iced_text(format!("Press "))
            .size(12)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(muted_fg),
            }),
        kbd(shortcut, KbdProps::new().size(KbdSize::One), theme),
        iced_text(format!(" to {}", label.to_lowercase()))
            .size(12)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(muted_fg),
            }),
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    tooltip(
        button(
            iced_text(label)
                .size(14)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(primary_fg),
                }),
        )
        .padding([8, 16])
        .style(
            move |_theme: &IcedTheme, _status| iced::widget::button::Style {
                background: Some(Background::Color(palette.primary)),
                text_color: palette.primary_foreground,
                border: iced::border::Border {
                    radius: theme.radius.md.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
        ),
        container(tooltip_content)
            .padding(8)
            .style(move |_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(Background::Color(palette.popover)),
                border: iced::border::Border {
                    color: palette.border,
                    width: 1.0,
                    radius: theme.radius.md.into(),
                },
                ..Default::default()
            }),
        tooltip::Position::Top,
    )
    .into()
}

fn tooltip_with_kbd_group<'a>(
    label: &'a str,
    keys: Vec<&'a str>,
    theme: &'a Theme,
) -> Element<'a, ()> {
    let palette = theme.palette;
    let muted_fg = palette.muted_foreground;
    let primary_fg = palette.primary_foreground;

    let items: Vec<Element<'a, ()>> = keys
        .into_iter()
        .map(|key| kbd(key, KbdProps::new().size(KbdSize::One), theme))
        .collect();

    let group_props = KbdGroupProps::new().gap(2.0);
    let tooltip_content = row![
        iced_text(format!("{} ", label))
            .size(12)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(muted_fg),
            }),
        kbd_group(items, &group_props),
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    tooltip(
        button(
            iced_text(label)
                .size(14)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(primary_fg),
                }),
        )
        .padding([8, 16])
        .style(
            move |_theme: &IcedTheme, _status| iced::widget::button::Style {
                background: Some(Background::Color(palette.primary)),
                text_color: palette.primary_foreground,
                border: iced::border::Border {
                    radius: theme.radius.md.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
        ),
        container(tooltip_content)
            .padding(8)
            .style(move |_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(Background::Color(palette.popover)),
                border: iced::border::Border {
                    color: palette.border,
                    width: 1.0,
                    radius: theme.radius.md.into(),
                },
                ..Default::default()
            }),
        tooltip::Position::Top,
    )
    .into()
}

fn action_button_with_tooltip<'a>(
    label: &'a str,
    description: &'a str,
    keys: Vec<&'a str>,
    theme: &'a Theme,
) -> Element<'a, ()> {
    let palette = theme.palette;
    let fg = palette.foreground;
    let muted_fg = palette.muted_foreground;

    let items: Vec<Element<'a, ()>> = keys
        .into_iter()
        .map(|key| kbd(key, KbdProps::new().size(KbdSize::One), theme))
        .collect();

    let group_props = KbdGroupProps::new().gap(2.0);
    let tooltip_content = column![
        iced_text(description)
            .size(12)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style { color: Some(fg) }),
        row![
            iced_text("Shortcut: ")
                .size(11)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(muted_fg),
                }),
            kbd_group(items, &group_props),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    ]
    .spacing(4);

    tooltip(
        button(
            iced_text(label)
                .size(14)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style { color: Some(fg) }),
        )
        .padding([8, 16])
        .style(
            move |_theme: &IcedTheme, _status| iced::widget::button::Style {
                background: Some(Background::Color(palette.secondary)),
                text_color: palette.secondary_foreground,
                border: iced::border::Border {
                    radius: theme.radius.md.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
        ),
        container(tooltip_content)
            .padding(10)
            .style(move |_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(Background::Color(palette.popover)),
                border: iced::border::Border {
                    color: palette.border,
                    width: 1.0,
                    radius: theme.radius.md.into(),
                },
                ..Default::default()
            }),
        tooltip::Position::Top,
    )
    .into()
}

fn tooltip_simple<'a>(label: &'a str, shortcut: &'a str, theme: &'a Theme) -> Element<'a, ()> {
    let palette = theme.palette;
    let fg = palette.foreground;

    let items: Vec<Element<'a, ()>> = shortcut
        .split('+')
        .map(|s| kbd(s.trim(), KbdProps::new().size(KbdSize::One), theme))
        .collect();

    let group_props = KbdGroupProps::new().gap(2.0);
    let tooltip_content = row![kbd_group(items, &group_props),];

    tooltip(
        button(
            iced_text(label)
                .size(14)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style { color: Some(fg) }),
        )
        .padding([8, 16])
        .style(
            move |_theme: &IcedTheme, _status| iced::widget::button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: palette.foreground,
                border: iced::border::Border {
                    color: palette.border,
                    width: 1.0,
                    radius: theme.radius.md.into(),
                },
                ..Default::default()
            },
        ),
        container(tooltip_content)
            .padding(8)
            .style(move |_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(Background::Color(palette.popover)),
                border: iced::border::Border {
                    color: palette.border,
                    width: 1.0,
                    radius: theme.radius.md.into(),
                },
                ..Default::default()
            }),
        tooltip::Position::Top,
    )
    .into()
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
