//! Carousel component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! The web component drives an embla-carousel instance; this port implements
//! the same behaviour natively: scroll-snapped slides with `start` / `center`
//! / `end` alignment, `trimSnaps` containment, per-slide basis fractions,
//! optional looping, pointer dragging with a settle animation, arrow-key
//! navigation, an autoplay plugin equivalent, and outline prev/next controls
//! hanging outside the viewport.
//!
//! The carousel is controlled: the application owns the selected snap index
//! and receives the next one from [`Carousel::on_select`], mirroring
//! `api.selectedScrollSnap()` + `api.scrollTo(...)` of the web API. The snap
//! count (for a "Slide x of y" readout) comes from [`Carousel::snap_count`].
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Carousel, CarouselItem, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     SlideSelected(usize),
//! }
//!
//! fn view<'a>(theme: &'a Theme, selected: usize) -> Element<'a, Message> {
//!     let mut carousel = Carousel::new(theme)
//!         .selected(selected)
//!         .on_select(Message::SlideSelected);
//!
//!     for slide in ["1", "2", "3"] {
//!         carousel = carousel.push(CarouselItem::new(iced::widget::text(slide)));
//!     }
//!
//!     carousel.into()
//! }
//! ```

mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

pub use types::CarouselOrientation;

/// Snap alignment options re-exported from [`shadcn_common`].
pub use shadcn_common::CarouselAlign;

use std::fmt;
use std::time::Duration;

use crate::iced_compat::widget::{column, container, row};
use crate::iced_compat::{Element, Length, alignment};

use shadcn_common::{
    AccentColor, CAROUSEL_ANIMATION_MS, CAROUSEL_CONTROL_OFFSET_PX, CAROUSEL_GAP_PX,
    carousel_can_scroll_next, carousel_can_scroll_prev, carousel_next_snap, carousel_previous_snap,
};

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::theme::Theme;

use geometry::Strip;
use render::{Autoplay, ChevronDirection, Track};

/// Default main-axis length of a vertical carousel (`h-[200px]` in the docs).
const DEFAULT_VERTICAL_LENGTH_PX: f32 = 200.0;

/// Default glyph size inside the prev/next controls (`size-4`).
const CONTROL_ICON_PX: f32 = 16.0;

/// Builder-first carousel styled from `shadcn-common` theme tokens.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Carousel, CarouselAlign, CarouselItem, Theme};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     SlideSelected(usize),
/// }
///
/// fn gallery<'a>(theme: &'a Theme, selected: usize) -> Element<'a, Message> {
///     Carousel::new(theme)
///         .align(CarouselAlign::Start)
///         .item_basis(1.0 / 3.0)
///         .selected(selected)
///         .on_select(Message::SlideSelected)
///         .push(CarouselItem::new(iced::widget::text("A")))
///         .push(CarouselItem::new(iced::widget::text("B")))
///         .into()
/// }
/// ```
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Carousel<'a, Message> {
    theme: &'a Theme,
    items: Vec<CarouselItem<'a, Message>>,
    orientation: CarouselOrientation,
    align: CarouselAlign,
    looped: bool,
    selected: usize,
    on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    item_basis: f32,
    gap: f32,
    width: Option<Length>,
    height: Option<Length>,
    animated: bool,
    duration: Duration,
    drag_enabled: bool,
    keyboard_enabled: bool,
    autoplay: Option<Duration>,
    autoplay_stop_on_interaction: bool,
    autoplay_pause_on_hover: bool,
    show_controls: bool,
    previous: CarouselPrevious<'a, Message>,
    next: CarouselNext<'a, Message>,
}

impl<Message> fmt::Debug for Carousel<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Carousel")
            .field("theme", &self.theme)
            .field("items", &self.items.len())
            .field("orientation", &self.orientation)
            .field("align", &self.align)
            .field("looped", &self.looped)
            .field("selected", &self.selected)
            .field("on_select", &self.on_select.is_some())
            .field("item_basis", &self.item_basis)
            .field("gap", &self.gap)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("animated", &self.animated)
            .field("duration", &self.duration)
            .field("drag_enabled", &self.drag_enabled)
            .field("keyboard_enabled", &self.keyboard_enabled)
            .field("autoplay", &self.autoplay)
            .field("show_controls", &self.show_controls)
            .finish()
    }
}

