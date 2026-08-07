//! Collapsible component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! The web component is a headless bits-ui primitive: a `Root` that owns the
//! open state, a `Trigger` button that flips it, and a `Content` panel that is
//! revealed with a height (or width) transition. The port keeps that
//! composition and the controlled contract — the application owns the boolean
//! and receives the next value through [`Collapsible::on_open_change`], exactly
//! like `bind:open` on the web.
//!
//! Beyond the web props (`open`, `onOpenChange`, `disabled`, `forceMount`) the
//! builder exposes what the web version leaves to utility classes: the layout
//! axis and slot alignment, spacing and padding, a themed surface, animation
//! timing and easing, and a chevron indicator that rotates with the panel — the
//! typed counterpart of `group-data-[state=open]:rotate-90`.
//!
//! Keyboard activation follows iced's own `button`, which does not take focus,
//! so `Space` / `Enter` on the trigger is not available the way it is in the
//! browser. Drive the state from application shortcuts instead.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Collapsible, CollapsibleContent, CollapsibleTrigger, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Toggled(bool),
//! }
//!
//! fn view(theme: &Theme, open: bool) -> Element<'_, Message> {
//!     Collapsible::new(theme)
//!         .open(open)
//!         .trigger(CollapsibleTrigger::text(
//!             "Can I use this in my project?",
//!             theme,
//!         ))
//!         .content(
//!             CollapsibleContent::new(theme).push(iced::widget::text(
//!                 "Yes. Free to use for personal and commercial projects.",
//!             )),
//!         )
//!         .on_open_change(Message::Toggled)
//!         .into()
//! }
//! ```

mod error;
mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use error::CollapsibleBuildError;
pub use types::{
    CollapsibleAlignment, CollapsibleEasing, CollapsibleIndicator, CollapsibleIndicatorPlacement,
    CollapsibleOrientation, CollapsibleState,
};

use std::fmt;
use std::time::Duration;

use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::widget::{button as button_widget, column, container, row};
use crate::iced_compat::{Element, Length};

use shadcn_common::AccentColor;
use twill_core::prelude::Padding;
use twill_core::prelude::theme::SemanticColor;

use crate::components::button::{ButtonRadius, ButtonSize, ButtonVariant};
use crate::theme::Theme;

use render::Animation;
use style::Surface;

/// Duration of one reveal transition, matching the shadcn accordion keyframes.
const DEFAULT_TRANSITION: Duration = Duration::from_millis(200);

/// Builder-first collapsible root: the state owner and slot container.
///
/// ```rust
/// use iced_shadcn_v2::{Collapsible, CollapsibleOrientation, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Toggled(bool),
/// }
///
/// let theme = Theme::light();
/// let root = Collapsible::<Message>::new(&theme)
///     .open(true)
///     .orientation(CollapsibleOrientation::Horizontal)
///     .on_open_change(Message::Toggled);
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Collapsible<'a, Message> {
    theme: &'a Theme,
    open: bool,
    disabled: bool,
    orientation: CollapsibleOrientation,
    align: CollapsibleAlignment,
    spacing: Option<f32>,
    width: Length,
    height: Length,
    padding: Option<crate::iced_compat::Padding>,
    surface: Surface,
    slots: Vec<CollapsibleSlot<'a, Message>>,
    animated: bool,
    duration: Duration,
    easing: CollapsibleEasing,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

