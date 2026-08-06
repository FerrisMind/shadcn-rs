//! Interactive playground for `iced-shadcn-v2::Snippet` + `shadcn-common` theme knobs.
//!
//! Mirrors the `button` example layout: the playground (style / base / accent /
//! mode / heading / font / radius selects, snippet variant / radius knobs and
//! the palette) lives on the left, while the live demos (usage, variants,
//! multiline, bounded-height scroll) render on the right. Copy feedback is
//! controlled by the example and resets after 500 ms.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example snippet`

use std::fmt;
use std::time::{Duration, Instant};

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Subscription, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, CopyButtonAction, CopyButtonState, CopyButtonStatus, FontHeading,
    FontId, FontPack, RadiusId, Snippet, SnippetRadius, SnippetVariant, StyleId, Theme, ThemeMode,
    fonts, iced_font,
};

const USAGE_TEXT: &str = "npx jsrepo add ui/snippet";
const MULTILINE_LINES: [&str; 2] = ["npx jsrepo add", "npx jsrepo add ui/snippet"];
const FEEDBACK_DELAY: Duration = Duration::from_millis(500);

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .subscription(Example::subscription)
        .window_size(iced::Size::new(1280.0, 820.0))
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SnippetId {
    Usage,
    Primary,
    Secondary,
    Destructive,
    Multiline,
    Scroll,
}

impl SnippetId {
    const COUNT: usize = 6;
    const ALL: [Self; Self::COUNT] = [
        Self::Usage,
        Self::Primary,
        Self::Secondary,
        Self::Destructive,
        Self::Multiline,
        Self::Scroll,
    ];

    const fn index(self) -> usize {
        match self {
            Self::Usage => 0,
            Self::Primary => 1,
            Self::Secondary => 2,
            Self::Destructive => 3,
            Self::Multiline => 4,
            Self::Scroll => 5,
        }
    }

    fn text(self) -> &'static str {
        match self {
            Self::Multiline | Self::Scroll => "npx jsrepo add\nnpx jsrepo add ui/snippet",
            _ => USAGE_TEXT,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Heading(Labelled<FontHeading>),
    Font(Labelled<FontId>),
    Radius(Labelled<RadiusId>),
    Variant(LabelledVariant),
    SnippetRadius(LabelledRadius),
    Copy(SnippetId, CopyButtonAction),
    Tick(Instant),
}

struct Example {
    theme: Theme,
    variant: SnippetVariant,
    radius: SnippetRadius,
    states: [CopyButtonState; SnippetId::COUNT],
    reset_at: [Option<Instant>; SnippetId::COUNT],
    press_count: u32,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            variant: SnippetVariant::Default,
            radius: SnippetRadius::Medium,
            states: [CopyButtonState::new(); SnippetId::COUNT],
            reset_at: [None; SnippetId::COUNT],
            press_count: 0,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Snippet".to_owned()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.reset_at.iter().any(Option::is_some) {
            iced::time::every(Duration::from_millis(16)).map(Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => {
                self.theme = self.theme.clone().with_style(style.0);
                Task::none()
            }
            Message::Base(base) => {
                self.theme = self.theme.clone().with_base(base.0);
                Task::none()
            }
            Message::Accent(accent) => {
                self.theme = self.theme.clone().with_accent(accent.into_option());
                Task::none()
            }
            Message::Mode(mode) => {
                self.theme = self.theme.clone().with_mode(mode.0);
                Task::none()
            }
            Message::Heading(heading) => {
                self.theme = self.theme.clone().with_font_heading(heading.0);
                Task::none()
            }
            Message::Font(font) => {
                self.theme = self.theme.clone().with_font(font.0);
                Task::none()
            }
            Message::Radius(radius) => {
                self.theme = self.theme.clone().with_radius(radius.0);
                Task::none()
            }
            Message::Variant(variant) => {
                self.variant = variant.0;
                Task::none()
            }
            Message::SnippetRadius(radius) => {
                self.radius = radius.0;
                Task::none()
            }
            Message::Copy(id, CopyButtonAction::Pressed) => {
                self.apply_action(id, CopyButtonAction::Pressed);
                self.press_count += 1;

                Task::batch([
                    iced::clipboard::write::<Message>(id.text().to_owned()),
                    Task::done(Message::Copy(id, CopyButtonAction::Success)),
                ])
            }
            Message::Copy(id, action) => {
                self.apply_action(id, action);
                Task::none()
            }
            Message::Tick(now) => {
                for id in SnippetId::ALL {
                    let index = id.index();
                    if self.reset_at[index].is_some_and(|deadline| now >= deadline) {
                        self.apply_action(id, CopyButtonAction::Reset);
                    }
                }
                Task::none()
            }
        }
    }

