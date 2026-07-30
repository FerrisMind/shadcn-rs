//! Builder-first tooltip component.
//!
//! Port of the shadcn-svelte tooltip (`Tooltip.Root` / `Trigger` /
//! `Content` + arrow) as a single iced builder: the trigger element is
//! wrapped by a custom widget that opens a floating bubble on hover. The
//! public API lives in this module; positioning math is shared through
//! [`shadcn_common::floating`], while widget/overlay internals live in
//! focused private submodules.

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::TooltipStyle;
pub use types::{TooltipAlign, TooltipSide};

use std::fmt;

use shadcn_common::{FloatingConfig, FloatingPadding, FloatingSticky, TOOLTIP_ANIMATION_MS};

use crate::iced_compat::widget::text::{Fragment, IntoFragment, LineHeight};
use crate::iced_compat::widget::{container, text};
use crate::iced_compat::{Element, Padding, Pixels, time::Duration};

use crate::fonts::iced_font;
use crate::theme::Theme;

/// Builder-first tooltip styled directly with iced types.
///
/// Wraps a trigger element and shows a floating label on hover, matching
/// shadcn-svelte defaults: opens instantly (`delayDuration = 0`), sits on
/// [`TooltipSide::Top`] with a diamond arrow, uses the swapped
/// `bg-foreground` / `text-background` pair, flips and shifts to stay
/// inside the window, and animates with the web
/// `fade-in-0 zoom-in-95 slide-in-from-*-2` entrance.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Button, ButtonVariant, Theme, Tooltip, TooltipSide};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Pressed,
/// }
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     Tooltip::text(
///         Button::text("Hover", theme)
///             .variant(ButtonVariant::Outline)
///             .on_press(Message::Pressed),
///         "Add to library",
///         theme,
///     )
///     .side(TooltipSide::Top)
///     .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Tooltip<'a, Message> {
    trigger: Element<'a, Message>,
    content: TooltipContent<'a, Message>,
    theme: &'a Theme,
    side: TooltipSide,
    align: TooltipAlign,
    side_offset: f32,
    align_offset: f32,
    delay: Duration,
    duration: Duration,
    animated: bool,
    disabled: bool,
    open: Option<bool>,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    arrow: bool,
    avoid_collisions: bool,
    collision_padding: FloatingPadding,
    sticky: FloatingSticky,
    hide_when_detached: bool,
    arrow_padding: Option<f32>,
    max_width: Option<f32>,
    style_override: Option<Box<dyn Fn(TooltipStyle) -> TooltipStyle + 'a>>,
}

