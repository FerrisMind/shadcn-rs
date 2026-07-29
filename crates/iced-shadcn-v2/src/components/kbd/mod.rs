//! Builder-first kbd component.
//!
//! The public API lives in this module; rendering, geometry, style resolution,
//! layout helpers, and error types are kept in focused private submodules.

mod error;
mod geometry;
mod min_width;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use error::KbdBuildError;
pub use types::{KbdRadius, KbdSurface};

use std::fmt;

use crate::iced_compat::alignment::Vertical;
use crate::iced_compat::widget::container;
use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Element, Length};

use twill_core::prelude::Padding;

use crate::theme::Theme;

/// Builder-first kbd styled directly with iced types.
///
/// Mirrors shadcn-svelte `Kbd.Root`: a small non-interactive chip displaying
/// textual keyboard input (`bg-muted`, `h-5`, `min-w-5`, `text-xs`). Content
/// can be a text label, an arbitrary element (e.g. an icon), or a label with
/// leading/trailing icon slots. The contextual web restyling (tooltip /
/// input-group ancestors) maps to [`KbdSurface`].
///
/// Theme tokens come from `shadcn-common` via [`Theme`]; per-style metrics
/// (height, padding, radius, text size) follow the `cn-kbd` rules of the
/// active style pack. Like the web `pointer-events-none` element, a kbd never
/// produces messages.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Kbd, KbdGroup, Theme};
///
/// fn shortcut(theme: &Theme) -> Element<'_, ()> {
///     KbdGroup::new()
///         .push(Kbd::text("Ctrl", theme))
///         .push(Kbd::text("Shift", theme))
///         .push(Kbd::text("P", theme))
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Kbd<'a, Message> {
    content: KbdContent<'a, Message>,
    theme: &'a Theme,
    surface: KbdSurface,
    radius: Option<KbdRadius>,
    width: Length,
    height: Option<Length>,
    min_width: Option<f32>,
    padding: Option<crate::iced_compat::Padding>,
    text_size: Option<f32>,
    icon_start: Option<Element<'a, Message>>,
    icon_end: Option<Element<'a, Message>>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