enum CollapsibleSlot<'a, Message> {
    Element(Element<'a, Message>),
    Trigger(Box<CollapsibleTrigger<'a, Message>>),
    Content(Box<CollapsibleContent<'a, Message>>),
}

impl<Message> fmt::Debug for Collapsible<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Collapsible")
            .field("theme", &self.theme)
            .field("open", &self.open)
            .field("disabled", &self.disabled)
            .field("orientation", &self.orientation)
            .field("align", &self.align)
            .field("spacing", &self.spacing)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("surface", &self.surface)
            .field("slots", &self.slots.len())
            .field("animated", &self.animated)
            .field("duration", &self.duration)
            .field("easing", &self.easing)
            .field("on_open_change", &self.on_open_change.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> Collapsible<'a, Message> {
    /// Creates a closed collapsible using the active theme.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            open: false,
            disabled: false,
            orientation: CollapsibleOrientation::default(),
            align: CollapsibleAlignment::default(),
            spacing: None,
            width: Length::Fill,
            height: Length::Shrink,
            padding: None,
            surface: Surface::NONE,
            slots: Vec::new(),
            animated: true,
            duration: DEFAULT_TRANSITION,
            easing: CollapsibleEasing::default(),
            on_open_change: None,
            style_override: None,
        }
    }

    /// Sets the controlled open state (`bind:open` on the web).
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Sets the controlled open state from a [`CollapsibleState`].
    pub fn state(self, state: CollapsibleState) -> Self {
        self.open(state.is_open())
    }

    /// Whether the panel is currently expanded.
    pub const fn is_open(&self) -> bool {
        self.open
    }

    /// Suppresses interaction on every trigger and dims them.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the axis the slots stack on and the content reveals along.
    pub fn orientation(mut self, orientation: CollapsibleOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the cross-axis alignment of the slots.
    pub fn align(mut self, align: CollapsibleAlignment) -> Self {
        self.align = align;
        self
    }

    /// Sets the gap between slots in pixels.
    ///
    /// Non-finite and negative values are normalized to `0.0`. The default is
    /// the `gap-2` (8 px) of the shadcn examples.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(geometry::normalize_px(spacing));
        self
    }

    /// Sets the root width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the root height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the root padding.
    ///
    /// [`twill_core::prelude::PaddingValue::Var`] cannot be resolved by iced
    /// and is rejected with
    /// [`CollapsibleBuildError::UnsupportedPaddingVariable`]. The same applies
    /// to [`twill_core::prelude::Spacing::Auto`], which has no fixed-size iced
    /// representation.
    ///
    /// # Errors
    ///
    /// Returns [`CollapsibleBuildError`] when any padding side contains a
    /// custom variable or `auto` value. The builder is consumed either way;
    /// rebuild it with a supported padding to recover.
    pub fn padding(mut self, padding: Padding) -> Result<Self, CollapsibleBuildError> {
        self.padding = Some(geometry::resolve_padding(padding)?);
        Ok(self)
    }

    /// Paints the root on a semantic surface and pairs the text color with it.
    pub fn background(mut self, background: SemanticColor) -> Self {
        self.surface.background = Some(background);
        self
    }

    /// Draws a one-pixel `border` around the root.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.surface.bordered = bordered;
        self
    }

    /// Sets the root corner radius in pixels.
    ///
    /// Non-finite and negative values are normalized to `0.0`.
    pub fn radius(mut self, radius: f32) -> Self {
        self.surface.radius = Some(geometry::normalize_px(radius));
        self
    }

    /// Appends a trigger slot.
    pub fn trigger(mut self, trigger: CollapsibleTrigger<'a, Message>) -> Self {
        self.slots.push(CollapsibleSlot::Trigger(Box::new(trigger)));
        self
    }

    /// Appends an animated content slot.
    pub fn content(mut self, content: CollapsibleContent<'a, Message>) -> Self {
        self.slots.push(CollapsibleSlot::Content(Box::new(content)));
        self
    }

    /// Appends arbitrary content that is always visible.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.slots.push(CollapsibleSlot::Element(child.into()));
        self
    }

    /// Appends every element from an iterator as always-visible content.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Enables or disables the reveal transition.
    ///
    /// A non-animated collapsible snaps to its state on the next frame.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the reveal duration (clamped to at least 1 ms).
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration.max(Duration::from_millis(1));
        self
    }

    /// Sets the reveal duration in milliseconds.
    pub fn duration_ms(self, duration_ms: u32) -> Self {
        self.duration(Duration::from_millis(u64::from(duration_ms)))
    }

    /// Sets the reveal timing curve.
    pub fn easing(mut self, easing: CollapsibleEasing) -> Self {
        self.easing = easing;
        self
    }

    /// Sets the callback invoked with the next state when a trigger is pressed.
    ///
    /// The collapsible stays controlled: it keeps painting [`Self::open`] until
    /// the application stores the new value.
    pub fn on_open_change<F>(mut self, on_open_change: F) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Sets or clears the open-change callback.
    ///
    /// A collapsible without a callback is inert but keeps its normal colors,
    /// which is how read-only previews are rendered.
    pub fn on_open_change_maybe<F>(mut self, on_open_change: Option<F>) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        self.on_open_change = on_open_change.map(|callback| Box::new(callback) as _);
        self
    }

    /// Alias for [`Self::on_open_change`] using the terminology of the other
    /// two-state controls in this crate.
    pub fn on_toggle<F>(self, on_toggle: F) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        self.on_open_change(on_toggle)
    }

    /// Sets a message emitted on every trigger press, ignoring the next state.
    pub fn on_press(self, message: Message) -> Self
    where
        Message: Clone + 'a,
    {
        self.on_open_change(move |_| message.clone())
    }

    /// Applies an iced container-style override after semantic resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }

    /// Builds the collapsible as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let Collapsible {
            theme,
            open,
            disabled,
            orientation,
            align,
            spacing,
            width,
            height,
            padding,
            surface,
            slots,
            animated,
            duration,
            easing,
            on_open_change,
            style_override,
        } = self;

        let animation = Animation {
            animated,
            duration,
            easing,
        };
        // One message serves every trigger, so the next state is resolved once
        // instead of per slot.
        let toggle = on_open_change
            .as_ref()
            .filter(|_| !disabled)
            .map(|callback| callback(!open));

        let children = slots
            .into_iter()
            .map(|slot| match slot {
                CollapsibleSlot::Element(element) => element,
                CollapsibleSlot::Trigger(trigger) => {
                    render::build_trigger(*trigger, open, disabled, toggle.clone(), animation)
                }
                CollapsibleSlot::Content(content) => {
                    render::build_content(*content, open, orientation, animation)
                }
            })
            .collect::<Vec<_>>();

        let spacing = spacing.unwrap_or(geometry::DEFAULT_SPACING);
        let body: Element<'a, Message> = match orientation {
            CollapsibleOrientation::Vertical => column(children)
                .spacing(spacing)
                .width(Length::Fill)
                .align_x(render::align_x(align))
                .into(),
            CollapsibleOrientation::Horizontal => row(children)
                .spacing(spacing)
                .align_y(render::align_y(align))
                .into(),
        };

        let mut resolved = style::resolve_surface(theme, surface);
        if let Some(style_override) = style_override.as_ref() {
            resolved = style_override(resolved);
        }

        container(body)
            .width(width)
            .height(height)
            .padding(padding.unwrap_or_default())
            .style(move |_iced_theme| resolved)
            .into()
    }
}

