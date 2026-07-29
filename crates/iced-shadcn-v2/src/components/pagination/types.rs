//! Configuration types and page-range math for the pagination component.

/// Default number of items per page, matching shadcn-svelte's `perPage`.
pub const DEFAULT_PER_PAGE: usize = 10;

/// Default number of sibling pages shown around the current page, matching
/// shadcn-svelte's `siblingCount`.
pub const DEFAULT_SIBLING_COUNT: usize = 1;

/// One slot in a computed pagination range.
///
/// Mirrors the bits-ui `PageItem` union: either a numbered page link or a
/// gap that the default layout renders as an ellipsis.
///
/// ```rust
/// use iced_shadcn_v2::PaginationItem;
///
/// assert_eq!(PaginationItem::Page(3).page(), Some(3));
/// assert!(PaginationItem::Ellipsis.is_ellipsis());
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PaginationItem {
    /// A numbered page link (1-based).
    Page(usize),
    /// A gap between non-adjacent page numbers.
    Ellipsis,
}

impl PaginationItem {
    /// Returns the 1-based page number, or `None` for an ellipsis.
    #[must_use]
    pub const fn page(self) -> Option<usize> {
        match self {
            Self::Page(page) => Some(page),
            Self::Ellipsis => None,
        }
    }

    /// Whether this slot is a gap between page numbers.
    #[must_use]
    pub const fn is_ellipsis(self) -> bool {
        matches!(self, Self::Ellipsis)
    }
}

/// Number of pages needed to show `count` items at `per_page` items each.
///
/// A `per_page` of zero is treated as one, and an empty collection still
/// produces a single page, matching bits-ui.
///
/// ```rust
/// use iced_shadcn_v2::pagination::total_pages;
///
/// assert_eq!(total_pages(95, 10), 10);
/// assert_eq!(total_pages(0, 10), 1);
/// ```
#[must_use]
pub fn total_pages(count: usize, per_page: usize) -> usize {
    count.div_ceil(per_page.max(1)).max(1)
}

/// Computes the visible page range for `page` of `total_pages`.
///
/// This is a port of the bits-ui `getPageItems` algorithm: the first and
/// last pages are always visible, `sibling_count` pages surround the
/// current page, and non-adjacent runs are separated by
/// [`PaginationItem::Ellipsis`]. `page` is clamped into
/// `1..=total_pages` and a zero `total_pages` is treated as one.
///
/// ```rust
/// use iced_shadcn_v2::PaginationItem::{Ellipsis, Page};
/// use iced_shadcn_v2::pagination::page_items;
///
/// assert_eq!(
///     page_items(5, 10, 1),
///     [Page(1), Ellipsis, Page(4), Page(5), Page(6), Ellipsis, Page(10)],
/// );
/// ```
#[must_use]
pub fn page_items(page: usize, total_pages: usize, sibling_count: usize) -> Vec<PaginationItem> {
    let total = total_pages.max(1);
    let page = page.clamp(1, total);

    let mut visible = std::collections::BTreeSet::new();
    visible.insert(1);
    visible.insert(total);

    let first_with_siblings = 3usize.saturating_add(sibling_count);
    let last_with_siblings = total.saturating_sub(2).saturating_sub(sibling_count);

    if first_with_siblings > last_with_siblings {
        // Every interior page fits without an ellipsis.
        visible.extend(2..total);
    } else if page < first_with_siblings {
        visible.extend(2..=first_with_siblings.min(total));
    } else if page > last_with_siblings {
        visible.extend(last_with_siblings.max(2)..=total.saturating_sub(1));
    } else {
        let start = page.saturating_sub(sibling_count).max(2);
        let end = page
            .saturating_add(sibling_count)
            .min(total.saturating_sub(1));
        visible.extend(start..=end);
    }

    let mut items = Vec::with_capacity(visible.len() + 2);
    let mut previous = 0usize;
    for page in visible {
        if page - previous > 1 {
            items.push(PaginationItem::Ellipsis);
        }
        items.push(PaginationItem::Page(page));
        previous = page;
    }

    items
}
