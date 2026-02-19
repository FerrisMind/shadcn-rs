use iced::border::Border;
use iced::widget::{Column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Element, Font, Length};

use iced_shadcn::{KbdGroupProps, KbdProps, KbdSize, Theme, kbd, kbd_group, kbd_shortcut};
use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
}

impl Example {
    fn update(&mut self, _message: ()) {}

    fn view(&self) -> Element<'_, ()> {
        let theme = &self.theme;

        let mut content = Column::new().spacing(16).width(Length::Fill);

        // -- Basic Usage --
        content = content.push(section_title("Basic Usage"));
        content = content.push(preview(
            theme,
            row![
                kbd("Ctrl", KbdProps::new(), theme),
                iced_text("+"),
                kbd("C", KbdProps::new(), theme),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
        ));

        // -- Modifier Keys --
        content = content.push(section_title("Modifier Keys"));
        content = content.push(preview(
            theme,
            row![
                kbd(
                    Icon::Command.unicode().to_string(),
                    KbdProps::new().font(Font::with_name("lucide")),
                    theme
                ),
                kbd(
                    Icon::ArrowBigUp.unicode().to_string(),
                    KbdProps::new().font(Font::with_name("lucide")),
                    theme
                ),
                kbd(
                    Icon::Option.unicode().to_string(),
                    KbdProps::new().font(Font::with_name("lucide")),
                    theme
                ),
                kbd(
                    Icon::ChevronUp.unicode().to_string(),
                    KbdProps::new().font(Font::with_name("lucide")),
                    theme
                ),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
        ));
        // -- Sizes --
        content = content.push(section_title("Sizes"));
        content = content.push(preview(
            theme,
            row![
                kbd("XS", KbdProps::new().size(KbdSize::Size1), theme),
                kbd("S", KbdProps::new().size(KbdSize::Size2), theme),
                kbd("M", KbdProps::new().size(KbdSize::Size3), theme),
                kbd("L", KbdProps::new().size(KbdSize::Five), theme),
                kbd("XL", KbdProps::new().size(KbdSize::Six), theme),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ));

        // -- Common Shortcuts --
        content = content.push(section_title("Common Shortcuts"));
        let shortcuts = [
            ("Copy", vec!["Ctrl", "C"]),
            ("Paste", vec!["Ctrl", "V"]),
            ("Cut", vec!["Ctrl", "X"]),
            ("Undo", vec!["Ctrl", "Z"]),
            ("Save", vec!["Ctrl", "S"]),
            ("Find", vec!["Ctrl", "K"]),
        ];
        let mut shortcuts_col = Column::new().spacing(8);
        for (action, keys) in shortcuts {
            shortcuts_col = shortcuts_col.push(
                row![
                    iced_text(format!("{action}:")).width(Length::Fixed(80.0)),
                    kbd_shortcut(keys, KbdProps::new(), theme),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            );
        }
        content = content.push(preview(theme, shortcuts_col));

        // -- Function Keys --
        content = content.push(section_title("Function Keys"));
        let fkeys: Vec<Element<'_, ()>> = (1..=12)
            .map(|i| kbd(format!("F{i}"), KbdProps::new(), theme))
            .collect();
        content = content.push(preview(
            theme,
            kbd_group(fkeys, &KbdGroupProps::new().gap(4.0)),
        ));

        // -- Arrow Keys --
        let icon_font = Font::with_name("lucide");
        content = content.push(section_title("Arrow Keys"));
        content = content.push(preview(
            theme,
            row![
                kbd(
                    Icon::ArrowUp.unicode().to_string(),
                    KbdProps::new().font(icon_font),
                    theme
                ),
                kbd(
                    Icon::ArrowDown.unicode().to_string(),
                    KbdProps::new().font(icon_font),
                    theme
                ),
                kbd(
                    Icon::ArrowLeft.unicode().to_string(),
                    KbdProps::new().font(icon_font),
                    theme
                ),
                kbd(
                    Icon::ArrowRight.unicode().to_string(),
                    KbdProps::new().font(icon_font),
                    theme
                ),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
        ));

        // -- Navigation --
        content = content.push(section_title("Navigation"));
        content = content.push(preview(
            theme,
            row![
                kbd("Home", KbdProps::new(), theme),
                kbd("End", KbdProps::new(), theme),
                kbd("PgUp", KbdProps::new(), theme),
                kbd("PgDn", KbdProps::new(), theme),
                kbd("Ins", KbdProps::new(), theme),
                kbd("Del", KbdProps::new(), theme),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
        ));

        // -- Special Keys --
        content = content.push(section_title("Special Keys"));
        content = content.push(preview(
            theme,
            row![
                kbd("Tab", KbdProps::new(), theme),
                kbd("Esc", KbdProps::new(), theme),
                kbd("Enter", KbdProps::new(), theme),
                kbd("Space", KbdProps::new(), theme),
                kbd("Backspace", KbdProps::new(), theme),
            ]
            .spacing(4)
            .align_y(Alignment::Center),
        ));

        app(theme, scrollable(content).into())
    }
}

fn section_title(title: &str) -> Element<'_, ()> {
    iced_text(title).size(16).into()
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

fn preview<'a>(
    theme: &Theme,
    content: impl Into<Element<'a, ()>>,
) -> iced::widget::Container<'a, ()> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(16)
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: radius.into(),
                width: 1.0,
                color: border,
            },
            ..iced::widget::container::Style::default()
        })
}
