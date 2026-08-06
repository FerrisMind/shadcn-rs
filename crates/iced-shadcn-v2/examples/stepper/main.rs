//! Interactive playground for `iced-shadcn-v2::Stepper` and its composed controls.
//!
//! Run with `cargo run -p iced-shadcn-v2 --example stepper`.

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, FontHeading, FontId, RadiusId, Stepper,
    StepperDescription, StepperIndicator, StepperItem, StepperNext, StepperOrientation,
    StepperPrevious, StepperTitle, StepperTrigger, StyleId, Theme, ThemeMode, fonts, iced_font,
};

const STEPS: [(&str, &str, &str); 4] = [
    (
        "Search",
        "Find a starting point",
        "Choose the source you want to use.",
    ),
    (
        "Download",
        "Bring it local",
        "Download the selected source to your workspace.",
    ),
    (
        "Configure",
        "Make it yours",
        "Review the settings before continuing.",
    ),
    (
        "Complete",
        "Ready to go",
        "Everything is configured and ready.",
    ),
];

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
    step: usize,
    vertical: bool,
    icon_indicators: bool,
    show_labels: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Font(Labelled<FontId>),
    Heading(Labelled<FontHeading>),
    Radius(Labelled<RadiusId>),
    StepChanged(usize),
    ToggleOrientation,
    ToggleIndicators,
    ToggleLabels,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            step: 1,
            vertical: false,
            icon_indicators: false,
            show_labels: true,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Stepper".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => self.theme = self.theme.clone().with_style(style.0),
            Message::Base(base) => self.theme = self.theme.clone().with_base(base.0),
            Message::Accent(accent) => {
                self.theme = self.theme.clone().with_accent(accent.into_option())
            }
            Message::Mode(mode) => self.theme = self.theme.clone().with_mode(mode.0),
            Message::Font(font) => self.theme = self.theme.clone().with_font(font.0),
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0)
            }
            Message::Radius(radius) => self.theme = self.theme.clone().with_radius(radius.0),
            Message::StepChanged(step) => self.step = step.clamp(1, STEPS.len()),
            Message::ToggleOrientation => self.vertical = !self.vertical,
            Message::ToggleIndicators => self.icon_indicators = !self.icon_indicators,
            Message::ToggleLabels => self.show_labels = !self.show_labels,
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        let controls = column![
            section_label("Theme (shadcn-common)", palette.muted_foreground, theme),
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
            control_select(
                "Font",
                &FONTS,
                Some(Labelled(theme.font_id())),
                Message::Font,
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
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
            text(format!(
                "step={} · orientation={} · indicators={} · labels={}",
                self.step,
                if self.vertical {
                    "vertical"
                } else {
                    "horizontal"
                },
                if self.icon_indicators {
                    "icons"
                } else {
                    "numbers"
                },
                self.show_labels,
            ))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(palette.muted_foreground),
        ]
        .spacing(8);

        let toggles = row![
            Button::text(
                if self.vertical {
                    "Use horizontal"
                } else {
                    "Use vertical"
                },
                theme,
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleOrientation),
            Button::text(
                if self.icon_indicators {
                    "Use numbers"
                } else {
                    "Use icons"
                },
                theme,
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleIndicators),
            Button::text(
                if self.show_labels {
                    "Hide labels"
                } else {
                    "Show labels"
                },
                theme,
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleLabels),
        ]
        .spacing(8)
        .wrap();

        let stepper = self.build_stepper(theme);
        let preview = column![
            section_label("Preview", palette.muted_foreground, theme),
            container(stepper)
                .padding(24)
                .width(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(palette.card)),
                    border: Border {
                        color: palette.border,
                        width: 1.0,
                        radius: theme.radius_scale().lg_px.into(),
                    },
                    ..container::Style::default()
                }),
            text(
                "Click an item, use the arrow keys after clicking the rail, or use Previous / Next."
            )
            .size(13)
            .color(palette.muted_foreground),
        ]
        .spacing(12);

        let content = column![
            text("iced-shadcn-v2 Stepper")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("A controlled, composable step navigator ported from shadcn-svelte-extras.")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            controls,
            toggles,
            preview,
        ]
        .spacing(16)
        .max_width(1100)
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

    fn build_stepper<'a>(&'a self, theme: &'a Theme) -> Stepper<'a, Message> {
        let items = STEPS
            .iter()
            .enumerate()
            .map(|(index, (_, title, description))| {
                let indicator = if self.icon_indicators {
                    StepperIndicator::text(INDICATORS[index], theme)
                } else {
                    StepperIndicator::text((index + 1).to_string(), theme)
                };

                let mut trigger = StepperTrigger::new(theme)
                    .indicator(indicator)
                    .width(Length::Fixed(150.0));
                if self.show_labels {
                    trigger = trigger
                        .title(StepperTitle::text(*title, theme))
                        .description(StepperDescription::text(*description, theme));
                }

                StepperItem::with_id(format!("step-{}", index + 1), trigger)
            });

        Stepper::with_items(theme, items)
            .step(self.step)
            .orientation(if self.vertical {
                StepperOrientation::Vertical
            } else {
                StepperOrientation::Horizontal
            })
            .on_step_change(Message::StepChanged)
            .previous(StepperPrevious::text("Previous", theme))
            .next(StepperNext::text("Next", theme))
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
    let palette = theme.palette;
    let font = iced_font(theme.font_pack().sans);

    row![
        text(label)
            .size(13)
            .width(72)
            .font(font)
            .color(palette.muted_foreground),
        pick_list(options, selected, on_select)
            .text_size(13)
            .font(font)
            .width(Length::Fixed(220.0))
            .style(move |_theme, _status| iced::widget::pick_list::Style {
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

fn section_label<'a>(label: &'static str, color: Color, theme: &Theme) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(theme.font_pack().heading))
        .color(color)
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

impl fmt::Display for Labelled<FontId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<FontHeading> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.title())
    }
}

impl fmt::Display for Labelled<RadiusId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.label())
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

const INDICATORS: [&str; 4] = ["⌕", "↓", "</>", "✓"];

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

const ACCENTS: [AccentOpt; 18] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Cyan),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Fuchsia),
    AccentOpt::Color(AccentColor::Green),
    AccentOpt::Color(AccentColor::Indigo),
    AccentOpt::Color(AccentColor::Lime),
    AccentOpt::Color(AccentColor::Orange),
    AccentOpt::Color(AccentColor::Pink),
    AccentOpt::Color(AccentColor::Purple),
    AccentOpt::Color(AccentColor::Red),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Sky),
    AccentOpt::Color(AccentColor::Teal),
    AccentOpt::Color(AccentColor::Violet),
    AccentOpt::Color(AccentColor::Yellow),
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
