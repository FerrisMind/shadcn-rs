//! Toggle component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! A toggle is a stateful button: the application owns the boolean `pressed`
//! value and receives the next one through [`Toggle::on_toggle`], mirroring the
//! `bind:pressed` contract of the web component. Visuals come from the active
//! style pack's `.cn-toggle` recipe, so a toggle changes typography, radius,
//! and footprint together with [`crate::Theme`] — Sera even goes uppercase,
//! exactly like its CSS.
//!
//! Beyond the web component's `pressed`, `variant`, `size`, and `disabled`
//! props, the builder exposes the `aria-invalid` treatment defined by the pack
//! CSS, radius presets, custom dimensions and padding, and a style escape
//! hatch — the same extras the [`crate::Button`] port offers.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Theme, Toggle, ToggleVariant};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     BoldToggled(bool),
//! }
//!
//! fn bold_toggle(theme: &Theme, bold: bool) -> Element<'_, Message> {
//!     Toggle::text("B", theme)
//!         .variant(ToggleVariant::Outline)
//!         .pressed(bold)
//!         .on_toggle(Message::BoldToggled)
//!         .into()
//! }
//! ```

mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::{ToggleRadius, ToggleSize, ToggleVariant};

use std::fmt;

use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::widget::button as iced_button;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Element, Length};

use crate::theme::Theme;

/// Builder-first toggle styled directly with iced types.
///
/// The control stays fully controlled: it keeps painting [`Self::pressed`]
/// until the application stores the value received from [`Self::on_toggle`].
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Theme, Toggle, ToggleSize, ToggleVariant};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     ItalicToggled(bool),
/// }
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     Toggle::text("Italic", theme)
///         .variant(ToggleVariant::Default)
///         .size(ToggleSize::Sm)
///         .pressed(true)
///         .on_toggle(Message::ItalicToggled)
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Toggle<'a, Message> {
    content: ToggleContent<'a, Message>,
    theme: &'a Theme,
    icon_start: Option<Element<'a, Message>>,
    icon_end: Option<Element<'a, Message>>,
    variant: ToggleVariant,
    size: ToggleSize,
    radius: Option<ToggleRadius>,
    pressed: bool,
    invalid: bool,
    disabled: bool,
    width: Length,
    height: Option<Length>,
    padding: Option<crate::iced_compat::Padding>,
    full_width: bool,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    style_override: Option<
        Box<dyn Fn(button_widget::Style, button_widget::Status) -> button_widget::Style + 'a>,
    >,
}

enum ToggleContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
    Icon(Element<'a, Message>),
}

