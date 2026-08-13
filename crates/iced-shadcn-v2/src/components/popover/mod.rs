//! Builder-first popover component.
//!
//! Port of the shadcn-svelte popover (`Popover.Root` / `Trigger` /
//! `Content` + `Header` / `Title` / `Description`) as a single iced
//! builder: the trigger element is wrapped by a custom widget that toggles
//! a floating, interactive surface on click. Clicks outside the surface
//! and <kbd>Esc</kbd> dismiss it. The public API lives in this module;
//! positioning math is shared through [`shadcn_common::floating`], while
//! widget/overlay internals live in focused private submodules.
//!
//! **Style packs:** unlike Form (identical `form.json` across packs), Popover
//! ships distinct `.cn-popover-*` recipes per style — radius, padding, ring
//! alpha, shadow, title size — via [`shadcn_common::popover_recipe`] and
//! `theme.style_id()`.
//!
//! Composed parts still follow the same [`Theme`]: trigger →
//! [`crate::Button`], with-form content → [`crate::Input`] / [`crate::Label`],
//! header slots → title/description recipes. Picking Rhea on the theme
//! therefore paints Rhea Popover chrome *and* Rhea Button / Input / Label
//! recipes — the same composite rule as Form, plus Popover’s own pack deltas.

mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use style::PopoverStyle;

/// Returns the style-pack radius used by a popover surface.
pub(crate) fn surface_radius(theme: &Theme) -> f32 {
    style::surface_radius(theme)
}
pub use types::{PopoverAlign, PopoverSide};

use std::fmt;

use shadcn_common::{
    FloatingConfig, FloatingPadding, FloatingSticky, POPOVER_ANIMATION_MS, TypeRecipe,
};

use crate::iced_compat::widget::text::{Fragment, IntoFragment, LineHeight};
use crate::iced_compat::widget::{column, container, text};
use crate::iced_compat::{Element, Length, Padding, Pixels, time::Duration};

use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme;

/// Default `sideOffset` of the shadcn-svelte popover content.
const DEFAULT_SIDE_OFFSET: f32 = 4.0;

/// Builder-first popover styled directly with iced types.
///
/// Wraps a trigger element and toggles a floating surface on click,
/// matching shadcn-svelte defaults: opens on [`PopoverSide::Bottom`] with
/// `sideOffset = 4`, paints the `bg-popover` / `text-popover-foreground`
/// pair with a `ring-1 ring-foreground/N` hairline and a drop shadow, is
/// `w-72` wide with `p-4` padding, flips and shifts to stay inside the
/// window, closes on outside clicks and <kbd>Esc</kbd>, and animates with
/// the web `fade-in-0 zoom-in-95 slide-in-from-*-2` entrance.
///
/// Content stays fully interactive — forms, buttons, and inputs inside the
/// surface receive events like any other widget.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{
///     Button, ButtonVariant, Popover, PopoverDescription, PopoverHeader, PopoverTitle, Theme,
/// };
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Pressed,
/// }
///
/// fn view(theme: &Theme) -> Element<'_, Message> {
///     Popover::new(
///         Button::text("Open Popover", theme)
///             .variant(ButtonVariant::Outline)
///             .on_press(Message::Pressed),
///         PopoverHeader::new(theme)
///             .title(PopoverTitle::text("Dimensions", theme))
///             .description(PopoverDescription::text(
///                 "Set the dimensions for the layer.",
///                 theme,
///             )),
///         theme,
///     )
///     .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Popover<'a, Message> {
    trigger: Element<'a, Message>,
    content: PopoverContent<'a, Message>,
    theme: &'a Theme,
    side: PopoverSide,
    align: PopoverAlign,
    side_offset: f32,
    align_offset: f32,
    width: Option<f32>,
    /// Overrides the style-pack content padding (`p-0` for composed views).
    content_padding: Option<f32>,
    /// Overrides the surface radius in px.
    radius: Option<f32>,
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
    style_override: Option<Box<dyn Fn(PopoverStyle) -> PopoverStyle + 'a>>,
}

