//! Star-rating component ported from shadcn-svelte-extras to iced-shadcn-v2.
//!
//! Mirrors bits-ui `RatingGroup` behaviour (value, max, min, allowHalf, disabled,
//! readonly, hoverPreview, orientation, RTL) with geometry from
//! [`shadcn_common::star_rating_recipe`]. The application owns the rating and
//! receives updates through [`StarRating::on_change`], matching the web
//! `bind:value` contract.
//!
//! Because a canvas widget cannot take keyboard focus in iced, the
//! `focus-visible` ring is painted from application state via
//! [`StarRating::focused`]. Arrow / Home / End / digit handling is exposed as
//! [`StarRating::apply_key`] so apps can forward keyboard events the same way
//! [`crate::RadioGroup`] exposes next/previous helpers.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{StarRating, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Rated(f32),
//! }
//!
//! fn rating(theme: &Theme, value: f32) -> Element<'_, Message> {
//!     StarRating::new(theme)
//!         .value(value)
//!         .allow_half(true)
//!         .on_change(Message::Rated)
//!         .into()
//! }
//! ```

mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{
    StarRatingOrientation, StarRatingSize, StarRatingState, StarRatingStatus, StarRatingStyle,
};

use std::fmt;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::{Color, Element, Length};

use shadcn_common::{
    Direction, Orientation, StarRatingConfig, StarRatingKey, adjust_rating, apply_key_effect,
    clamp_rating, star_rating_key_delta,
};

use crate::theme::Theme;

/// Builder-first star rating styled from `shadcn-common` theme tokens.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct StarRating<'a, Message> {
    pub(super) theme: &'a Theme,
    pub(super) value: f32,
    pub(super) min: f32,
    pub(super) max: f32,
    pub(super) allow_half: bool,
    pub(super) orientation: StarRatingOrientation,
    pub(super) direction: Direction,
    pub(super) disabled: bool,
    pub(super) readonly: bool,
    pub(super) required: bool,
    pub(super) hover_preview: bool,
    pub(super) focused: bool,
    pub(super) star_size: StarRatingSize,
    pub(super) color: Option<Color>,
    pub(super) name: Option<&'a str>,
    pub(super) width: Option<Length>,
    pub(super) height: Option<Length>,
    pub(super) on_change: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    pub(super) style_override:
        Option<Box<dyn Fn(StarRatingStyle, StarRatingStatus) -> StarRatingStyle + 'a>>,
}

