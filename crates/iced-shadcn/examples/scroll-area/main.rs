use iced::border::Border;
use iced::widget::{Column, Row, column, container, row, scrollable, text as iced_text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    ButtonRadius, ScrollAreaProps, ScrollAreaScrollbars, ScrollAreaSize, Theme, scroll_area,
};

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
            page_header(theme),
            section_title("Sizes"),
            self.vertical_preview(
                "size Size1",
                ScrollAreaProps::new()
                    .size(ScrollAreaSize::Size1)
                    .scrollbars(ScrollAreaScrollbars::Vertical),
            ),
            self.vertical_preview(
                "size Size2",
                ScrollAreaProps::new()
                    .size(ScrollAreaSize::Size2)
                    .scrollbars(ScrollAreaScrollbars::Vertical),
            ),
            self.vertical_preview(
                "size Size3",
                ScrollAreaProps::new()
                    .size(ScrollAreaSize::Size3)
                    .scrollbars(ScrollAreaScrollbars::Vertical),
            ),
            section_title("Scrollbars"),
            self.vertical_preview(
                "vertical scrollbar",
                ScrollAreaProps::new().scrollbars(ScrollAreaScrollbars::Vertical),
            ),
            self.horizontal_preview(
                "horizontal scrollbar",
                ScrollAreaProps::new().scrollbars(ScrollAreaScrollbars::Horizontal),
            ),
            self.both_preview(
                "both scrollbars",
                ScrollAreaProps::new().scrollbars(ScrollAreaScrollbars::Both),
            ),
            section_title("Visual Width"),
            self.vertical_preview(
                "scrollbar_width 18px",
                ScrollAreaProps::new()
                    .scrollbars(ScrollAreaScrollbars::Vertical)
                    .scrollbar_width(18.0),
            ),
            self.vertical_preview(
                "rail 18px, thumb 8px",
                ScrollAreaProps::new()
                    .scrollbars(ScrollAreaScrollbars::Vertical)
                    .scrollbar_rail_width(18.0)
                    .scrollbar_thumb_width(8.0),
            ),
            self.vertical_preview(
                "rail 10px, thumb 18px",
                ScrollAreaProps::new()
                    .scrollbars(ScrollAreaScrollbars::Vertical)
                    .scrollbar_rail_width(10.0)
                    .scrollbar_thumb_width(18.0),
            ),
            section_title("Layout"),
            self.vertical_preview(
                "margin 12px",
                ScrollAreaProps::new()
                    .scrollbars(ScrollAreaScrollbars::Vertical)
                    .scrollbar_margin(12.0),
            ),
            self.vertical_preview(
                "embedded spacing 12px",
                ScrollAreaProps::new()
                    .scrollbars(ScrollAreaScrollbars::Vertical)
                    .scrollbar_spacing(12.0),
            ),
            section_title("Radius"),
            self.vertical_preview(
                "radius small",
                ScrollAreaProps::new()
                    .scrollbars(ScrollAreaScrollbars::Vertical)
                    .radius(ButtonRadius::Small),
            ),
            self.vertical_preview(
                "radius large",
                ScrollAreaProps::new()
                    .scrollbars(ScrollAreaScrollbars::Vertical)
                    .radius(ButtonRadius::Large),
            ),
            self.vertical_preview(
                "radius full",
                ScrollAreaProps::new()
                    .scrollbars(ScrollAreaScrollbars::Vertical)
                    .radius(ButtonRadius::Full),
            ),
            section_title("Combined"),
            self.both_preview(
                "size3 + both + rail 16 + thumb 8 + margin 8 + spacing 10 + radius large",
                ScrollAreaProps::new()
                    .size(ScrollAreaSize::Size3)
                    .scrollbars(ScrollAreaScrollbars::Both)
                    .scrollbar_rail_width(16.0)
                    .scrollbar_thumb_width(8.0)
                    .scrollbar_margin(8.0)
                    .scrollbar_spacing(10.0)
                    .radius(ButtonRadius::Large),
            ),
        ]
        .spacing(16)
        .max_width(980);

        app(
            theme,
            scrollable(container(content).width(Length::Fill)).into(),
        )
    }

    fn vertical_preview<'a>(&'a self, label: &'a str, props: ScrollAreaProps) -> Element<'a, ()> {
        let theme = &self.theme;

        preview(
            theme,
            label,
            container(scroll_area(vertical_content(), props, theme))
                .width(Length::Fixed(320.0))
                .height(Length::Fixed(180.0))
                .into(),
        )
    }

    fn horizontal_preview<'a>(&'a self, label: &'a str, props: ScrollAreaProps) -> Element<'a, ()> {
        let theme = &self.theme;

        preview(
            theme,
            label,
            container(scroll_area(horizontal_content(theme), props, theme))
                .width(Length::Fixed(420.0))
                .height(Length::Fixed(140.0))
                .into(),
        )
    }

    fn both_preview<'a>(&'a self, label: &'a str, props: ScrollAreaProps) -> Element<'a, ()> {
        let theme = &self.theme;

        preview(
            theme,
            label,
            container(scroll_area(both_content(theme), props, theme))
                .width(Length::Fixed(420.0))
                .height(Length::Fixed(220.0))
                .into(),
        )
    }
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