impl<Message> fmt::Debug for Toggle<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            ToggleContent::Label(_) => "label",
            ToggleContent::Element(_) => "element",
            ToggleContent::Icon(_) => "icon",
        };

        formatter
            .debug_struct("Toggle")
            .field("content", &content)
            .field("theme", &self.theme)
            .field("icon_start", &self.icon_start.is_some())
            .field("icon_end", &self.icon_end.is_some())
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("pressed", &self.pressed)
            .field("invalid", &self.invalid)
            .field("disabled", &self.disabled)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("full_width", &self.full_width)
            .field("on_toggle", &self.on_toggle.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Toggle<'a, Message> {
    /// Creates a new toggle from arbitrary content.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::new(text("Bold"), &theme);
    /// ```
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(ToggleContent::Element(content.into()), theme)
    }

    /// Creates a text toggle styled by the pack's toggle typography.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::text("Italic", &theme);
    /// ```
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(ToggleContent::Label(label.into_fragment()), theme)
    }

    /// Creates an icon-only toggle with the pack's square footprint.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::icon(text("B"), &theme);
    /// ```
    pub fn icon(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(ToggleContent::Icon(content.into()), theme)
    }

    fn from_content(content: ToggleContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            icon_start: None,
            icon_end: None,
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            radius: None,
            pressed: false,
            invalid: false,
            disabled: false,
            width: Length::Shrink,
            height: None,
            padding: None,
            full_width: false,
            on_toggle: None,
            style_override: None,
        }
    }

    /// Sets the visual treatment of the toggle.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Theme, Toggle, ToggleVariant};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::text("Bold", &theme).variant(ToggleVariant::Outline);
    /// ```
    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the preset control size.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Theme, Toggle, ToggleSize};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::text("Bold", &theme).size(ToggleSize::Lg);
    /// ```
    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the toggle corner radius.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Theme, Toggle, ToggleRadius};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::text("Bold", &theme).radius(ToggleRadius::Full);
    /// ```
    pub fn radius(mut self, radius: ToggleRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Places an icon slot before the label.
    ///
    /// The slot keeps the pack's square icon footprint and the label gap, and
    /// the leading padding tightens to the pack's `inline-start` value — the
    /// same treatment as `has-data-[icon=inline-start]` in the web component.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::text("Bold", &theme).icon_start(text("B"));
    /// ```
    pub fn icon_start(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon_start = Some(icon.into());
        self
    }

    /// Places an icon slot after the label.
    ///
    /// ```rust
    /// use iced::widget::text;
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::text("Bold", &theme).icon_end(text("↗"));
    /// ```
    pub fn icon_end(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon_end = Some(icon.into());
        self
    }

    /// Sets the controlled state (`aria-pressed`).
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::text("Bold", &theme).pressed(true);
    /// ```
    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    /// Paints the `aria-invalid` destructive treatment.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::text("Bold", &theme).invalid(true);
    /// ```
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Disables the toggle while retaining its configured content.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::text("Bold", &theme).disabled(true);
    /// ```
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets a custom toggle width.
    ///
    /// ```rust
    /// use iced::Length;
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::text("Bold", &theme).width(Length::Fixed(120.0));
    /// ```
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets a custom toggle height.
    ///
    /// ```rust
    /// use iced::Length;
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::text("Bold", &theme).height(Length::Fixed(48.0));
    /// ```
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Overrides the pack's default horizontal padding.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::text("Bold", &theme).padding([0, 16]);
    /// ```
    pub fn padding(mut self, padding: impl Into<crate::iced_compat::Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Makes the toggle fill the available width.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::text("Bold", &theme).full_width();
    /// ```
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    /// Sets the callback invoked with the next state when the toggle is pressed.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Toggled(bool),
    /// }
    ///
    /// let theme = Theme::light();
    /// let toggle = Toggle::text("Bold", &theme).on_toggle(Message::Toggled);
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
    /// A toggle without a callback is inert but keeps its normal colors, which
    /// is how read-only previews are rendered.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Toggled(bool),
    /// }
    ///
    /// let theme = Theme::light();
    /// let toggle = Toggle::text("Bold", &theme).on_toggle_maybe(Some(Message::Toggled));
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
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Changed(bool),
    /// }
    ///
    /// let theme = Theme::light();
    /// let toggle = Toggle::text("Bold", &theme).on_change(Message::Changed);
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
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Pressed,
    /// }
    ///
    /// let theme = Theme::light();
    /// let toggle = Toggle::text("Bold", &theme).on_press(Message::Pressed);
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
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Pressed,
    /// }
    ///
    /// let theme = Theme::light();
    /// let toggle = Toggle::text("Bold", &theme).on_press_maybe(Some(Message::Pressed));
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

    /// Applies a narrow iced-style escape hatch after internal style resolution.
    ///
    /// ```rust
    /// use iced::Color;
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let toggle = Toggle::<Message>::text("Bold", &theme).style_override(|mut style, _| {
    ///     style.text_color = Color::from_rgb(1.0, 0.0, 1.0);
    ///     style
    /// });
    /// ```
    pub fn style_override(
        mut self,
        style_override: impl Fn(button_widget::Style, button_widget::Status) -> button_widget::Style
        + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    pub(crate) fn chain_style_override(
        mut self,
        style_override: impl Fn(button_widget::Style, button_widget::Status) -> button_widget::Style
        + 'a,
    ) -> Self {
        let previous = self.style_override.take();
        self.style_override = Some(Box::new(move |style, status| {
            let style = previous
                .as_ref()
                .map_or(style, |previous| previous(style, status));
            style_override(style, status)
        }));
        self
    }

    /// Builds the underlying `iced` button widget.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Theme, Toggle};
    ///
    /// #[derive(Debug, Clone)]
    /// enum Message {
    ///     Toggled(bool),
    /// }
    ///
    /// let theme = Theme::light();
    /// let widget = Toggle::text("Bold", &theme)
    ///     .on_toggle(Message::Toggled)
    ///     .into_button();
    /// ```
    pub fn into_button(self) -> button_widget::Button<'a, Message>
    where
        Message: Clone + 'a,
    {
        let Toggle {
            content,
            theme,
            icon_start,
            icon_end,
            variant,
            size,
            radius,
            pressed,
            invalid,
            disabled,
            width,
            height,
            padding,
            full_width,
            on_toggle,
            style_override,
        } = self;

        let icon_only = matches!(content, ToggleContent::Icon(_));
        let has_icon_start = icon_start.is_some();
        let has_icon_end = icon_end.is_some();
        let control_height_px = size.control_height(theme);
        let control_height = height.unwrap_or(Length::Fixed(control_height_px));
        let resolved_width = geometry::resolve_toggle_width(
            width,
            control_height,
            full_width,
            icon_only,
            control_height_px,
        );

        let content = render::build_content(content, icon_start, icon_end, size, theme);
        let content = render::build_wrapper(content, full_width, icon_only);
        let resolved_padding = padding.unwrap_or_else(|| {
            size.default_padding(theme, icon_only, has_icon_start, has_icon_end)
        });
        let message = on_toggle.map(|callback| callback(!pressed));

        let mut widget = iced_button(content)
            .padding(resolved_padding)
            .width(resolved_width)
            .height(control_height);

        if let Some(message) = message
            && !disabled
        {
            widget = widget.on_press(message);
        }

        widget.style(move |_iced_theme, status| {
            let mut style = style::resolve_toggle_style(
                theme, variant, pressed, radius, invalid, disabled, status,
            );

            if let Some(override_fn) = style_override.as_ref() {
                style = override_fn(style, status);
            }

            style
        })
    }
}

impl<'a, Message> From<Toggle<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(toggle: Toggle<'a, Message>) -> Self {
        toggle.into_button().into()
    }
}