impl<'a, Message> Carousel<'a, Message> {
    /// Creates an empty horizontal carousel.
    ///
    /// `theme` is required because styling is derived from `shadcn-common`
    /// theme tokens instead of `iced::Theme`.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Carousel, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let carousel = Carousel::<Message>::new(&theme);
    /// ```
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            items: Vec::new(),
            orientation: CarouselOrientation::Horizontal,
            align: CarouselAlign::Start,
            looped: false,
            selected: 0,
            on_select: None,
            item_basis: 1.0,
            gap: CAROUSEL_GAP_PX,
            width: None,
            height: None,
            animated: true,
            duration: Duration::from_millis(CAROUSEL_ANIMATION_MS as u64),
            drag_enabled: true,
            keyboard_enabled: true,
            autoplay: None,
            autoplay_stop_on_interaction: true,
            autoplay_pause_on_hover: false,
            show_controls: true,
            previous: CarouselPrevious::new(),
            next: CarouselNext::new(),
        }
    }

    /// Appends a slide.
    pub fn push(mut self, item: CarouselItem<'a, Message>) -> Self {
        self.items.push(item);
        self
    }

    /// Appends every slide from an iterator.
    pub fn extend(mut self, items: impl IntoIterator<Item = CarouselItem<'a, Message>>) -> Self {
        self.items.extend(items);
        self
    }

    /// Sets the axis the carousel scrolls along (`orientation`).
    pub fn orientation(mut self, orientation: CarouselOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the snap alignment of slides inside the viewport (`opts.align`).
    pub fn align(mut self, align: CarouselAlign) -> Self {
        self.align = align;
        self
    }

    /// Makes the strip wrap around (`opts.loop`).
    pub fn looped(mut self, looped: bool) -> Self {
        self.looped = looped;
        self
    }

    /// Sets the controlled snap index (`api.selectedScrollSnap()`).
    ///
    /// Out-of-range indices are clamped to the last snap.
    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }

    /// Sets the callback invoked with the next snap index.
    ///
    /// The carousel stays controlled: buttons, drags, arrow keys, and
    /// autoplay all report through this callback, and the strip scrolls once
    /// the application stores the new index. Without it the carousel is
    /// static and its controls are disabled.
    pub fn on_select(mut self, on_select: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Sets the default fraction of the viewport each slide occupies
    /// (`basis-full` = `1.0`, `basis-1/3` ≈ `0.333`).
    ///
    /// The value is clamped into `0.05..=1.0`. Individual slides can override
    /// it with [`CarouselItem::basis`].
    pub fn item_basis(mut self, basis: f32) -> Self {
        self.item_basis = geometry::sanitize_basis(basis);
        self
    }

    /// Sets the spacing between slides in logical pixels (the web `ps-*` /
    /// `-ms-*` pair; `16.0` by default).
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = if gap.is_finite() { gap.max(0.0) } else { 0.0 };
        self
    }

    /// Sets the viewport width.
    ///
    /// For horizontal carousels this is the scroll axis (defaults to
    /// [`Length::Fill`]); for vertical ones it is the cross axis.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the viewport height.
    ///
    /// For horizontal carousels the height defaults to the tallest slide;
    /// vertical carousels scroll along it and default to a fixed
    /// `200` px viewport, matching the docs demo (`h-[200px]`).
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Enables or disables the settle animation.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the duration of the settle animation.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Enables or disables pointer dragging (`opts.watchDrag`).
    pub fn drag_enabled(mut self, drag_enabled: bool) -> Self {
        self.drag_enabled = drag_enabled;
        self
    }

    /// Enables or disables arrow-key navigation while hovered.
    pub fn keyboard(mut self, keyboard: bool) -> Self {
        self.keyboard_enabled = keyboard;
        self
    }

    /// Advances to the next snap every `delay` (the embla autoplay plugin;
    /// its stock delay is [`CAROUSEL_AUTOPLAY_DELAY_MS`]).
    pub fn autoplay(mut self, delay: Duration) -> Self {
        self.autoplay = Some(delay);
        self
    }

    /// Whether any user interaction stops autoplay for good
    /// (`stopOnInteraction`, `true` by default).
    pub fn autoplay_stop_on_interaction(mut self, stop: bool) -> Self {
        self.autoplay_stop_on_interaction = stop;
        self
    }

    /// Holds autoplay while the pointer is over the strip (the docs demo's
    /// `onmouseenter={plugin.stop}` / `onmouseleave={plugin.reset}` wiring).
    pub fn autoplay_pause_on_hover(mut self, pause: bool) -> Self {
        self.autoplay_pause_on_hover = pause;
        self
    }

    /// Shows or hides the prev/next controls.
    pub fn controls(mut self, show: bool) -> Self {
        self.show_controls = show;
        self
    }

    /// Hides the prev/next controls.
    pub fn hide_controls(self) -> Self {
        self.controls(false)
    }

    /// Replaces the previous-control configuration.
    pub fn previous(mut self, previous: CarouselPrevious<'a, Message>) -> Self {
        self.previous = previous;
        self
    }

    /// Replaces the next-control configuration.
    pub fn next(mut self, next: CarouselNext<'a, Message>) -> Self {
        self.next = next;
        self
    }

    /// Number of scroll snaps (`api.scrollSnapList().length`).
    ///
    /// With multiple slides per view and no looping, trailing slides share
    /// the final snap (embla `containScroll: "trimSnaps"`), so this can be
    /// smaller than the slide count.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{Carousel, CarouselItem, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let mut carousel = Carousel::<Message>::new(&theme).item_basis(1.0 / 3.0);
    ///
    /// for _ in 0..5 {
    ///     carousel = carousel.push(CarouselItem::new(iced::widget::text("x")));
    /// }
    ///
    /// assert_eq!(carousel.snap_count(), 3);
    /// ```
    pub fn snap_count(&self) -> usize {
        self.strip().snap_count()
    }

    /// Whether a "previous" step is available (`api.canScrollPrev()`).
    pub fn can_scroll_prev(&self) -> bool {
        let strip = self.strip();
        carousel_can_scroll_prev(
            self.clamped_selected(&strip),
            strip.snap_count(),
            self.looped,
        )
    }

    /// Whether a "next" step is available (`api.canScrollNext()`).
    pub fn can_scroll_next(&self) -> bool {
        let strip = self.strip();
        carousel_can_scroll_next(
            self.clamped_selected(&strip),
            strip.snap_count(),
            self.looped,
        )
    }

    /// Normalized strip geometry for the configured slides.
    fn strip(&self) -> Strip {
        let bases: Vec<f32> = self
            .items
            .iter()
            .map(|item| item.basis.unwrap_or(self.item_basis))
            .collect();

        Strip::new(&bases, self.align, self.looped)
    }

    /// Selected snap clamped into the snap list.
    fn clamped_selected(&self, strip: &Strip) -> usize {
        self.selected.min(strip.snap_count().saturating_sub(1))
    }

    /// Builds the composed root element.
    fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let Carousel {
            theme,
            items,
            orientation,
            align,
            looped,
            selected,
            on_select,
            item_basis,
            gap,
            width,
            height,
            animated,
            duration,
            drag_enabled,
            keyboard_enabled,
            autoplay,
            autoplay_stop_on_interaction,
            autoplay_pause_on_hover,
            show_controls,
            previous,
            next,
        } = self;

        let bases: Vec<f32> = items
            .iter()
            .map(|item| item.basis.unwrap_or(item_basis))
            .collect();
        let strip = Strip::new(&bases, align, looped);
        let snap_count = strip.snap_count();
        let selected = selected.min(snap_count.saturating_sub(1));

        let can_prev =
            on_select.is_some() && carousel_can_scroll_prev(selected, snap_count, looped);
        let can_next =
            on_select.is_some() && carousel_can_scroll_next(selected, snap_count, looped);
        let previous_message = on_select.as_ref().and_then(|on_select| {
            carousel_previous_snap(selected, snap_count, looped).map(on_select)
        });
        let next_message = on_select
            .as_ref()
            .and_then(|on_select| carousel_next_snap(selected, snap_count, looped).map(on_select));

        let (main, cross) = match orientation {
            CarouselOrientation::Horizontal => (
                width.unwrap_or(Length::Fill),
                height.unwrap_or(Length::Shrink),
            ),
            CarouselOrientation::Vertical => (
                height.unwrap_or(Length::Fixed(DEFAULT_VERTICAL_LENGTH_PX)),
                width.unwrap_or(Length::Fill),
            ),
        };

        let children: Vec<Element<'a, Message>> = items
            .into_iter()
            .map(|item| {
                let slot = container(item.content);
                match orientation {
                    CarouselOrientation::Horizontal => slot.width(Length::Fill),
                    CarouselOrientation::Vertical => slot.height(Length::Fill),
                }
                .into()
            })
            .collect();

        let track: Element<'a, Message> = Track {
            children,
            strip,
            selected,
            orientation,
            looped,
            gap,
            main,
            cross,
            animated,
            duration,
            drag_enabled,
            keyboard_enabled,
            autoplay: autoplay.map(|delay| Autoplay {
                delay,
                stop_on_interaction: autoplay_stop_on_interaction,
                pause_on_hover: autoplay_pause_on_hover,
            }),
            on_select,
        }
        .into();

        if !show_controls {
            return track;
        }

        let gutter = Length::Fixed(CAROUSEL_CONTROL_OFFSET_PX);

        match orientation {
            CarouselOrientation::Horizontal => {
                let prev = previous.into_button(
                    theme,
                    ChevronDirection::Left,
                    previous_message,
                    !can_prev,
                );
                let next =
                    next.into_button(theme, ChevronDirection::Right, next_message, !can_next);
                let mut root = row(vec![
                    container(prev)
                        .width(gutter)
                        .align_x(alignment::Horizontal::Left)
                        .into(),
                    track,
                    container(next)
                        .width(gutter)
                        .align_x(alignment::Horizontal::Right)
                        .into(),
                ])
                .align_y(alignment::Vertical::Center);

                if matches!(main, Length::Fill | Length::FillPortion(_)) {
                    root = root.width(Length::Fill);
                }

                root.into()
            }
            CarouselOrientation::Vertical => {
                let prev =
                    previous.into_button(theme, ChevronDirection::Up, previous_message, !can_prev);
                let next = next.into_button(theme, ChevronDirection::Down, next_message, !can_next);
                let mut root = column(vec![
                    container(prev)
                        .height(gutter)
                        .align_y(alignment::Vertical::Top)
                        .into(),
                    track,
                    container(next)
                        .height(gutter)
                        .align_y(alignment::Vertical::Bottom)
                        .into(),
                ])
                .align_x(alignment::Horizontal::Center);

                if matches!(cross, Length::Fill | Length::FillPortion(_)) {
                    root = root.width(Length::Fill);
                }

                root.into()
            }
        }
    }
}

