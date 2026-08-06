//! Switch component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! The switch is a controlled two-state control: the application owns the
//! boolean and receives the next value through [`Switch::on_toggle`], exactly
//! like the `bind:checked` contract of the web component. Track and thumb
//! geometry come from the active style pack (`.cn-switch`), so a switch changes
//! shape together with [`crate::Theme`]. The thumb slides between both ends
//! instead of jumping, mirroring the `transition-transform` of the original.
//!
//! Beyond the web component's `checked`, `size`, and `disabled` props, the
//! builder exposes the states the pack CSS defines but the web props do not
//! (focus ring, `aria-invalid` ring), per-switch accent colors, radius
//! presets, animation control, and a style escape hatch.
//!
//! Because a canvas widget cannot take keyboard focus in iced, the focus ring is
//! painted from application state via [`Switch::focused`]. The widget always
//! reserves the ring's width around the track, so toggling focus never reflows
//! the layout; that reserve also acts as the pointer slop the web component adds
//! with its `after:-inset-*` pseudo-element.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Switch, SwitchSize, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     AirplaneModeToggled(bool),
//! }
//!
//! fn airplane_mode(theme: &Theme, enabled: bool) -> Element<'_, Message> {
//!     Switch::new(theme)
//!         .checked(enabled)
//!         .size(SwitchSize::Default)
//!         .on_toggle(Message::AirplaneModeToggled)
//!         .into()
//! }
//! ```

mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{SwitchRadius, SwitchSize, SwitchState, SwitchStatus, SwitchStyle};

use std::fmt;
use std::time::Duration;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::{Color, Element};

use shadcn_common::AccentColor;

use crate::theme::Theme;

/// Duration of one thumb transition (`transition-transform` default).
const DEFAULT_TRANSITION: Duration = Duration::from_millis(150);

/// Builder-first switch styled from `shadcn-common` theme tokens.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{AccentColor, Switch, SwitchSize, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Toggled(bool),
/// }
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     Switch::new(theme)
///         .checked(true)
///         .size(SwitchSize::Sm)
///         .color(AccentColor::Emerald)
///         .on_toggle(Message::Toggled)
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Switch<'a, Message> {
    theme: &'a Theme,
    checked: bool,
    size: SwitchSize,
    disabled: bool,
    focused: bool,
    invalid: bool,
    /// `None` = theme primary; `Some` = accent overlay from `shadcn-common`.
    color: Option<AccentColor>,
    checked_color: Option<Color>,
    track_color: Option<Color>,
    thumb_color: Option<Color>,
    radius: Option<SwitchRadius>,
    animated: bool,
    duration: Duration,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    style_override: Option<Box<dyn Fn(SwitchStyle, SwitchStatus) -> SwitchStyle + 'a>>,
}