enum TooltipContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for Tooltip<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            TooltipContent::Label(_) => "label",
            TooltipContent::Element(_) => "element",
        };

        formatter
            .debug_struct("Tooltip")
            .field("content", &content)
            .field("theme", &self.theme)
            .field("side", &self.side)
            .field("align", &self.align)
            .field("side_offset", &self.side_offset)
            .field("align_offset", &self.align_offset)
            .field("delay", &self.delay)
            .field("duration", &self.duration)
            .field("animated", &self.animated)
            .field("disabled", &self.disabled)
            .field("open", &self.open)
            .field("on_open_change", &self.on_open_change.is_some())
            .field("arrow", &self.arrow)
            .field("avoid_collisions", &self.avoid_collisions)
            .field("collision_padding", &self.collision_padding)
            .field("sticky", &self.sticky)
            .field("hide_when_detached", &self.hide_when_detached)
            .field("arrow_padding", &self.arrow_padding)
            .field("max_width", &self.max_width)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Tooltip<'a, Message> {
    /// Creates a tooltip with arbitrary content over `trigger`.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`. Custom content that sets its
    /// own text colors opts out of the fade-in of the label text.
    pub fn new(
        trigger: impl Into<Element<'a, Message>>,
        content: impl Into<Element<'a, Message>>,
        theme: &'a Theme,
    ) -> Self {
        Self::from_content(
            trigger.into(),
            TooltipContent::Element(content.into()),
            theme,
        )
    }

    /// Creates a text tooltip over `trigger`.
    pub fn text(
        trigger: impl Into<Element<'a, Message>>,
        label: impl IntoFragment<'a>,
        theme: &'a Theme,
    ) -> Self {
        Self::from_content(
            trigger.into(),
            TooltipContent::Label(label.into_fragment()),
            theme,
        )
    }

    fn from_content(
        trigger: Element<'a, Message>,
        content: TooltipContent<'a, Message>,
        theme: &'a Theme,
    ) -> Self {
        let defaults = FloatingConfig::default();

        Self {
            trigger,
            content,
            theme,
            side: TooltipSide::default(),
            align: TooltipAlign::default(),
            side_offset: defaults.side_offset,
            align_offset: defaults.align_offset,
            delay: Duration::ZERO,
            duration: Duration::from_millis(TOOLTIP_ANIMATION_MS),
            animated: true,
            disabled: false,
            open: None,
            on_open_change: None,
            arrow: true,
            avoid_collisions: defaults.avoid_collisions,
            collision_padding: defaults.collision_padding,
            sticky: defaults.sticky,
            hide_when_detached: defaults.hide_when_detached,
            arrow_padding: None,
            max_width: None,
            style_override: None,
        }
    }

    /// Sets the side of the trigger the tooltip opens on.
    pub fn side(mut self, side: TooltipSide) -> Self {
        self.side = side;
        self
    }

    /// Sets the alignment along the trigger edge.
    pub fn align(mut self, align: TooltipAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets the gap between the trigger and the bubble (`sideOffset`).
    ///
    /// The arrow adds its own gap on top, exactly like the web floating
    /// layer offsets the content by the arrow height.
    pub fn side_offset(mut self, offset: f32) -> Self {
        self.side_offset = offset;
        self
    }

    /// Sets the offset along the trigger edge (`alignOffset`).
    pub fn align_offset(mut self, offset: f32) -> Self {
        self.align_offset = offset;
        self
    }

    /// Sets the hover delay before the tooltip opens (`delayDuration`).
    ///
    /// shadcn-svelte's provider defaults to zero — tooltips open instantly.
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Sets the duration of the open/close animation.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Enables or disables the open/close animation.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Prevents the tooltip from opening while keeping the trigger active.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Controls the open state explicitly instead of following hover.
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Controls the open state when `Some`, follows hover when `None`.
    pub fn open_maybe(mut self, open: Option<bool>) -> Self {
        self.open = open;
        self
    }

    /// Notifies about open-state changes (`onOpenChange`).
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Shows or hides the diamond arrow.
    pub fn arrow(mut self, arrow: bool) -> Self {
        self.arrow = arrow;
        self
    }

    /// Flips and shifts the bubble to keep it inside the window
    /// (`avoidCollisions`).
    pub fn avoid_collisions(mut self, avoid: bool) -> Self {
        self.avoid_collisions = avoid;
        self
    }

    /// Sets the minimum distance kept from the window edges
    /// (`collisionPadding`).
    pub fn collision_padding(mut self, padding: impl Into<FloatingPadding>) -> Self {
        self.collision_padding = padding.into();
        self
    }

    /// Sets the cross-axis shift behavior while avoiding collisions
    /// (`sticky`).
    pub fn sticky(mut self, sticky: FloatingSticky) -> Self {
        self.sticky = sticky;
        self
    }

    /// Hides the bubble when the trigger is scrolled outside the window
    /// (`hideWhenDetached`).
    pub fn hide_when_detached(mut self, hide: bool) -> Self {
        self.hide_when_detached = hide;
        self
    }

    /// Sets the minimum distance between the arrow and the bubble corners
    /// (`arrowPadding`). Defaults to clearing the corner radius.
    pub fn arrow_padding(mut self, padding: f32) -> Self {
        self.arrow_padding = Some(padding);
        self
    }

    /// Overrides the maximum bubble width (`max-w-xs` by default).
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width.max(0.0));
        self
    }

    /// Patches the resolved [`TooltipStyle`] (colors, radius, arrow
    /// geometry) after theme resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(TooltipStyle) -> TooltipStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> From<Tooltip<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(tooltip: Tooltip<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(tooltip.theme);
        let mut resolved = style::resolve_style(tooltip.theme);

        if let Some(style_override) = tooltip.style_override.as_ref() {
            resolved = style_override(resolved);
        }

        let inner: Element<'a, Message> = match tooltip.content {
            TooltipContent::Label(label) => text(label)
                .size(recipe.typography.size_px)
                .line_height(LineHeight::Absolute(Pixels(
                    recipe.typography.line_height_px,
                )))
                .font(iced_font(tooltip.theme.font_pack().sans))
                .into(),
            TooltipContent::Element(element) => element,
        };

        let content = container(inner)
            .padding(Padding {
                top: recipe.pad_y_px,
                right: recipe.pad_x_px,
                bottom: recipe.pad_y_px,
                left: recipe.pad_x_px,
            })
            .max_width(tooltip.max_width.unwrap_or(recipe.max_width_px));

        // The arrow must clear the rounded corner unless the caller asks
        // for a tighter clamp explicitly.
        let arrow_padding = tooltip
            .arrow_padding
            .unwrap_or(resolved.radius + resolved.arrow_size / 2.0);

        let config = FloatingConfig::default()
            .side(tooltip.side.to_floating())
            .align(tooltip.align.to_floating())
            .side_offset(tooltip.side_offset)
            .align_offset(tooltip.align_offset)
            .avoid_collisions(tooltip.avoid_collisions)
            .collision_padding(tooltip.collision_padding)
            .sticky(tooltip.sticky)
            .hide_when_detached(tooltip.hide_when_detached)
            .arrow_padding(arrow_padding);

        Element::new(render::TooltipWidget {
            trigger: tooltip.trigger,
            content: content.into(),
            config,
            delay: tooltip.delay,
            duration: tooltip.duration,
            animated: tooltip.animated,
            disabled: tooltip.disabled,
            open_override: tooltip.open,
            on_open_change: tooltip.on_open_change,
            arrow: tooltip.arrow,
            style: resolved,
        })
    }
}