impl<'a, Message> From<Carousel<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(carousel: Carousel<'a, Message>) -> Self {
        carousel.into_element()
    }
}

/// Creates an empty [`Carousel`], mirroring the other component helpers.
///
/// ```rust
/// use iced_shadcn_v2::{Theme, carousel};
///
/// # #[derive(Debug, Clone)]
/// # enum Message {}
/// let theme = Theme::light();
/// let widget = carousel::<Message>(&theme);
/// ```
pub fn carousel<Message>(theme: &Theme) -> Carousel<'_, Message> {
    Carousel::new(theme)
}

/// One slide of a [`Carousel`].
#[must_use = "carousel items do nothing unless pushed into a Carousel"]
pub struct CarouselItem<'a, Message> {
    content: Element<'a, Message>,
    basis: Option<f32>,
}

impl<Message> fmt::Debug for CarouselItem<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CarouselItem")
            .field("basis", &self.basis)
            .finish_non_exhaustive()
    }
}

impl<'a, Message> CarouselItem<'a, Message> {
    /// Wraps arbitrary content into a slide.
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            basis: None,
        }
    }

    /// Overrides the viewport fraction of this slide (`basis-1/2` ≈ `0.5`).
    ///
    /// The value is clamped into `0.05..=1.0`.
    pub fn basis(mut self, basis: f32) -> Self {
        self.basis = Some(geometry::sanitize_basis(basis));
        self
    }
}

