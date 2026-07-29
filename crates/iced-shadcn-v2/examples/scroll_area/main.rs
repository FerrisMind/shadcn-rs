//! Interactive playground for `iced-shadcn-v2::ScrollArea` + `shadcn-common` theme knobs.
//!
//! Run: `cargo run -p iced-shadcn-v2 --example scroll_area`

use std::fmt;

use iced::widget::operation::{AbsoluteOffset, RelativeOffset, scroll_to, snap_to};
use iced::widget::scrollable::Viewport;
use iced::widget::{Id, column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Task};

use iced_shadcn_v2::{
    AccentColor, BaseColor, Button, ButtonVariant, FontId, Padding, RadiusId, ScrollArea,
    ScrollAreaAnchor, ScrollAreaOrientation, ScrollAreaRadius, ScrollAreaScrollbar, Separator,
    SeparatorOrientation, Spacing, StyleId, Theme, ThemeMode, fonts, iced_font, separator,
};

/// Name of the scroll area the toolbar drives programmatically.
const NOTES: &str = "notes";

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
    orientation: OrientationOpt,
    scrollbar: ScrollbarOpt,
    anchor: AnchorOpt,
    radius: RadiusOpt,
    thumb: ThumbOpt,
    bordered: bool,
    auto_scroll: bool,
    offset: AbsoluteOffset,
}

