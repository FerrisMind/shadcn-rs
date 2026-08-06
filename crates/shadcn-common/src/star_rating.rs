//! Backend-agnostic star-rating / bits-ui `RatingGroup` behaviour.
//!
//! Ports the pure state math from bits-ui's `RatingGroupRootState` so iced and
//! egui share one source of truth for item fill states, pointer half-steps,
//! keyboard increments, and first-star clear-to-zero.

use crate::interaction_keys::{Direction, Orientation};

/// Visual fill of one star in a rating group.
///
/// Matches bits-ui / shadcn-svelte-extras `"active" | "partial" | "inactive"`.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StarRatingItemState {
    /// The rating covers this star fully (`value >= index + 1`).
    Active,
    /// Half-star fill when [`allow_half`](StarRatingConfig::allow_half) is on
    /// (`index + 0.5 <= value < index + 1`).
    Partial,
    /// Empty outline.
    #[default]
    Inactive,
}

impl StarRatingItemState {
    /// kebab-case token used by the web `data-state` attribute.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Partial => "partial",
            Self::Inactive => "inactive",
        }
    }
}

impl std::fmt::Display for StarRatingItemState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One star slot handed to renderers / composition APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StarRatingItem {
    /// Zero-based index of the star (`0..max`).
    pub index: usize,
    /// Fill state for the rating currently being shown (committed or hover).
    pub state: StarRatingItemState,
}

/// Configuration shared by every star-rating backend.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StarRatingConfig {
    /// Inclusive lower bound (`0` in the web default).
    pub min: f32,
    /// Inclusive upper bound and star count (`5` in the web default).
    pub max: f32,
    /// Whether half-star values (`n + 0.5`) are allowed.
    pub allow_half: bool,
    /// Axis used for pointer half-detection and keyboard arrows.
    pub orientation: Orientation,
    /// Inline direction for half-star sides and horizontal arrows.
    pub direction: Direction,
}

impl Default for StarRatingConfig {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 5.0,
            allow_half: false,
            orientation: Orientation::Horizontal,
            direction: Direction::Ltr,
        }
    }
}

impl StarRatingConfig {
    /// Default five-star, whole-step configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            min: 0.0,
            max: 5.0,
            allow_half: false,
            orientation: Orientation::Horizontal,
            direction: Direction::Ltr,
        }
    }

    /// Number of star slots to paint (`max` floored, at least `1`).
    #[must_use]
    pub fn star_count(self) -> usize {
        let count = self.max.floor() as isize;
        if count < 1 { 1 } else { count as usize }
    }

    /// Step used by arrow keys / PageUp / PageDown.
    #[must_use]
    pub const fn step(self) -> f32 {
        if self.allow_half { 0.5 } else { 1.0 }
    }
}

/// Clamps and sanitises a rating into `[min, max]`.
///
/// Non-finite inputs fall back to `min`. When `allow_half` is false the value
/// is snapped onto whole numbers; when true it snaps onto a `0.5` grid.
#[must_use]
pub fn clamp_rating(value: f32, config: StarRatingConfig) -> f32 {
    let min = finite_or(config.min, 0.0);
    let max = finite_or(config.max, 5.0).max(min);
    let value = if value.is_finite() { value } else { min };
    let clamped = value.clamp(min, max);
    let step = if config.allow_half { 0.5 } else { 1.0 };
    let steps = ((clamped - min) / step).round();
    (min + steps * step).clamp(min, max)
}

/// Resolves fill state for a single star under `value`.
#[must_use]
pub fn item_state(index: usize, value: f32, allow_half: bool) -> StarRatingItemState {
    let item_value = index as f32 + 1.0;
    if value >= item_value {
        StarRatingItemState::Active
    } else if allow_half && value >= item_value - 0.5 {
        StarRatingItemState::Partial
    } else {
        StarRatingItemState::Inactive
    }
}

/// Builds the `items` snippet payload for a rating (and optional hover preview).
#[must_use]
pub fn items(value: f32, config: StarRatingConfig) -> Vec<StarRatingItem> {
    let value = clamp_rating(value, config);
    (0..config.star_count())
        .map(|index| StarRatingItem {
            index,
            state: item_state(index, value, config.allow_half),
        })
        .collect()
}

