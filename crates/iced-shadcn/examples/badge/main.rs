use iced::border::Border;
use iced::widget::{Column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{AccentColor, BadgeProps, BadgeSize, BadgeVariant, Theme, badge};

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

        let mut content = Column::new().spacing(16).width(Length::Fill);

        // -- Sizes --
        content = content.push(section_title("Sizes"));
        content = content.push(preview(
            theme,
            row![
                badge("Size 1", BadgeProps::new().size(BadgeSize::Size1), theme),
                badge("Size 2", BadgeProps::new().size(BadgeSize::Size2), theme),
                badge("Size 3", BadgeProps::new().size(BadgeSize::Size3), theme),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ));

        // -- Variants --
        content = content.push(section_title("Variants"));
        content = content.push(preview(
            theme,
            row![
                badge(
                    "Default",
                    BadgeProps::new().variant(BadgeVariant::Default),
                    theme,
                ),
                badge(
                    "Secondary",
                    BadgeProps::new().variant(BadgeVariant::Secondary),
                    theme,
                ),
                badge(
                    "Outline",
                    BadgeProps::new().variant(BadgeVariant::Outline),
                    theme,
                ),
                badge(
                    "Destructive",
                    BadgeProps::new().variant(BadgeVariant::Destructive),
                    theme,
                ),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ));

        // -- Custom colors --
        content = content.push(section_title("Custom Colors"));
        content = content.push(preview(
            theme,
            row![
                badge(
                    "Error",
                    BadgeProps::new()
                        .variant(BadgeVariant::Default)
                        .color(AccentColor::Red),
                    theme,
                ),
                badge(
                    "Success",
                    BadgeProps::new()
                        .variant(BadgeVariant::Default)
                        .color(AccentColor::Green),
                    theme,
                ),
                badge(
                    "Warning",
                    BadgeProps::new()
                        .variant(BadgeVariant::Default)
                        .color(AccentColor::Yellow),
                    theme,
                ),
                badge(
                    "Info",
                    BadgeProps::new()
                        .variant(BadgeVariant::Default)
                        .color(AccentColor::Blue),
                    theme,
                ),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ));

        // -- High Contrast --
        content = content.push(section_title("High Contrast"));
        content = content.push(preview(
            theme,
            row![
                badge(
                    "Normal",
                    BadgeProps::new().variant(BadgeVariant::Default),
                    theme,
                ),
                badge(
                    "High Contrast",
                    BadgeProps::new()
                        .variant(BadgeVariant::Default)
                        .high_contrast(true),
                    theme,
                ),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ));

        // -- Link Badge (href) --
        content = content.push(section_title("Link Badge (href)"));
        content = content.push(preview(
            theme,
            row![badge(
                "Visit shadcn-rs",
                BadgeProps::new()
                    .variant(BadgeVariant::Default)
                    .href("https://github.com/nicepkg/shadcn-rs"),
                theme,
            ),]
            .spacing(8)
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