#[derive(Debug, Clone)]
enum Message {
    Style(Labelled<StyleId>),
    Base(Labelled<BaseColor>),
    Accent(AccentOpt),
    Mode(Labelled<ThemeMode>),
    Radius(Labelled<RadiusId>),
    Orientation(OrientationOpt),
    Scrollbar(ScrollbarOpt),
    Anchor(AnchorOpt),
    FrameRadius(RadiusOpt),
    Thumb(ThumbOpt),
    ToggleBordered,
    ToggleAutoScroll,
    Scrolled(Viewport),
    ScrollToTop,
    ScrollToEnd,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            orientation: OrientationOpt::Vertical,
            scrollbar: ScrollbarOpt::Default,
            anchor: AnchorOpt::Start,
            radius: RadiusOpt::Medium,
            thumb: ThumbOpt::Theme,
            bordered: true,
            auto_scroll: false,
            offset: AbsoluteOffset { x: 0.0, y: 0.0 },
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 ScrollArea".to_owned()
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
            Message::Radius(radius) => {
                self.theme = self.theme.clone().with_radius(radius.0);
            }
            Message::Orientation(orientation) => {
                self.orientation = orientation;
            }
            Message::Scrollbar(scrollbar) => {
                self.scrollbar = scrollbar;
            }
            Message::Anchor(anchor) => {
                self.anchor = anchor;
            }
            Message::FrameRadius(radius) => {
                self.radius = radius;
            }
            Message::Thumb(thumb) => {
                self.thumb = thumb;
            }
            Message::ToggleBordered => {
                self.bordered = !self.bordered;
            }
            Message::ToggleAutoScroll => {
                self.auto_scroll = !self.auto_scroll;
            }
            Message::Scrolled(viewport) => {
                self.offset = viewport.absolute_offset();
            }
            Message::ScrollToTop => {
                return scroll_to(Id::new(NOTES), AbsoluteOffset { x: 0.0, y: 0.0 });
            }
            Message::ScrollToEnd => {
                return snap_to(Id::new(NOTES), RelativeOffset::END);
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let p = &theme.palette;

        let controls = column![
            section_label("Theme (shadcn-common)", theme),
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
                "Radius",
                &RADII,
                Some(Labelled(theme.radius_id())),
                Message::Radius,
                theme,
            ),
            section_label("Scroll-area knobs", theme),
            control_select(
                "Orientation",
                &ORIENTATIONS,
                Some(self.orientation),
                Message::Orientation,
                theme,
            ),
            control_select(
                "Scrollbar",
                &SCROLLBARS,
                Some(self.scrollbar),
                Message::Scrollbar,
                theme,
            ),
            control_select(
                "Anchor",
                &ANCHORS,
                Some(self.anchor),
                Message::Anchor,
                theme
            ),
            control_select(
                "Frame radius",
                &FRAME_RADII,
                Some(self.radius),
                Message::FrameRadius,
                theme,
            ),
            control_select("Thumb", &THUMBS, Some(self.thumb), Message::Thumb, theme),
            row![
                Button::text(
                    if self.bordered {
                        "Border on"
                    } else {
                        "Border off"
                    },
                    theme,
                )
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleBordered),
                Button::text(
                    if self.auto_scroll {
                        "Auto-scroll on"
                    } else {
                        "Auto-scroll off"
                    },
                    theme,
                )
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleAutoScroll),
                Button::text("Scroll to top", theme)
                    .variant(ButtonVariant::Secondary)
                    .on_press(Message::ScrollToTop),
                Button::text("Scroll to end", theme)
                    .variant(ButtonVariant::Secondary)
                    .on_press(Message::ScrollToEnd),
            ]
            .spacing(12)
            .wrap(),
        ]
        .spacing(8);

        let readout = text(format!(
            "offset x={:.0} y={:.0} · {} · {} · anchor={} · thumb={}",
            self.offset.x, self.offset.y, self.orientation, self.scrollbar, self.anchor, self.thumb,
        ))
        .size(12)
        .font(iced_font(theme.font_pack().mono))
        .color(p.muted_foreground);

        let content = column![
            text("iced-shadcn-v2 ScrollArea")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(p.foreground),
            text("A themed rail and thumb over iced's own scrolling, driven by Theme tokens")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(p.muted_foreground),
            controls,
            section_label("Playground (drives the toolbar above)", theme),
            self.playground(theme),
            readout,
            section_label("Vertical", theme),
            self.vertical_demo(theme),
            section_label("Horizontal", theme),
            self.horizontal_demo(theme),
            section_label("Both axes", theme),
            self.both_demo(theme),
            section_label("Hidden rail (still scrolls)", theme),
            self.hidden_demo(theme),
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

    /// The scroll area every toolbar knob is wired to.
    fn playground<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        let scrollbar = self.scrollbar.resolve().anchor(self.anchor.resolve());

        let mut area = ScrollArea::new(paragraphs(theme), theme)
            .orientation(self.orientation.resolve())
            .width(Length::Fixed(420.0))
            .height(Length::Fixed(220.0))
            .radius(self.radius.resolve())
            .thumb_radius(self.thumb.radius())
            .bordered(self.bordered)
            .auto_scroll(self.auto_scroll)
            .scrollbar(scrollbar)
            .id(Id::new(NOTES))
            .on_scroll(Message::Scrolled);

        if let Some(color) = self.thumb.color(theme) {
            area = area.thumb_color(color);
        }

        area.padding(Padding::all(Spacing::S4))
            .expect("scale padding is supported")
            .into()
    }

    fn vertical_demo<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        let tags = (1..=24).fold(column![].spacing(8), |list, index| {
            list.push(
                column![
                    text(format!("v1.2.0-beta.{index}"))
                        .size(14)
                        .color(theme.palette.foreground),
                    separator(Separator::new(theme)),
                ]
                .spacing(8),
            )
        });

        ScrollArea::new(tags, theme)
            .width(Length::Fixed(300.0))
            .height(Length::Fixed(200.0))
            .radius(ScrollAreaRadius::Medium)
            .bordered(true)
            .padding(Padding::all(Spacing::S4))
            .expect("scale padding is supported")
            .into()
    }

    fn horizontal_demo<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        let works = ["Ornella Binni", "Tom Byrom", "Vladimir Malyav", "Ana Cruz"];
        let gallery = works.iter().fold(row![].spacing(16), |gallery, artist| {
            gallery.push(artwork(artist, theme))
        });

        ScrollArea::new(gallery, theme)
            .orientation(ScrollAreaOrientation::Horizontal)
            .width(Length::Fixed(384.0))
            .radius(ScrollAreaRadius::Medium)
            .bordered(true)
            .padding(Padding::all(Spacing::S4))
            .expect("scale padding is supported")
            .into()
    }

    fn both_demo<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        let grid = (1..=14).fold(column![].spacing(6), |grid, row_index| {
            grid.push(
                (1..=8)
                    .fold(row![].spacing(6), |line, cell| {
                        line.push(cell_label(row_index, cell, theme))
                    })
                    .align_y(Alignment::Center),
            )
        });

        ScrollArea::new(grid, theme)
            .orientation(ScrollAreaOrientation::Both)
            .width(Length::Fixed(360.0))
            .height(Length::Fixed(200.0))
            .radius(ScrollAreaRadius::Medium)
            .bordered(true)
            .background(theme.palette.card)
            .padding(Padding::all(Spacing::S3))
            .expect("scale padding is supported")
            .into()
    }

    fn hidden_demo<'a>(&self, theme: &'a Theme) -> Element<'a, Message> {
        ScrollArea::new(paragraphs(theme), theme)
            .vertical_scrollbar(ScrollAreaScrollbar::hidden())
            .width(Length::Fixed(300.0))
            .height(Length::Fixed(140.0))
            .radius(ScrollAreaRadius::Medium)
            .bordered(true)
            .padding(Padding::all(Spacing::S4))
            .expect("scale padding is supported")
            .into()
    }
}