fn page_header<'a>(theme: &Theme) -> Element<'a, ()> {
    let foreground = theme.palette.foreground;
    let muted = theme.palette.muted_foreground;

    column![
        iced_text("Scroll Area")
            .size(28)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(foreground),
            }),
        iced_text(
            "This example shows every scrollbar option: sizes, axis modes, radius, width, rail, thumb, margin, spacing, and combined setups."
        )
        .size(14)
        .style(move |_theme| iced::widget::text::Style {
            color: Some(muted),
        }),
    ]
    .spacing(8)
    .into()
}

fn section_title<'a>(title: &'a str) -> Element<'a, ()> {
    let text_color = iced::Theme::Light.extended_palette().background.base.text;

    iced_text(title)
        .size(20)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(text_color),
        })
        .into()
}

fn preview<'a>(theme: &Theme, label: &'a str, content: Element<'a, ()>) -> Element<'a, ()> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;
    let foreground = theme.palette.foreground;

    container(
        row![
            iced_text(label)
                .width(Length::Fixed(220.0))
                .size(13)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(foreground),
                }),
            content,
        ]
        .spacing(12)
        .align_y(Alignment::Start),
    )
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
    .into()
}

fn vertical_content() -> Column<'static, ()> {
    let text_color = iced::Theme::Light.extended_palette().background.base.text;
    let mut col = Column::new().spacing(8);
    for i in 1..=50 {
        col = col.push(iced_text(format!("Item {i}")).size(12).style(
            move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(text_color),
            },
        ));
    }
    col
}

fn horizontal_content(theme: &Theme) -> Row<'static, ()> {
    let mut row = Row::new().spacing(12).align_y(Alignment::Center);
    for i in 1..=20 {
        let bg = theme.palette.muted;
        row = row.push(
            container(iced_text(format!("Card {i}")).size(12))
                .padding(12)
                .width(Length::Fixed(140.0))
                .height(Length::Fixed(80.0))
                .style(move |_theme| iced::widget::container::Style {
                    background: Some(Background::Color(bg)),
                    ..iced::widget::container::Style::default()
                }),
        );
    }
    row
}

fn both_content(theme: &Theme) -> Column<'static, ()> {
    let mut col = Column::new().spacing(12);
    for row_index in 1..=12 {
        let mut row = Row::new().spacing(12).align_y(Alignment::Center);
        for col_index in 1..=8 {
            let bg = if (row_index + col_index) % 2 == 0 {
                theme.palette.muted
            } else {
                theme.palette.accent
            };
            row = row.push(
                container(iced_text(format!("R{row_index} / C{col_index}")).size(12))
                    .padding(12)
                    .width(Length::Fixed(120.0))
                    .height(Length::Fixed(56.0))
                    .style(move |_theme| iced::widget::container::Style {
                        background: Some(Background::Color(bg)),
                        ..iced::widget::container::Style::default()
                    }),
            );
        }
        col = col.push(row);
    }
    col
}
