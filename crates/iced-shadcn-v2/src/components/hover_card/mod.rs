//! Builder-first hover-card component.
//!
//! Port of the shadcn-svelte hover card (`HoverCard.Root` / `Trigger` /
//! `Content`, the bits-ui link preview) as a single iced builder: the
//! trigger element is wrapped by a custom widget that opens a floating,
//! interactive surface on hover. The card opens after `openDelay`, stays
//! up while the cursor is over the trigger or the content, and closes
//! after `closeDelay` once both are left. The public API lives in this
//! module; positioning math is shared through [`shadcn_common::floating`],
//! while widget/overlay internals live in focused private submodules.

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::HoverCardStyle;
pub use types::{HoverCardAlign, HoverCardSide};

use std::fmt;

use shadcn_common::{
    FloatingConfig, FloatingPadding, FloatingSticky, HOVER_CARD_ANIMATION_MS,
    HOVER_CARD_CLOSE_DELAY_MS, HOVER_CARD_OPEN_DELAY_MS,
};

use crate::iced_compat::widget::text::{Fragment, IntoFragment, LineHeight};
use crate::iced_compat::widget::{container, text};
use crate::iced_compat::{Element, Length, Padding, Pixels, time::Duration};

use crate::fonts::iced_font;
use crate::theme::Theme;

/// Default `sideOffset` of the shadcn-svelte hover-card content.
const DEFAULT_SIDE_OFFSET: f32 = 4.0;

/// Builder-first hover card styled directly with iced types.
///
/// Wraps a trigger element and opens a floating surface on hover, matching
/// shadcn-svelte defaults: opens on [`HoverCardSide::Bottom`] with
/// `sideOffset = 4` after a 700 ms `openDelay`, stays open while the
/// cursor is over the trigger or the content, closes 300 ms after both
/// are left (`closeDelay`), paints the `bg-popover` /
/// `text-popover-foreground` pair with a `ring-1 ring-foreground/N`
/// hairline and a drop shadow, is `w-64` wide with `p-4` padding, flips
/// and shifts to stay inside the window, and animates with the web
/// `fade-in-0 zoom-in-95 slide-in-from-*-2` entrance.
///
/// Content stays fully interactive — links, buttons, and inputs inside
/// the surface receive events like any other widget.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Button, ButtonVariant, HoverCard, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Pressed,
/// }
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     HoverCard::text(
///         Button::text("@sveltejs", theme)
///             .variant(ButtonVariant::Link)
///             .on_press(Message::Pressed),
///         "Cybernetically enhanced web apps.",
///         theme,
///     )
///     .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct HoverCard<'a, Message> {
    trigger: Element<'a, Message>,
    content: HoverCardContent<'a, Message>,
    theme: &'a Theme,
    side: HoverCardSide,
    align: HoverCardAlign,
    side_offset: f32,
    align_offset: f32,
    open_delay: Duration,
    close_delay: Duration,
    width: Option<f32>,
    duration: Duration,
    animated: bool,
    disabled: bool,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    avoid_collisions: bool,
    collision_padding: FloatingPadding,
    sticky: FloatingSticky,
    hide_when_detached: bool,
    close_on_click_outside: bool,
    close_on_escape: bool,
    style_override: Option<Box<dyn Fn(HoverCardStyle) -> HoverCardStyle + 'a>>,
}

