//! Interactive playground for `iced-shadcn-v2::Slider`.
//!
//! The layout mirrors shadcn-svelte's slider demos: a single-value slider, a
//! multi-thumb range, a vertical slider, disabled and focused states, and the
//! same control rendered under every style pack.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example slider`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, FontId, Slider, SliderOrientation, StyleId,
    Theme, ThemeMode, fonts, iced_font,
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
    volume: f32,
    price: Vec<f32>,
    balance: f32,
    temperature: f32,
    step: StepOpt,
    focused: bool,
    committed: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Volume(f32),
    Price(Vec<f32>),
    Balance(f32),
    Temperature(f32),
    Step(StepOpt),
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    ToggleFocused,
    Committed,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            volume: 33.0,
            price: vec![25.0, 75.0],
            balance: 0.0,
            temperature: 21.0,
            step: StepOpt::One,
            focused: false,
            committed: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Slider".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Volume(value) => self.volume = value,
            Message::Price(values) => self.price = values,
            Message::Balance(value) => self.balance = value,
            Message::Temperature(value) => self.temperature = value,
            Message::Step(step) => self.step = step,
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
            Message::ToggleFocused => self.focused = !self.focused,
            Message::Committed => self.committed += 1,
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
            section_label("Slider knobs", theme),
            control_select("Step", &STEPS, Some(self.step), Message::Step, theme),
            Button::text(
                if self.focused {
                    "Focus ring: on"
                } else {
                    "Focus ring: off"
                },
                theme,
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleFocused),
        ]
        .spacing(8);

        // shadcn `slider-demo`: a single thumb over `0..=100`.
        let volume = field(
            "Volume",
            format!("{:.0}", self.volume),
            Slider::new(theme)
                .value(self.volume)
                .step(self.step.into())
                .focused(self.focused)
                .on_change(Message::Volume)
                .on_release(Message::Committed)
                .width(Length::Fixed(420.0)),
            theme,
        );

        // shadcn `slider-multiple`: two thumbs that cannot cross.
        let price = field(
            "Price range",
            format!(
                "{:.0} – {:.0}",
                self.price.first().copied().unwrap_or_default(),
                self.price.last().copied().unwrap_or_default(),
            ),
            Slider::new(theme)
                .values(self.price.clone())
                .step(self.step.into())
                .focused(self.focused)
                .on_change_values(Message::Price)
                .on_release(Message::Committed)
                .width(Length::Fixed(420.0)),
            theme,
        );

        // A negative range with a continuous (step-free) travel.
        let balance = field(
            "Balance (continuous, -50..=50)",
            format!("{:.1}", self.balance),
            Slider::new(theme)
                .range(-50.0..=50.0)
                .value(self.balance)
                .continuous()
                .focused(self.focused)
                .on_change(Message::Balance)
                .width(Length::Fixed(420.0)),
            theme,
        );

        let disabled = field(
            "Disabled",
            "60".to_owned(),
            Slider::new(theme)
                .value(60.0)
                .disabled(true)
                .width(Length::Fixed(420.0)),
            theme,
        );

        // shadcn `slider-vertical`: `min-h-40` worth of vertical travel.
        let vertical = row![
            captioned(
                "vertical",
                Slider::new(theme)
                    .orientation(SliderOrientation::Vertical)
                    .range(16.0..=30.0)
                    .step(0.5)
                    .value(self.temperature)
                    .focused(self.focused)
                    .on_change(Message::Temperature)
                    .height(Length::Fixed(180.0)),
                theme,
            ),
            captioned(
                "vertical range",
                Slider::new(theme)
                    .orientation(SliderOrientation::Vertical)
                    .values(self.price.clone())
                    .step(self.step.into())
                    .focused(self.focused)
                    .on_change_values(Message::Price)
                    .height(Length::Fixed(180.0)),
                theme,
            ),
            captioned(
                "vertical disabled",
                Slider::new(theme)
                    .orientation(SliderOrientation::Vertical)
                    .value(40.0)
                    .disabled(true)
                    .height(Length::Fixed(180.0)),
                theme,
            ),
            column![
                text(format!("{:.1} °C", self.temperature))
                    .size(14)
                    .font(iced_font(theme.font_pack().sans))
                    .color(palette.foreground),
                text(format!("released {} times", self.committed))
                    .size(13)
                    .color(palette.muted_foreground),
            ]
            .spacing(6),
        ]
        .spacing(32)
        .align_y(Alignment::End);

        let accents = column![
            accent_row("theme", None, self.volume, theme),
            accent_row("blue", Some(AccentColor::Blue), self.volume, theme),
            accent_row("emerald", Some(AccentColor::Emerald), self.volume, theme),
            accent_row("rose", Some(AccentColor::Rose), self.volume, theme),
        ]
        .spacing(10);

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
            text("iced-shadcn-v2 Slider")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("shadcn-svelte parity: single & multiple thumbs, min/max/step, orientation")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            controls,
            section_label("Preview", theme),
            volume,
            price,
            balance,
            disabled,
            section_label("Vertical", theme),
            vertical,
            section_label("Accents", theme),
            accents,
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
}

fn field<'a>(
    label: &'a str,
    value: String,
    slider: Slider<'a, Message>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        row![
            text(label)
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(theme.palette.foreground)
                .width(Length::Fill),
            text(value)
                .size(13)
                .font(iced_font(theme.font_pack().mono))
                .color(theme.palette.muted_foreground),
        ]
        .width(Length::Fixed(420.0))
        .align_y(Alignment::Center),
        slider,
    ]
    .spacing(8)
    .into()
}

/// Accent previews stay live: they all drive the shared volume value.
fn accent_row<'a>(
    caption: &'static str,
    accent: Option<AccentColor>,
    value: f32,
    theme: &'a Theme,
) -> Element<'a, Message> {
    let mut slider = Slider::new(theme)
        .value(value)
        .width(Length::Fixed(320.0))
        .on_change(Message::Volume);

    if let Some(accent) = accent {
        slider = slider.color(accent);
    }

    row![
        text(caption)
            .size(11)
            .width(80)
            .font(iced_font(theme.font_pack().mono))
            .color(theme.palette.muted_foreground),
        slider,
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .into()
}

fn captioned<'a>(
    caption: &'static str,
    slider: Slider<'a, Message>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        slider,
        text(caption)
            .size(11)
            .font(iced_font(theme.font_pack().mono))
            .color(theme.palette.muted_foreground),
    ]
    .spacing(8)
    .align_x(Alignment::Center)
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum StepOpt {
    Continuous,
    One,
    Five,
    TwentyFive,
}

impl From<StepOpt> for f32 {
    fn from(step: StepOpt) -> Self {
        match step {
            StepOpt::Continuous => 0.0,
            StepOpt::One => 1.0,
            StepOpt::Five => 5.0,
            StepOpt::TwentyFive => 25.0,
        }
    }
}

impl fmt::Display for StepOpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            StepOpt::Continuous => "continuous",
            StepOpt::One => "1",
            StepOpt::Five => "5",
            StepOpt::TwentyFive => "25",
        })
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

const STEPS: [StepOpt; 4] = [
    StepOpt::Continuous,
    StepOpt::One,
    StepOpt::Five,
    StepOpt::TwentyFive,
];