impl<'a, Message> From<Collapsible<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(collapsible: Collapsible<'a, Message>) -> Self {
        collapsible.into_element()
    }
}

/// Builds a collapsible as an iced [`Element`](iced_core::Element).
///
/// ```rust
/// use iced_shadcn_v2::{Collapsible, Theme, collapsible};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Toggled(bool),
/// }
///
/// let theme = Theme::light();
/// let element = collapsible(Collapsible::<Message>::new(&theme));
/// ```
pub fn collapsible<'a, Message>(collapsible: Collapsible<'a, Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    collapsible.into_element()
}

/// The button that flips the open state of a [`Collapsible`].
///
/// Visual treatment is delegated to [`crate::Button`], so a trigger accepts the
/// same variants, sizes, radii, and accent colors. The chevron indicator is
/// opt-in and rotates with the panel, mirroring
/// `group-data-[state=open]:rotate-90` of the shadcn file-tree example.
///
/// ```rust
/// use iced_shadcn_v2::{ButtonVariant, CollapsibleIndicator, CollapsibleTrigger, Theme};
///
/// # #[derive(Debug, Clone)]
/// # enum Message {}
/// let theme = Theme::light();
/// let trigger = CollapsibleTrigger::<Message>::text("src", &theme)
///     .variant(ButtonVariant::Ghost)
///     .indicator(CollapsibleIndicator::Chevron)
///     .full_width(true);
/// ```
#[must_use = "builders do nothing unless added to a Collapsible"]
pub struct CollapsibleTrigger<'a, Message> {
    theme: &'a Theme,
    content: TriggerContent<'a, Message>,
    variant: ButtonVariant,
    size: ButtonSize,
    radius: Option<ButtonRadius>,
    /// `None` = theme primary; `Some` = accent overlay from `shadcn-common`.
    color: Option<AccentColor>,
    width: Length,
    full_width: bool,
    disabled: bool,
    indicator: Option<CollapsibleIndicator>,
    indicator_placement: CollapsibleIndicatorPlacement,
    gap: Option<f32>,
    height: Option<Length>,
    padding: Option<crate::iced_compat::Padding>,
    on_press: Option<Message>,
    style_override: Option<
        Box<dyn Fn(button_widget::Style, button_widget::Status) -> button_widget::Style + 'a>,
    >,
}

