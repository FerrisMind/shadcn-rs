//! Mapping of semantic scroll-area tokens to iced scrollable styles.
//!
//! Follows `.cn-scroll-area-scrollbar` and `.cn-scroll-area-thumb`: the rail is
//! transparent, the thumb is painted with the theme `border` token, and hover
//! and drag emphasise the thumb the way `transition-colors` does on the web.

use crate::iced_compat::border::Border;
use crate::iced_compat::widget::{container, scrollable};
use crate::iced_compat::{Background, Color, Shadow, Vector};

use crate::theme::Theme;

/// Emphasis applied to a hovered thumb.
const HOVERED_SHIFT: f32 = 0.15;
/// Emphasis applied to a dragged thumb.
const DRAGGED_SHIFT: f32 = 0.3;
/// Opacity of the autoscroll overlay surface.
const AUTO_SCROLL_OPACITY: f32 = 0.9;
/// Opacity of the autoscroll overlay hairline.
const AUTO_SCROLL_BORDER_OPACITY: f32 = 0.8;
/// Blur radius of the autoscroll overlay shadow.
const AUTO_SCROLL_BLUR: f32 = 4.0;
/// Opacity of the autoscroll overlay shadow.
const AUTO_SCROLL_SHADOW_OPACITY: f32 = 0.3;
/// Radius that reads as a pill at every rendered size.
const PILL_RADIUS: f32 = 9999.0;

/// Status-independent inputs of a resolved scroll-area style.
#[derive(Debug, Clone, Copy)]
pub(super) struct Tokens<'a> {
    pub(super) theme: &'a Theme,
    pub(super) frame_radius: f32,
    pub(super) bordered: bool,
    pub(super) background: Option<Color>,
    pub(super) thumb_radius: f32,
    pub(super) track_color: Option<Color>,
    pub(super) thumb_color: Option<Color>,
}

pub(super) fn resolve_scroll_area_style(
    tokens: Tokens<'_>,
    status: scrollable::Status,
) -> scrollable::Style {
    let (vertical, horizontal) = match status {
        scrollable::Status::Active {
            is_horizontal_scrollbar_disabled,
            is_vertical_scrollbar_disabled,
        } => (
            RailState::new(is_vertical_scrollbar_disabled, Emphasis::Idle),
            RailState::new(is_horizontal_scrollbar_disabled, Emphasis::Idle),
        ),
        scrollable::Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            is_horizontal_scrollbar_disabled,
            is_vertical_scrollbar_disabled,
        } => (
            RailState::new(
                is_vertical_scrollbar_disabled,
                Emphasis::hovered(is_vertical_scrollbar_hovered),
            ),
            RailState::new(
                is_horizontal_scrollbar_disabled,
                Emphasis::hovered(is_horizontal_scrollbar_hovered),
            ),
        ),
        scrollable::Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
            is_horizontal_scrollbar_disabled,
            is_vertical_scrollbar_disabled,
        } => (
            RailState::new(
                is_vertical_scrollbar_disabled,
                Emphasis::dragged(is_vertical_scrollbar_dragged),
            ),
            RailState::new(
                is_horizontal_scrollbar_disabled,
                Emphasis::dragged(is_horizontal_scrollbar_dragged),
            ),
        ),
    };

    scrollable::Style {
        container: frame_style(tokens),
        vertical_rail: rail_style(tokens, vertical),
        horizontal_rail: rail_style(tokens, horizontal),
        // The reference `<ScrollArea.Corner />` paints no surface of its own.
        gap: None,
        auto_scroll: auto_scroll_style(tokens.theme),
    }
}

/// Resolved appearance of one rail for the current status.
#[derive(Debug, Clone, Copy)]
struct RailState {
    disabled: bool,
    emphasis: Emphasis,
}

impl RailState {
    const fn new(disabled: bool, emphasis: Emphasis) -> Self {
        Self { disabled, emphasis }
    }
}

/// How strongly the thumb of one rail is emphasised.
#[derive(Debug, Clone, Copy)]
enum Emphasis {
    Idle,
    Hovered,
    Dragged,
}

impl Emphasis {
    const fn hovered(active: bool) -> Self {
        if active { Self::Hovered } else { Self::Idle }
    }

    const fn dragged(active: bool) -> Self {
        if active { Self::Dragged } else { Self::Idle }
    }

    const fn shift(self) -> f32 {
        match self {
            Self::Idle => 0.0,
            Self::Hovered => HOVERED_SHIFT,
            Self::Dragged => DRAGGED_SHIFT,
        }
    }
}

/// Frame drawn behind the viewport: the `relative` root of the web component
/// plus whatever border, radius, and surface the application asked for.
fn frame_style(tokens: Tokens<'_>) -> container::Style {
    let border_width = if tokens.bordered { 1.0 } else { 0.0 };
    let border_color = if tokens.bordered {
        tokens.theme.palette.border
    } else {
        Color::TRANSPARENT
    };

    container::Style {
        background: tokens.background.map(Background::Color),
        border: Border {
            radius: tokens.frame_radius.into(),
            width: border_width,
            color: border_color,
        },
        snap: true,
        ..container::Style::default()
    }
}

fn rail_style(tokens: Tokens<'_>, state: RailState) -> scrollable::Rail {
    let theme = tokens.theme;

    let thumb = if state.disabled {
        // iced only mounts a rail on an overflowing axis; a disabled one is
        // kept invisible so it can never flash over the content.
        Color::TRANSPARENT
    } else {
        let base = tokens.thumb_color.unwrap_or(theme.palette.border);
        shift_toward(base, theme.is_dark(), state.emphasis.shift())
    };

    scrollable::Rail {
        // `border-l-transparent` / `border-t-transparent`: the rail itself is
        // only a hit area unless the application gives it a surface.
        background: tokens.track_color.map(Background::Color),
        border: Border::default(),
        scroller: scrollable::Scroller {
            background: Background::Color(thumb),
            border: Border {
                radius: tokens.thumb_radius.into(),
                ..Border::default()
            },
        },
    }
}

/// Overlay iced paints while the middle mouse button auto-scrolls, styled from
/// the popover surface so it reads as part of the theme.
fn auto_scroll_style(theme: &Theme) -> scrollable::AutoScroll {
    scrollable::AutoScroll {
        background: Background::Color(with_alpha(theme.palette.popover, AUTO_SCROLL_OPACITY)),
        border: Border {
            radius: PILL_RADIUS.into(),
            width: 1.0,
            color: with_alpha(theme.palette.border, AUTO_SCROLL_BORDER_OPACITY),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, AUTO_SCROLL_SHADOW_OPACITY),
            offset: Vector::ZERO,
            blur_radius: AUTO_SCROLL_BLUR,
        },
        icon: theme.palette.muted_foreground,
    }
}

/// Darkens a color in light mode and lightens it in dark mode, so emphasis
/// stays visible whichever thumb color the application picked.
fn shift_toward(color: Color, dark: bool, amount: f32) -> Color {
    let target = if dark { Color::WHITE } else { Color::BLACK };
    mix_color(color, target, amount)
}

fn mix_color(from: Color, to: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color {
        r: from.r + (to.r - from.r) * t,
        g: from.g + (to.g - from.g) * t,
        b: from.b + (to.b - from.b) * t,
        a: from.a + (to.a - from.a) * t,
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: color.a * alpha.clamp(0.0, 1.0),
        ..color
    }
}
