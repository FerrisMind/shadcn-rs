//! Pagination component ported from shadcn-svelte to iced-shadcn-v2.
//!
//! The root [`Pagination`] builder owns the same controlled state as the
//! bits-ui primitive behind shadcn-svelte: a total item `count`, a
//! `per_page` size, the current 1-based `page`, and a `sibling_count`
//! window around the active page. From that state it derives the visible
//! page range (with ellipsis gaps), boundary-aware previous/next controls,
//! and emits the next page through [`Pagination::on_page_change`]; the
//! application owns the page value and feeds it back on every update.
//!
//! Page links reuse [`crate::Button`] variants exactly like the web
//! component: the active page renders as `outline`, every other control as
//! `ghost`. For custom layouts the subcomponents are exported standalone —
//! [`PaginationLink`], [`PaginationPrevious`], [`PaginationNext`], and
//! [`PaginationEllipsis`] — together with the pure range math
//! ([`page_items`], [`total_pages`]).
//!
//! The state contract follows the upstream
//! [shadcn-svelte Pagination](https://github.com/huntabyte/shadcn-svelte/tree/next/docs/src/lib/registry/ui/pagination)
//! registry component.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{Pagination, Theme};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     PageChanged(usize),
//! }
//!
//! fn pager(theme: &Theme, page: usize) -> Element<'_, Message> {
//!     Pagination::new(theme)
//!         .count(95)
//!         .per_page(10)
//!         .page(page)
//!         .on_page_change(Message::PageChanged)
//!         .into()
//! }
//! ```

mod geometry;
mod render;
mod types;

#[cfg(test)]
mod tests;

pub use types::{DEFAULT_PER_PAGE, DEFAULT_SIBLING_COUNT, PaginationItem, page_items, total_pages};

use std::fmt;

use crate::iced_compat::widget::text::{Fragment, IntoFragment};
use crate::iced_compat::{Element, Length};

use shadcn_common::AccentColor;

use crate::components::button::{ButtonSize, ButtonVariant};
use crate::theme::Theme;

/// Crate-internal direction of a boundary control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NavDirection {
    Previous,
    Next,
}

type PageCallback<'a, Message> = Box<dyn Fn(usize) -> Message + 'a>;

/// Builder-first root for a controlled pagination bar.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct Pagination<'a, Message> {
    theme: &'a Theme,
    count: usize,
    per_page: usize,
    page: usize,
    sibling_count: usize,
    link_size: ButtonSize,
    controls_size: ButtonSize,
    active_variant: ButtonVariant,
    inactive_variant: ButtonVariant,
    color: Option<AccentColor>,
    spacing: Option<f32>,
    show_controls: bool,
    show_links: bool,
    show_labels: bool,
    previous_label: Fragment<'a>,
    next_label: Fragment<'a>,
    disabled: bool,
    width: Length,
    on_page_change: Option<PageCallback<'a, Message>>,
}

impl<Message> fmt::Debug for Pagination<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Pagination")
            .field("theme", &self.theme)
            .field("count", &self.count)
            .field("per_page", &self.per_page)
            .field("page", &self.page)
            .field("sibling_count", &self.sibling_count)
            .field("link_size", &self.link_size)
            .field("controls_size", &self.controls_size)
            .field("active_variant", &self.active_variant)
            .field("inactive_variant", &self.inactive_variant)
            .field("color", &self.color)
            .field("spacing", &self.spacing)
            .field("show_controls", &self.show_controls)
            .field("show_links", &self.show_links)
            .field("show_labels", &self.show_labels)
            .field("previous_label", &self.previous_label)
            .field("next_label", &self.next_label)
            .field("disabled", &self.disabled)
            .field("width", &self.width)
            .field("on_page_change", &self.on_page_change.is_some())
            .finish()
    }
}