    fn apply_action(&mut self, id: SnippetId, action: CopyButtonAction) {
        let index = id.index();
        let update = iced_shadcn_v2::copy_button_reduce(self.states[index], action);
        self.states[index] = update.state();

        if update.should_reset() {
            self.reset_at[index] = Some(Instant::now() + FEEDBACK_DELAY);
        } else if self.states[index].status().is_idle() {
            self.reset_at[index] = None;
        }
    }

    fn status(&self, id: SnippetId) -> CopyButtonStatus {
        self.states[id.index()].status()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;

        let snippet = |id: SnippetId, variant: SnippetVariant| -> Element<'_, Message> {
            Snippet::new(id.text(), theme)
                .variant(variant)
                .radius(self.radius)
                .max_width(320.0)
                .copy_status(self.status(id))
                .on_copy(Message::Copy(id, CopyButtonAction::Pressed))
                .into()
        };

        // --- Playground (left pane) -------------------------------------

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
            section_label("Snippet knobs", p.muted_foreground, theme.font_pack()),
            control_select(
                "Variant",
                &VARIANTS,
                Some(LabelledVariant(self.variant)),
                Message::Variant,
                theme,
            ),
            control_select(
                "Radius",
                &SNIPPET_RADII,
                Some(LabelledRadius(self.radius)),
                Message::SnippetRadius,
                theme,
            ),
            section_label("Palette", p.muted_foreground, theme.font_pack()),
            row![
                swatch("bg", p.background, p.border),
                swatch("fg", p.foreground, p.border),
                swatch("card", p.card, p.border),
                swatch("primary", p.primary, p.border),
                swatch("accent", p.accent, p.border),
                swatch("muted", p.muted, p.border),
                swatch("border", p.border, p.foreground),
            ]
            .spacing(8)
            .wrap(),
        ]
        .spacing(8);

        let left_pane = container(
            scrollable(container(controls).padding(4))
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fixed(360.0))
        .height(Length::Fill)
        .padding(16)
        .style(move |_| container::Style {
            background: Some(Background::Color(p.card)),
            border: Border {
                color: p.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..container::Style::default()
        });

        // --- Demos (right pane) -----------------------------------------

        let variants = column![
            snippet(SnippetId::Usage, SnippetVariant::Default),
            snippet(SnippetId::Primary, SnippetVariant::Primary),
            snippet(SnippetId::Secondary, SnippetVariant::Secondary),
            snippet(SnippetId::Destructive, SnippetVariant::Destructive),
        ]
        .spacing(8)
        .width(Length::Fill);

        let multiline = Snippet::lines(MULTILINE_LINES, theme)
            .radius(self.radius)
            .max_width(320.0)
            .copy_status(self.status(SnippetId::Multiline))
            .on_copy(Message::Copy(
                SnippetId::Multiline,
                CopyButtonAction::Pressed,
            ));

        let scroll = Snippet::lines(MULTILINE_LINES, theme)
            .radius(self.radius)
            .max_width(320.0)
            .height(Length::Fixed(40.0))
            .copy_status(self.status(SnippetId::Scroll))
            .on_copy(Message::Copy(SnippetId::Scroll, CopyButtonAction::Pressed));

        let status_text = if self.states.iter().any(|state| !state.status().is_idle()) {
            "copied ✓ (resets after 500 ms)"
        } else {
            "idle"
        };

        let right_pane = container(
            scrollable(
                column![
                    text("iced-shadcn-v2 Snippet")
                        .size(32)
                        .font(iced_font(theme.font_pack().heading))
                        .color(p.foreground),
                    text("Padded mono frame with a floating copy button")
                        .size(14)
                        .font(iced_font(theme.font_pack().sans))
                        .color(p.muted_foreground),
                    section_label("Usage", p.muted_foreground, theme.font_pack()),
                    snippet(SnippetId::Usage, self.variant),
                    section_label("Variants", p.muted_foreground, theme.font_pack()),
                    variants,
                    section_label("Multiline", p.muted_foreground, theme.font_pack()),
                    multiline,
                    section_label(
                        "Bounded height (scroll)",
                        p.muted_foreground,
                        theme.font_pack()
                    ),
                    scroll,
                    text(format!(
                        "clipboard: {status_text} · presses: {}",
                        self.press_count
                    ))
                    .size(13)
                    .font(iced_font(theme.font_pack().mono))
                    .color(p.muted_foreground),
                    text("Press any copy button to write its text to the clipboard.")
                        .size(13)
                        .font(iced_font(theme.font_pack().sans))
                        .color(p.muted_foreground),
                ]
                .spacing(16)
                .max_width(760)
                .padding(8),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(16);

        container(
            row![left_pane, right_pane]
                .spacing(16)
                .align_y(Alignment::Start)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(16)
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
struct LabelledVariant(SnippetVariant);

impl fmt::Display for LabelledVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            SnippetVariant::Default => f.write_str("default"),
            SnippetVariant::Primary => f.write_str("primary"),
            SnippetVariant::Secondary => f.write_str("secondary"),
            SnippetVariant::Destructive => f.write_str("destructive"),
            // `SnippetVariant` is `#[non_exhaustive]`.
            _ => f.write_str("unknown"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LabelledRadius(SnippetRadius);

impl fmt::Display for LabelledRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            SnippetRadius::None => f.write_str("none"),
            SnippetRadius::Small => f.write_str("sm"),
            SnippetRadius::Medium => f.write_str("md (default)"),
            SnippetRadius::Large => f.write_str("lg"),
            SnippetRadius::Full => f.write_str("full"),
            // `SnippetRadius` is `#[non_exhaustive]`.
            _ => f.write_str("unknown"),
        }
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

const VARIANTS: [LabelledVariant; 4] = [
    LabelledVariant(SnippetVariant::Default),
    LabelledVariant(SnippetVariant::Primary),
    LabelledVariant(SnippetVariant::Secondary),
    LabelledVariant(SnippetVariant::Destructive),
];

const SNIPPET_RADII: [LabelledRadius; 5] = [
    LabelledRadius(SnippetRadius::None),
    LabelledRadius(SnippetRadius::Small),
    LabelledRadius(SnippetRadius::Medium),
    LabelledRadius(SnippetRadius::Large),
    LabelledRadius(SnippetRadius::Full),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_text_matches_reference_join() {
        assert_eq!(
            SnippetId::Multiline.text(),
            "npx jsrepo add\nnpx jsrepo add ui/snippet"
        );
        assert_eq!(SnippetId::Usage.text(), USAGE_TEXT);
    }

    #[test]
    fn feedback_state_is_scoped_to_the_pressed_snippet() {
        let mut example = Example::default();

        example.apply_action(SnippetId::Primary, CopyButtonAction::Success);

        assert_eq!(
            example.status(SnippetId::Primary),
            CopyButtonStatus::Success
        );
        assert_eq!(example.status(SnippetId::Usage), CopyButtonStatus::Idle);

        example.apply_action(SnippetId::Primary, CopyButtonAction::Reset);
        assert_eq!(example.status(SnippetId::Primary), CopyButtonStatus::Idle);
    }
}
