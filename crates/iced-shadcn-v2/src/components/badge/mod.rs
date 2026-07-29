//! Builder-first badge component.
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

pub use error::BadgeBuildError;
pub use types::{BadgeRadius, BadgeVariant};

use std::fmt;

use crate::iced_compat::widget::button as button_widget;
use crate::iced_compat::widget::button as iced_button;
use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Element, Length};

use shadcn_common::AccentColor;
use twill_core::prelude::Padding;

use crate::theme::Theme;

/// Builder-first badge styled directly with iced types.
///
/// Mirrors shadcn-svelte `Badge`: variants (`default` / `secondary` /
/// `destructive` / `outline` / `ghost` / `link`), optional accent color,
/// leading/trailing icons, inline spinner, and optional press handling for the
/// “as link” pattern.
///
/// Theme tokens come from `shadcn-common` via [`Theme`]. Non-interactive badges
/// render as a styled `container`; interactive ones (`on_press`) use an iced
/// `button` so hover/press treatments match the web `[a]:hover:*` rules.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{
///     AccentColor, Badge, BadgeBuildError, BadgeVariant, Theme,
/// };
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Open,
/// }
///
/// fn status_badge(theme: &Theme) -> Result<Element<'_, Message>, BadgeBuildError> {
///     Ok(Badge::text("New", theme)
///         .variant(BadgeVariant::Secondary)
///         .color(AccentColor::Blue)
///         .on_press(Message::Open)
///         .into())
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Badge<'a, Message> {
    content: BadgeContent<'a, Message>,
    theme: &'a Theme,
    variant: BadgeVariant,
    radius: Option<BadgeRadius>,
    /// `None` = theme primary; `Some` = accent overlay from `shadcn-common`.
    color: Option<AccentColor>,
    width: Length,
    height: Option<Length>,
    padding: Option<crate::iced_compat::Padding>,
    loading: bool,
    disabled: bool,
    icon_start: Option<Element<'a, Message>>,
    icon_end: Option<Element<'a, Message>>,
    on_press: Option<Message>,
    style_override: Option<BadgeStyleOverride<'a>>,
}

enum BadgeContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