enum PopoverContent<'a, Message> {
    Label(Fragment<'a>),
    Element(Element<'a, Message>),
}

impl<Message> fmt::Debug for Popover<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match &self.content {
            PopoverContent::Label(_) => "label",
            PopoverContent::Element(_) => "element",
        };

        formatter
            .debug_struct("Popover")
            .field("content", &content)
            .field("theme", &self.theme)
            .field("side", &self.side)
            .field("align", &self.align)
            .field("side_offset", &self.side_offset)
            .field("align_offset", &self.align_offset)
            .field("width", &self.width)
            .field("content_padding", &self.content_padding)
            .field("radius", &self.radius)
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

impl<'a, Message> Popover<'a, Message> {
    /// Creates a popover with arbitrary content over `trigger`.
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
            PopoverContent::Element(content.into()),
            theme,
        )
    }

    /// Creates a text popover over `trigger`, typeset with the body
    /// typography of the active style pack (`text-sm` / `text-xs`).
    pub fn text(
        trigger: impl Into<Element<'a, Message>>,
        label: impl IntoFragment<'a>,
        theme: &'a Theme,
    ) -> Self {
        Self::from_content(
            trigger.into(),
            PopoverContent::Label(label.into_fragment()),
            theme,
        )
    }

    fn from_content(
        trigger: Element<'a, Message>,
        content: PopoverContent<'a, Message>,
        theme: &'a Theme,
    ) -> Self {
        let defaults = FloatingConfig::default();

        Self {
            trigger,
            content,
            theme,
            side: PopoverSide::default(),
            align: PopoverAlign::default(),
            side_offset: DEFAULT_SIDE_OFFSET,
            align_offset: defaults.align_offset,
            width: None,
            content_padding: None,
            radius: None,
            duration: Duration::from_millis(POPOVER_ANIMATION_MS),
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

    /// Sets the side of the trigger the popover opens on.
    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    /// Sets the alignment along the trigger edge.
    pub fn align(mut self, align: PopoverAlign) -> Self {
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

    /// Overrides the surface width in px (`w-72` — 288 px — by default).
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(0.0));
        self
    }

    /// Overrides the content padding in px (`p-0` for composed content).
    ///
    /// `None` (the default) keeps the style-pack recipe padding. Set this to
    /// zero when the content owns its own surface geometry, as the emoji
    /// picker popover does.
    pub fn content_padding(mut self, padding: f32) -> Self {
        self.content_padding = Some(padding.max(0.0));
        self
    }

    /// Overrides the style-pack surface radius in px.
    ///
    /// `None` (the default) keeps the active style-pack recipe radius.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius.max(0.0));
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

    /// Prevents the popover from opening while keeping the trigger active.
    ///
    /// An already open popover closes when it becomes disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Controls the open state explicitly instead of following trigger
    /// clicks. Combine with [`Self::on_open_change`] to observe open and
    /// dismiss requests.
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Controls the open state when `Some`, follows trigger clicks when
    /// `None`.
    pub fn open_maybe(mut self, open: Option<bool>) -> Self {
        self.open = open;
        self
    }

    /// Opens the popover on first mount when uncontrolled (`defaultOpen`).
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    /// Notifies about open-state change requests (`onOpenChange`): trigger
    /// clicks, outside clicks, and <kbd>Esc</kbd>.
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

    /// Keeps the popover open on outside clicks
    /// (`interactOutsideBehavior: "ignore"`).
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Keeps the popover open on <kbd>Esc</kbd>
    /// (`escapeKeydownBehavior: "ignore"`).
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Patches the resolved [`PopoverStyle`] (colors, ring, radius,
    /// shadow) after theme resolution.
    pub fn style_override(
        mut self,
        style_override: impl Fn(PopoverStyle) -> PopoverStyle + 'a,
    ) -> Self {
        self.style_override = Some(Box::new(style_override));
        self
    }
}

impl<'a, Message> From<Popover<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(popover: Popover<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(popover.theme);
        let mut resolved = style::resolve_style(popover.theme);

        if let Some(style_override) = popover.style_override.as_ref() {
            resolved = style_override(resolved);
        }

        let inner: Element<'a, Message> = match popover.content {
            PopoverContent::Label(label) => text(label)
                .size(recipe.typography.size_px)
                .line_height(LineHeight::Absolute(Pixels(
                    recipe.typography.line_height_px,
                )))
                .font(iced_font(popover.theme.font_pack().sans))
                .into(),
            PopoverContent::Element(element) => element,
        };

        if let Some(radius) = popover.radius {
            resolved.radius = radius;
        }

        let content = container(inner)
            .padding(Padding::new(
                popover.content_padding.unwrap_or(recipe.pad_px),
            ))
            .width(Length::Fixed(popover.width.unwrap_or(recipe.width_px)));

        let config = FloatingConfig::default()
            .side(popover.side.to_floating())
            .align(popover.align.to_floating())
            .side_offset(popover.side_offset)
            .align_offset(popover.align_offset)
            .avoid_collisions(popover.avoid_collisions)
            .collision_padding(popover.collision_padding)
            .sticky(popover.sticky)
            .hide_when_detached(popover.hide_when_detached);

        Element::new(render::PopoverWidget {
            trigger: popover.trigger,
            content: content.into(),
            config,
            duration: popover.duration,
            animated: popover.animated,
            disabled: popover.disabled,
            open_override: popover.open,
            default_open: popover.default_open,
            on_open_change: popover.on_open_change,
            close_on_click_outside: popover.close_on_click_outside,
            close_on_escape: popover.close_on_escape,
            style: resolved,
        })
    }
}