impl<Message> fmt::Debug for StarRating<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StarRating")
            .field("theme", &self.theme)
            .field("value", &self.value)
            .field("min", &self.min)
            .field("max", &self.max)
            .field("allow_half", &self.allow_half)
            .field("orientation", &self.orientation)
            .field("direction", &self.direction)
            .field("disabled", &self.disabled)
            .field("readonly", &self.readonly)
            .field("required", &self.required)
            .field("hover_preview", &self.hover_preview)
            .field("focused", &self.focused)
            .field("star_size", &self.star_size)
            .field("color", &self.color)
            .field("name", &self.name)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("on_change", &self.on_change.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> StarRating<'a, Message> {
    /// Creates a five-star rating starting at `0`.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{StarRating, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let _rating = StarRating::<Message>::new(&theme);
    /// ```
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            value: 0.0,
            min: 0.0,
            max: 5.0,
            allow_half: false,
            orientation: StarRatingOrientation::Horizontal,
            direction: Direction::Ltr,
            disabled: false,
            readonly: false,
            required: false,
            hover_preview: true,
            focused: false,
            star_size: StarRatingSize::Default,
            color: None,
            name: None,
            width: None,
            height: None,
            on_change: None,
            style_override: None,
        }
    }

    /// Sets the controlled rating value.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{StarRating, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let _rating = StarRating::<Message>::new(&theme).value(3.0);
    /// ```
    pub fn value(mut self, value: f32) -> Self {
        self.value = sanitize(value, self.min);
        self
    }

    /// Inclusive lower bound (`min` on the web component).
    pub fn min(mut self, min: f32) -> Self {
        self.min = if min.is_finite() { min } else { 0.0 };
        if self.max < self.min {
            self.max = self.min;
        }
        self
    }

    /// Inclusive upper bound and star count (`max` on the web component).
    pub fn max(mut self, max: f32) -> Self {
        let max = if max.is_finite() { max } else { 5.0 };
        self.max = max.max(self.min).max(1.0);
        self
    }

    /// Enables half-star ratings (`allowHalf`).
    pub fn allow_half(mut self, allow_half: bool) -> Self {
        self.allow_half = allow_half;
        self
    }

    /// Sets the layout axis.
    pub fn orientation(mut self, orientation: StarRatingOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets LTR / RTL mirroring for half-stars and horizontal arrows.
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Disables interaction and dims the group (`disabled`).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Keeps the current value visible but non-interactive (`readonly`).
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Marks the control as required for form APIs (`required`).
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Toggles hover preview (`hoverPreview`, default `true`).
    pub fn hover_preview(mut self, hover_preview: bool) -> Self {
        self.hover_preview = hover_preview;
        self
    }

    /// Paints the focus-visible ring from application focus state.
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Sets the per-star footprint (`size-5` by default, `size-10` via [`StarRatingSize::Lg`]).
    pub fn star_size(mut self, size: StarRatingSize) -> Self {
        self.star_size = size;
        self
    }

    /// Overrides the fill / stroke color (`text-primary` by default).
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Optional form field name (API parity; iced has no hidden input).
    pub fn name(mut self, name: impl Into<&'a str>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Overrides the widget width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Overrides the widget height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Receives the next rating when the user commits a change.
    pub fn on_change(mut self, callback: impl Fn(f32) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Escape hatch for colors / opacity.
    pub fn style_override(
        mut self,
        callback: impl Fn(StarRatingStyle, StarRatingStatus) -> StarRatingStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(callback));
        self
    }

    /// Whether clicks and hover preview are accepted.
    #[must_use]
    pub fn is_interactive(&self) -> bool {
        self.on_change.is_some() && !self.disabled && !self.readonly
    }

    /// Shared config used by `shadcn-common` helpers.
    #[must_use]
    pub fn config(&self) -> StarRatingConfig {
        StarRatingConfig {
            min: self.min,
            max: self.max,
            allow_half: self.allow_half,
            orientation: match self.orientation {
                StarRatingOrientation::Horizontal => Orientation::Horizontal,
                StarRatingOrientation::Vertical => Orientation::Vertical,
            },
            direction: self.direction,
        }
    }

    /// Applies a keyboard key the way bits-ui `RatingGroup` does.
    ///
    /// Returns the next value when the key changes the rating; `None` when the
    /// control is locked or the key is irrelevant.
    #[must_use]
    pub fn apply_key(&self, key: StarRatingKey) -> Option<f32> {
        if self.disabled || self.readonly {
            return None;
        }
        let effect = star_rating_key_delta(key, self.config())?;
        Some(apply_key_effect(self.value, effect, self.config()))
    }

    /// Steps the rating by one keyboard increment (or `-1` / `+1` when half is off).
    #[must_use]
    pub fn adjusted(&self, delta: f32) -> f32 {
        adjust_rating(self.value, delta, self.config())
    }

    pub(super) fn emit_change(&self, value: f32) -> Option<Message> {
        let callback = self.on_change.as_ref()?;
        Some(callback(clamp_rating(value, self.config())))
    }
}

/// Wraps a [`StarRating`] into a fixed-size canvas widget.
pub fn star_rating<'a, Message: 'a>(
    rating: StarRating<'a, Message>,
) -> canvas::Canvas<StarRating<'a, Message>, Message> {
    let (width, height) = geometry::resolved_dimensions(&rating);
    canvas::Canvas::new(rating).width(width).height(height)
}

impl<'a, Message: 'a> From<StarRating<'a, Message>> for Element<'a, Message> {
    fn from(rating: StarRating<'a, Message>) -> Self {
        star_rating(rating).into()
    }
}

fn sanitize(value: f32, fallback: f32) -> f32 {
    if value.is_finite() {
        value
    } else if fallback.is_finite() {
        fallback
    } else {
        0.0
    }
}
