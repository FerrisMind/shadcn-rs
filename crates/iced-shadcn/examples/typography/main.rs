use iced::border::Border;
use iced::font;
use iced::widget::text::{IntoFragment, LineHeight, Rich, Span};
use iced::widget::{column, container, row, rule, scrollable, space, text as iced_text};
use iced::{Alignment, Background, Color, Element, Font, Length};

use iced_shadcn::{
    HeadingProps, SeparatorOrientation, SeparatorProps, SeparatorSize, TextAlign, TextProps,
    TextSize, TextWeight, Theme, heading, separator, text,
};

#[derive(Clone, Copy, Debug)]
enum TextVariant {
    H1,
    H2,
    H3,
    H4,
    Lead,
    Large,
    Body,
    Small,
    Muted,
}

fn shadcn_text<'a>(
    content: impl IntoFragment<'a>,
    variant: TextVariant,
    theme: &'a Theme,
) -> iced::widget::Text<'a> {
    match variant {
        TextVariant::H1 => heading(
            content,
            HeadingProps::new()
                .size(TextSize::Eight)
                .weight(TextWeight::Bold),
            theme,
        ),
        TextVariant::H2 => heading(
            content,
            HeadingProps::new()
                .size(TextSize::Seven)
                .weight(TextWeight::Medium),
            theme,
        ),
        TextVariant::H3 => heading(
            content,
            HeadingProps::new()
                .size(TextSize::Six)
                .weight(TextWeight::Medium),
            theme,
        ),
        TextVariant::H4 => heading(
            content,
            HeadingProps::new()
                .size(TextSize::Five)
                .weight(TextWeight::Medium),
            theme,
        ),
        TextVariant::Lead => iced_text(content)
            .size(20)
            .line_height(LineHeight::Relative(1.75))
            .style(|_theme| iced::widget::text::Style {
                color: Some(theme.palette.muted_foreground),
            }),
        TextVariant::Large => text(
            content,
            TextProps::new()
                .size(TextSize::Four)
                .weight(TextWeight::Medium),
            theme,
        ),
        TextVariant::Body => iced_text(content)
            .size(16)
            .line_height(LineHeight::Relative(1.75))
            .style(|_theme| iced::widget::text::Style {
                color: Some(theme.palette.foreground),
            }),
        TextVariant::Small => text(
            content,
            TextProps::new()
                .size(TextSize::Two)
                .weight(TextWeight::Medium),
            theme,
        ),
        TextVariant::Muted => {
            iced_text(content)
                .size(14)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(theme.palette.muted_foreground),
                })
        }
    }
}

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {}

impl Example {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let background = theme.palette.background;
        let border = theme.palette.border;
        let radius = theme.radius.md;

        let paragraph_with_link = Rich::<(), Message>::with_spans(vec![
            Span::new("The king thought long and hard, and finally came up with ")
                .color(theme.palette.foreground),
            Span::new("a brilliant plan")
                .color(theme.palette.primary)
                .underline(true)
                .font(Font {
                    weight: font::Weight::Medium,
                    ..Font::DEFAULT
                }),
            Span::new(": he would tax the jokes in the kingdom.").color(theme.palette.foreground),
        ])
        .size(16)
        .line_height(LineHeight::Relative(1.75))
        .width(Length::Fill);

        let demo_table = table_view(theme, true);