/// Styled popover header: a tight column for title and description
/// (`.cn-popover-header`).
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct PopoverHeader<'a, Message> {
    theme: &'a Theme,
    children: Vec<Element<'a, Message>>,
    spacing: Option<f32>,
}

impl<Message> fmt::Debug for PopoverHeader<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PopoverHeader")
            .field("theme", &self.theme)
            .field("children", &self.children.len())
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<'a, Message> PopoverHeader<'a, Message> {
    /// Creates an empty header.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            children: Vec::new(),
            spacing: None,
        }
    }

    /// Appends the title slot.
    pub fn title(self, title: PopoverTitle<'a, Message>) -> Self
    where
        Message: 'a,
    {
        self.push(title)
    }

    /// Appends the description slot.
    pub fn description(self, description: PopoverDescription<'a, Message>) -> Self
    where
        Message: 'a,
    {
        self.push(description)
    }

    /// Appends arbitrary header content.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Overrides the gap between header rows (`gap-1` by default).
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing.max(0.0));
        self
    }
}

impl<'a, Message> From<PopoverHeader<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(header: PopoverHeader<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(header.theme);

        column(header.children)
            .spacing(header.spacing.unwrap_or(recipe.header_gap_px))
            .width(Length::Fill)
            .into()
    }
}

/// Styled popover title (`.cn-popover-title`): heading font, style-pack
/// weight and size, inheriting the popover foreground color.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct PopoverTitle<'a, Message> {
    content: PopoverContent<'a, Message>,
    theme: &'a Theme,
}

impl<Message> fmt::Debug for PopoverTitle<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PopoverTitle")
            .field("theme", &self.theme)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> PopoverTitle<'a, Message> {
    /// Creates a title from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: PopoverContent::Element(content.into()),
            theme,
        }
    }

    /// Creates a style-pack-aware text title.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: PopoverContent::Label(label.into_fragment()),
            theme,
        }
    }
}

impl<'a, Message> From<PopoverTitle<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(title: PopoverTitle<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(title.theme);

        typeset(title.content, recipe.title, title.theme, true, None)
    }
}

/// Styled popover description (`.cn-popover-description`):
/// `text-muted-foreground` body copy.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct PopoverDescription<'a, Message> {
    content: PopoverContent<'a, Message>,
    theme: &'a Theme,
}

impl<Message> fmt::Debug for PopoverDescription<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PopoverDescription")
            .field("theme", &self.theme)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> PopoverDescription<'a, Message> {
    /// Creates a description from arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: &'a Theme) -> Self {
        Self {
            content: PopoverContent::Element(content.into()),
            theme,
        }
    }

    /// Creates a style-pack-aware text description.
    pub fn text(label: impl IntoFragment<'a>, theme: &'a Theme) -> Self {
        Self {
            content: PopoverContent::Label(label.into_fragment()),
            theme,
        }
    }
}

impl<'a, Message> From<PopoverDescription<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(description: PopoverDescription<'a, Message>) -> Element<'a, Message> {
        let recipe = style::recipe(description.theme);
        let muted = description.theme.palette.muted_foreground;

        typeset(
            description.content,
            recipe.description,
            description.theme,
            false,
            Some(muted),
        )
    }
}

/// Typesets a text slot with a [`TypeRecipe`]; element content passes
/// through untouched.
fn typeset<'a, Message: 'a>(
    content: PopoverContent<'a, Message>,
    recipe: TypeRecipe,
    theme: &Theme,
    heading: bool,
    color: Option<crate::iced_compat::Color>,
) -> Element<'a, Message> {
    match content {
        PopoverContent::Label(label) => {
            let pack = theme.font_pack();
            let mut font = iced_font(if heading { pack.heading } else { pack.sans });
            font.weight = iced_font_weight(recipe.weight);

            let label = if recipe.uppercase {
                label.into_owned().to_uppercase().into()
            } else {
                label
            };

            let mut widget = text(label)
                .size(recipe.size_px)
                .line_height(LineHeight::Absolute(Pixels(recipe.line_height_px)))
                .font(font);

            if let Some(color) = color {
                widget = widget.color(color);
            }

            widget.into()
        }
        PopoverContent::Element(element) => element,
    }
}