impl<'a, Message> Pagination<'a, Message> {
    /// Creates a pagination bar with shadcn-svelte defaults: no items,
    /// [`DEFAULT_PER_PAGE`] items per page, page 1 active, and
    /// [`DEFAULT_SIBLING_COUNT`] siblings around the current page.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            count: 0,
            per_page: DEFAULT_PER_PAGE,
            page: 1,
            sibling_count: DEFAULT_SIBLING_COUNT,
            link_size: ButtonSize::Icon,
            controls_size: ButtonSize::Default,
            active_variant: ButtonVariant::Outline,
            inactive_variant: ButtonVariant::Ghost,
            color: None,
            spacing: None,
            show_controls: true,
            show_links: true,
            show_labels: true,
            previous_label: Fragment::from("Previous"),
            next_label: Fragment::from("Next"),
            disabled: false,
            width: Length::Shrink,
            on_page_change: None,
        }
    }

    /// Sets the total number of paginated items.
    pub fn count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }

    /// Sets how many items each page holds. Zero is treated as one.
    pub fn per_page(mut self, per_page: usize) -> Self {
        self.per_page = per_page.max(1);
        self
    }

    /// Sets the controlled current page (1-based). Zero is treated as one;
    /// values past the last page are clamped when the bar is built.
    pub fn page(mut self, page: usize) -> Self {
        self.page = page.max(1);
        self
    }

    /// Sets how many pages stay visible on each side of the current page.
    pub fn sibling_count(mut self, sibling_count: usize) -> Self {
        self.sibling_count = sibling_count;
        self
    }

    /// Sets the footprint of numbered page links (`icon` by default).
    pub fn link_size(mut self, size: ButtonSize) -> Self {
        self.link_size = size;
        self
    }

    /// Sets the footprint of the previous/next controls (`default` size by
    /// default).
    pub fn controls_size(mut self, size: ButtonSize) -> Self {
        self.controls_size = size;
        self
    }

    /// Sets the button variant of the active page link (`outline` by
    /// default).
    pub fn active_variant(mut self, variant: ButtonVariant) -> Self {
        self.active_variant = variant;
        self
    }

    /// Sets the button variant of inactive links and boundary controls
    /// (`ghost` by default).
    pub fn inactive_variant(mut self, variant: ButtonVariant) -> Self {
        self.inactive_variant = variant;
        self
    }

    /// Applies an accent color overlay to every control in the bar.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the gap between items in style-pack spacing units, so
    /// `spacing(1.0)` matches shadcn's `gap-1` default.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(geometry::normalize_spacing(spacing));
        self
    }

    /// Shows or hides the previous/next boundary controls.
    pub fn show_controls(mut self, show_controls: bool) -> Self {
        self.show_controls = show_controls;
        self
    }

    /// Shows or hides the numbered page links. Hiding them yields the
    /// compact previous/next layout of shadcn's `pagination-simple` demo.
    pub fn show_links(mut self, show_links: bool) -> Self {
        self.show_links = show_links;
        self
    }

    /// Shows or hides the "Previous"/"Next" text, mirroring the web
    /// component's small-screen icon-only collapse.
    pub fn show_labels(mut self, show_labels: bool) -> Self {
        self.show_labels = show_labels;
        self
    }

    /// Sets the text of the previous control.
    pub fn previous_label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.previous_label = label.into_fragment();
        self
    }

    /// Sets the text of the next control.
    pub fn next_label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.next_label = label.into_fragment();
        self
    }

    /// Disables the whole bar, including every link and control.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the bar width. With a `Fill` width the items stay centered,
    /// matching the web component's `mx-auto … justify-center` root.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the callback receiving the next 1-based page after a press on
    /// any link or boundary control.
    pub fn on_page_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize) -> Message + 'a,
    {
        self.on_page_change = Some(Box::new(callback));
        self
    }

    /// Sets or clears the callback receiving the next 1-based page.
    pub fn on_page_change_maybe<F>(mut self, callback: Option<F>) -> Self
    where
        F: Fn(usize) -> Message + 'a,
    {
        self.on_page_change = callback.map(|callback| Box::new(callback) as _);
        self
    }

    /// Number of pages derived from the configured count and page size.
    #[must_use]
    pub fn total_pages(&self) -> usize {
        types::total_pages(self.count, self.per_page)
    }

    /// The visible page range for the current configuration, including
    /// ellipsis gaps. Useful for fully custom layouts.
    #[must_use]
    pub fn items(&self) -> Vec<PaginationItem> {
        types::page_items(self.current_page(), self.total_pages(), self.sibling_count)
    }

    /// Whether a page exists before the current one.
    #[must_use]
    pub fn has_previous(&self) -> bool {
        self.current_page() > 1
    }

    /// Whether a page exists after the current one.
    #[must_use]
    pub fn has_next(&self) -> bool {
        self.current_page() < self.total_pages()
    }

    /// The 1-based inclusive `(start, end)` item range shown on the current
    /// page, or `None` when the bar has no items. Mirrors the `range`
    /// snippet value exposed by the bits-ui root.
    #[must_use]
    pub fn item_range(&self) -> Option<(usize, usize)> {
        if self.count == 0 {
            return None;
        }

        let per_page = self.per_page.max(1);
        let start = (self.current_page() - 1)
            .saturating_mul(per_page)
            .saturating_add(1);
        let end = start.saturating_add(per_page - 1).min(self.count);
        Some((start, end))
    }

    fn current_page(&self) -> usize {
        self.page.clamp(1, self.total_pages())
    }

    /// Builds the bar as an iced [`Element`](iced_core::Element).
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        render::build_pagination(self)
    }
}