enum TriggerContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
    Icon(Element<'a, Message>),
    Indicator,
}

impl<Message> fmt::Debug for CollapsibleTrigger<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            TriggerContent::Label(_) => "label",
            TriggerContent::Element(_) => "element",
            TriggerContent::Icon(_) => "icon",
            TriggerContent::Indicator => "indicator",
        };

        formatter
            .debug_struct("CollapsibleTrigger")
            .field("theme", &self.theme)
            .field("content", &content)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("color", &self.color)
            .field("width", &self.width)
            .field("full_width", &self.full_width)
            .field("disabled", &self.disabled)
            .field("indicator", &self.indicator)
            .field("indicator_placement", &self.indicator_placement)
            .field("gap", &self.gap)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("on_press", &self.on_press.is_some())
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> CollapsibleTrigger<'a, Message> {
    /// Creates a trigger from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self::from_content(TriggerContent::Element(content.into()), theme)
    }

    /// Creates a text trigger carrying the pack's button typography.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self::from_content(TriggerContent::Label(label.into_fragment()), theme)
    }

    /// Creates an icon-only trigger with a square footprint.
    pub fn icon(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            size: ButtonSize::Icon,
            ..Self::from_content(TriggerContent::Icon(content.into()), theme)
        }
    }

    /// Creates a square trigger whose only content is the rotating chevron.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{ButtonVariant, CollapsibleTrigger, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let trigger =
    ///     CollapsibleTrigger::<Message>::chevron(&theme).variant(ButtonVariant::Outline);
    /// ```
    pub fn chevron(theme: &'a Theme) -> Self {
        Self {
            size: ButtonSize::Icon,
            indicator: Some(CollapsibleIndicator::default()),
            ..Self::from_content(TriggerContent::Indicator, theme)
        }
    }

    fn from_content(content: TriggerContent<'a, Message>, theme: &'a Theme) -> Self {
        Self {
            theme,
            content,
            variant: ButtonVariant::Ghost,
            size: ButtonSize::default(),
            radius: None,
            color: None,
            width: Length::Shrink,
            full_width: false,
            disabled: false,
            indicator: None,
            indicator_placement: CollapsibleIndicatorPlacement::default(),
            gap: None,
            height: None,
            padding: None,
            on_press: None,
            style_override: None,
        }
    }

    /// Sets the visual treatment of the trigger button.
    ///
    /// The default is [`ButtonVariant::Ghost`], the treatment the shadcn
    /// examples reach for; the web primitive itself is unstyled.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the preset control size.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the trigger corner radius.
    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Applies an accent color overlay to the trigger's theme tokens.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets a custom trigger width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Makes the trigger fill the available width (`w-full`).
    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    /// Disables this trigger regardless of the root state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Paints a rotating chevron next to the label.
    pub fn indicator(mut self, indicator: CollapsibleIndicator) -> Self {
        self.indicator = Some(indicator);
        self
    }

    /// Sets or clears the rotating chevron.
    pub fn indicator_maybe(mut self, indicator: Option<CollapsibleIndicator>) -> Self {
        self.indicator = indicator;
        self
    }

    /// Sets the side of the label the chevron is painted on.
    pub fn indicator_placement(mut self, placement: CollapsibleIndicatorPlacement) -> Self {
        self.indicator_placement = placement;
        self
    }

    /// Sets the gap between the chevron and the label in pixels.
    ///
    /// Non-finite and negative values are normalized to `0.0`. The default is
    /// the `gap-*` of the active style pack.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(geometry::normalize_px(gap));
        self
    }

    /// Sets the trigger height independently from its style-pack size.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets all supported sides of the trigger button padding.
    ///
    /// # Errors
    ///
    /// Returns [`CollapsibleBuildError`] when any padding side contains a
    /// custom variable or `auto` value. The builder is consumed either way;
    /// rebuild the trigger with a supported padding to recover.
    pub fn padding(mut self, padding: Padding) -> Result<Self, CollapsibleBuildError> {
        self.padding = Some(geometry::resolve_padding(padding)?);
        Ok(self)
    }

    /// Overrides the message this trigger emits.
    ///
    /// Without it the trigger emits the root's [`Collapsible::on_open_change`]
    /// message for the next state.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets or clears the per-trigger message override.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
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
}

