//! Slider component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! The slider is controlled: the application owns the numbers and receives the
//! next one from [`Slider::on_change`] (or the whole set from
//! [`Slider::on_change_values`]), just like the `bind:value` contract of the web
//! component. Both `type="single"` and `type="multiple"` are covered — any
//! number of thumbs share one track, cannot cross each other, and snap to the
//! configured step. Track, range, and thumb geometry come from the active style
//! pack (`.cn-slider*`), so a slider restyles together with [`crate::Theme`].
//!
//! Because a canvas widget cannot take keyboard focus in iced, the
//! `focus-visible` ring is painted from application state via
//! [`Slider::focused`]; hovering or dragging a thumb rings it automatically. The
//! widget always reserves the ring's width around the track so the ring is never
//! clipped and toggling focus never reflows the layout.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Slider, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     VolumeChanged(f32),
//! }
//!
//! fn volume(theme: &Theme, value: f32) -> Element<'_, Message> {
//!     Slider::new(theme)
//!         .value(value)
//!         .range(0.0..=100.0)
//!         .step(1.0)
//!         .on_change(Message::VolumeChanged)
//!         .into()
//! }
//! ```

mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{SliderOrientation, SliderRadius, SliderState, SliderStatus, SliderStyle};

use std::fmt;
use std::ops::RangeInclusive;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::{Color, Element, Length};

use shadcn_common::AccentColor;

use crate::theme::Theme;

/// Default range of a slider (`min = 0`, `max = 100` in shadcn-svelte).
const DEFAULT_MIN: f32 = 0.0;
const DEFAULT_MAX: f32 = 100.0;
/// Default step of a slider (`step = 1` in shadcn-svelte).
const DEFAULT_STEP: f32 = 1.0;

/// Builder-first slider styled from `shadcn-common` theme tokens.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Slider, SliderOrientation, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     PriceChanged(Vec<f32>),
/// }
///
/// fn price_range<'a>(theme: &'a Theme, values: &[f32]) -> Element<'a, Message> {
///     Slider::new(theme)
///         .values(values.to_vec())
///         .orientation(SliderOrientation::Horizontal)
///         .on_change_values(Message::PriceChanged)
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Slider<'a, Message> {
    theme: &'a Theme,
    values: Vec<f32>,
    min: f32,
    max: f32,
    step: f32,
    orientation: SliderOrientation,
    disabled: bool,
    focused: bool,
    /// `None` = theme primary; `Some` = accent overlay from `shadcn-common`.
    color: Option<AccentColor>,
    range_color: Option<Color>,
    track_color: Option<Color>,
    thumb_color: Option<Color>,
    radius: Option<SliderRadius>,
    thumb_radius: Option<SliderRadius>,
    width: Option<Length>,
    height: Option<Length>,
    on_change: Option<OnChange<'a, Message>>,
    on_release: Option<Box<dyn Fn() -> Message + 'a>>,
    style_override: Option<Box<dyn Fn(SliderStyle, SliderStatus) -> SliderStyle + 'a>>,
}