/// Writes fill states into `out` without allocating (egui / iced hot paths).
///
/// Returns the number of items written. Truncates to `out.len()` when the
/// buffer is shorter than [`StarRatingConfig::star_count`].
pub fn items_into(value: f32, config: StarRatingConfig, out: &mut [StarRatingItem]) -> usize {
    let value = clamp_rating(value, config);
    let count = config.star_count().min(out.len());
    for (index, slot) in out.iter_mut().take(count).enumerate() {
        *slot = StarRatingItem {
            index,
            state: item_state(index, value, config.allow_half),
        };
    }
    count
}

/// Maps a pointer position inside one star to a rating value.
///
/// `fraction` is the normalised position along the star's main axis
/// (`0.0` = leading edge, `1.0` = trailing edge) before RTL mirroring.
/// Matches bits-ui `calculateRatingFromPointer`.
#[must_use]
pub fn rating_from_pointer(index: usize, fraction: f32, config: StarRatingConfig) -> f32 {
    let rating_value = index as f32 + 1.0;
    if !config.allow_half {
        return clamp_rating(rating_value, config);
    }

    let fraction = if fraction.is_finite() {
        fraction.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let normalized = match config.direction {
        Direction::Ltr => fraction,
        Direction::Rtl => 1.0 - fraction,
    };
    let raw = if normalized < 0.5 {
        rating_value - 0.5
    } else {
        rating_value
    };
    clamp_rating(raw, config)
}

/// Whether a click on the first star should clear the rating to `0`.
///
/// bits-ui clears when `min == 0`, the first star is already at the clicked
/// half/whole value, and the current value is greater than zero.
#[must_use]
pub fn should_clear_on_first_star(
    index: usize,
    current: f32,
    pointer_rating: f32,
    config: StarRatingConfig,
) -> bool {
    index == 0
        && config.min <= 0.0
        && current > 0.0
        && (pointer_rating - current).abs() <= f32::EPSILON
}

/// Applies a click: either clears to zero or commits `pointer_rating`.
#[must_use]
pub fn apply_click(
    index: usize,
    current: f32,
    pointer_rating: f32,
    config: StarRatingConfig,
) -> f32 {
    if should_clear_on_first_star(index, current, pointer_rating, config) {
        clamp_rating(0.0, config)
    } else {
        clamp_rating(pointer_rating, config)
    }
}

/// Adds `delta` to `value` and clamps.
#[must_use]
pub fn adjust_rating(value: f32, delta: f32, config: StarRatingConfig) -> f32 {
    let delta = if delta.is_finite() { delta } else { 0.0 };
    clamp_rating(value + delta, config)
}

/// Keyboard / Page key delta for a rating group (bits-ui `handlers`).
#[must_use]
pub fn key_delta(key: StarRatingKey, config: StarRatingConfig) -> Option<StarRatingKeyEffect> {
    let step = config.step();
    let rtl = matches!(config.direction, Direction::Rtl);
    let horizontal = matches!(config.orientation, Orientation::Horizontal);

    match key {
        StarRatingKey::ArrowUp => Some(StarRatingKeyEffect::Adjust(step)),
        StarRatingKey::ArrowDown => Some(StarRatingKeyEffect::Adjust(-step)),
        StarRatingKey::ArrowRight if horizontal => {
            Some(StarRatingKeyEffect::Adjust(if rtl { -step } else { step }))
        }
        StarRatingKey::ArrowLeft if horizontal => {
            Some(StarRatingKeyEffect::Adjust(if rtl { step } else { -step }))
        }
        StarRatingKey::ArrowRight | StarRatingKey::ArrowLeft => None,
        StarRatingKey::Home => Some(StarRatingKeyEffect::Set(config.min)),
        StarRatingKey::End => Some(StarRatingKeyEffect::Set(config.max)),
        StarRatingKey::PageUp => Some(StarRatingKeyEffect::Adjust(1.0)),
        StarRatingKey::PageDown => Some(StarRatingKeyEffect::Adjust(-1.0)),
        StarRatingKey::Digit(n) => {
            let n = f32::from(n);
            if n >= config.min && n <= config.max {
                Some(StarRatingKeyEffect::Set(n))
            } else {
                None
            }
        }
    }
}

/// Applies a [`StarRatingKeyEffect`] to the current value.
#[must_use]
pub fn apply_key_effect(value: f32, effect: StarRatingKeyEffect, config: StarRatingConfig) -> f32 {
    match effect {
        StarRatingKeyEffect::Adjust(delta) => adjust_rating(value, delta, config),
        StarRatingKeyEffect::Set(next) => clamp_rating(next, config),
    }
}

/// Semantic keys understood by a star rating (subset of bits-ui handlers).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StarRatingKey {
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Home,
    End,
    PageUp,
    PageDown,
    /// Digit `0..=9` typed directly.
    Digit(u8),
}

