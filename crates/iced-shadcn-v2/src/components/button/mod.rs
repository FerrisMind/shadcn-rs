//! Builder-first button component.
//!
//! The public API lives in this module; rendering, geometry, style resolution,
//! and error types are kept in focused private submodules.

mod error;
mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use error::ButtonBuildError;
pub use types::{ButtonRadius, ButtonSize, ButtonVariant};

pub(crate) use types::CornerFlatten;

use std::fmt;

use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::widget::button as iced_button;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Element, Length};

use shadcn_common::AccentColor;
use twill_core::prelude::Padding;

use crate::theme::Theme;

/// Builder-first button styled directly with iced types.
///
/// Theme tokens come from `shadcn-common` via [`Theme`]; iced styles are built
/// directly on top of `twill-core` tokens, without an intermediate style layer.
///
/// **Who owns style packs (Vega, Nova, …)?** The app’s [`Theme`], not this
/// builder. Pass `&theme` into every button. To show two style systems at once,
/// build two themes and pass a different reference to each button. To vary
/// treatment under one theme, use [`Self::variant`], [`Self::color`],
/// [`Self::radius`], and [`Self::size`].
///
/// [`Self::style_override`] only patches the resolved iced `button::Style`
/// (fill, text, border, shadow). It is not [`shadcn_common::StyleId`].
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{
///     AccentColor, Button, ButtonBuildError, ButtonSize, ButtonVariant, Padding, Spacing, Theme,
/// };
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Save,
/// }
///
/// fn save_button(theme: &Theme) -> Result<Element<'_, Message>, ButtonBuildError> {
///     Ok(Button::text("Save", theme)
///         .variant(ButtonVariant::Default)
///         .size(ButtonSize::Lg)
///         .color(AccentColor::Blue)
///         .padding(Padding::all(Spacing::S4))?
///         .on_press(Message::Save)
///         .into())
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Button<'a, Message> {
    content: ButtonContent<'a, Message>,
    theme: &'a Theme,
    variant: ButtonVariant,
    size: ButtonSize,
    radius: Option<ButtonRadius>,
    /// `None` = theme primary; `Some` = accent overlay from `shadcn-common`.
    color: Option<AccentColor>,
    width: Length,
    height: Option<Length>,
    padding: Option<crate::iced_compat::Padding>,
    full_width: bool,
    loading: bool,
    disabled: bool,
    on_press: Option<Message>,
    on_press_with: Option<Box<dyn Fn() -> Message + 'a>>,
    group_corners: Option<CornerFlatten>,
    style_override: Option<
        Box<dyn Fn(button_widget::Style, button_widget::Status) -> button_widget::Style + 'a>,
    >,
}

enum ButtonContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
    Icon(Element<'a, Message>),
}