/// Configuration of the "previous slide" control (`<Carousel.Previous/>`).
#[must_use = "control builders do nothing unless attached to a Carousel"]
pub struct CarouselPrevious<'a, Message> {
    inner: Control<'a, Message>,
}

/// Configuration of the "next slide" control (`<Carousel.Next/>`).
#[must_use = "control builders do nothing unless attached to a Carousel"]
pub struct CarouselNext<'a, Message> {
    inner: Control<'a, Message>,
}

/// Shared prev/next control settings.
struct Control<'a, Message> {
    variant: ButtonVariant,
    size: ButtonSize,
    color: Option<AccentColor>,
    icon: Option<Element<'a, Message>>,
    disabled: bool,
}

impl<Message> Default for Control<'_, Message> {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Outline,
            size: ButtonSize::IconSm,
            color: None,
            icon: None,
            disabled: false,
        }
    }
}

impl<Message> fmt::Debug for Control<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Control")
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("color", &self.color)
            .field("icon", &self.icon.is_some())
            .field("disabled", &self.disabled)
            .finish()
    }
}

impl<'a, Message> Control<'a, Message> {
    /// Builds the control button.
    fn into_button(
        self,
        theme: &'a Theme,
        direction: ChevronDirection,
        on_press: Option<Message>,
        disabled: bool,
    ) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let disabled = disabled || self.disabled;
        let glyph = self.icon.unwrap_or_else(|| {
            render::control_glyph(
                direction,
                style::control_icon_color(theme, self.variant, self.color, disabled),
                CONTROL_ICON_PX,
            )
        });

