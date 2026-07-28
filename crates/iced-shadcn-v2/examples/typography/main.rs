//! Interactive playground for `iced-shadcn-v2` typography.
//!
//! Reproduces the shadcn-svelte typography demo (“The Joke Tax Chronicles”):
//! h1–h4, lead, paragraphs, blockquote, list, table, inline code, and the
//! small/large/muted samples — with live style-pack / base / mode switching.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example typography`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Element, Length, Task};

use iced_shadcn_v2::{
    BaseColor, FontId, StyleId, Theme, ThemeMode, Typography, TypographyList, TypographyTable,
    TypographyVariant, fonts, iced_font,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Typography".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => {
                self.theme = self.theme.clone().with_style(style.0);
            }
            Message::Base(base) => {
                self.theme = self.theme.clone().with_base(base.0);
            }
            Message::Mode(mode) => {
                self.theme = self.theme.clone().with_mode(mode.0);
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;

        let controls = column![
            section_label("Theme (shadcn-common)", theme),
            control_select(
                "Style",
                &STYLES,
                Some(Labelled(theme.style_id())),
                Message::Style,
                theme,
            ),
            control_select(
                "Base",
                &BASES,
                Some(Labelled(theme.base())),
                Message::Base,
                theme,
            ),
            control_select(
                "Mode",
                &MODES,
                Some(Labelled(theme.mode())),
                Message::Mode,
                theme,
            ),
        ]
        .spacing(8);

        let article = demo_article(theme);

        let samples = column![
            section_label("Standalone samples", theme),
            sample_card(
                "inline-code",
                Typography::inline_code("@lucide/svelte", theme),
                theme,
            ),
            sample_card(
                "lead",
                Typography::lead(
                    "A modal dialog that interrupts the user with important content and expects \
                     a response.",
                    theme,
                ),
                theme,
            ),
            sample_card(
                "large",
                Typography::large("Are you sure absolutely sure?", theme),
                theme,
            ),
            sample_card("small", Typography::small("Email address", theme), theme),
            sample_card(
                "muted",
                Typography::muted("Enter your email address.", theme),
                theme,
            ),
            sample_card(
                "h1 at lg:text-5xl",
                Typography::h1("Taxing Laughter", theme).size(48.0),
                theme,
            ),
        ]
        .spacing(8);

        let content = column![controls, article, samples]
            .spacing(24)
            .max_width(720)
            .padding(8);

        container(
            scrollable(
                container(content)
                    .width(Length::Fill)
                    .center_x(Length::Fill)
                    .padding(24),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(p.background)),
            text_color: Some(p.foreground),
            ..container::Style::default()
        })
        .into()
    }
}

/// The full typography-demo article, using variant default margins.
fn demo_article(theme: &Theme) -> Element<'_, Message> {
    column![
        Typography::h1("Taxing Laughter: The Joke Tax Chronicles", theme),
        Typography::lead(
            "Once upon a time, in a far-off land, there was a very lazy king who spent all day \
             lounging on his throne. One day, his advisors came to him with a problem: the \
             kingdom was running out of money.",
            theme,
        )
        .default_margin(true),
        Typography::h2("The King's Plan", theme).default_margin(true),
        Typography::p(
            "The king thought long and hard, and finally came up with a brilliant plan: he \
             would tax the jokes in the kingdom.",
            theme,
        )
        .default_margin(true),
        Typography::blockquote(
            "\"After all,\" he said, \"everyone enjoys a good joke, so it's only fair that they \
             should pay for the privilege.\"",
            theme,
        )
        .default_margin(true),
        Typography::h3("The Joke Tax", theme).default_margin(true),
        Typography::p(
            "The king's subjects were not amused. They grumbled and complained, but the king \
             was firm:",
            theme,
        )
        .default_margin(true),
        spaced(
            TypographyList::new(theme)
                .item("1st level of puns: 5 gold coins")
                .item("2nd level of jokes: 10 gold coins")
                .item("3rd level of one-liners: 20 gold coins")
                .into_element(),
        ),
        Typography::p(
            "As a result, people stopped telling jokes, and the kingdom fell into a gloom. But \
             there was one person who refused to let the king's foolishness get him down: a \
             court jester named Jokester.",
            theme,
        )
        .default_margin(true),
        Typography::h3("Jokester's Revolt", theme).default_margin(true),
        Typography::p(
            "Jokester began sneaking into the castle in the middle of the night and leaving \
             jokes all over the place: under the king's pillow, in his soup, even in the royal \
             toilet. The king was furious, but he couldn't seem to stop Jokester.",
            theme,
        )
        .default_margin(true),
        Typography::p(
            "And then, one day, the people of the kingdom discovered that the jokes left by \
             Jokester were so funny that they couldn't help but laugh. And once they started \
             laughing, they couldn't stop.",
            theme,
        )
        .default_margin(true),
        Typography::h3("The People's Rebellion", theme).default_margin(true),
        Typography::p(
            "The people of the kingdom, feeling uplifted by the laughter, started to tell jokes \
             and puns again, and soon the entire kingdom was in on the joke.",
            theme,
        )
        .default_margin(true),
        table_block(theme),
        Typography::p(
            "The king, seeing how much happier his subjects were, realized the error of his \
             ways and repealed the joke tax. Jokester was declared a hero, and the kingdom \
             lived happily ever after.",
            theme,
        )
        .default_margin(true),
        Typography::p(
            "The moral of the story is: never underestimate the power of a good laugh and \
             always be careful of bad ideas.",
            theme,
        )
        .default_margin(true),
    ]
    .into()
}