impl<Message> fmt::Debug for Button<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            ButtonContent::Label(_) => "label",
            ButtonContent::Element(_) => "element",
            ButtonContent::Icon(_) => "icon",
        };

        formatter
            .debug_struct("Button")
            .field("content", &content)
            .field("theme", &self.theme)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("color", &self.color)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("full_width", &self.full_width)
            .field("loading", &self.loading)
            .field("disabled", &self.disabled)
            .field("on_press", &self.on_press.is_some())
            .field("on_press_with", &self.on_press_with.is_some())
            .field("group_corners", &self.group_corners)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Button<'a, Message> {
    /// Creates a new button from arbitrary content.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(ButtonContent::Element(content.into()), theme)
    }

    /// Creates a text button.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(ButtonContent::Label(label.into_fragment()), theme)
    }

    /// Creates an icon button.
    pub fn icon(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(ButtonContent::Icon(content.into()), theme)
    }

    fn from_content(content: ButtonContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            radius: None,
            color: None,
            width: Length::Shrink,
            height: None,
            padding: None,
            full_width: false,
            loading: false,
            disabled: false,
            on_press: None,
            on_press_with: None,
            group_corners: None,
            style_override: None,
        }
    }

    /// Sets the visual treatment of the button.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the preset control size.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the button corner radius.
    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Applies an accent color overlay to the button's theme tokens.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Alias for [`Button::color`] retained for semantic color APIs.
    pub fn tone(self, color: AccentColor) -> Self {
        self.color(color)
    }

    /// Use the theme primary (no per-button accent overlay).
    pub fn theme_primary(mut self) -> Self {
        self.color = None;
        self
    }

    /// Sets a custom button width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets a custom button height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets all supported sides of the button padding.
    ///
    /// [`twill_core::prelude::PaddingValue::Var`] cannot be resolved by iced
    /// and is rejected with [`ButtonBuildError::UnsupportedPaddingVariable`].
    /// The same applies to [`twill_core::prelude::Spacing::Auto`], which has
    /// no fixed-size iced representation.
    ///
    /// # Errors
    ///
    /// Returns [`ButtonBuildError`] when any padding side contains a custom
    /// variable or `auto` value. The builder is consumed either way; rebuild
    /// the button with a supported padding to recover.
    pub fn padding(mut self, padding: Padding) -> Result<Self, ButtonBuildError> {
        self.padding = Some(geometry::resolve_padding(padding)?);
        Ok(self)
    }

    /// Makes the button fill the available width.
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    /// Applies already-resolved iced padding, used by sibling components that
    /// validate padding through their own geometry helpers.
    pub(crate) fn padding_resolved(mut self, padding: crate::iced_compat::Padding) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Shows an animated spinner to the left of the label and disables press.
    ///
    /// Icon-only buttons replace their glyph with the spinner.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Disables the button while retaining its configured content.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the message emitted when the button is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self.on_press_with = None;
        self
    }

    /// Sets or clears the message emitted when the button is pressed.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self.on_press_with = None;
        self
    }

    /// Sets a lazy message factory invoked only when the button is pressed.
    ///
    /// This is useful for buttons whose message contains data computed from
    /// the clicked item. [`Self::on_press`] remains the cheaper choice for a
    /// message that can be constructed while the view is built.
    pub fn on_press_with(mut self, on_press: impl Fn() -> Message + 'a) -> Self {
        self.on_press = None;
        self.on_press_with = Some(Box::new(on_press));
        self
    }

    /// Applies a narrow iced-style escape hatch after internal style resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(button_widget::Style, button_widget::Status) -> button_widget::Style
        + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Crate-internal: flattens the given corners to a zero radius after
    /// variant styling and any [`Self::style_override`]. Used by group
    /// containers (button-group) to merge adjacent controls.
    pub(crate) fn flatten_corners(mut self, corners: CornerFlatten) -> Self {
        self.group_corners = corners.is_any().then_some(corners);
        self
    }

    /// Crate-internal: whether the configured variant paints a 1 px border
    /// in its resting state. Drives border merging inside group containers.
    pub(crate) fn has_resting_border(&self) -> bool {
        matches!(
            self.variant,
            ButtonVariant::Outline | ButtonVariant::Surface
        )
    }

    /// Builds the underlying `iced` button widget.
    pub fn into_button(self) -> button_widget::Button<'a, Message>
    where
        Message: Clone + 'a,
    {
        let Button {
            content,
            theme,
            variant,
            size,
            radius,
            color,
            width,
            height,
            padding,
            full_width,
            loading,
            disabled,
            on_press,
            on_press_with,
            group_corners,
            style_override,
        } = self;

        let icon = matches!(content, ButtonContent::Icon(_)) || size.is_icon();
        let control_height_px = size.control_height(theme);
        let control_height = height.unwrap_or(Length::Fixed(control_height_px));
        let resolved_width = geometry::resolve_button_width(
            width,
            control_height,
            full_width,
            icon,
            control_height_px,
        );

        let content = render::build_content(content, variant, size, loading, color, theme);
        let content = render::build_wrapper(content, full_width, icon);
        let disabled_state = disabled || loading || (on_press.is_none() && on_press_with.is_none());
        let resolved_padding = padding.unwrap_or_else(|| size.default_padding(theme));

        let mut widget = iced_button(content)
            .padding(resolved_padding)
            .width(resolved_width)
            .height(control_height);

        if !disabled_state {
            if let Some(on_press_with) = on_press_with {
                widget = widget.on_press_with(on_press_with);
            } else if let Some(message) = on_press {
                widget = widget.on_press(message);
            }
        }

        widget.style(move |_iced_theme, status| {
            let mut style =
                style::resolve_button_style(theme, variant, radius, color, disabled_state, status);

            if let Some(override_fn) = style_override.as_ref() {
                style = override_fn(style, status);
            }

            if let Some(corners) = group_corners {
                let radius = &mut style.border.radius;

                if corners.top_left {
                    radius.top_left = 0.0;
                }
                if corners.top_right {
                    radius.top_right = 0.0;
                }
                if corners.bottom_right {
                    radius.bottom_right = 0.0;
                }
                if corners.bottom_left {
                    radius.bottom_left = 0.0;
                }
            }

            style
        })
    }
}

impl<'a, Message> From<Button<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(button: Button<'a, Message>) -> Self {
        button.into_button().into()
    }
}

/// Crate-internal: default button corner radius in px for the active style
/// pack. Shared with group containers so sibling surfaces (e.g. the
/// button-group text cell) round consistently with buttons.
pub(crate) fn default_radius_px(theme: &Theme) -> f32 {
    style::default_radius_px(theme)
}
