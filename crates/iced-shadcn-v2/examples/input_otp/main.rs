//! Interactive playground for `iced-shadcn-v2::InputOtp` + `shadcn-common` theme knobs.
//!
//! Mirrors the shadcn-svelte input-otp examples (demo, pattern, separator,
//! invalid, form) with the same theme controls as the `button` example.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example input_otp`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, FontHeading, FontId, FontPack, InputOtp, InputOtpPattern,
    InputOtpRadius, RadiusId, StyleId, Theme, ThemeMode, fonts, iced_font, input_otp,
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
    demo: String,
    pattern: String,
    separated: String,
    pin: String,
    completed: Option<String>,
    submitted: u32,
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
    DemoChanged(String),
    DemoComplete(String),
    PatternChanged(String),
    SeparatedChanged(String),
    PinChanged(String),
    Submitted,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            demo: String::new(),
            pattern: String::new(),
            separated: String::new(),
            pin: "12".to_owned(),
            completed: None,
            submitted: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 InputOtp".to_owned()
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
            Message::Font(font) => {
                self.theme = self.theme.clone().with_font(font.0);
            }
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0);
            }
            Message::Radius(radius) => {
                self.theme = self.theme.clone().with_radius(radius.0);
            }
            Message::DemoChanged(value) => {
                if value.len() < 6 {
                    self.completed = None;
                }
                self.demo = value;
            }
            Message::DemoComplete(value) => {
                self.completed = Some(value);
            }
            Message::PatternChanged(value) => {
                self.pattern = value;
            }
            Message::SeparatedChanged(value) => {
                self.separated = value;
            }
            Message::PinChanged(value) => {
                self.pin = value;
            }
            Message::Submitted => {
                self.submitted += 1;
            }
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

        // shadcn `input-otp-demo`: two groups of three with a separator.
        let demo = column![
            InputOtp::new(theme)
                .value(&self.demo)
                .groups([3, 3])
                .on_input(Message::DemoChanged)
                .on_complete(Message::DemoComplete)
                .into_element(),
            hint_line(
                match &self.completed {
                    Some(code) => format!("Completed: {code}"),
                    None if self.demo.is_empty() => "Enter your one-time password.".to_owned(),
                    None => format!("You entered: {}", self.demo),
                },
                theme,
            ),
        ]
        .spacing(8);

        // shadcn `input-otp-pattern`: one group, digits and chars only.
        let pattern = column![
            input_otp(&self.pattern, theme)
                .pattern(InputOtpPattern::DigitsAndChars)
                .on_input(Message::PatternChanged)
                .into_element(),
            hint_line(
                "Digits and letters only (REGEXP_ONLY_DIGITS_AND_CHARS).",
                theme
            ),
        ]
        .spacing(8);

        // shadcn `input-otp-separator`: three groups of two.
        let separated = InputOtp::new(theme)
            .value(&self.separated)
            .groups([2, 2, 2])
            .pattern(InputOtpPattern::Digits)
            .on_input(Message::SeparatedChanged)
            .into_element();

        // shadcn `input-otp-invalid` + form submit on Enter.
        let pin_invalid = self.pin.len() < 6;
        let form = column![
            InputOtp::new(theme)
                .value(&self.pin)
                .groups([3, 3])
                .pattern(InputOtpPattern::Digits)
                .invalid(pin_invalid)
                .on_input(Message::PinChanged)
                .on_submit(Message::Submitted)
                .into_element(),
            hint_line(
                if pin_invalid {
                    "Your one-time password must be at least 6 characters.".to_owned()
                } else {
                    format!(
                        "Valid! Press Enter to submit (submitted: {}).",
                        self.submitted
                    )
                },
                theme,
            ),
        ]
        .spacing(8);

        let states = row![
            InputOtp::<Message>::new(theme)
                .value("123456")
                .disabled(true),
            InputOtp::<Message>::new(theme)
                .value("42")
                .max_length(4)
                .color(AccentColor::Emerald)
                .radius(InputOtpRadius::Full)
                .on_input(|_| Message::Submitted)
                .on_input_maybe(None::<fn(String) -> Message>),
        ]
        .spacing(24)
        .align_y(Alignment::Center)
        .wrap();

        let title_px = 32u32;

        let content = column![
            text("iced-shadcn-v2 InputOtp")
                .size(title_px)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text(
                "Click a field and type; Backspace deletes, Ctrl+V pastes, Ctrl+Backspace clears."
            )
            .size(14)
            .font(iced_font(theme.font_pack().sans))
            .color(p.muted_foreground),
            controls,
            section_label(
                "Demo (groups 3 + 3, onComplete)",
                p.muted_foreground,
                theme.font_pack()
            ),
            demo,
            section_label(
                "Pattern (single group)",
                p.muted_foreground,
                theme.font_pack()
            ),
            pattern,
            section_label(
                "Separators (2 + 2 + 2, digits)",
                p.muted_foreground,
                theme.font_pack()
            ),
            separated,
            section_label(
                "Form (invalid until 6 digits, Enter submits)",
                p.muted_foreground,
                theme.font_pack()
            ),
            form,
            section_label(
                "States (disabled · read-only with accent + pill radius)",
                p.muted_foreground,
                theme.font_pack()
            ),
            states,
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

fn hint_line<'a>(line: impl text::IntoFragment<'a>, theme: &Theme) -> Element<'a, Message> {
    text(line)
        .size(13)
        .font(iced_font(theme.font_pack().sans))
        .color(theme.palette.muted_foreground)
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.write_str("none"),
            Self::Color(color) => f.write_str(color.as_str()),
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