/// Controlled-value callback of a [`Slider`].
enum OnChange<'a, Message> {
    /// Receives the value of the thumb being moved.
    Single(Box<dyn Fn(f32) -> Message + 'a>),
    /// Receives every value, with the moved thumb updated.
    Multiple(Box<dyn Fn(Vec<f32>) -> Message + 'a>),
}

impl<Message> fmt::Debug for Slider<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let on_change = match &self.on_change {
            Some(OnChange::Single(_)) => "single",
            Some(OnChange::Multiple(_)) => "multiple",
            None => "none",
        };

        formatter
            .debug_struct("Slider")
            .field("theme", &self.theme)
            .field("values", &self.values)
            .field("min", &self.min)
            .field("max", &self.max)
            .field("step", &self.step)
            .field("orientation", &self.orientation)
            .field("disabled", &self.disabled)
            .field("focused", &self.focused)
            .field("color", &self.color)
            .field("range_color", &self.range_color)
            .field("track_color", &self.track_color)
            .field("thumb_color", &self.thumb_color)
            .field("radius", &self.radius)
            .field("thumb_radius", &self.thumb_radius)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("on_change", &on_change)
            .field("on_release", &self.on_release.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Slider<'a, Message> {
    /// Creates a single-thumb slider over `0.0..=100.0` with a step of `1.0`.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme);
    /// ```
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            values: vec![DEFAULT_MIN],
            min: DEFAULT_MIN,
            max: DEFAULT_MAX,
            step: DEFAULT_STEP,
            orientation: SliderOrientation::Horizontal,
            disabled: false,
            focused: false,
            color: None,
            range_color: None,
            track_color: None,
            thumb_color: None,
            radius: None,
            thumb_radius: None,
            width: None,
            height: None,
            on_change: None,
            on_release: None,
            style_override: None,
        }
    }

    /// Sets a single controlled value (`type="single"`).
    ///
    /// Non-finite values are ignored; the value is clamped into the range and
    /// snapped onto the step grid when the slider is painted.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).value(33.0);
    /// ```
    pub fn value(mut self, value: f32) -> Self {
        self.values = vec![sanitize(value, self.min)];
        self
    }

    /// Sets several controlled values, one per thumb (`type="multiple"`).
    ///
    /// Thumbs keep the order they are given and cannot cross each other. An
    /// empty set leaves the track without thumbs, which renders a plain rail.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).values(vec![25.0, 75.0]);
    /// ```
    pub fn values(mut self, values: impl Into<Vec<f32>>) -> Self {
        let min = self.min;
        self.values = values
            .into()
            .into_iter()
            .map(|value| sanitize(value, min))
            .collect();
        self
    }

    /// Sets the inclusive value range.
    ///
    /// A reversed range is swapped and an empty one is widened by `1.0`, so the
    /// slider always has a usable travel instead of dividing by zero. Ranges
    /// whose width cannot be represented as a finite `f32` (e.g.
    /// `-f32::MAX..=f32::MAX`) or that cannot be widened (e.g.
    /// `f32::MAX..=f32::MAX`) fall back to the default `0.0..=100.0`.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).range(-50.0..=50.0);
    /// ```
    pub fn range(mut self, range: RangeInclusive<f32>) -> Self {
        let (start, end) = (*range.start(), *range.end());
        let start = sanitize(start, DEFAULT_MIN);
        let end = sanitize(end, DEFAULT_MAX);
        let (min, max) = if start <= end {
            (start, end)
        } else {
            (end, start)
        };

        // Both bounds can be finite while their span is not: the width of
        // `-f32::MAX..=f32::MAX` overflows to infinity, and near `f32::MAX`
        // the `min + 1.0` widening is absorbed by f32 precision. Either way
        // the geometry cannot work with the span, so reject it entirely.
        let width = max - min;
        if !width.is_finite() || (width.abs() <= f32::EPSILON && min + 1.0 == min) {
            self.min = DEFAULT_MIN;
            self.max = DEFAULT_MAX;
            return self;
        }

        self.min = min;
        self.max = if width.abs() <= f32::EPSILON {
            min + 1.0
        } else {
            max
        };
        self
    }

    /// Sets the lower bound of the range.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).min(10.0);
    /// ```
    pub fn min(self, min: f32) -> Self {
        let max = self.max;
        self.range(sanitize(min, DEFAULT_MIN)..=max)
    }

    /// Sets the upper bound of the range.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).max(10.0);
    /// ```
    pub fn max(self, max: f32) -> Self {
        let min = self.min;
        self.range(min..=sanitize(max, DEFAULT_MAX))
    }

    /// Sets the step values snap to.
    ///
    /// A non-positive or non-finite step makes the slider continuous.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).step(5.0);
    /// ```
    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    /// Lets values move freely, without snapping to a step.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).continuous();
    /// ```
    pub fn continuous(mut self) -> Self {
        self.step = 0.0;
        self
    }

    /// Sets the axis the slider runs along.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, SliderOrientation, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).orientation(SliderOrientation::Vertical);
    /// ```
    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Suppresses interaction and dims the control.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).disabled(true);
    /// ```
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Paints the `focus-visible` ring around every thumb.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).focused(true);
    /// ```
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Applies an accent color overlay to the range (and primary-filled thumbs).
    ///
    /// ```rust
    /// use iced_shadcn_v2::{AccentColor, Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).color(AccentColor::Blue);
    /// ```
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self.range_color = None;
        self
    }

    /// Uses the theme primary (no per-slider accent overlay).
    ///
    /// ```rust
    /// use iced_shadcn_v2::{AccentColor, Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme)
    ///     .color(AccentColor::Blue)
    ///     .theme_primary();
    /// ```
    pub fn theme_primary(mut self) -> Self {
        self.color = None;
        self.range_color = None;
        self
    }

    /// Uses an explicit iced color for the selected range.
    ///
    /// ```rust
    /// use iced::Color;
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).range_color(Color::BLACK);
    /// ```
    pub fn range_color(mut self, color: Color) -> Self {
        self.range_color = Some(color);
        self.color = None;
        self
    }

    /// Uses an explicit iced color for the track.
    ///
    /// ```rust
    /// use iced::Color;
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).track_color(Color::from_rgb(0.9, 0.9, 0.9));
    /// ```
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = Some(color);
        self
    }

    /// Uses an explicit iced color for the thumbs.
    ///
    /// ```rust
    /// use iced::Color;
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).thumb_color(Color::WHITE);
    /// ```
    pub fn thumb_color(mut self, color: Color) -> Self {
        self.thumb_color = Some(color);
        self
    }

    /// Sets the corner radius of the track and its range.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, SliderRadius, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).radius(SliderRadius::None);
    /// ```
    pub fn radius(mut self, radius: SliderRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Sets the corner radius of the thumbs.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, SliderRadius, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).thumb_radius(SliderRadius::Full);
    /// ```
    pub fn thumb_radius(mut self, radius: SliderRadius) -> Self {
        self.thumb_radius = Some(radius);
        self
    }

    /// Sets the preferred widget width.
    ///
    /// ```rust
    /// use iced::Length;
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).width(Length::Fixed(240.0));
    /// ```
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the preferred widget height.
    ///
    /// Vertical sliders default to [`Length::Fill`]; give them a fixed height (or
    /// a bounded parent) to match the `min-h-40` floor of the web component.
    ///
    /// ```rust
    /// use iced::Length;
    /// use iced_shadcn_v2::{Slider, SliderOrientation, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme)
    ///     .orientation(SliderOrientation::Vertical)
    ///     .height(Length::Fixed(200.0));
    /// ```
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Minimum length a vertical slider should get from its layout.
    ///
    /// Mirrors `data-vertical:min-h-40` from the style packs.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// assert_eq!(Slider::<Message>::new(&theme).min_length(), 160.0);
    /// ```
    pub fn min_length(&self) -> f32 {
        self.theme.style.slider().min_length_px
    }

    /// Sets the callback invoked with the value of the thumb being moved.
    ///
    /// The slider stays controlled: it keeps painting the configured values
    /// until the application stores the new one.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Changed(f32),
    /// }
    ///
    /// let theme = Theme::light();
    /// let slider = Slider::new(&theme).on_change(Message::Changed);
    /// ```
    pub fn on_change<F>(mut self, on_change: F) -> Self
    where
        F: Fn(f32) -> Message + 'a,
    {
        self.on_change = Some(OnChange::Single(Box::new(on_change)));
        self
    }

    /// Sets the callback invoked with every value after a thumb moves.
    ///
    /// This is the multi-thumb counterpart of [`Self::on_change`].
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Changed(Vec<f32>),
    /// }
    ///
    /// let theme = Theme::light();
    /// let slider = Slider::new(&theme)
    ///     .values(vec![25.0, 75.0])
    ///     .on_change_values(Message::Changed);
    /// ```
    pub fn on_change_values<F>(mut self, on_change: F) -> Self
    where
        F: Fn(Vec<f32>) -> Message + 'a,
    {
        self.on_change = Some(OnChange::Multiple(Box::new(on_change)));
        self
    }

    /// Sets the message emitted when a drag ends.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Changed(f32),
    ///     Committed,
    /// }
    ///
    /// let theme = Theme::light();
    /// let slider = Slider::new(&theme)
    ///     .on_change(Message::Changed)
    ///     .on_release(Message::Committed);
    /// ```
    pub fn on_release(mut self, message: Message) -> Self
    where
        Message: Clone + 'a,
    {
        self.on_release = Some(Box::new(move || message.clone()));
        self
    }

    /// Patches the resolved [`SliderStyle`] right before it is painted.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let slider = Slider::<Message>::new(&theme).style_override(|mut style, status| {
    ///     if status.dragging {
    ///         style.ring_width += 2.0;
    ///     }
    ///
    ///     style
    /// });
    /// ```
    pub fn style_override(
        mut self,
        style_override: impl Fn(SliderStyle, SliderStatus) -> SliderStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the underlying iced canvas widget.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Slider, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let canvas = Slider::<Message>::new(&theme).into_canvas();
    /// ```
    pub fn into_canvas(self) -> canvas::Canvas<Self, Message> {
        let (width, height) = geometry::resolved_dimensions(&self);

        canvas::Canvas::new(self).width(width).height(height)
    }

    /// Whether the slider reports value changes.
    fn is_interactive(&self) -> bool {
        self.on_change.is_some()
    }

    /// Fractions of the lowest and highest value, for the range fill.
    fn range_fractions(&self) -> Option<(f32, f32)> {
        let mut values = self
            .values
            .iter()
            .copied()
            .filter(|value| value.is_finite());
        let first = values.next()?;
        let (lowest, highest) = values.fold((first, first), |(lowest, highest), value| {
            (lowest.min(value), highest.max(value))
        });

        Some((
            geometry::snapped_fraction(lowest, self.min, self.max, self.step),
            geometry::snapped_fraction(highest, self.min, self.max, self.step),
        ))
    }

    /// Keeps a thumb between its neighbors so thumbs never cross.
    fn clamp_to_neighbors(&self, index: usize, value: f32) -> f32 {
        let lower = index
            .checked_sub(1)
            .and_then(|previous| self.values.get(previous).copied())
            .unwrap_or(self.min);
        let upper = self.values.get(index + 1).copied().unwrap_or(self.max);

        value.clamp(lower.min(upper), upper.max(lower))
    }

    /// Action published when thumb `index` moves to `value`.
    fn change_action(&self, index: usize, value: f32) -> canvas::Action<Message> {
        let Some(current) = self.values.get(index).copied() else {
            return canvas::Action::request_redraw();
        };

        let value = self.clamp_to_neighbors(index, value);
        if (value - current).abs() <= f32::EPSILON {
            return canvas::Action::request_redraw();
        }

        match self.on_change.as_ref() {
            Some(OnChange::Single(callback)) => canvas::Action::publish(callback(value)),
            Some(OnChange::Multiple(callback)) => {
                let mut values = self.values.clone();
                values[index] = value;
                canvas::Action::publish(callback(values))
            }
            None => canvas::Action::request_redraw(),
        }
    }
}

/// Replaces a non-finite input with `fallback`.
fn sanitize(value: f32, fallback: f32) -> f32 {
    if value.is_finite() { value } else { fallback }
}

/// Wraps a [`Slider`] builder into an iced canvas widget.
///
/// ```rust
/// use iced_shadcn_v2::{Slider, Theme, slider};
///
/// # #[derive(Debug, Clone)]
/// # enum Message {}
/// let theme = Theme::light();
/// let widget = slider(Slider::<Message>::new(&theme));
/// ```
pub fn slider<Message>(
    slider: Slider<'_, Message>,
) -> canvas::Canvas<Slider<'_, Message>, Message> {
    slider.into_canvas()
}

impl<'a, Message: 'a> From<Slider<'a, Message>> for Element<'a, Message> {
    fn from(slider: Slider<'a, Message>) -> Self {
        slider.into_canvas().into()
    }
}
