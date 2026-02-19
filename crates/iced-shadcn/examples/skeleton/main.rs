use iced::border::Border;
use iced::widget::{Column, column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{SkeletonProps, Theme, skeleton, skeleton_text};

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

        // -- Text skeleton --
        content = content.push(section_title("Text Loading Placeholder"));
        content = content.push(preview(theme, skeleton_text(3, 16.0, theme)));

        // -- Card-like layout --
        content = content.push(section_title("Card-like Layout"));
        content = content.push(preview(
            theme,
            row![
                skeleton(
                    SkeletonProps::new()
                        .width(Length::Fixed(48.0))
                        .height(Length::Fixed(48.0))
                        .circle(true),
                    theme,
                ),
                column![
                    skeleton(
                        SkeletonProps::new()
                            .width(Length::Fixed(200.0))
                            .height(Length::Fixed(16.0)),
                        theme,
                    ),
                    skeleton(
                        SkeletonProps::new()
                            .width(Length::Fixed(150.0))
                            .height(Length::Fixed(14.0)),
                        theme,
                    ),
                ]
                .spacing(8),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        ));

        // -- Image placeholder --
        content = content.push(section_title("Image Placeholder"));
        content = content.push(preview(
            theme,
            skeleton(
                SkeletonProps::new()
                    .width(Length::Fixed(300.0))
                    .height(Length::Fixed(180.0))
                    .radius(theme.radius.lg),
                theme,
            ),
        ));

        // -- Button placeholder --
        content = content.push(section_title("Button Placeholder"));
        content = content.push(preview(
            theme,
            skeleton(
                SkeletonProps::new()
                    .width(Length::Fixed(120.0))
                    .height(Length::Fixed(36.0)),
                theme,
            ),
        ));

        // -- Input field placeholder --
        content = content.push(section_title("Input Field Placeholder"));
        content = content.push(preview(
            theme,
            skeleton(SkeletonProps::new().height(Length::Fixed(40.0)), theme),
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