fn paragraphs<'a>(theme: &'a Theme) -> Element<'a, Message> {
    let body = "Jokester began sneaking into the castle in the middle of the night and leaving \
                jokes all over the place: under the king's pillow, in his soup, even in the royal \
                toilet. The king was furious, but he couldn't seem to stop Jokester. And then, \
                one day, the people of the kingdom discovered that the jokes left by Jokester \
                were so funny that they couldn't help but laugh. And once they started laughing, \
                they couldn't stop.";

    column![
        text(body).size(14).color(theme.palette.foreground),
        text(body).size(14).color(theme.palette.muted_foreground),
    ]
    .spacing(12)
    .into()
}

fn artwork<'a>(artist: &'a str, theme: &'a Theme) -> Element<'a, Message> {
    let p = theme.palette;

    column![
        container(text(""))
            .width(150)
            .height(200)
            .style(move |_| container::Style {
                background: Some(Background::Color(p.muted)),
                border: Border {
                    color: p.border,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..container::Style::default()
            }),
        text(format!("Photo by {artist}"))
            .size(12)
            .color(p.muted_foreground),
    ]
    .spacing(8)
    .into()
}

fn cell_label<'a>(row_index: u32, cell: u32, theme: &'a Theme) -> Element<'a, Message> {
    let p = theme.palette;

    container(
        text(format!("R{row_index}C{cell}"))
            .size(12)
            .font(iced_font(theme.font_pack().mono))
            .color(p.foreground),
    )
    .width(64)
    .padding(6)
    .style(move |_| container::Style {
        background: Some(Background::Color(p.secondary)),
        border: Border {
            radius: 4.0.into(),
            ..Border::default()
        },
        ..container::Style::default()
    })
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
            .width(96)
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

