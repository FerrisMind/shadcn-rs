//! Pagination range math for shadcn backends.
//!
//! Port of the bits-ui `getPageItems` algorithm used by shadcn-svelte.
//! Rendering stays in iced/egui adapters.

use std::collections::BTreeSet;
use std::fmt;

/// Default sibling pages around the current page (`siblingCount`).
pub const DEFAULT_SIBLING_COUNT: usize = 1;

/// Default boundary pages kept at each edge.
///
/// Reserved for APIs that expose Zag-style `boundaryCount`; the bits-ui
/// window used by [`page_items`] always keeps the first and last page.
pub const DEFAULT_BOUNDARY_COUNT: usize = 1;

/// One slot in a computed pagination range.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl fmt::Display for PaginationItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Page(page) => write!(formatter, "{page}"),
            Self::Ellipsis => formatter.write_str("ellipsis"),
        }
    }
}

/// Inputs for [`page_items`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageContext {
    /// Current 1-based page.
    pub page: usize,
    /// Total number of pages.
    pub total_pages: usize,
    /// Pages shown on each side of the current page.
    pub sibling_count: usize,
    /// Kept for API stability; bits-ui always anchors first/last pages.
    pub boundary_count: usize,
}

impl PageContext {
    /// Builds a context with shadcn defaults for sibling/boundary counts.
    #[must_use]
    pub const fn new(page: usize, total_pages: usize) -> Self {
        Self {
            page,
            total_pages,
            sibling_count: DEFAULT_SIBLING_COUNT,
            boundary_count: DEFAULT_BOUNDARY_COUNT,
        }
    }

    /// Sets sibling count.
    #[must_use]
    pub const fn sibling_count(mut self, sibling_count: usize) -> Self {
        self.sibling_count = sibling_count;
        self
    }

    /// Sets boundary count (currently unused by the bits-ui window).
    #[must_use]
    pub const fn boundary_count(mut self, boundary_count: usize) -> Self {
        self.boundary_count = boundary_count;
        self
    }
}

/// Number of pages needed to show `count` items at `per_page` items each.
///
/// A `per_page` of zero is treated as one; an empty collection still yields one
/// page.
#[must_use]
pub fn total_pages(count: usize, per_page: usize) -> usize {
    count.div_ceil(per_page.max(1)).max(1)
}

/// Computes the visible page/ellipsis sequence for `ctx`.
///
/// First and last pages are always visible, `sibling_count` pages surround the
/// current page, and non-adjacent runs are separated by
/// [`PaginationItem::Ellipsis`]. `page` is clamped into `1..=total_pages`, and
/// a zero `total_pages` is treated as one.
#[must_use]
pub fn page_items(ctx: PageContext) -> Vec<PaginationItem> {
    let total = ctx.total_pages.max(1);
    let page = ctx.page.clamp(1, total);
    let sibling_count = ctx.sibling_count;

    let mut visible = BTreeSet::new();
    visible.insert(1);
    visible.insert(total);

    let first_with_siblings = 3usize.saturating_add(sibling_count);
    let last_with_siblings = total.saturating_sub(2).saturating_sub(sibling_count);

    if first_with_siblings > last_with_siblings {
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

#[cfg(test)]
mod tests {
    use super::*;
    use PaginationItem::{Ellipsis, Page};

    #[test]
    fn total_pages_matches_bits_ui_defaults() {
        assert_eq!(total_pages(95, 10), 10);
        assert_eq!(total_pages(0, 10), 1);
        assert_eq!(total_pages(5, 0), 5);
    }

    #[test]
    fn page_items_shows_full_range_when_small() {
        assert_eq!(
            page_items(PageContext::new(1, 5)),
            [Page(1), Page(2), Page(3), Page(4), Page(5)]
        );
        assert_eq!(page_items(PageContext::new(1, 1)), [Page(1)]);
    }

    #[test]
    fn page_items_matches_bits_ui_windows() {
        assert_eq!(
            page_items(PageContext::new(1, 10)),
            [Page(1), Page(2), Page(3), Page(4), Ellipsis, Page(10)]
        );
        assert_eq!(
            page_items(PageContext::new(5, 10)),
            [
                Page(1),
                Ellipsis,
                Page(4),
                Page(5),
                Page(6),
                Ellipsis,
                Page(10)
            ]
        );
        assert_eq!(
            page_items(PageContext::new(10, 10)),
            [Page(1), Ellipsis, Page(7), Page(8), Page(9), Page(10)]
        );
    }

    #[test]
    fn page_items_respects_sibling_count() {
        assert_eq!(
            page_items(PageContext::new(10, 20).sibling_count(2)),
            [
                Page(1),
                Ellipsis,
                Page(8),
                Page(9),
                Page(10),
                Page(11),
                Page(12),
                Ellipsis,
                Page(20),
            ]
        );
    }

    #[test]
    fn page_items_normalizes_degenerate_inputs() {
        assert_eq!(page_items(PageContext::new(7, 0)), [Page(1)]);
        assert_eq!(
            page_items(PageContext::new(0, 3)),
            [Page(1), Page(2), Page(3)]
        );
        assert_eq!(
            page_items(PageContext::new(99, 10).sibling_count(usize::MAX)),
            [
                Page(1),
                Page(2),
                Page(3),
                Page(4),
                Page(5),
                Page(6),
                Page(7),
                Page(8),
                Page(9),
                Page(10),
            ]
        );
    }
}
