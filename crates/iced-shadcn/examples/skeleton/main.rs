use iced::border::Border;
use iced::widget::{Column, column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{SkeletonAnimation, SkeletonProps, Theme, skeleton, skeleton_shimmer_label};

pub fn main() -> iced::Result {
    iced_shadcn::profiling::init_runtime();

    iced::application(Example::default, Example::update, Example::view).run()
}

struct Example {
    theme: Theme,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::dark(),
        }
    }
}

impl Example {
    fn update(&mut self, _message: ()) {}

    fn view(&self) -> Element<'_, ()> {
        let theme = &self.theme;
        let mut content = Column::new().spacing(16).width(Length::Fill);

        content = content.push(
            iced_text("Shimmer")
                .size(30)
                .color(theme.palette.foreground),
        );
        content = content.push(
            iced_text(
                "An animated text shimmer component for creating eye-catching loading states and progressive reveal effects.",
            )
            .size(15)
            .color(theme.palette.muted_foreground),
        );

        // Usage
        content = content.push(section_title(theme, "Usage"));
        content = content.push(preview(
            theme,
            skeleton_shimmer_label(
                "Hello, this is a shimmer effect!",
                16.0,
                2000,
                2.0,
                35.0,
                theme,
            ),
        ));

        // Custom Duration
        content = content.push(section_title(theme, "Custom Duration"));
        content = content.push(preview(
            theme,
            row![
                skeleton_shimmer_label("Fast shimmer animation", 16.0, 1000, 2.0, 23.0, theme),
                skeleton_shimmer_label("Slow shimmer animation", 16.0, 4000, 2.0, 23.0, theme),
            ]
            .spacing(24),
        ));

        // Custom Spread
        content = content.push(section_title(theme, "Custom Spread"));
        content = content.push(preview(
            theme,
            column![
                skeleton_shimmer_label("Narrow shimmer spread", 16.0, 2000, 1.0, 21.0, theme),
                skeleton_shimmer_label("Wide shimmer spread", 16.0, 2000, 4.0, 18.0, theme),
            ]
            .spacing(12),
        ));

        // Different HTML Elements
        content = content.push(section_title(theme, "Different HTML Elements"));
        content = content.push(preview(
            theme,
            column![
                skeleton_shimmer_label("Heading 1 with Shimmer", 34.0, 2000, 2.0, 23.0, theme),
                skeleton_shimmer_label("Heading 2 with Shimmer", 28.0, 2000, 2.0, 23.0, theme),
                skeleton_shimmer_label("Inline shimmer text", 18.0, 2000, 2.0, 19.0, theme),
            ]
            .spacing(10),
        ));

        // AI Loading State
        content = content.push(section_title(theme, "AI Loading State"));
        content = content.push(preview(
            theme,
            column![
                skeleton_shimmer_label("Analyzing your request...", 14.0, 2000, 2.0, 27.0, theme),
                skeleton_shimmer_label("Processing with AI...", 14.0, 1500, 2.0, 23.0, theme),
                skeleton_shimmer_label("Generating response...", 14.0, 2500, 2.0, 23.0, theme),
            ]
            .spacing(10),
        ));

        // Usage with AI SDK
        content = content.push(section_title(theme, "Usage with AI SDK"));
        content = content.push(preview(
            theme,
            row![
                skeleton(
                    SkeletonProps::new()
                        .width(Length::Fixed(28.0))
                        .height(Length::Fixed(28.0))
                        .circle(true),
                    theme,
                ),
                skeleton_shimmer_label("Thinking and processing...", 16.0, 2000, 2.0, 30.0, theme),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        ));

        // Skeleton Blocks (Shimmer)
        content = content.push(section_title(theme, "Skeleton Blocks (Shimmer)"));
        content = content.push(preview(
            theme,
            column![
                skeleton(
                    SkeletonProps::new()
                        .animation(SkeletonAnimation::Shimmer)
                        .width(Length::Fixed(320.0))
                        .height(Length::Fixed(14.0))
                        .content_length(32.0),
                    theme,
                ),
                skeleton(
                    SkeletonProps::new()
                        .animation(SkeletonAnimation::Shimmer)
                        .width(Length::Fixed(260.0))
                        .height(Length::Fixed(14.0))
                        .content_length(24.0),
                    theme,
                ),
                row![
                    skeleton(
                        SkeletonProps::new()
                            .animation(SkeletonAnimation::Shimmer)
                            .width(Length::Fixed(44.0))
                            .height(Length::Fixed(44.0))
                            .circle(true),
                        theme,
                    ),
                    column![
                        skeleton(
                            SkeletonProps::new()
                                .animation(SkeletonAnimation::Shimmer)
                                .width(Length::Fixed(220.0))
                                .height(Length::Fixed(12.0))
                                .content_length(22.0),
                            theme,
                        ),
                        skeleton(
                            SkeletonProps::new()
                                .animation(SkeletonAnimation::Shimmer)
                                .width(Length::Fixed(170.0))
                                .height(Length::Fixed(12.0))
                                .content_length(17.0),
                            theme,
                        ),
                    ]
                    .spacing(8),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            ]
            .spacing(10),
        ));

        // Skeleton Blocks (Pulse)
        content = content.push(section_title(theme, "Skeleton Blocks (Pulse)"));
        content = content.push(preview(
            theme,
            column![
                skeleton(
                    SkeletonProps::new()
                        .width(Length::Fixed(300.0))
                        .height(Length::Fixed(14.0)),
                    theme,
                ),
                skeleton(
                    SkeletonProps::new()
                        .width(Length::Fixed(210.0))
                        .height(Length::Fixed(14.0)),
                    theme,
                ),
                skeleton(
                    SkeletonProps::new()
                        .width(Length::Fixed(120.0))
                        .height(Length::Fixed(36.0))
                        .radius(theme.radius.md),
                    theme,
                ),
            ]
            .spacing(10),
        ));

        app(theme, scrollable(content).into())
    }
}

fn section_title<'a>(theme: &Theme, title: &'a str) -> Element<'a, ()> {
    iced_text(title)
        .size(16)
        .color(theme.palette.foreground)
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