        let demo = preview(
            theme,
            column![
                shadcn_text(
                    "Taxing Laughter: The Joke Tax Chronicles",
                    TextVariant::H1,
                    theme,
                ),
                v_space(24.0),
                shadcn_text(
                    "Once upon a time, in a far-off land, there was a very lazy king who spent all day lounging on his throne. One day, his advisors came to him with a problem: the kingdom was running out of money.",
                    TextVariant::Lead,
                    theme,
                )
                .width(Length::Fill),
                v_space(40.0),
                column![
                    shadcn_text("The King's Plan", TextVariant::H2, theme),
                    separator(
                        SeparatorProps::new()
                            .orientation(SeparatorOrientation::Horizontal)
                            .size(SeparatorSize::Four),
                        theme,
                    ),
                ]
                .spacing(8),
                v_space(24.0),
                paragraph_with_link,
                v_space(24.0),
                blockquote_widget(theme),
                v_space(32.0),
                shadcn_text("The Joke Tax", TextVariant::H3, theme),
                v_space(24.0),
                shadcn_text(
                    "The king's subjects were not amused. They grumbled and complained, but the king was firm:",
                    TextVariant::Body,
                    theme,
                )
                .width(Length::Fill),
                v_space(24.0),
                container(list_view(theme)).padding(iced::padding::left(24.0)),
                v_space(24.0),
                shadcn_text(
                    "As a result, people stopped telling jokes, and the kingdom fell into a gloom. But there was one person who refused to let the king's foolishness get him down: a court jester named Jokester.",
                    TextVariant::Body,
                    theme,
                )
                .width(Length::Fill),
                v_space(32.0),
                shadcn_text("Jokester's Revolt", TextVariant::H3, theme),
                v_space(24.0),
                shadcn_text(
                    "Jokester began sneaking into the castle in the middle of the night and leaving jokes all over the place: under the king's pillow, in his soup, even in the royal toilet. The king was furious, but he couldn't seem to stop Jokester.",
                    TextVariant::Body,
                    theme,
                )
                .width(Length::Fill),
                v_space(24.0),
                shadcn_text(
                    "And then, one day, the people of the kingdom discovered that the jokes left by Jokester were so funny that they couldn't help but laugh. And once they started laughing, they couldn't stop.",
                    TextVariant::Body,
                    theme,
                )
                .width(Length::Fill),
                v_space(32.0),
                shadcn_text("The People's Rebellion", TextVariant::H3, theme),
                v_space(24.0),
                shadcn_text(
                    "The people of the kingdom, feeling uplifted by the laughter, started to tell jokes and puns again, and soon the entire kingdom was in on the joke.",
                    TextVariant::Body,
                    theme,
                )
                .width(Length::Fill),
                v_space(24.0),
                demo_table,
                v_space(24.0),
                shadcn_text(
                    "The king, seeing how much happier his subjects were, realized the error of his ways and repealed the joke tax. Jokester was declared a hero, and the kingdom lived happily ever after.",
                    TextVariant::Body,
                    theme,
                )
                .width(Length::Fill),
                v_space(24.0),
                shadcn_text(
                    "The moral of the story is: never underestimate the power of a good laugh and always be careful of bad ideas.",
                    TextVariant::Body,
                    theme,
                )
                .width(Length::Fill),
            ]
            .spacing(0)
            .width(Length::Fill),
        );

        let h1 = preview(
            theme,
            container(
                heading(
                    "Taxing Laughter: The Joke Tax Chronicles",
                    HeadingProps::new()
                        .size(TextSize::Eight)
                        .weight(TextWeight::Bold)
                        .align(TextAlign::Center),
                    theme,
                )
                .width(Length::Fill),
            )
            .width(Length::Fill),
        );

        let h2 = preview(
            theme,
            column![
                shadcn_text("The People of the Kingdom", TextVariant::H2, theme),
                separator(
                    SeparatorProps::new()
                        .orientation(SeparatorOrientation::Horizontal)
                        .size(SeparatorSize::Four),
                    theme,
                ),
            ]
            .spacing(8),
        );

        let h3 = preview(theme, shadcn_text("The Joke Tax", TextVariant::H3, theme));
        let h4 = preview(
            theme,
            shadcn_text("People stopped telling jokes", TextVariant::H4, theme),
        );

        let paragraph = preview(
            theme,
            shadcn_text(
                "The king, seeing how much happier his subjects were, realized the error of his ways and repealed the joke tax.",
                TextVariant::Body,
                theme,
            )
            .width(Length::Fill),
        );

        let blockquote = preview(theme, blockquote_widget(theme));
        let table = preview(theme, table_view(theme, true));
        let list = preview(
            theme,
            container(list_view(theme)).padding(iced::padding::left(24.0)),
        );

        let inline_code = preview(theme, inline_code(theme));

        let lead = preview(
            theme,
            shadcn_text(
                "A modal dialog that interrupts the user with important content and expects a response.",
                TextVariant::Lead,
                theme,
            )
            .width(Length::Fill),
        );

        let large = preview(
            theme,
            shadcn_text("Are you absolutely sure?", TextVariant::Large, theme),
        );

        let small = preview(
            theme,
            shadcn_text("Email address", TextVariant::Small, theme),
        );
        let muted = preview(
            theme,
            shadcn_text("Enter your email address.", TextVariant::Muted, theme),
        );

