//! Interactive playground for `iced-shadcn-v2::Code` + `shadcn-common` theme knobs.
//!
//! Mirrors the `button` example layout: style / base / accent / mode / font /
//! heading / radius controls, then Code demos (variants, highlights, copy,
//! overflow, hideLines, optional scroll).
//!
//! Run: `cargo run -p iced-shadcn-v2 --example code`

use std::fmt;
use std::time::{Duration, Instant};

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Subscription, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, Code, CodeCopyButton, CodeOverflow, CodeVariant,
    CopyButtonAction, CopyButtonState, FontHeading, FontId, FontPack, RadiusId, StyleId, Theme,
    ThemeMode, fonts, iced_font,
};

const RUST_SNIPPET: &str = r#"use iced::widget::{button, column, text};

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Debug, Default)]
struct Counter {
    value: i32,
}

fn main() -> iced::Result {
    iced::run("Counter", Counter::update, Counter::view)
}
"#;

const TS_SNIPPET: &str = r#"export function greet(name: string): string {
  return `Hello, ${name}!`;
}
"#;

const FEEDBACK_DELAY: Duration = Duration::from_millis(500);

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .subscription(Example::subscription)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    copy_state: CopyButtonState,
    reset_at: Option<Instant>,
    overflow_collapsed: bool,
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
    Copy(CopyButtonAction),
    Tick(Instant),
    ToggleOverflow,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            copy_state: CopyButtonState::new(),
            reset_at: None,
            overflow_collapsed: true,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Code".to_owned()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.reset_at.is_some() {
            iced::time::every(Duration::from_millis(16)).map(Message::Tick)
        } else {
            Subscription::none()
        }
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
            Message::Copy(CopyButtonAction::Pressed) => {
                let update =
                    iced_shadcn_v2::copy_button_reduce(self.copy_state, CopyButtonAction::Pressed);
                self.copy_state = update.state();
                self.reset_at = update
                    .should_reset()
                    .then(|| Instant::now() + FEEDBACK_DELAY);
                return Task::batch([
                    iced::clipboard::write::<Message>(RUST_SNIPPET.trim_end().to_owned()),
                    Task::done(Message::Copy(CopyButtonAction::Success)),
                ]);
            }
            Message::Copy(action) => {
                let update = iced_shadcn_v2::copy_button_reduce(self.copy_state, action);
                self.copy_state = update.state();
                self.reset_at = update
                    .should_reset()
                    .then(|| Instant::now() + FEEDBACK_DELAY);
            }
            Message::Tick(now) => {
                if self.reset_at.is_some_and(|deadline| now >= deadline) {
                    let update = iced_shadcn_v2::copy_button_reduce(
                        self.copy_state,
                        CopyButtonAction::Reset,
                    );
                    self.copy_state = update.state();
                    self.reset_at = None;
                }
            }
            Message::ToggleOverflow => {
                self.overflow_collapsed = !self.overflow_collapsed;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;

        let copy = || {
            CodeCopyButton::new(Message::Copy(CopyButtonAction::Pressed))
                .status(self.copy_state.status())
        };

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

        let swatches = row![
            swatch("bg", p.background, p.border),
            swatch("fg", p.foreground, p.border),
            swatch("card", p.card, p.border),
            swatch("secondary", p.secondary, p.border),
            swatch("muted", p.muted, p.border),
            swatch("border", p.border, p.foreground),
        ]
        .spacing(8)
        .wrap();

        let content = column![
            text("iced-shadcn-v2 Code")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("Syntax-highlighted code with line numbers, highlights, copy, and overflow")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            section_label("Palette", p.muted_foreground, theme.font_pack()),
            swatches,
            section_label(
                "Default (line numbers + highlights + copy)",
                p.muted_foreground,
                theme.font_pack(),
            ),
            Code::new(RUST_SNIPPET, "rust", theme)
                .highlight(vec![1u32.into(), 3u32.into(), (11u32, 14u32).into()])
                .copy_button(copy()),
            section_label("Variants", p.muted_foreground, theme.font_pack()),
            Code::new(TS_SNIPPET, "typescript", theme).variant(CodeVariant::Default),
            Code::new(TS_SNIPPET, "typescript", theme).variant(CodeVariant::Secondary),
            section_label("No line numbers", p.muted_foreground, theme.font_pack()),
            Code::new(TS_SNIPPET, "typescript", theme).hide_lines(true),
            section_label(
                "Optional scroll (explicit height)",
                p.muted_foreground,
                theme.font_pack(),
            ),
            Code::new(RUST_SNIPPET, "rust", theme)
                .copy_button(copy())
                .height(Length::Fixed(200.0)),
            section_label(
                "Collapsed overflow (uncontrolled expand)",
                p.muted_foreground,
                theme.font_pack(),
            ),
            Code::new(RUST_SNIPPET, "rust", theme).overflow(CodeOverflow::new(true)),
            section_label("Controlled overflow", p.muted_foreground, theme.font_pack()),
            Code::new(RUST_SNIPPET, "rust", theme)
                .hide_lines(true)
                .overflow(
                    CodeOverflow::new(true)
                        .collapsed_override(Some(self.overflow_collapsed))
                        .max_height(180.0)
                        .on_collapse_change(|_| Message::ToggleOverflow),
                ),
            Button::text("Toggle controlled overflow", theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleOverflow),
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

fn swatch<'a>(label: &'static str, fill: Color, border: Color) -> Element<'a, Message> {
    column![
        container(text(""))
            .width(36)
            .height(36)
            .style(move |_| container::Style {
                background: Some(Background::Color(fill)),
                border: Border {
                    color: border,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..container::Style::default()
            }),
        text(label).size(10).color(border),
    ]
    .spacing(4)
    .align_x(Alignment::Center)
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
