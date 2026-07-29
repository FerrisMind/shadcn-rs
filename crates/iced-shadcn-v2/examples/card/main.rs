//! Interactive playground for `iced-shadcn-v2::Card` + `shadcn-common` theme knobs.
//!
//! The layout follows the other v2 examples: shared theme controls first,
//! component-specific controls next, then a live preview and focused galleries
//! for the supported Card compositions.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example card`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, Card, CardAction, CardContent, CardDescription,
    CardFooter, CardHeader, CardRadius, CardSize, CardTitle, FontId, StyleId, Theme, ThemeMode,
    fonts, iced_font,
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
    size: SizeOpt,
    spacing: SpacingOpt,
    radius: RadiusOpt,
    notice: String,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Size(SizeOpt),
    Spacing(SpacingOpt),
    Radius(RadiusOpt),
    Action,
    Reset,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            size: SizeOpt::Default,
            spacing: SpacingOpt::Theme,
            radius: RadiusOpt::Theme,
            notice: "Ready to create a project".to_owned(),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Card".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => {
                self.theme = self.theme.clone().with_style(style.0);
            }
            Message::Base(base) => {
                self.theme = self.theme.clone().with_base(base.0);
            }
            Message::Accent(accent) => {
                self.theme = self.theme.clone().with_accent(accent.into_option());
            }
            Message::Mode(mode) => {
                self.theme = self.theme.clone().with_mode(mode.0);
            }
            Message::Size(size) => self.size = size,
            Message::Spacing(spacing) => self.spacing = spacing,
            Message::Radius(radius) => self.radius = radius,
            Message::Action => {
                self.notice = "Action pressed — the card emitted a message".to_owned();
            }
            Message::Reset => {
                self.notice = "Ready to create a project".to_owned();
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        let controls = column![
            section_label("Theme", theme),
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
                "Accent",
                &ACCENTS,
                Some(AccentOpt::from_option(theme.accent())),
                Message::Accent,
                theme,
            ),
            control_select(
                "Mode",
                &MODES,
                Some(Labelled(theme.mode())),
                Message::Mode,
                theme,
            ),
            section_label("Card knobs", theme),
            control_select("Size", &SIZES, Some(self.size), Message::Size, theme),
            control_select(
                "Spacing",
                &SPACINGS,
                Some(self.spacing),
                Message::Spacing,
                theme,
            ),
            control_select("Radius", &RADII, Some(self.radius), Message::Radius, theme,),
            Button::text("Reset demo", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Reset),
        ]
        .spacing(8);

        let preview = column![
            section_label("Preview", theme),
            self.form_card(theme).width(Length::Fill),
            text(format!(
                "size={} · spacing={} · radius={}",
                self.size, self.spacing, self.radius
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(palette.muted_foreground),
            text(&self.notice)
                .size(13)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.foreground),
        ]
        .spacing(8);

        let composition = row![
            self.form_card(theme).width(Length::Fixed(420.0)),
            self.media_card(theme).width(Length::Fixed(420.0)),
        ]
        .spacing(24)
        .align_y(Alignment::Start)
        .wrap();

        let variations = row![
            self.bordered_card(theme).width(Length::Fixed(420.0)),
            self.square_card(theme).width(Length::Fixed(420.0)),
        ]
        .spacing(24)
        .align_y(Alignment::Start)
        .wrap();

        let style_buttons = row![
            style_button(StyleId::Vega, theme),
            style_button(StyleId::Nova, theme),
            style_button(StyleId::Maia, theme),
            style_button(StyleId::Lyra, theme),
            style_button(StyleId::Mira, theme),
            style_button(StyleId::Luma, theme),
            style_button(StyleId::Sera, theme),
            style_button(StyleId::Rhea, theme),
        ]
        .spacing(8)
        .wrap();

        let content = column![
            text("iced-shadcn-v2 Card")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("shadcn-svelte parity: header, title, description, action, content, footer")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            controls,
            preview,
            section_label("Composition", theme),
            composition,
            section_label("Borders, background, and radius", theme),
            variations,
            section_label("All style packs", theme),
            style_buttons,
        ]
        .spacing(16)
        .max_width(960)
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
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..container::Style::default()
        })
        .into()
    }

    fn card<'a>(&self, theme: &'a Theme) -> Card<'a, Message> {
        let card = Card::new(theme)
            .size(self.size.into())
            .radius(self.radius.into());

        match self.spacing {
            SpacingOpt::Theme => card,
            SpacingOpt::Compact => card.spacing(12.0),
            SpacingOpt::Relaxed => card.spacing(24.0),
            SpacingOpt::Custom => card.spacing(32.0),
        }
    }

    fn form_card<'a>(&self, theme: &'a Theme) -> Card<'a, Message> {
        let header = CardHeader::new(theme)
            .title(CardTitle::text("Create project", theme))
            .description(CardDescription::text(
                "Deploy your new project in one click.",
                theme,
            ))
            .action(CardAction::new(
                Button::text("Docs", theme)
                    .variant(ButtonVariant::Link)
                    .on_press(Message::Action),
            ));

        let form = column![
            field("Name", "shadcn-rs", theme),
            field("Framework", "SvelteKit", theme),
        ]
        .spacing(12)
        .width(Length::Fill);

        self.card(theme)
            .header(header)
            .content(CardContent::new(theme).push(form))
            .footer(
                CardFooter::new(theme)
                    .column()
                    .spacing(8.0)
                    .push(
                        Button::text("Create project", theme)
                            .full_width()
                            .on_press(Message::Action),
                    )
                    .push(
                        Button::text("Cancel", theme)
                            .variant(ButtonVariant::Outline)
                            .full_width()
                            .on_press(Message::Action),
                    ),
            )
    }

    fn media_card<'a>(&self, theme: &'a Theme) -> Card<'a, Message> {
        let palette = theme.palette;
        let media = container(
            text("Edge-to-edge first child")
                .size(20)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.card_foreground),
        )
        .height(Length::Fixed(132.0))
        .width(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(palette.muted)),
            text_color: Some(palette.foreground),
            ..container::Style::default()
        });

        self.card(theme)
            .top_padding(0.0)
            .push(media)
            .header(
                CardHeader::new(theme)
                    .title(CardTitle::text("Media card", theme))
                    .description(CardDescription::text(
                        "The first child can touch the rounded card edge.",
                        theme,
                    )),
            )
            .footer(
                CardFooter::new(theme)
                    .border_top()
                    .justify_end()
                    .push(Button::text("View event", theme).on_press(Message::Action)),
            )
    }

    fn bordered_card<'a>(&self, theme: &'a Theme) -> Card<'a, Message> {
        self.card(theme)
            .header(
                CardHeader::new(theme)
                    .title(CardTitle::text("Terms of Service", theme))
                    .description(CardDescription::text(
                        "Review the terms before accepting the agreement.",
                        theme,
                    ))
                    .border_bottom(),
            )
            .content(
                CardContent::new(theme).push(
                    text("The content section accepts arbitrary iced elements and keeps its card inset.")
                        .size(14)
                        .color(theme.palette.muted_foreground),
                ),
            )
            .footer(
                CardFooter::new(theme)
                    .border_top()
                    .background(theme.palette.muted)
                    .justify_end()
                    .push(
                        Button::text("Accept", theme)
                            .variant(ButtonVariant::Default)
                            .on_press(Message::Action),
                    ),
            )
    }

    fn square_card<'a>(&self, theme: &'a Theme) -> Card<'a, Message> {
        self.card(theme)
            .radius(CardRadius::None)
            .header(
                CardHeader::new(theme)
                    .title(CardTitle::text("Square override", theme))
                    .description(CardDescription::text(
                        "This card overrides the selected radius locally.",
                        theme,
                    )),
            )
            .content(CardContent::new(theme).push(text("Arbitrary child content is accepted.")))
    }
}