/// Result of resolving a key against a rating configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StarRatingKeyEffect {
    /// Add this delta to the current value.
    Adjust(f32),
    /// Jump to this absolute value.
    Set(f32),
}

/// Default `aria-valuetext`: `"{value} out of {max}"`.
#[must_use]
pub fn aria_valuetext(value: f32, max: f32) -> String {
    format!("{value} out of {max}")
}

/// Clamps a hover preview value, or clears it when interaction is blocked.
#[must_use]
pub fn hover_preview_value(
    proposed: Option<f32>,
    config: StarRatingConfig,
    readonly: bool,
    disabled: bool,
    hover_preview: bool,
) -> Option<f32> {
    if readonly || disabled || !hover_preview {
        return None;
    }
    proposed.map(|value| clamp_rating(value, config))
}

/// Value painted for items: hover preview wins over the committed value.
#[must_use]
pub fn display_value(committed: f32, hover: Option<f32>, config: StarRatingConfig) -> f32 {
    clamp_rating(hover.unwrap_or(committed), config)
}

fn finite_or(value: f32, fallback: f32) -> f32 {
    if value.is_finite() { value } else { fallback }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn items_cover_active_partial_and_inactive() {
        let config = StarRatingConfig {
            allow_half: true,
            ..StarRatingConfig::new()
        };
        let list = items(3.5, config);
        assert_eq!(list.len(), 5);
        assert_eq!(list[0].state, StarRatingItemState::Active);
        assert_eq!(list[1].state, StarRatingItemState::Active);
        assert_eq!(list[2].state, StarRatingItemState::Active);
        assert_eq!(list[3].state, StarRatingItemState::Partial);
        assert_eq!(list[4].state, StarRatingItemState::Inactive);
    }

    #[test]
    fn pointer_half_respects_rtl() {
        let ltr = StarRatingConfig {
            allow_half: true,
            direction: Direction::Ltr,
            ..StarRatingConfig::new()
        };
        let rtl = StarRatingConfig {
            allow_half: true,
            direction: Direction::Rtl,
            ..StarRatingConfig::new()
        };
        assert_eq!(rating_from_pointer(2, 0.25, ltr), 2.5);
        assert_eq!(rating_from_pointer(2, 0.75, ltr), 3.0);
        assert_eq!(rating_from_pointer(2, 0.25, rtl), 3.0);
        assert_eq!(rating_from_pointer(2, 0.75, rtl), 2.5);
    }

    #[test]
    fn first_star_clears_when_clicking_the_same_value() {
        let config = StarRatingConfig::new();
        assert!(should_clear_on_first_star(0, 1.0, 1.0, config));
        assert!(!should_clear_on_first_star(0, 1.0, 0.5, config));
        assert_eq!(apply_click(0, 1.0, 1.0, config), 0.0);
    }

    #[test]
    fn arrow_keys_flip_in_rtl() {
        let rtl = StarRatingConfig {
            direction: Direction::Rtl,
            ..StarRatingConfig::new()
        };
        assert_eq!(
            key_delta(StarRatingKey::ArrowRight, rtl),
            Some(StarRatingKeyEffect::Adjust(-1.0))
        );
        assert_eq!(
            key_delta(StarRatingKey::ArrowLeft, rtl),
            Some(StarRatingKeyEffect::Adjust(1.0))
        );
    }

    #[test]
    fn clamp_snaps_to_half_grid() {
        let config = StarRatingConfig {
            allow_half: true,
            ..StarRatingConfig::new()
        };
        assert_eq!(clamp_rating(2.4, config), 2.5);
        assert_eq!(clamp_rating(2.2, config), 2.0);
        assert_eq!(clamp_rating(f32::NAN, config), 0.0);
    }
}