impl<'a, Message> From<Pagination<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(pagination: Pagination<'a, Message>) -> Self {
        pagination.into_element()
    }
}

/// Standalone numbered page link for custom pagination layouts.
///
/// Renders through [`crate::Button`]: `outline` while active, `ghost`
/// otherwise, with an `icon` footprint by default — the same treatment
/// shadcn-svelte's `Pagination.Link` applies via `buttonVariants`.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct PaginationLink<'a, Message> {
    theme: &'a Theme,
    page: usize,
    content: Option<Element<'a, Message>>,
    active: bool,
    size: ButtonSize,
    active_variant: ButtonVariant,
    inactive_variant: ButtonVariant,
    color: Option<AccentColor>,
    disabled: bool,
    on_press: Option<Message>,
}

impl<Message> fmt::Debug for PaginationLink<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PaginationLink")
            .field("theme", &self.theme)
            .field("page", &self.page)
            .field("content", &self.content.is_some())
            .field("active", &self.active)
            .field("size", &self.size)
            .field("active_variant", &self.active_variant)
            .field("inactive_variant", &self.inactive_variant)
            .field("color", &self.color)
            .field("disabled", &self.disabled)
            .field("on_press", &self.on_press.is_some())
            .finish()
    }
}

impl<'a, Message> PaginationLink<'a, Message> {
    /// Creates a link for the given 1-based page, labeled with its number.
    pub fn new(page: usize, theme: &'a Theme) -> Self {
        Self {
            theme,
            page: page.max(1),
            content: None,
            active: false,
            size: ButtonSize::Icon,
            active_variant: ButtonVariant::Outline,
            inactive_variant: ButtonVariant::Ghost,
            color: None,
            disabled: false,
            on_press: None,
        }
    }

    /// Returns the 1-based page this link targets.
    #[must_use]
    pub fn page(&self) -> usize {
        self.page
    }

    /// Replaces the page-number label with arbitrary iced content.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Marks this link as the current page.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Sets the link footprint (`icon` by default).
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the button variant used while active (`outline` by default).
    pub fn active_variant(mut self, variant: ButtonVariant) -> Self {
        self.active_variant = variant;
        self
    }

    /// Sets the button variant used while inactive (`ghost` by default).
    pub fn inactive_variant(mut self, variant: ButtonVariant) -> Self {
        self.inactive_variant = variant;
        self
    }

    /// Applies an accent color overlay to the link.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Disables the link.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the message emitted when the link is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets or clears the message emitted when the link is pressed.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }
}