/// `my-6` wrapper around the demo table.
fn table_block(theme: &Theme) -> Element<'_, Message> {
    spaced(
        TypographyTable::new(theme)
            .header(["King's Treasury", "People's happiness"])
            .row(["Empty", "Overflowing"])
            .row(["Modest", "Satisfied"])
            .row(["Full", "Ecstatic"])
            .into_element(),
    )
}

/// `mt-6` article-flow margin for blocks without a built-in margin knob.
fn spaced(body: Element<'_, Message>) -> Element<'_, Message> {
    container(body)
        .padding(iced::Padding {
            top: 24.0,
            ..iced::Padding::ZERO
        })
        .into()
}

fn sample_card<'a>(
    title: &'static str,
    body: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let p = &theme.palette;

    column![
        Typography::muted(title, theme),
        container(body.into())
            .padding(16)
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(p.card)),
                border: Border {
                    color: p.border,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..container::Style::default()
            }),
    ]
    .spacing(6)
    .into()
}

fn control_select<'a, T, F>(
    label: &'static str,
    options: &'a [T],
    selected: Option<T>,
    on_select: F,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    T: Clone + PartialEq + fmt::Display + 'a,
    F: Fn(T) -> Message + 'a,
{
    let p = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(80)
            .font(font)
            .color(p.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(220.0))
            .style(move |_theme, _status| pick_list::Style {
                background: Background::Color(p.background),
                text_color: p.foreground,
                placeholder_color: p.muted_foreground,
                handle_color: p.muted_foreground,
                border: Border {
                    color: p.input,
                    width: 1.0,
                    radius: 6.0.into(),
                },
            }),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn section_label<'a>(label: &'static str, theme: &'a Theme) -> Element<'a, Message> {
    Typography::text(label, theme)
        .variant(TypographyVariant::Large)
        .color(theme.palette.muted_foreground)
        .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Labelled<T>(T);

impl fmt::Display for Labelled<StyleId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<BaseColor> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<ThemeMode> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

const STYLES: [Labelled<StyleId>; 8] = [
    Labelled(StyleId::Vega),
    Labelled(StyleId::Nova),
    Labelled(StyleId::Maia),
    Labelled(StyleId::Lyra),
    Labelled(StyleId::Mira),
    Labelled(StyleId::Luma),
    Labelled(StyleId::Sera),
    Labelled(StyleId::Rhea),
];

const BASES: [Labelled<BaseColor>; 7] = [
    Labelled(BaseColor::Neutral),
    Labelled(BaseColor::Zinc),
    Labelled(BaseColor::Stone),
    Labelled(BaseColor::Mauve),
    Labelled(BaseColor::Mist),
    Labelled(BaseColor::Olive),
    Labelled(BaseColor::Taupe),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];
