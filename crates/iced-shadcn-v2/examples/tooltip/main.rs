//! Interactive playground for `iced-shadcn-v2::Tooltip` + `shadcn-common` theme knobs.
//!
//! Mirrors the shadcn-svelte tooltip demo (outline "Hover" trigger with an
//! "Add to library" bubble) and adds rows for sides, alignment, delay,
//! controlled open state, and custom content.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example tooltip`

use std::fmt;
use std::time::Duration;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    BaseColor, Button, ButtonVariant, FontHeading, FontId, FontPack, Kbd, RadiusId, StyleId, Theme,
    ThemeMode, Tooltip, TooltipAlign, TooltipSide, fonts, iced_font,
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
        "iced-shadcn-v2 Tooltip".to_owned()
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

        // The shadcn-svelte tooltip demo: outline trigger + text content.
        let demo = row![Tooltip::text(
            Button::text("Hover", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::Pressed),
            "Add to library",
            theme,
        )]
        .spacing(12);

        let sides = row![
            Tooltip::text(
                Button::text("Top", theme)
                    .variant(ButtonVariant::Outline)
                    .on_press(Message::Pressed),
                "Tooltip on top",
                theme,
            )
            .side(TooltipSide::Top),
            Tooltip::text(
                Button::text("Right", theme)
                    .variant(ButtonVariant::Outline)
                    .on_press(Message::Pressed),
                "Tooltip on right",
                theme,
            )
            .side(TooltipSide::Right),
            Tooltip::text(
                Button::text("Bottom", theme)
                    .variant(ButtonVariant::Outline)
                    .on_press(Message::Pressed),
                "Tooltip on bottom",
                theme,
            )
            .side(TooltipSide::Bottom),
            Tooltip::text(
                Button::text("Left", theme)
                    .variant(ButtonVariant::Outline)
                    .on_press(Message::Pressed),
                "Tooltip on left",
                theme,
            )
            .side(TooltipSide::Left),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let aligns = row![
            Tooltip::text(
                Button::text("Start", theme)
                    .variant(ButtonVariant::Secondary)
                    .on_press(Message::Pressed),
                "align = start",
                theme,
            )
            .align(TooltipAlign::Start),
            Tooltip::text(
                Button::text("Center", theme)
                    .variant(ButtonVariant::Secondary)
                    .on_press(Message::Pressed),
                "align = center",
                theme,
            )
            .align(TooltipAlign::Center),
            Tooltip::text(
                Button::text("End", theme)
                    .variant(ButtonVariant::Secondary)
                    .on_press(Message::Pressed),
                "align = end",
                theme,
            )
            .align(TooltipAlign::End),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let behaviors = row![
            Tooltip::text(
                Button::text("Delay 700ms", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                "Opened after a 700 ms hover delay",
                theme,
            )
            .delay(Duration::from_millis(700)),
            Tooltip::text(
                Button::text("No arrow", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                "sideOffset = 6, arrow hidden",
                theme,
            )
            .arrow(false)
            .side_offset(6.0),
            Tooltip::text(
                Button::text("Disabled", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                "You should never see this",
                theme,
            )
            .disabled(true),
            Tooltip::text(
                Button::text("Long content", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                "Tooltips wrap once they reach the max-w-xs limit of 320 \
                 pixels, exactly like the web component does with longer \
                 descriptions.",
                theme,
            ),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let controlled = column![
            row![
                Tooltip::text(
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
                    "Controlled by app state",
                    theme,
                )
                .open(self.controlled_open)
                .on_open_change(Message::OpenChanged),
                Tooltip::new(
                    Button::text("Custom content", theme)
                        .variant(ButtonVariant::Outline)
                        .on_press(Message::Pressed),
                    row![
                        text("Print").size(12),
                        Element::<Message>::from(Kbd::text("⌘P", theme)),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                    theme,
                )
                .side(TooltipSide::Bottom),
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
            text("iced-shadcn-v2 Tooltip")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("shadcn-svelte Tooltip.Root / Trigger / Content + arrow, ported onto an iced overlay")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            section_label("Demo", p.muted_foreground, theme.font_pack()),
            demo,
            section_label("Sides", p.muted_foreground, theme.font_pack()),
            sides,
            section_label("Alignment", p.muted_foreground, theme.font_pack()),
            aligns,
            section_label(
                "Delay / arrow / disabled / wrapping",
                p.muted_foreground,
                theme.font_pack()
            ),
            behaviors,
            section_label(
                "Controlled open + custom content",
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