impl<'a, Message> From<PaginationLink<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(link: PaginationLink<'a, Message>) -> Self {
        render::build_link(render::LinkConfig {
            theme: link.theme,
            page: link.page,
            content: link.content,
            active: link.active,
            size: link.size,
            active_variant: link.active_variant,
            inactive_variant: link.inactive_variant,
            color: link.color,
            disabled: link.disabled,
            on_press: link.on_press,
        })
    }
}

/// Standalone "go to previous page" control for custom layouts.
///
/// A `ghost` button pairing a chevron with a "Previous" label, matching
/// shadcn-svelte's `Pagination.Previous`. Boundary handling stays with the
/// caller: leave [`Self::on_press`] unset (or set [`Self::disabled`]) on
/// the first page.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct PaginationPrevious<'a, Message> {
    theme: &'a Theme,
    label: Fragment<'a>,
    show_label: bool,
    icon: Option<Element<'a, Message>>,
    size: ButtonSize,
    variant: ButtonVariant,
    color: Option<AccentColor>,
    disabled: bool,
    on_press: Option<Message>,
}

impl<Message> fmt::Debug for PaginationPrevious<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_nav(
            formatter,
            "PaginationPrevious",
            NavParts {
                theme: self.theme,
                label: &self.label,
                show_label: self.show_label,
                has_icon: self.icon.is_some(),
                size: self.size,
                variant: self.variant,
                color: self.color,
                disabled: self.disabled,
                has_on_press: self.on_press.is_some(),
            },
        )
    }
}

impl<'a, Message> PaginationPrevious<'a, Message> {
    /// Creates a previous control with the default chevron and label.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            label: Fragment::from("Previous"),
            show_label: true,
            icon: None,
            size: ButtonSize::Default,
            variant: ButtonVariant::Ghost,
            color: None,
            disabled: false,
            on_press: None,
        }
    }

    /// Sets the control label.
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = label.into_fragment();
        self
    }

    /// Shows or hides the text label next to the chevron.
    pub fn show_label(mut self, show_label: bool) -> Self {
        self.show_label = show_label;
        self
    }

    /// Collapses the control to its chevron icon.
    pub fn icon_only(self) -> Self {
        self.show_label(false)
    }

    /// Replaces the default chevron with arbitrary iced content.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Sets the control footprint (`default` size by default).
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the button variant (`ghost` by default).
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Applies an accent color overlay to the control.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Disables the control.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the message emitted when the control is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets or clears the message emitted when the control is pressed.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }
}

impl<'a, Message> From<PaginationPrevious<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: PaginationPrevious<'a, Message>) -> Self {
        render::build_nav(render::NavConfig {
            theme: control.theme,
            direction: NavDirection::Previous,
            label: control.label,
            show_label: control.show_label,
            icon: control.icon,
            size: control.size,
            variant: control.variant,
            color: control.color,
            disabled: control.disabled,
            on_press: control.on_press,
        })
    }
}

/// Standalone "go to next page" control for custom layouts.
///
/// A `ghost` button pairing a "Next" label with a chevron, matching
/// shadcn-svelte's `Pagination.Next`. Boundary handling stays with the
/// caller: leave [`Self::on_press`] unset (or set [`Self::disabled`]) on
/// the last page.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct PaginationNext<'a, Message> {
    theme: &'a Theme,
    label: Fragment<'a>,
    show_label: bool,
    icon: Option<Element<'a, Message>>,
    size: ButtonSize,
    variant: ButtonVariant,
    color: Option<AccentColor>,
    disabled: bool,
    on_press: Option<Message>,
}

impl<Message> fmt::Debug for PaginationNext<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_nav(
            formatter,
            "PaginationNext",
            NavParts {
                theme: self.theme,
                label: &self.label,
                show_label: self.show_label,
                has_icon: self.icon.is_some(),
                size: self.size,
                variant: self.variant,
                color: self.color,
                disabled: self.disabled,
                has_on_press: self.on_press.is_some(),
            },
        )
    }
}

