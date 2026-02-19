use iced::border::Border;
use iced::widget::{Column, column, container, row, scrollable, slider, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{AccentColor, ProgressProps, ProgressSize, ProgressVariant, Theme, progress};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

struct Example {
    theme: Theme,
    progress_value: f32,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            progress_value: 60.0,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SliderChanged(f32),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(value) => self.progress_value = value,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;

        let mut content = Column::new().spacing(16).width(Length::Fill);

        // -- Determinate --
        content = content.push(section_title("Determinate Progress"));
        content = content.push(preview(
            theme,
            column![
                slider(0.0..=100.0, self.progress_value, Message::SliderChanged).width(300),
                container(progress(
                    ProgressProps::new().value(self.progress_value),
                    theme,
                ))
                .width(Length::Fixed(300.0)),
            ]
            .spacing(12),
        ));

        // -- Sizes --
        content = content.push(section_title("Sizes"));
        content = content.push(preview(
            theme,
            column![
                container(progress(
                    ProgressProps::new().value(75.0).size(ProgressSize::Size1),
                    theme,
                ))
                .width(Length::Fixed(300.0)),
                container(progress(
                    ProgressProps::new().value(75.0).size(ProgressSize::Size2),
                    theme,
                ))
                .width(Length::Fixed(300.0)),
                container(progress(
                    ProgressProps::new().value(75.0).size(ProgressSize::Size3),
                    theme,
                ))
                .width(Length::Fixed(300.0)),
            ]
            .spacing(12),
        ));

        // -- Variants --
        content = content.push(section_title("Variants"));
        content = content.push(preview(
            theme,
            column![
                row![
                    iced_text("Classic").width(Length::Fixed(80.0)),
                    container(progress(
                        ProgressProps::new()
                            .value(60.0)
                            .variant(ProgressVariant::Classic),
                        theme,
                    ))
                    .width(Length::Fixed(200.0)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
                row![
                    iced_text("Surface").width(Length::Fixed(80.0)),
                    container(progress(
                        ProgressProps::new()
                            .value(60.0)
                            .variant(ProgressVariant::Surface),
                        theme,
                    ))
                    .width(Length::Fixed(200.0)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
                row![
                    iced_text("Soft").width(Length::Fixed(80.0)),
                    container(progress(
                        ProgressProps::new()
                            .value(60.0)
                            .variant(ProgressVariant::Soft),
                        theme,
                    ))
                    .width(Length::Fixed(200.0)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            ]
            .spacing(12),
        ));

        // -- Custom Colors --
        content = content.push(section_title("Custom Colors"));
        content = content.push(preview(
            theme,
            column![
                container(progress(
                    ProgressProps::new().value(80.0).color(AccentColor::Green),
                    theme,
                ))
                .width(Length::Fixed(300.0)),
                container(progress(
                    ProgressProps::new().value(45.0).color(AccentColor::Red),
                    theme,
                ))
                .width(Length::Fixed(300.0)),
            ]
            .spacing(8),
        ));

        // -- Indeterminate --
        content = content.push(section_title("Indeterminate (Loading)"));
        content = content.push(preview(
            theme,
            container(progress(ProgressProps::new().indeterminate(), theme))
                .width(Length::Fixed(300.0)),
        ));

        app(theme, scrollable(content).into())
    }
}

fn section_title<M: 'static>(title: &str) -> Element<'_, M> {
    iced_text(title).size(16).into()
}

fn app<'a>(theme: &Theme, content: Element<'a, Message>) -> Element<'a, Message> {
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
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
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