/// The panel a [`Collapsible`] reveals.
///
/// The child keeps its natural size and is cropped to the revealed band, which
/// is the iced equivalent of the `overflow-hidden` height transition on the web.
///
/// ```rust
/// use iced_shadcn_v2::{CollapsibleContent, Theme};
///
/// # #[derive(Debug, Clone)]
/// # enum Message {}
/// let theme = Theme::light();
/// let content = CollapsibleContent::<Message>::new(&theme)
///     .push(iced::widget::text("utils.ts"))
///     .push(iced::widget::text("stores.ts"))
///     .spacing(4.0);
/// ```
#[must_use = "builders do nothing unless added to a Collapsible"]
pub struct CollapsibleContent<'a, Message> {
    theme: &'a Theme,
    children: Vec<Element<'a, Message>>,
    spacing: Option<f32>,
    padding: Option<crate::iced_compat::Padding>,
    width: Length,
    height: Length,
    surface: Surface,
    force_mount: bool,
    style_override: Option<Box<dyn Fn(container::Style) -> container::Style + 'a>>,
}

impl<Message> fmt::Debug for CollapsibleContent<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CollapsibleContent")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .field("padding", &self.padding)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("surface", &self.surface)
            .field("force_mount", &self.force_mount)
            .field("style_override", &self.style_override.is_some())
            .finish()
    }
}

impl<'a, Message> CollapsibleContent<'a, Message> {
    /// Creates an empty content panel.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            spacing: None,
            padding: None,
            width: Length::Fill,
            height: Length::Shrink,
            surface: Surface::NONE,
            force_mount: false,
            style_override: None,
        }
    }

    /// Creates a content panel from an iterator of children.
    pub fn with_children(
        theme: &'a Theme,
        children: impl IntoIterator<Item = Element<'a, Message>>,
    ) -> Self {
        Self {
            children: children.into_iter().collect(),
            ..Self::new(theme)
        }
    }

    /// Appends a child to the panel.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends every element from an iterator to the panel.
    pub fn extend(self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the gap between panel children in pixels.
    ///
    /// Non-finite and negative values are normalized to `0.0`.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(geometry::normalize_px(spacing));
        self
    }

    /// Sets the panel padding, e.g. the `mt-1 ml-5` inset of a file tree.
    ///
    /// # Errors
    ///
    /// Returns [`CollapsibleBuildError`] when any padding side contains a
    /// custom variable or `auto` value. The builder is consumed either way;
    /// rebuild it with a supported padding to recover.
    pub fn padding(mut self, padding: Padding) -> Result<Self, CollapsibleBuildError> {
        self.padding = Some(geometry::resolve_padding(padding)?);
        Ok(self)
    }

    /// Sets the panel width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the natural panel height revealed when fully open.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Paints the panel on a semantic surface and pairs the text color with it.
    pub fn background(mut self, background: SemanticColor) -> Self {
        self.surface.background = Some(background);
        self
    }

    /// Draws a one-pixel `border` around the panel.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.surface.bordered = bordered;
        self
    }

    /// Sets the panel corner radius in pixels.
    ///
    /// Non-finite and negative values are normalized to `0.0`.
    pub fn radius(mut self, radius: f32) -> Self {
        self.surface.radius = Some(geometry::normalize_px(radius));
        self
    }

    /// Keeps the collapsed panel part of the interface, like `forceMount`.
    ///
    /// A collapsed panel is normally inert: it is not painted and receives no
    /// events, focus operations, or overlays. With `force_mount` it keeps taking
    /// part in all of them at zero revealed size, which is what a
    /// scroll-into-view or focus request on hidden content needs.
    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    /// Applies an iced container-style override after semantic resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(container::Style) -> container::Style + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}