impl<Message> fmt::Debug for Switch<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Switch")
            .field("theme", &self.theme)
            .field("checked", &self.checked)
            .field("size", &self.size)
            .field("disabled", &self.disabled)
            .field("focused", &self.focused)
            .field("invalid", &self.invalid)
            .field("color", &self.color)
            .field("checked_color", &self.checked_color)
            .field("track_color", &self.track_color)
            .field("thumb_color", &self.thumb_color)
            .field("radius", &self.radius)
            .field("animated", &self.animated)
            .field("duration", &self.duration)
            .field("on_toggle", &self.on_toggle.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Switch<'a, Message> {
    /// Creates an unchecked switch using the active theme.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme);
    /// ```
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            checked: false,
            size: SwitchSize::Default,
            disabled: false,
            focused: false,
            invalid: false,
            color: None,
            checked_color: None,
            track_color: None,
            thumb_color: None,
            radius: None,
            animated: true,
            duration: DEFAULT_TRANSITION,
            on_toggle: None,
            style_override: None,
        }
    }

    /// Sets the controlled state.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme).checked(true);
    /// ```
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Sets the preset footprint.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, SwitchSize, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme).size(SwitchSize::Sm);
    /// ```
    pub fn size(mut self, size: SwitchSize) -> Self {
        self.size = size;
        self
    }

    /// Suppresses interaction and dims the control.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme).disabled(true);
    /// ```
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Paints the `focus-visible` ring of the active style pack.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme).focused(true);
    /// ```
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Paints the `aria-invalid` destructive border and ring.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme).invalid(true);
    /// ```
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Applies an accent color overlay to the checked track and thumb.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{AccentColor, Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme).color(AccentColor::Blue);
    /// ```
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self.checked_color = None;
        self
    }

    /// Uses the theme primary (no per-switch accent overlay).
    ///
    /// ```rust
    /// use iced_shadcn_v2::{AccentColor, Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme)
    ///     .color(AccentColor::Blue)
    ///     .theme_primary();
    /// ```
    pub fn theme_primary(mut self) -> Self {
        self.color = None;
        self.checked_color = None;
        self
    }

    /// Uses an explicit iced color for the checked track.
    ///
    /// ```rust
    /// use iced::Color;
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme).checked_color(Color::BLACK);
    /// ```
    pub fn checked_color(mut self, color: Color) -> Self {
        self.checked_color = Some(color);
        self.color = None;
        self
    }

    /// Uses an explicit iced color for the unchecked track.
    ///
    /// ```rust
    /// use iced::Color;
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme).track_color(Color::from_rgb(0.9, 0.9, 0.9));
    /// ```
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = Some(color);
        self
    }

    /// Uses an explicit iced color for the thumb.
    ///
    /// ```rust
    /// use iced::Color;
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme).thumb_color(Color::WHITE);
    /// ```
    pub fn thumb_color(mut self, color: Color) -> Self {
        self.thumb_color = Some(color);
        self
    }

    /// Sets the corner radius of the track and thumb.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, SwitchRadius, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme).radius(SwitchRadius::None);
    /// ```
    pub fn radius(mut self, radius: SwitchRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Enables or disables the thumb transition.
    ///
    /// A non-animated switch snaps to its state on the next frame.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme).animated(false);
    /// ```
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the thumb transition duration (clamped to at least 1 ms).
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme).duration(Duration::from_millis(250));
    /// ```
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration.max(Duration::from_millis(1));
        self
    }

    /// Sets the thumb transition duration in milliseconds.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme).duration_ms(250);
    /// ```
    pub fn duration_ms(self, duration_ms: u32) -> Self {
        self.duration(Duration::from_millis(u64::from(duration_ms)))
    }

    /// Sets the callback invoked with the next state when the switch is pressed.
    ///
    /// The switch stays controlled: it keeps painting [`Self::checked`] until the
    /// application stores the new value.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Toggled(bool),
    /// }
    ///
    /// let theme = Theme::light();
    /// let switch = Switch::new(&theme).on_toggle(Message::Toggled);
    /// ```
    pub fn on_toggle<F>(mut self, on_toggle: F) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }

    /// Sets or clears the toggle callback.
    ///
    /// A switch without a callback is inert but keeps its normal colors, which
    /// is how read-only previews are rendered.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Toggled(bool),
    /// }
    ///
    /// let theme = Theme::light();
    /// let switch = Switch::new(&theme).on_toggle_maybe(Some(Message::Toggled));
    /// ```
    pub fn on_toggle_maybe<F>(mut self, on_toggle: Option<F>) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        self.on_toggle = on_toggle.map(|callback| Box::new(callback) as _);
        self
    }

    /// Alias for [`Self::on_toggle`] using the terminology of shadcn-svelte.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Changed(bool),
    /// }
    ///
    /// let theme = Theme::light();
    /// let switch = Switch::new(&theme).on_change(Message::Changed);
    /// ```
    pub fn on_change<F>(self, on_change: F) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        self.on_toggle(on_change)
    }

    /// Sets a message emitted on every press, ignoring the next state.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Pressed,
    /// }
    ///
    /// let theme = Theme::light();
    /// let switch = Switch::new(&theme).on_press(Message::Pressed);
    /// ```
    pub fn on_press(self, message: Message) -> Self
    where
        Message: Clone + 'a,
    {
        self.on_toggle(move |_| message.clone())
    }

    /// Sets or clears the message emitted on every press.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Pressed,
    /// }
    ///
    /// let theme = Theme::light();
    /// let switch = Switch::new(&theme).on_press_maybe(Some(Message::Pressed));
    /// ```
    pub fn on_press_maybe(self, message: Option<Message>) -> Self
    where
        Message: Clone + 'a,
    {
        match message {
            Some(message) => self.on_press(message),
            None => self.on_toggle_maybe(None::<fn(bool) -> Message>),
        }
    }

    /// Patches the resolved [`SwitchStyle`] right before it is painted.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let switch = Switch::<Message>::new(&theme).style_override(|mut style, status| {
    ///     if status.hovered {
    ///         style.border_width += 1.0;
    ///     }
    ///
    ///     style
    /// });
    /// ```
    pub fn style_override(
        mut self,
        style_override: impl Fn(SwitchStyle, SwitchStatus) -> SwitchStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the underlying iced canvas widget.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Switch, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let canvas = Switch::<Message>::new(&theme).into_canvas();
    /// ```
    pub fn into_canvas(self) -> canvas::Canvas<Self, Message> {
        let (width, height) = geometry::resolved_dimensions(self.theme, self.size);

        canvas::Canvas::new(self).width(width).height(height)
    }
}

/// Wraps a [`Switch`] builder into an iced canvas widget.
///
/// ```rust
/// use iced_shadcn_v2::{Switch, Theme, switch};
///
/// # #[derive(Debug, Clone)]
/// # enum Message {}
/// let theme = Theme::light();
/// let widget = switch(Switch::<Message>::new(&theme));
/// ```
pub fn switch<Message>(
    switch: Switch<'_, Message>,
) -> canvas::Canvas<Switch<'_, Message>, Message> {
    switch.into_canvas()
}

impl<'a, Message: 'a> From<Switch<'a, Message>> for Element<'a, Message> {
    fn from(switch: Switch<'a, Message>) -> Self {
        switch.into_canvas().into()
    }
}