fn field<'a>(label: &'static str, value: &'static str, theme: &'a Theme) -> Element<'a, Message> {
    let palette = theme.palette;

    column![
        text(label)
            .size(13)
            .font(iced_font(theme.font_pack().sans))
            .color(palette.foreground),
        container(
            text(value)
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
        )
        .padding(8)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(palette.background)),
            border: Border {
                color: palette.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..container::Style::default()
        }),
    ]
    .spacing(6)
    .width(Length::Fill)
    .into()
}

fn style_button(style: StyleId, theme: &Theme) -> Element<'_, Message> {
    Button::text(style.as_str(), theme)
        .variant(ButtonVariant::Outline)
        .on_press(Message::Style(Labelled(style)))
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
    let palette = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(80)
            .font(font)
            .color(palette.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(220.0))
            .style(move |_theme, _status| pick_list::Style {
                background: Background::Color(palette.background),
                text_color: palette.foreground,
                placeholder_color: palette.muted_foreground,
                handle_color: palette.muted_foreground,
                border: Border {
                    color: palette.input,
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
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(theme.palette.muted_foreground)
        .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Labelled<T>(T);

impl fmt::Display for Labelled<StyleId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<BaseColor> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl fmt::Display for Labelled<ThemeMode> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccentOpt {
    None,
    Color(AccentColor),
}

impl AccentOpt {
    const fn from_option(accent: Option<AccentColor>) -> Self {
        match accent {
            None => Self::None,
            Some(color) => Self::Color(color),
        }
    }

    const fn into_option(self) -> Option<AccentColor> {
        match self {
            Self::None => None,
            Self::Color(color) => Some(color),
        }
    }
}

impl fmt::Display for AccentOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => formatter.write_str("none"),
            Self::Color(color) => formatter.write_str(color.as_str()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SizeOpt {
    Default,
    Sm,
}

impl From<SizeOpt> for CardSize {
    fn from(size: SizeOpt) -> Self {
        match size {
            SizeOpt::Default => Self::Default,
            SizeOpt::Sm => Self::Sm,
        }
    }
}

impl fmt::Display for SizeOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Default => "default",
            Self::Sm => "sm",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpacingOpt {
    Theme,
    Compact,
    Relaxed,
    Custom,
}

impl fmt::Display for SpacingOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Theme => "theme",
            Self::Compact => "12 px",
            Self::Relaxed => "24 px",
            Self::Custom => "32 px",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RadiusOpt {
    Theme,
    None,
    Small,
    Medium,
    Large,
    Xl,
    Full,
}

impl From<RadiusOpt> for CardRadius {
    fn from(radius: RadiusOpt) -> Self {
        match radius {
            RadiusOpt::Theme => Self::Theme,
            RadiusOpt::None => Self::None,
            RadiusOpt::Small => Self::Small,
            RadiusOpt::Medium => Self::Medium,
            RadiusOpt::Large => Self::Large,
            RadiusOpt::Xl => Self::Xl,
            RadiusOpt::Full => Self::Full,
        }
    }
}

impl fmt::Display for RadiusOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Theme => "theme",
            Self::None => "none",
            Self::Small => "sm",
            Self::Medium => "md",
            Self::Large => "lg",
            Self::Xl => "xl",
            Self::Full => "full",
        })
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

const ACCENTS: [AccentOpt; 6] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Violet),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const SIZES: [SizeOpt; 2] = [SizeOpt::Default, SizeOpt::Sm];

const SPACINGS: [SpacingOpt; 4] = [
    SpacingOpt::Theme,
    SpacingOpt::Compact,
    SpacingOpt::Relaxed,
    SpacingOpt::Custom,
];

const RADII: [RadiusOpt; 7] = [
    RadiusOpt::Theme,
    RadiusOpt::None,
    RadiusOpt::Small,
    RadiusOpt::Medium,
    RadiusOpt::Large,
    RadiusOpt::Xl,
    RadiusOpt::Full,
];