impl<'a, Message> PaginationNext<'a, Message> {
    /// Creates a next control with the default chevron and label.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            label: Fragment::from("Next"),
            show_label: true,
            icon: None,
            size: ButtonSize::Default,
            variant: ButtonVariant::Ghost,
            color: None,
            disabled: false,
            on_press: None,
        }
    }

    /// Sets the control label.
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = label.into_fragment();
        self
    }

    /// Shows or hides the text label next to the chevron.
    pub fn show_label(mut self, show_label: bool) -> Self {
        self.show_label = show_label;
        self
    }

    /// Collapses the control to its chevron icon.
    pub fn icon_only(self) -> Self {
        self.show_label(false)
    }

    /// Replaces the default chevron with arbitrary iced content.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Sets the control footprint (`default` size by default).
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the button variant (`ghost` by default).
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Applies an accent color overlay to the control.
    pub fn color(mut self, color: AccentColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Disables the control.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the message emitted when the control is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets or clears the message emitted when the control is pressed.
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }
}

impl<'a, Message> From<PaginationNext<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: PaginationNext<'a, Message>) -> Self {
        render::build_nav(render::NavConfig {
            theme: control.theme,
            direction: NavDirection::Next,
            label: control.label,
            show_label: control.show_label,
            icon: control.icon,
            size: control.size,
            variant: control.variant,
            color: control.color,
            disabled: control.disabled,
            on_press: control.on_press,
        })
    }
}

/// Non-interactive gap marker for custom pagination layouts.
///
/// A muted "more pages" glyph centered in the same square footprint as a
/// page link, matching shadcn-svelte's `Pagination.Ellipsis`.
#[must_use = "builders do nothing unless turned into an iced Element"]
#[derive(Debug)]
pub struct PaginationEllipsis<'a> {
    theme: &'a Theme,
    size: ButtonSize,
}

impl<'a> PaginationEllipsis<'a> {
    /// Creates an ellipsis with the default `icon` link footprint.
    pub fn new(theme: &'a Theme) -> Self {
        Self {
            theme,
            size: ButtonSize::Icon,
        }
    }

    /// Sets the footprint so the glyph lines up with resized links.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
}

impl<'a, Message> From<PaginationEllipsis<'a>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(ellipsis: PaginationEllipsis<'a>) -> Self {
        render::build_ellipsis(ellipsis.theme, ellipsis.size)
    }
}

/// Shorthand for [`Pagination::new`] with the count and current page set.
///
/// ```rust,no_run
/// use iced::Element;
/// use iced_shadcn_v2::{Theme, pagination};
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     PageChanged(usize),
/// }
///
/// fn pager(theme: &Theme, page: usize) -> Element<'_, Message> {
///     pagination(95, page, theme)
///         .on_page_change(Message::PageChanged)
///         .into()
/// }
/// ```
pub fn pagination<'a, Message>(
    count: usize,
    page: usize,
    theme: &'a Theme,
) -> Pagination<'a, Message> {
    Pagination::new(theme).count(count).page(page)
}

/// Shared field set behind the previous/next `Debug` implementations.
struct NavParts<'a> {
    theme: &'a Theme,
    label: &'a Fragment<'a>,
    show_label: bool,
    has_icon: bool,
    size: ButtonSize,
    variant: ButtonVariant,
    color: Option<AccentColor>,
    disabled: bool,
    has_on_press: bool,
}

fn debug_nav(
    formatter: &mut fmt::Formatter<'_>,
    name: &'static str,
    parts: NavParts<'_>,
) -> fmt::Result {
    formatter
        .debug_struct(name)
        .field("theme", &parts.theme)
        .field("label", &parts.label)
        .field("show_label", &parts.show_label)
        .field("icon", &parts.has_icon)
        .field("size", &parts.size)
        .field("variant", &parts.variant)
        .field("color", &parts.color)
        .field("disabled", &parts.disabled)
        .field("on_press", &parts.has_on_press)
        .finish()
}