        let mut button = Button::icon(glyph, theme)
            .variant(self.variant)
            .size(self.size)
            .disabled(disabled)
            .on_press_maybe(on_press);

        if let Some(color) = self.color {
            button = button.color(color);
        }

        if let Some(radius) = style::control_radius_px(theme) {
            button = button.style_override(move |mut style, _status| {
                style.border.radius = radius.into();
                style
            });
        }

        button.into()
    }
}

macro_rules! control_builder {
    ($name:ident, $doc_slot:literal) => {
        impl<Message> fmt::Debug for $name<'_, Message> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_struct(stringify!($name))
                    .field("inner", &self.inner)
                    .finish()
            }
        }

        impl<Message> Default for $name<'_, Message> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<'a, Message> $name<'a, Message> {
            #[doc = concat!("Creates the default ", $doc_slot, " control (outline icon-sm button).")]
            pub fn new() -> Self {
                Self {
                    inner: Control::default(),
                }
            }

            /// Sets the visual treatment of the control button.
            pub fn variant(mut self, variant: ButtonVariant) -> Self {
                self.inner.variant = variant;
                self
            }

            /// Sets the preset control size.
            pub fn size(mut self, size: ButtonSize) -> Self {
                self.inner.size = size;
                self
            }

            /// Applies an accent color overlay to the control button.
            pub fn color(mut self, color: AccentColor) -> Self {
                self.inner.color = Some(color);
                self
            }

            /// Replaces the default chevron with arbitrary iced content.
            pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
                self.inner.icon = Some(icon.into());
                self
            }

            /// Force-disables the control regardless of scroll position.
            pub fn disabled(mut self, disabled: bool) -> Self {
                self.inner.disabled = disabled;
                self
            }

            /// Builds the control button.
            fn into_button(
                self,
                theme: &'a Theme,
                direction: ChevronDirection,
                on_press: Option<Message>,
                disabled: bool,
            ) -> Element<'a, Message>
            where
                Message: Clone + 'a,
            {
                self.inner.into_button(theme, direction, on_press, disabled)
            }
        }
    };
}

control_builder!(CarouselPrevious, "\"previous slide\"");
control_builder!(CarouselNext, "\"next slide\"");