enum HoverCardContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for HoverCard<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            HoverCardContent::Label(_) => "label",
            HoverCardContent::Element(_) => "element",
        };

        formatter
            .debug_struct("HoverCard")
            .field("content", &content)
            .field("theme", &self.theme)
            .field("side", &self.side)
            .field("align", &self.align)
            .field("side_offset", &self.side_offset)
            .field("align_offset", &self.align_offset)
            .field("open_delay", &self.open_delay)
            .field("close_delay", &self.close_delay)
            .field("width", &self.width)
            .field("duration", &self.duration)
            .field("animated", &self.animated)
            .field("disabled", &self.disabled)
            .field("open", &self.open)
            .field("default_open", &self.default_open)
            .field("on_open_change", &self.on_open_change.is_some())
            .field("avoid_collisions", &self.avoid_collisions)
            .field("collision_padding", &self.collision_padding)
            .field("sticky", &self.sticky)
            .field("hide_when_detached", &self.hide_when_detached)
            .field("close_on_click_outside", &self.close_on_click_outside)
            .field("close_on_escape", &self.close_on_escape)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> HoverCard<'a, Message> {
    /// Creates a hover card with arbitrary content over `trigger`.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`. Custom content that sets its
    /// own text colors opts out of the fade-in of the default text color.
    pub fn new(
        trigger: impl Into<Element<'a, Message>>,
        content: impl Into<Element<'a, Message>>,
        theme: &'a Theme,
    ) -> Self {
        Self::from_content(
            trigger.into(),
            HoverCardContent::Element(content.into()),
            theme,
        )
    }

    /// Creates a text hover card over `trigger`, typeset with the body
    /// typography of the active style pack (`text-sm` / `text-xs`).
    pub fn text(
        trigger: impl Into<Element<'a, Message>>,
        label: impl IntoFragment<'a>,
        theme: &'a Theme,
    ) -> Self {
        Self::from_content(
            trigger.into(),
            HoverCardContent::Label(label.into_fragment()),
            theme,
        )
    }

    fn from_content(
        trigger: Element<'a, Message>,
        content: HoverCardContent<'a, Message>,
        theme: &'a Theme,
    ) -> Self {
        let defaults = FloatingConfig::default();

        Self {
            trigger,
            content,
            theme,
            side: HoverCardSide::default(),
            align: HoverCardAlign::default(),
            side_offset: DEFAULT_SIDE_OFFSET,
            align_offset: defaults.align_offset,
            open_delay: Duration::from_millis(HOVER_CARD_OPEN_DELAY_MS),
            close_delay: Duration::from_millis(HOVER_CARD_CLOSE_DELAY_MS),
            width: None,
            duration: Duration::from_millis(HOVER_CARD_ANIMATION_MS),
            animated: true,
            disabled: false,
            open: None,
            default_open: false,
            on_open_change: None,
            avoid_collisions: defaults.avoid_collisions,
            collision_padding: defaults.collision_padding,
            sticky: defaults.sticky,
            hide_when_detached: defaults.hide_when_detached,
            close_on_click_outside: true,
            close_on_escape: true,
            style_override: None,
        }
    }

    /// Sets the side of the trigger the card opens on.
    pub fn side(mut self, side: HoverCardSide) -> Self {
        self.side = side;
        self
    }

    /// Sets the alignment along the trigger edge.
    pub fn align(mut self, align: HoverCardAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets the gap between the trigger and the surface (`sideOffset`).
    ///
    /// shadcn-svelte defaults to 4.
    pub fn side_offset(mut self, offset: f32) -> Self {
        self.side_offset = offset;
        self
    }

    /// Sets the offset along the trigger edge (`alignOffset`).
    pub fn align_offset(mut self, offset: f32) -> Self {
        self.align_offset = offset;
        self
    }

    /// Sets the hover delay before the card opens (`openDelay`).
    ///
    /// bits-ui defaults to 700 ms.
    pub fn open_delay(mut self, delay: Duration) -> Self {
        self.open_delay = delay;
        self
    }

    /// Sets the delay before the card closes once the cursor left both
    /// the trigger and the content (`closeDelay`).
    ///
    /// bits-ui defaults to 300 ms.
    pub fn close_delay(mut self, delay: Duration) -> Self {
        self.close_delay = delay;
        self
    }

    /// Overrides the surface width in px (`w-64` — 256 px — by default;
    /// some style packs use `w-72`).
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(0.0));
        self
    }

    /// Sets the duration of the open/close animation (`duration-100`).
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Enables or disables the open/close animation.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Prevents the card from opening while keeping the trigger active
    /// (`disabled`).
    ///
    /// An already open card closes when it becomes disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Controls the open state explicitly instead of following hover.
    /// Combine with [`Self::on_open_change`] to observe open and dismiss
    /// requests.
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Controls the open state when `Some`, follows hover when `None`.
    pub fn open_maybe(mut self, open: Option<bool>) -> Self {
        self.open = open;
        self
    }

    /// Opens the card on first mount when uncontrolled (initial
    /// `bind:open` value).
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    /// Notifies about open-state change requests (`onOpenChange`): hover
    /// open/close after the delays, outside clicks, and <kbd>Esc</kbd>.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Flips and shifts the surface to keep it inside the window
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

    /// Hides the surface when the trigger is scrolled outside the window
    /// (`hideWhenDetached`).
    pub fn hide_when_detached(mut self, hide: bool) -> Self {
        self.hide_when_detached = hide;
        self
    }

    /// Keeps the card open on outside clicks
    /// (`interactOutsideBehavior: "ignore"`).
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Keeps the card open on <kbd>Esc</kbd>
    /// (`escapeKeydownBehavior: "ignore"`).
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Patches the resolved [`HoverCardStyle`] (colors, ring, radius,
    /// shadow) after theme resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(HoverCardStyle) -> HoverCardStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> From<HoverCard<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(hover_card: HoverCard<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(hover_card.theme);
        let mut resolved = style::resolve_style(hover_card.theme);

        if let Some(style_override) = hover_card.style_override.as_ref() {
            resolved = style_override(resolved);
        }

        let inner: Element<'a, Message> = match hover_card.content {
            HoverCardContent::Label(label) => text(label)
                .size(recipe.typography.size_px)
                .line_height(LineHeight::Absolute(Pixels(
                    recipe.typography.line_height_px,
                )))
                .font(iced_font(hover_card.theme.font_pack().sans))
                .into(),
            HoverCardContent::Element(element) => element,
        };

        let content = container(inner)
            .padding(Padding::new(recipe.pad_px))
            .width(Length::Fixed(hover_card.width.unwrap_or(recipe.width_px)));

        let config = FloatingConfig::default()
            .side(hover_card.side.to_floating())
            .align(hover_card.align.to_floating())
            .side_offset(hover_card.side_offset)
            .align_offset(hover_card.align_offset)
            .avoid_collisions(hover_card.avoid_collisions)
            .collision_padding(hover_card.collision_padding)
            .sticky(hover_card.sticky)
            .hide_when_detached(hover_card.hide_when_detached);

        Element::new(render::HoverCardWidget {
            trigger: hover_card.trigger,
            content: content.into(),
            config,
            open_delay: hover_card.open_delay,
            close_delay: hover_card.close_delay,
            duration: hover_card.duration,
            animated: hover_card.animated,
            disabled: hover_card.disabled,
            open_override: hover_card.open,
            default_open: hover_card.default_open,
            on_open_change: hover_card.on_open_change,
            close_on_click_outside: hover_card.close_on_click_outside,
            close_on_escape: hover_card.close_on_escape,
            style: resolved,
        })
    }
}