fn section_label<'a>(label: &'static str, theme: &'a Theme) -> Element<'a, Message> {
    row![
        text(label)
            .size(18)
            .font(iced_font(theme.font_pack().heading))
            .color(theme.palette.muted_foreground),
        separator(
            Separator::new(theme)
                .orientation(SeparatorOrientation::Horizontal)
                .length(Length::Fill),
        ),
    ]
    .spacing(12)
    .align_y(Alignment::Center)
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
        f.write_str(self.0.label())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrientationOpt {
    Vertical,
    Horizontal,
    Both,
}

impl OrientationOpt {
    const fn resolve(self) -> ScrollAreaOrientation {
        match self {
            Self::Vertical => ScrollAreaOrientation::Vertical,
            Self::Horizontal => ScrollAreaOrientation::Horizontal,
            Self::Both => ScrollAreaOrientation::Both,
        }
    }
}

impl fmt::Display for OrientationOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Both => "both",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScrollbarOpt {
    Default,
    Slim,
    Wide,
    Inset,
    Gutter,
    Hidden,
}

impl ScrollbarOpt {
    fn resolve(self) -> ScrollAreaScrollbar {
        match self {
            Self::Default => ScrollAreaScrollbar::new(),
            Self::Slim => ScrollAreaScrollbar::new().width(6.0).padding(1.0),
            Self::Wide => ScrollAreaScrollbar::new().width(16.0).padding(3.0),
            Self::Inset => ScrollAreaScrollbar::new().margin(4.0),
            Self::Gutter => ScrollAreaScrollbar::new().spacing(8.0),
            Self::Hidden => ScrollAreaScrollbar::hidden(),
        }
    }
}

impl fmt::Display for ScrollbarOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Default => "default (10px, p-px)",
            Self::Slim => "slim (6px)",
            Self::Wide => "wide (16px)",
            Self::Inset => "inset (margin 4px)",
            Self::Gutter => "embedded gutter (8px)",
            Self::Hidden => "hidden",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnchorOpt {
    Start,
    End,
}

impl AnchorOpt {
    const fn resolve(self) -> ScrollAreaAnchor {
        match self {
            Self::Start => ScrollAreaAnchor::Start,
            Self::End => ScrollAreaAnchor::End,
        }
    }
}

impl fmt::Display for AnchorOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Start => "start",
            Self::End => "end",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RadiusOpt {
    Theme,
    None,
    Medium,
    Large,
    Full,
    Custom,
}

impl RadiusOpt {
    const fn resolve(self) -> ScrollAreaRadius {
        match self {
            Self::Theme => ScrollAreaRadius::Theme,
            Self::None => ScrollAreaRadius::None,
            Self::Medium => ScrollAreaRadius::Medium,
            Self::Large => ScrollAreaRadius::Large,
            Self::Full => ScrollAreaRadius::Full,
            Self::Custom => ScrollAreaRadius::Custom(20.0),
        }
    }
}

impl fmt::Display for RadiusOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Theme => "theme (square)",
            Self::None => "none",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Full => "full",
            Self::Custom => "custom (20px)",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThumbOpt {
    Theme,
    Primary,
    MutedForeground,
    Squared,
}

impl ThumbOpt {
    const fn radius(self) -> ScrollAreaRadius {
        match self {
            Self::Squared => ScrollAreaRadius::None,
            Self::Theme | Self::Primary | Self::MutedForeground => ScrollAreaRadius::Theme,
        }
    }

    fn color(self, theme: &Theme) -> Option<Color> {
        match self {
            Self::Theme | Self::Squared => None,
            Self::Primary => Some(theme.palette.primary),
            Self::MutedForeground => Some(theme.palette.muted_foreground),
        }
    }
}

impl fmt::Display for ThumbOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Theme => "theme border",
            Self::Primary => "primary",
            Self::MutedForeground => "muted foreground",
            Self::Squared => "theme border, square",
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

const ACCENTS: [AccentOpt; 6] = [
    AccentOpt::None,
    AccentOpt::Color(AccentColor::Blue),
    AccentOpt::Color(AccentColor::Amber),
    AccentOpt::Color(AccentColor::Emerald),
    AccentOpt::Color(AccentColor::Rose),
    AccentOpt::Color(AccentColor::Violet),
];

const MODES: [Labelled<ThemeMode>; 2] = [Labelled(ThemeMode::Light), Labelled(ThemeMode::Dark)];

const RADII: [Labelled<RadiusId>; 5] = [
    Labelled(RadiusId::Default),
    Labelled(RadiusId::None),
    Labelled(RadiusId::Small),
    Labelled(RadiusId::Medium),
    Labelled(RadiusId::Large),
];

const ORIENTATIONS: [OrientationOpt; 3] = [
    OrientationOpt::Vertical,
    OrientationOpt::Horizontal,
    OrientationOpt::Both,
];

const SCROLLBARS: [ScrollbarOpt; 6] = [
    ScrollbarOpt::Default,
    ScrollbarOpt::Slim,
    ScrollbarOpt::Wide,
    ScrollbarOpt::Inset,
    ScrollbarOpt::Gutter,
    ScrollbarOpt::Hidden,
];

const ANCHORS: [AnchorOpt; 2] = [AnchorOpt::Start, AnchorOpt::End];

const FRAME_RADII: [RadiusOpt; 6] = [
    RadiusOpt::Theme,
    RadiusOpt::None,
    RadiusOpt::Medium,
    RadiusOpt::Large,
    RadiusOpt::Full,
    RadiusOpt::Custom,
];

const THUMBS: [ThumbOpt; 4] = [
    ThumbOpt::Theme,
    ThumbOpt::Primary,
    ThumbOpt::MutedForeground,
    ThumbOpt::Squared,
];