        let content = column![
            row![demo].spacing(20).align_y(Alignment::Center),
            row![h1, h2].spacing(20).align_y(Alignment::Center),
            row![h3, h4].spacing(20).align_y(Alignment::Center),
            row![paragraph, blockquote]
                .spacing(20)
                .align_y(Alignment::Center),
            row![table, list].spacing(20).align_y(Alignment::Center),
            row![inline_code, lead]
                .spacing(20)
                .align_y(Alignment::Center),
            row![large, small].spacing(20).align_y(Alignment::Center),
            row![muted].spacing(20).align_y(Alignment::Center),
        ]
        .spacing(20)
        .align_x(Alignment::Center);

        container(scrollable(content))
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
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
}

fn preview<'a, Message: 'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(20)
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

fn v_space(height: f32) -> iced::widget::Space {
    space().height(Length::Fixed(height))
}

fn list_item<'a, Message>(content: &'a str, theme: &'a Theme) -> iced::widget::Row<'a, Message> {
    row![
        shadcn_text("•", TextVariant::Body, theme),
        shadcn_text(content, TextVariant::Body, theme).width(Length::Fill),
    ]
    .spacing(8)
    .align_y(Alignment::Start)
}

fn list_view<'a, Message: 'a>(theme: &'a Theme) -> iced::widget::Column<'a, Message> {
    column![
        list_item("1st level of puns: 5 gold coins", theme),
        list_item("2nd level of jokes: 10 gold coins", theme),
        list_item("3rd level of one-liners : 20 gold coins", theme),
    ]
    .spacing(8)
}

fn blockquote_widget<'a, Message: 'a>(theme: &'a Theme) -> iced::widget::Row<'a, Message> {
    row![
        rule::vertical(2).style({
            let border = theme.palette.border;
            move |_theme| rule::Style {
                color: border,
                radius: 0.0.into(),
                fill_mode: rule::FillMode::Full,
                snap: true,
            }
        }),
        shadcn_text(
            "\"After all,\" he said, \"everyone enjoys a good joke, so it's only fair that they should pay for the privilege.\"",
            TextVariant::Body,
            theme,
        )
        .font(Font {
            style: font::Style::Italic,
            ..Font::DEFAULT
        })
        .width(Length::Fill),
    ]
    .spacing(24)
    .align_y(Alignment::Start)
}

fn table_view<'a, Message: 'a>(
    theme: &'a Theme,
    with_header: bool,
) -> iced::widget::Column<'a, Message> {
    let header = table_row(theme, ["King's Treasury", "People's happiness"], true, None);

    let body = column![
        table_row(theme, ["Empty", "Overflowing"], false, None),
        table_row(
            theme,
            ["Modest", "Satisfied"],
            false,
            Some(theme.palette.muted)
        ),
        table_row(theme, ["Full", "Ecstatic"], false, None),
    ]
    .spacing(0)
    .width(Length::Fill);

    if with_header {
        column![header, body].spacing(0).width(Length::Fill)
    } else {
        body
    }
}

fn table_row<'a, Message: 'a>(
    theme: &'a Theme,
    cells: [&'a str; 2],
    bold: bool,
    background: Option<Color>,
) -> iced::widget::Row<'a, Message> {
    let font = if bold {
        Font {
            weight: font::Weight::Bold,
            ..Font::DEFAULT
        }
    } else {
        Font::DEFAULT
    };
    let foreground = theme.palette.foreground;

    row![
        table_cell(
            theme,
            iced_text(cells[0])
                .size(14)
                .font(font)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(foreground),
                }),
            background,
        ),
        table_cell(
            theme,
            iced_text(cells[1])
                .size(14)
                .font(font)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(foreground),
                }),
            background,
        ),
    ]
    .spacing(0)
}

fn table_cell<'a, Message>(
    theme: &'a Theme,
    content: iced::widget::Text<'a>,
    background: Option<Color>,
) -> iced::widget::Container<'a, Message> {
    let border = Border {
        radius: 0.0.into(),
        width: 1.0,
        color: theme.palette.border,
    };
    let background = background.map(Background::Color);

    container(content)
        .padding([6.0, 8.0])
        .width(Length::FillPortion(1))
        .style(move |_theme| iced::widget::container::Style {
            background,
            border,
            ..iced::widget::container::Style::default()
        })
}

fn inline_code<'a>(theme: &Theme) -> Rich<'a, (), Message> {
    Rich::<(), Message>::with_spans(vec![
        Span::new("@radix-ui/react-alert-dialog")
            .size(14)
            .font(Font::MONOSPACE)
            .color(theme.palette.foreground)
            .background(theme.palette.input)
            .border(Border {
                radius: theme.radius.sm.into(),
                width: 1.0,
                color: theme.palette.border,
            })
            .padding([2.0, 4.0]),
    ])
}
