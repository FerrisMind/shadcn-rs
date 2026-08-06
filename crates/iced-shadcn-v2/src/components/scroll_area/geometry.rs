//! Scroll-area padding, radius, and scrollbar geometry.

use crate::iced_compat::widget::scrollable;
use shadcn_common::StyleId;
use twill_core::prelude::{Padding, PaddingValue, Spacing};

use super::error::ScrollAreaBuildError;
use super::types::{
    ScrollAreaAnchor, ScrollAreaOrientation, ScrollAreaRadius, ScrollAreaScrollbar,
};
use crate::theme::Theme;

/// Radius that reads as a pill at every rendered size.
const PILL_RADIUS: f32 = 9999.0;

pub(super) fn resolve_padding(
    padding: Padding,
) -> Result<crate::iced_compat::Padding, ScrollAreaBuildError> {
    let (top, right, bottom, left) = padding.sides();

    Ok(crate::iced_compat::Padding {
        top: top.map(padding_value_px).transpose()?.unwrap_or(0.0),
        right: right.map(padding_value_px).transpose()?.unwrap_or(0.0),
        bottom: bottom.map(padding_value_px).transpose()?.unwrap_or(0.0),
        left: left.map(padding_value_px).transpose()?.unwrap_or(0.0),
    })
}

fn padding_value_px(value: PaddingValue) -> Result<f32, ScrollAreaBuildError> {
    match value {
        PaddingValue::Scale(scale) => Ok(match scale {
            Spacing::S0 => 0.0,
            Spacing::Px => 1.0,
            Spacing::S0_5 => 2.0,
            Spacing::S1 => 4.0,
            Spacing::S1_5 => 6.0,
            Spacing::S2 => 8.0,
            Spacing::S2_5 => 10.0,
            Spacing::S3 => 12.0,
            Spacing::S3_5 => 14.0,
            Spacing::S4 => 16.0,
            Spacing::S5 => 20.0,
            Spacing::S6 => 24.0,
            Spacing::S7 => 28.0,
            Spacing::S8 => 32.0,
            Spacing::S9 => 36.0,
            Spacing::S10 => 40.0,
            Spacing::S11 => 44.0,
            Spacing::S12 => 48.0,
            Spacing::S14 => 56.0,
            Spacing::S16 => 64.0,
            Spacing::S20 => 80.0,
            Spacing::S24 => 96.0,
            Spacing::S28 => 112.0,
            Spacing::S32 => 128.0,
            Spacing::S36 => 144.0,
            Spacing::S40 => 160.0,
            Spacing::S44 => 176.0,
            Spacing::S48 => 192.0,
            Spacing::S52 => 208.0,
            Spacing::S56 => 224.0,
            Spacing::S60 => 240.0,
            Spacing::S64 => 256.0,
            Spacing::S72 => 288.0,
            Spacing::S80 => 320.0,
            Spacing::S96 => 384.0,
            Spacing::Auto => return Err(ScrollAreaBuildError::UnsupportedPaddingAuto),
        }),
        PaddingValue::Px(px) => Ok(px.max(0.0)),
        PaddingValue::Rem(rem) => Ok((rem * 16.0).max(0.0)),
        PaddingValue::Var(name) => Err(ScrollAreaBuildError::UnsupportedPaddingVariable {
            name: name.as_str(),
        }),
    }
}

/// Frame radius in pixels.
///
/// The reference viewport is `rounded-[inherit]`, so the frame has no radius of
/// its own until the application asks for one.
pub(super) fn frame_radius_px(theme: &Theme, radius: ScrollAreaRadius) -> f32 {
    match radius {
        ScrollAreaRadius::Theme => 0.0,
        other => scale_radius_px(theme, other),
    }
}

/// Thumb radius in pixels, following `.cn-scroll-area-thumb`.
pub(super) fn thumb_radius_px(theme: &Theme, radius: ScrollAreaRadius) -> f32 {
    match radius {
        ScrollAreaRadius::Theme => match theme.style_id() {
            // rounded-none
            StyleId::Lyra | StyleId::Sera => 0.0,
            // rounded-full
            StyleId::Vega
            | StyleId::Nova
            | StyleId::Maia
            | StyleId::Mira
            | StyleId::Luma
            | StyleId::Rhea => PILL_RADIUS,
        },
        other => scale_radius_px(theme, other),
    }
}

/// Resolves every preset except [`ScrollAreaRadius::Theme`] against the theme
/// radius scale.
fn scale_radius_px(theme: &Theme, radius: ScrollAreaRadius) -> f32 {
    let scale = theme.style.radius;

    let resolved = match radius {
        ScrollAreaRadius::None => 0.0,
        ScrollAreaRadius::Small => scale.sm_px,
        ScrollAreaRadius::Medium => scale.md_px,
        ScrollAreaRadius::Large => scale.lg_px,
        ScrollAreaRadius::Xl => scale.xl_px,
        ScrollAreaRadius::Full => PILL_RADIUS,
        ScrollAreaRadius::Custom(value) if value.is_finite() => value,
        // `Theme` is resolved per slot by the callers; anything else is custom.
        _ => 0.0,
    };

    resolved.max(0.0)
}

/// Builds the iced scroll direction for one orientation and its two rails.
pub(super) fn direction(
    orientation: ScrollAreaOrientation,
    vertical: ScrollAreaScrollbar,
    horizontal: ScrollAreaScrollbar,
) -> scrollable::Direction {
    match orientation {
        ScrollAreaOrientation::Vertical => scrollable::Direction::Vertical(rail(vertical)),
        ScrollAreaOrientation::Horizontal => scrollable::Direction::Horizontal(rail(horizontal)),
        ScrollAreaOrientation::Both => scrollable::Direction::Both {
            vertical: rail(vertical),
            horizontal: rail(horizontal),
        },
    }
}

/// Maps one [`ScrollAreaScrollbar`] onto its iced counterpart.
fn rail(config: ScrollAreaScrollbar) -> scrollable::Scrollbar {
    let anchor = match config.anchor {
        ScrollAreaAnchor::Start => scrollable::Anchor::Start,
        ScrollAreaAnchor::End => scrollable::Anchor::End,
    };

    if config.hidden {
        // A hidden rail keeps the axis scrollable while reserving no space.
        return scrollable::Scrollbar::hidden().anchor(anchor);
    }

    let rail = scrollable::Scrollbar::new()
        .width(config.width)
        .scroller_width(config.thumb_width())
        .margin(config.margin)
        .anchor(anchor);

    match config.spacing {
        Some(spacing) => rail.spacing(spacing),
        None => rail,
    }
}