enum KbdContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for Kbd<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            KbdContent::Label(_) => "label",
            KbdContent::Element(_) => "element",
        };

        formatter
            .debug_struct("Kbd")
            .field("content", &content)
            .field("theme", &self.theme)
            .field("surface", &self.surface)
            .field("radius", &self.radius)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("min_width", &self.min_width)
            .field("padding", &self.padding)
            .field("text_size", &self.text_size)
            .field("icon_start", &self.icon_start.is_some())
            .field("icon_end", &self.icon_end.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Kbd<'a, Message> {
    /// Creates a new kbd from arbitrary content (e.g. an icon).
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(KbdContent::Element(content.into()), theme)
    }

    /// Creates a text kbd (`<Kbd.Root>B</Kbd.Root>`).
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(KbdContent::Label(label.into_fragment()), theme)
    }

    fn from_content(content: KbdContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            content,
            theme,
            surface: KbdSurface::Default,
            radius: None,
            width: Length::Shrink,
            height: None,
            min_width: None,
            padding: None,
            text_size: None,
            icon_start: None,
            icon_end: None,
            style_override: None,
        }
    }

    /// Declares the surface the kbd sits on (tooltip, input group, …).
    pub fn surface(mut self, surface: KbdSurface) -> Self {
        self.surface = surface;
        self
    }

    /// Sets the kbd corner radius (defaults to the style-pack value).
    pub fn radius(mut self, radius: KbdRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Sets a custom kbd width (defaults to `w-fit` with `min-w-5`).
    ///
    /// The minimum width only applies while the width is [`Length::Shrink`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets a custom kbd height (defaults to `h-5` / 20 px; 22 px for
    /// Luma / Sera).
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets the minimum width in px (defaults to the style-pack `min-w-5`).
    ///
    /// Pass `0.0` to let the kbd hug its content. Negative values are
    /// clamped to `0.0`.
    pub fn min_width(mut self, min_width: f32) -> Self {
        self.min_width = Some(min_width.max(0.0));
        self
    }

    /// Sets all supported sides of the kbd padding.
    ///
    /// [`twill_core::prelude::PaddingValue::Var`] cannot be resolved by iced
    /// and is rejected with [`KbdBuildError::UnsupportedPaddingVariable`].
    /// The same applies to [`twill_core::prelude::Spacing::Auto`], which has
    /// no fixed-size iced representation.
    ///
    /// # Errors
    ///
    /// Returns [`KbdBuildError`] when any padding side contains a custom
    /// variable or `auto` value. The builder is consumed either way; rebuild
    /// the kbd with a supported padding to recover.
    pub fn padding(mut self, padding: Padding) -> Result<Self, KbdBuildError> {
        self.padding = Some(geometry::resolve_padding(padding)?);
        Ok(self)
    }

    /// Sets the label text size in px (defaults to the style-pack `text-xs`).
    ///
    /// Values are clamped to at least 1 px. Element content is unaffected.
    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(text_size.max(1.0));
        self
    }

    /// Sets a leading icon rendered in a 12 px slot (shadcn `[&_svg]:size-3`).
    pub fn icon_start(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon_start = Some(icon.into());
        self
    }

    /// Sets a trailing icon rendered in a 12 px slot.
    pub fn icon_end(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon_end = Some(icon.into());
        self
    }

    /// Applies a narrow iced-style escape hatch after internal style
    /// resolution (the equivalent of the svelte `class` override).
    ///
    /// Text labels inherit `container::Style::text_color`, so changing it
    /// here recolors the label as well.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the kbd as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let Kbd {
            content,
            theme,
            surface,
            radius,
            width,
            height,
            min_width,
            padding,
            text_size,
            icon_start,
            icon_end,
            style_override,
        } = self;

        let resolved_padding = padding.unwrap_or_else(|| geometry::default_padding(theme));
        let control_height = height.unwrap_or(Length::Fixed(geometry::control_height(theme)));
        let text_size = text_size.unwrap_or_else(|| geometry::text_size(theme));

        let body = render::build_content(content, icon_start, icon_end, text_size, theme);
        let body = render::build_wrapper(body);

        // `min-w-*` only matters while the kbd hugs its content (`w-fit`);
        // the minimum is applied inside the padding (border-box like the web).
        let body = if width == Length::Shrink {
            let min = min_width.unwrap_or_else(|| geometry::min_width(theme));
            let inner_min = (min - resolved_padding.left - resolved_padding.right).max(0.0);
            min_width::min_width(body, inner_min)
        } else {
            body
        };

        container(body)
            .padding(resolved_padding)
            .width(width)
            .height(control_height)
            .style(move |_iced_theme| {
                let mut resolved = style::resolve_container_style(theme, surface, radius);

                if let Some(override_fn) = style_override.as_ref() {
                    resolved = override_fn(resolved);
                }

                resolved
            })
            .into()
    }
}

impl<'a, Message> From<Kbd<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(kbd: Kbd<'a, Message>) -> Self {
        kbd.into_element()
    }
}

/// Horizontal cluster of keys and text — port of shadcn-svelte `Kbd.Group`
/// (`inline-flex items-center gap-1`).
///
/// Children are usually [`Kbd`]s, but any element is accepted — the web
/// `kbd-demo` mixes kbds with a plain-text separator
/// (`<Kbd.Root>Ctrl</Kbd.Root> <span>+</span> <Kbd.Root>B</Kbd.Root>`),
/// while `kbd-group-demo` keeps the plus inside a single key
/// (`<Kbd.Root>Ctrl + B</Kbd.Root>`).
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Kbd, KbdGroup, Theme};
///
/// fn keys(theme: &Theme) -> Element<'_, ()> {
///     KbdGroup::new()
///         .push(Kbd::text("Ctrl", theme))
///         .push(iced::widget::text("+").size(12))
///         .push(Kbd::text("B", theme))
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct KbdGroup<'a, Message> {
    children: Vec<Element<'a, Message>>,
    spacing: f32,
}

impl<Message> fmt::Debug for KbdGroup<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("KbdGroup")
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> KbdGroup<'a, Message> {
    /// Creates an empty group with the default `gap-1` (4 px) spacing.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: geometry::DEFAULT_KBD_GAP,
        }
    }

    /// Creates a group with the given children.
    pub fn with_children(children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        Self {
            children: children.into_iter().collect(),
            spacing: geometry::DEFAULT_KBD_GAP,
        }
    }

    /// Appends a child (a [`Kbd`] or any other element).
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends every child of the given iterator.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the gap between children in px (clamped to at least 0 px).
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing.max(0.0);
        self
    }

    /// Builds the group as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        crate::iced_compat::widget::row(self.children)
            .spacing(self.spacing)
            .align_y(Vertical::Center)
            .into()
    }
}

impl<Message> Default for KbdGroup<'_, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<KbdGroup<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(group: KbdGroup<'a, Message>) -> Self {
        group.into_element()
    }
}