enum BadgeStyleOverride<'a> {
    Container(Box<dyn Fn(container::Style) -> container::Style + 'a>),
    Button(Box<dyn Fn(button_widget::Style, button_widget::Status) -> button_widget::Style + 'a>),
}

impl<Message> fmt::Debug for Badge<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            BadgeContent::Label(_) => "label",
            BadgeContent::Element(_) => "element",
        };

        formatter
            .debug_struct("Badge")
            .field("content", &content)
            .field("theme", &self.theme)
            .field("variant", &self.variant)
            .field("radius", &self.radius)
            .field("color", &self.color)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("loading", &self.loading)
            .field("disabled", &self.disabled)
            .field("icon_start", &self.icon_start.is_some())
            .field("icon_end", &self.icon_end.is_some())
            .field("on_press", &self.on_press.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Badge<'a, Message> {
    /// Creates a new badge from arbitrary content.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(BadgeContent::Element(content.into()), theme)
    }

    /// Creates a text badge.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(BadgeContent::Label(label.into_fragment()), theme)
    }

    fn from_content(content: BadgeContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            variant: BadgeVariant::Default,
            radius: None,
            color: None,
            width: Length::Shrink,
            height: None,
            padding: None,
            loading: false,
            disabled: false,
            icon_start: None,
            icon_end: None,
            on_press: None,
            style_override: None,
        }
    }

    /// Sets the visual treatment of the badge.
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the badge corner radius.
    pub fn radius(mut self, radius: BadgeRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Applies an accent color overlay to the badge's theme tokens.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Alias for [`Badge::color`] retained for semantic color APIs.
    pub fn tone(self, color: AccentColor) -> Self {
        self.color(color)
    }

    /// Use the theme primary (no per-badge accent overlay).
    pub fn theme_primary(mut self) -> Self {
        self.color = None;
        self
    }

    /// Sets a custom badge width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets a custom badge height (defaults to `h-5` / 20 px).
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets all supported sides of the badge padding.
    ///
    /// [`twill_core::prelude::PaddingValue::Var`] cannot be resolved by iced
    /// and is rejected with [`BadgeBuildError::UnsupportedPaddingVariable`].
    /// The same applies to [`twill_core::prelude::Spacing::Auto`], which has
    /// no fixed-size iced representation.
    ///
    /// # Errors
    ///
    /// Returns [`BadgeBuildError`] when any padding side contains a custom
    /// variable or `auto` value. The builder is consumed either way; rebuild
    /// the badge with a supported padding to recover.
    pub fn padding(mut self, padding: Padding) -> Result<Self, BadgeBuildError> {
        self.padding = Some(geometry::resolve_padding(padding)?);
        Ok(self)
    }

    /// Shows an animated spinner to the left of the label (replaces `icon_start`).
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Disables an interactive badge while retaining its configured content.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets a leading icon (shadcn `data-icon="inline-start"`).
    pub fn icon_start(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon_start = Some(icon.into());
        self
    }

    /// Sets a trailing icon (shadcn `data-icon="inline-end"`).
    pub fn icon_end(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon_end = Some(icon.into());
        self
    }

    /// Sets the message emitted when the badge is pressed (as-link pattern).
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets or clears the message emitted when the badge is pressed.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Applies a narrow iced-style escape hatch after internal style resolution.
    ///
    /// Used for non-interactive (container) badges. Prefer
    /// [`Self::button_style_override`] when the badge is interactive.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(BadgeStyleOverride::Container(Box::new(style_override)));
        self
    }

    /// Applies a narrow iced-style escape hatch for interactive badges.
    pub fn button_style_override(
        mut self,
        style_override: impl Fn(button_widget::Style, button_widget::Status) -> button_widget::Style
        + 'a,
    ) -> Self {
        self.style_override = Some(BadgeStyleOverride::Button(Box::new(style_override)));
        self
    }

    /// Builds the badge as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let Badge {
            content,
            theme,
            variant,
            radius,
            color,
            width,
            height,
            padding,
            loading,
            disabled,
            icon_start,
            icon_end,
            on_press,
            style_override,
        } = self;

        let has_icon_start = loading || icon_start.is_some();
        let has_icon_end = icon_end.is_some();
        let recipe = theme.style.badge();
        let resolved_padding = padding
            .unwrap_or_else(|| geometry::default_padding(theme, has_icon_start, has_icon_end));
        let control_height = height.unwrap_or_else(|| {
            Length::Fixed(recipe.height_px.unwrap_or_else(|| {
                // Sera has no fixed height — shrink to content with a sensible floor.
                recipe.typography.line_height_px.max(16.0)
            }))
        });

        let body = render::build_content(
            content, icon_start, icon_end, variant, loading, color, theme,
        );
        let body = render::build_wrapper(body);

        if let Some(message) = on_press.filter(|_| !disabled && !loading) {
            let mut widget = iced_button(body)
                .padding(resolved_padding)
                .width(width)
                .height(control_height)
                .on_press(message);

            widget = widget.style(move |_iced_theme, status| {
                let mut style =
                    style::resolve_button_style(theme, variant, radius, color, false, status);

                if let Some(BadgeStyleOverride::Button(override_fn)) = style_override.as_ref() {
                    style = override_fn(style, status);
                }

                style
            });

            widget.into()
        } else {
            let mut widget = container(body)
                .padding(resolved_padding)
                .width(width)
                .height(control_height);

            widget = widget.style(move |_iced_theme| {
                let mut resolved = if disabled {
                    // Reuse the disabled button visual as a container style.
                    let button_style = style::resolve_button_style(
                        theme,
                        variant,
                        radius,
                        color,
                        true,
                        button_widget::Status::Disabled,
                    );
                    container::Style {
                        background: button_style.background,
                        text_color: Some(button_style.text_color),
                        border: button_style.border,
                        shadow: button_style.shadow,
                        snap: button_style.snap,
                    }
                } else {
                    style::resolve_container_style(theme, variant, radius, color)
                };

                if let Some(BadgeStyleOverride::Container(override_fn)) = style_override.as_ref() {
                    resolved = override_fn(resolved);
                }

                resolved
            });

            widget.into()
        }
    }
}

impl<'a, Message> From<Badge<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(badge: Badge<'a, Message>) -> Self {
        badge.into_element()
    }
}
