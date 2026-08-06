//! Interactive playground for `iced-shadcn-v2::Skeleton`.
//!
//! Mirrors the shadcn-svelte skeleton examples: avatar, text, card, pill,
//! pulse and static placeholders, plus theme controls.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example skeleton`

use std::fmt;

use iced::widget::{column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    BaseColor, FontId, FontPack, RadiusId, SemanticColor, Skeleton, SkeletonAnimation,
    SkeletonShape, StyleId, Theme, ThemeMode, fonts, iced_font,
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
    animation: AnimationOpt,
    shape: ShapeOpt,
    fill: FillOpt,
    duration: DurationOpt,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Mode(Labelled<ThemeMode>),
    Radius(Labelled<RadiusId>),
    Animation(AnimationOpt),
    Shape(ShapeOpt),
    Fill(FillOpt),
    Duration(DurationOpt),
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            animation: AnimationOpt::Pulse,
            shape: ShapeOpt::Rounded,
            fill: FillOpt::Muted,
            duration: DurationOpt(2000),
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 Skeleton".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Style(style) => self.theme = self.theme.clone().with_style(style.0),
            Message::Base(base) => self.theme = self.theme.clone().with_base(base.0),
            Message::Mode(mode) => self.theme = self.theme.clone().with_mode(mode.0),
            Message::Radius(radius) => self.theme = self.theme.clone().with_radius(radius.0),
            Message::Animation(animation) => self.animation = animation,
            Message::Shape(shape) => self.shape = shape,
            Message::Fill(fill) => self.fill = fill,
            Message::Duration(duration) => self.duration = duration,
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = &theme.palette;

        let controls = column![
            section_label(
                "Theme (shadcn-common)",
                palette.muted_foreground,
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
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
            section_label(
                "Skeleton knobs",
                palette.muted_foreground,
                theme.font_pack()
            ),
            control_select(
                "Animation",
                &ANIMATIONS,
                Some(self.animation),
                Message::Animation,
                theme,
            ),
            control_select("Shape", &SHAPES, Some(self.shape), Message::Shape, theme),
            control_select("Fill", &FILLS, Some(self.fill), Message::Fill, theme),
            control_select(
                "Duration",
                &DURATIONS,
                Some(self.duration),
                Message::Duration,
                theme,
            ),
        ]
        .spacing(8);

        let preview = self.preview(theme);

        let content = column![
            text("iced-shadcn-v2 Skeleton")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("Theme-driven loading placeholders based on shadcn-svelte")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            controls,
            preview,
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

    fn preview<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        let palette = &theme.palette;
        let skeleton = |width, height, shape| self.build_skeleton(theme, width, height, shape);

        let avatar_and_text = row![
            skeleton(
                Length::Fixed(48.0),
                Length::Fixed(48.0),
                SkeletonShape::Circle,
            ),
            column![
                skeleton(
                    Length::Fixed(250.0),
                    Length::Fixed(16.0),
                    SkeletonShape::Rounded(self.shape.radius().into()),
                ),
                skeleton(
                    Length::Fixed(200.0),
                    Length::Fixed(16.0),
                    SkeletonShape::Rounded(self.shape.radius().into()),
                ),
            ]
            .spacing(8),
        ]
        .spacing(16)
        .align_y(Alignment::Center);

        let card = column![
            skeleton(
                Length::Fill,
                Length::Fixed(144.0),
                SkeletonShape::Rounded(SkeletonRadiusOpt::Large.into()),
            ),
            column![
                skeleton(
                    Length::Fixed(250.0),
                    Length::Fixed(16.0),
                    SkeletonShape::Rounded(self.shape.radius().into()),
                ),
                skeleton(
                    Length::Fixed(200.0),
                    Length::Fixed(16.0),
                    SkeletonShape::Rounded(self.shape.radius().into()),
                ),
            ]
            .spacing(8),
        ]
        .spacing(12);

        let static_skeleton: Element<'_, Message> = Skeleton::new(theme)
            .width(Length::Fixed(120.0))
            .height(Length::Fixed(28.0))
            .animation(SkeletonAnimation::Static)
            .color(self.fill.semantic())
            .into();

        let variants = row![
            skeleton(
                Length::Fixed(120.0),
                Length::Fixed(28.0),
                SkeletonShape::Rounded(SkeletonRadiusOpt::Full.into()),
            ),
            static_skeleton,
        ]
        .spacing(12)
        .align_y(Alignment::Center)
        .wrap();

        let demo = column![
            section_label("Avatar + text", palette.muted_foreground, theme.font_pack()),
            avatar_and_text,
            section_label(
                "Card placeholder",
                palette.muted_foreground,
                theme.font_pack()
            ),
            card,
            section_label(
                "Pulse / static",
                palette.muted_foreground,
                theme.font_pack()
            ),
            variants,
        ]
        .spacing(12);

        column![
            section_label(
                "Preview (skeleton-demo)",
                palette.muted_foreground,
                theme.font_pack()
            ),
            container(demo)
                .padding(24)
                .width(Length::Fill)
                .style(move |_| container::Style {
                    background: Some(Background::Color(palette.card)),
                    border: Border {
                        color: palette.border,
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    ..container::Style::default()
                }),
        ]
        .spacing(8)
        .into()
    }

    fn build_skeleton<'a>(
        &self,
        theme: &'a Theme,
        width: Length,
        height: Length,
        shape: SkeletonShape,
    ) -> Element<'a, Message> {
        Skeleton::new(theme)
            .width(width)
            .height(height)
            .shape(shape)
            .animation(self.animation.into())
            .color(self.fill.semantic())
            .duration_ms(self.duration.0)
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

fn section_label<'a>(
    label: &'static str,
    color: Color,
    font_pack: FontPack,
) -> Element<'a, Message> {
    text(label)
        .size(18)
        .font(iced_font(font_pack.heading))
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

impl fmt::Display for Labelled<RadiusId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnimationOpt {
    Pulse,
    Static,
}

impl From<AnimationOpt> for SkeletonAnimation {
    fn from(animation: AnimationOpt) -> Self {
        match animation {
            AnimationOpt::Pulse => Self::Pulse,
            AnimationOpt::Static => Self::Static,
        }
    }
}

impl fmt::Display for AnimationOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Pulse => "pulse (default)",
            Self::Static => "static",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShapeOpt {
    Rounded,
    Circle,
}

impl ShapeOpt {
    fn radius(self) -> SkeletonRadiusOpt {
        match self {
            Self::Rounded => SkeletonRadiusOpt::Medium,
            Self::Circle => SkeletonRadiusOpt::Full,
        }
    }
}

impl fmt::Display for ShapeOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Rounded => "rounded-md",
            Self::Circle => "circle",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SkeletonRadiusOpt {
    Medium,
    Large,
    Full,
}

impl From<SkeletonRadiusOpt> for iced_shadcn_v2::SkeletonRadius {
    fn from(radius: SkeletonRadiusOpt) -> Self {
        match radius {
            SkeletonRadiusOpt::Medium => Self::Medium,
            SkeletonRadiusOpt::Large => Self::Large,
            SkeletonRadiusOpt::Full => Self::Full,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FillOpt {
    Muted,
    Accent,
    Secondary,
    Card,
}

impl FillOpt {
    fn semantic(self) -> SemanticColor {
        match self {
            Self::Muted => SemanticColor::Muted,
            Self::Accent => SemanticColor::Accent,
            Self::Secondary => SemanticColor::Secondary,
            Self::Card => SemanticColor::Card,
        }
    }
}

impl fmt::Display for FillOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Muted => "muted (default)",
            Self::Accent => "accent",
            Self::Secondary => "secondary",
            Self::Card => "card",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DurationOpt(u32);

impl fmt::Display for DurationOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ms", self.0)
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

const RADII: [Labelled<RadiusId>; 5] = [
    Labelled(RadiusId::Default),
    Labelled(RadiusId::None),
    Labelled(RadiusId::Small),
    Labelled(RadiusId::Medium),
    Labelled(RadiusId::Large),
];

const ANIMATIONS: [AnimationOpt; 2] = [AnimationOpt::Pulse, AnimationOpt::Static];

const SHAPES: [ShapeOpt; 2] = [ShapeOpt::Rounded, ShapeOpt::Circle];

const FILLS: [FillOpt; 4] = [
    FillOpt::Muted,
    FillOpt::Accent,
    FillOpt::Secondary,
    FillOpt::Card,
];

const DURATIONS: [DurationOpt; 4] = [
    DurationOpt(1000),
    DurationOpt(2000),
    DurationOpt(4000),
    DurationOpt(8000),
];
