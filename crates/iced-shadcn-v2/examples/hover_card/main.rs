//! Interactive playground for `iced-shadcn-v2::HoverCard` + `shadcn-common` theme knobs.
//!
//! Mirrors the shadcn-svelte hover-card demos (the "@sveltejs" link trigger
//! with an avatar profile card, and the sides row) and adds rows for
//! delays, dismiss behaviors, and controlled open state.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example hover_card`

use std::fmt;
use std::time::Duration;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    Avatar, BaseColor, Button, ButtonVariant, FontHeading, FontId, FontPack, HoverCard,
    HoverCardAlign, HoverCardSide, RadiusId, StyleId, Theme, ThemeMode, fonts, iced_font,
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
    controlled_open: bool,
    open_changes: u32,
    last_open: Option<bool>,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Heading(Labelled<FontHeading>),
    Font(Labelled<FontId>),
    Radius(Labelled<RadiusId>),
    ToggleControlled,
    ControlledOpenChanged(bool),
    OpenChanged(bool),
    Pressed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            controlled_open: false,
            open_changes: 0,
            last_open: None,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 HoverCard".to_owned()
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
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0);
            }
            Message::Font(font) => {
                self.theme = self.theme.clone().with_font(font.0);
            }
            Message::Radius(radius) => {
                self.theme = self.theme.clone().with_radius(radius.0);
            }
            Message::ToggleControlled => {
                self.controlled_open = !self.controlled_open;
            }
            Message::ControlledOpenChanged(open) => {
                self.controlled_open = open;
            }
            Message::OpenChanged(open) => {
                self.open_changes += 1;
                self.last_open = Some(open);
            }
            Message::Pressed => {}
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;

        let controls = column![
            section_label(
                "Theme (shadcn-common)",
                p.muted_foreground,
                theme.font_pack()
            ),
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
            control_select(
                "Heading",
                &HEADINGS,
                Some(Labelled(theme.font_heading())),
                Message::Heading,
                theme,
            ),
            control_select(
                "Font",
                &FONTS,
                Some(Labelled(theme.font_id())),
                Message::Font,
                theme,
            ),
            control_select(
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
        ]
        .spacing(8);

        // The shadcn-svelte hover-card demo: "@sveltejs" link trigger with
        // an avatar profile card (w-80).
        let demo = row![
            HoverCard::new(
                Button::text("@sveltejs", theme)
                    .variant(ButtonVariant::Link)
                    .on_press(Message::Pressed),
                profile_card(theme),
                theme,
            )
            .width(320.0)
        ]
        .spacing(12);

        // The "Sides" demo of the create playground: openDelay/closeDelay
        // of 100 ms so the cards respond quickly.
        let sides = row![
            side_demo("Top", HoverCardSide::Top, theme),
            side_demo("Right", HoverCardSide::Right, theme),
            side_demo("Bottom", HoverCardSide::Bottom, theme),
            side_demo("Left", HoverCardSide::Left, theme),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let alignments = row![
            align_demo("Start", HoverCardAlign::Start, theme),
            align_demo("Center", HoverCardAlign::Center, theme),
            align_demo("End", HoverCardAlign::End, theme),
        ]
        .spacing(24)
        .align_y(Alignment::Center)
        .wrap();

        let behaviors = row![
            HoverCard::text(
                Button::text("Default delays", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                "Opens after 700 ms, closes 300 ms after leaving.",
                theme,
            ),
            HoverCard::text(
                Button::text("Instant", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                "openDelay = 0, closeDelay = 0",
                theme,
            )
            .open_delay(Duration::ZERO)
            .close_delay(Duration::ZERO),
            HoverCard::text(
                Button::text("Disabled", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                "You should never see this",
                theme,
            )
            .open_delay(Duration::from_millis(100))
            .disabled(true),
            HoverCard::text(
                Button::text("sideOffset 12", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                "Opens with a 12 px gap",
                theme,
            )
            .open_delay(Duration::from_millis(100))
            .close_delay(Duration::from_millis(100))
            .side_offset(12.0),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let controlled = column![
            row![
                Button::text(
                    if self.controlled_open {
                        "Force close"
                    } else {
                        "Force open"
                    },
                    theme,
                )
                .variant(ButtonVariant::Default)
                .on_press(Message::ToggleControlled),
                HoverCard::text(
                    Button::text("Controlled trigger", theme)
                        .variant(ButtonVariant::Outline)
                        .on_press(Message::Pressed),
                    "Controlled by app state",
                    theme,
                )
                .open(self.controlled_open)
                .on_open_change(Message::ControlledOpenChanged),
                HoverCard::text(
                    Button::text("Watch onOpenChange", theme)
                        .variant(ButtonVariant::Outline)
                        .on_press(Message::Pressed),
                    "Every open and close request is reported below.",
                    theme,
                )
                .open_delay(Duration::from_millis(100))
                .close_delay(Duration::from_millis(100))
                .on_open_change(Message::OpenChanged),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
            text(format!(
                "onOpenChange fired {} time(s), last = {}",
                self.open_changes,
                self.last_open
                    .map_or_else(|| "—".to_owned(), |open| open.to_string()),
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.muted_foreground),
        ]
        .spacing(12);

        let content = column![
            text("iced-shadcn-v2 HoverCard")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("shadcn-svelte HoverCard.Root / Trigger / Content (bits-ui link preview), ported onto an iced overlay")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            section_label("Basic (hover the link)", p.muted_foreground, theme.font_pack()),
            demo,
            section_label("Sides", p.muted_foreground, theme.font_pack()),
            sides,
            section_label("Alignments", p.muted_foreground, theme.font_pack()),
            alignments,
            section_label(
                "Delays / disabled / offsets",
                p.muted_foreground,
                theme.font_pack()
            ),
            behaviors,
            section_label(
                "Controlled open + onOpenChange",
                p.muted_foreground,
                theme.font_pack()
            ),
            controlled,
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
            background: Some(Background::Color(p.background)),
            text_color: Some(p.foreground),
            ..container::Style::default()
        })
        .into()
    }
}

/// The profile card of the shadcn-svelte demo: avatar on the left, name,
/// description, and a "Joined" footnote on the right.
fn profile_card(theme: &Theme) -> Element<'_, Message> {
    let p = &theme.palette;
    let pack = theme.font_pack();

    row![
        Avatar::new(theme).fallback_text("SK"),
        column![
            text("@sveltejs")
                .size(14)
                .font(semibold(pack.sans))
                .color(p.popover_foreground),
            text("Cybernetically enhanced web apps.")
                .size(14)
                .font(iced_font(pack.sans))
                .color(p.popover_foreground),
            text("Joined September 2022")
                .size(12)
                .font(iced_font(pack.sans))
                .color(p.muted_foreground),
        ]
        .spacing(4),
    ]
    .spacing(16)
    .into()
}

fn semibold(font: FontId) -> iced::Font {
    let mut font = iced_font(font);
    font.weight = iced::font::Weight::Semibold;
    font
}

fn side_demo<'a>(
    label: &'static str,
    side: HoverCardSide,
    theme: &'a Theme,
) -> Element<'a, Message> {
    HoverCard::text(
        Button::text(label, theme)
            .variant(ButtonVariant::Outline)
            .on_press(Message::Pressed),
        format!(
            "This hover card appears on the {} side of the trigger.",
            label.to_lowercase()
        ),
        theme,
    )
    .side(side)
    .open_delay(Duration::from_millis(100))
    .close_delay(Duration::from_millis(100))
    .into()
}

fn align_demo<'a>(
    label: &'static str,
    align: HoverCardAlign,
    theme: &'a Theme,
) -> Element<'a, Message> {
    HoverCard::text(
        Button::text(label, theme)
            .variant(ButtonVariant::Secondary)
            .on_press(Message::Pressed),
        format!("Aligned to {}", label.to_lowercase()),
        theme,
    )
    .align(align)
    .open_delay(Duration::from_millis(100))
    .close_delay(Duration::from_millis(100))
    .width(180.0)
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
            .width(72)
            .font(font)
            .color(p.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(200.0))
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

fn section_label<'a>(label: &'static str, color: Color, pack: FontPack) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(pack.heading))
        .color(color)
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

impl fmt::Display for Labelled<FontId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<FontHeading> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<RadiusId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.label())
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

const FONTS: [Labelled<FontId>; 5] = [
    Labelled(FontId::Geist),
    Labelled(FontId::Inter),
    Labelled(FontId::InstrumentSerif),
    Labelled(FontId::GeistMono),
    Labelled(FontId::JetBrainsMono),
];

const HEADINGS: [Labelled<FontHeading>; 6] = [
    Labelled(FontHeading::Inherit),
    Labelled(FontHeading::Font(FontId::Geist)),
    Labelled(FontHeading::Font(FontId::Inter)),
    Labelled(FontHeading::Font(FontId::InstrumentSerif)),
    Labelled(FontHeading::Font(FontId::GeistMono)),
    Labelled(FontHeading::Font(FontId::JetBrainsMono)),
];

const RADII: [Labelled<RadiusId>; 5] = [
    Labelled(RadiusId::Default),
    Labelled(RadiusId::None),
    Labelled(RadiusId::Small),
    Labelled(RadiusId::Medium),
    Labelled(RadiusId::Large),
];
