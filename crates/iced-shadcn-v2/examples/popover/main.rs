//! Interactive playground for `iced-shadcn-v2::Popover` + `shadcn-common` theme knobs.
//!
//! Mirrors the shadcn-svelte popover demos (outline "Open Popover" trigger
//! with a Dimensions header, the with-form variant, and the alignment row)
//! and adds rows for sides, dismiss behaviors, and controlled open state.
//!
//! Popover has pack-specific `.cn-popover-*` recipes. The Style picker also
//! restyles composed Button / Input / Label through the shared [`Theme`]
//! (same composite rule as Form when a host has no pack deltas).
//!
//! Run: `cargo run -p iced-shadcn-v2 --example popover`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    BaseColor, Button, ButtonSize, ButtonVariant, FontHeading, FontId, FontPack, Input, Label,
    Popover, PopoverAlign, PopoverDescription, PopoverHeader, PopoverSide, PopoverTitle, RadiusId,
    StyleId, Theme, ThemeMode, fonts, iced_font,
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
    width_value: String,
    height_value: String,
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
    WidthChanged(String),
    HeightChanged(String),
    ToggleControlled,
    ControlledOpenChanged(bool),
    OpenChanged(bool),
    Pressed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            width_value: "100%".to_owned(),
            height_value: "25px".to_owned(),
            controlled_open: false,
            open_changes: 0,
            last_open: None,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Popover".to_owned()
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
            Message::WidthChanged(value) => {
                self.width_value = value;
            }
            Message::HeightChanged(value) => {
                self.height_value = value;
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

        // The shadcn-svelte popover demo: outline trigger, header with
        // title + description, align = start.
        let demo = row![
            Popover::new(
                Button::text("Open Popover", theme)
                    .variant(ButtonVariant::Outline)
                    .on_press(Message::Pressed),
                PopoverHeader::new(theme)
                    .title(PopoverTitle::text("Dimensions", theme))
                    .description(PopoverDescription::text(
                        "Set the dimensions for the layer.",
                        theme,
                    )),
                theme,
            )
            .align(PopoverAlign::Start)
        ]
        .spacing(12);

        // The "With Form" demo: w-64 content with two labeled inputs.
        let with_form = row![
            Popover::new(
                Button::text("Open Popover", theme)
                    .variant(ButtonVariant::Outline)
                    .on_press(Message::Pressed),
                column![
                    Element::<Message>::from(
                        PopoverHeader::new(theme)
                            .title(PopoverTitle::text("Dimensions", theme))
                            .description(PopoverDescription::text(
                                "Set the dimensions for the layer.",
                                theme,
                            ))
                    ),
                    form_field("Width", &self.width_value, Message::WidthChanged, theme),
                    form_field("Height", &self.height_value, Message::HeightChanged, theme),
                ]
                .spacing(16),
                theme,
            )
            .align(PopoverAlign::Start)
            .width(256.0)
        ]
        .spacing(12);

        // The "Alignments" demo: w-40 popovers aligned start/center/end.
        let alignments = row![
            Popover::text(
                Button::text("Start", theme)
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .on_press(Message::Pressed),
                "Aligned to start",
                theme,
            )
            .align(PopoverAlign::Start)
            .width(160.0),
            Popover::text(
                Button::text("Center", theme)
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .on_press(Message::Pressed),
                "Aligned to center",
                theme,
            )
            .align(PopoverAlign::Center)
            .width(160.0),
            Popover::text(
                Button::text("End", theme)
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .on_press(Message::Pressed),
                "Aligned to end",
                theme,
            )
            .align(PopoverAlign::End)
            .width(160.0),
        ]
        .spacing(24)
        .align_y(Alignment::Center)
        .wrap();

        let sides = row![
            side_demo("Top", PopoverSide::Top, theme),
            side_demo("Right", PopoverSide::Right, theme),
            side_demo("Bottom", PopoverSide::Bottom, theme),
            side_demo("Left", PopoverSide::Left, theme),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let behaviors = row![
            Popover::text(
                Button::text("Keeps open outside", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                "Outside clicks are ignored; press Esc or the trigger to close.",
                theme,
            )
            .close_on_click_outside(false)
            .width(220.0),
            Popover::text(
                Button::text("No Esc", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                "Esc is ignored; click outside to close.",
                theme,
            )
            .close_on_escape(false)
            .width(180.0),
            Popover::text(
                Button::text("Disabled", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                "You should never see this",
                theme,
            )
            .disabled(true),
            Popover::text(
                Button::text("sideOffset 12", theme)
                    .variant(ButtonVariant::Ghost)
                    .on_press(Message::Pressed),
                "Opens with a 12 px gap",
                theme,
            )
            .side_offset(12.0)
            .width(180.0),
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let controlled = column![
            row![
                // External control: the popover trigger must not toggle the
                // same state from its own on_press, otherwise the click
                // opens (onOpenChange) and immediately closes (on_press).
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
                Popover::text(
                    Button::text("Controlled trigger", theme)
                        .variant(ButtonVariant::Outline)
                        .on_press(Message::Pressed),
                    "Controlled by app state",
                    theme,
                )
                .open(self.controlled_open)
                .on_open_change(Message::ControlledOpenChanged)
                .width(200.0),
                Popover::text(
                    Button::text("Watch onOpenChange", theme)
                        .variant(ButtonVariant::Outline)
                        .on_press(Message::Pressed),
                    "Every open and dismiss request is reported below.",
                    theme,
                )
                .on_open_change(Message::OpenChanged)
                .width(220.0),
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
            text("iced-shadcn-v2 Popover")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("shadcn-svelte Popover.Root / Trigger / Content + Header, ported onto an iced overlay")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            section_label("Basic", p.muted_foreground, theme.font_pack()),
            demo,
            section_label("With form", p.muted_foreground, theme.font_pack()),
            with_form,
            section_label("Alignments", p.muted_foreground, theme.font_pack()),
            alignments,
            section_label("Sides", p.muted_foreground, theme.font_pack()),
            sides,
            section_label(
                "Dismiss behaviors / disabled / offsets",
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

/// One horizontal `Field` row of the with-form demo: label + input.
fn form_field<'a>(
    label: &'static str,
    value: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
    theme: &'a Theme,
) -> Element<'a, Message> {
    row![
        container(Element::<Message>::from(Label::text(label, theme)))
            .width(Length::FillPortion(1)),
        container(
            Input::new(theme)
                .value(value)
                .on_input(on_input)
                .width(Length::Fill)
        )
        .width(Length::FillPortion(1)),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn side_demo<'a>(label: &'static str, side: PopoverSide, theme: &'a Theme) -> Element<'a, Message> {
    Popover::text(
        Button::text(label, theme)
            .variant(ButtonVariant::Secondary)
            .on_press(Message::Pressed),
        format!("Popover on {}", label.to_lowercase()),
        theme,
    )
    .side(side)
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
