//! Pagination range math ported from Zag `@zag-js/pagination`.
//!
//! Pure page-windowing for backends; rendering and a11y stay in iced/egui.

/// Default sibling pages around the current page.
pub const DEFAULT_SIBLING_COUNT: usize = 1;

/// Default boundary pages kept at each edge.
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

/// Inputs for [`page_items`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageContext {
    /// Current 1-based page.
    pub page: usize,
    /// Total number of pages.
    pub total_pages: usize,
    /// Pages shown on each side of the current page.
    pub sibling_count: usize,
    /// Pages always shown at the start and end.
    pub boundary_count: usize,
}

impl PageContext {
    /// Builds a context with Zag defaults for sibling/boundary counts.
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

    /// Sets boundary count.
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
/// Port of Zag `getRange` + `transform`. Empty `total_pages` yields an empty
/// list; a single page yields `[Page(1)]`.
#[must_use]
pub fn page_items(ctx: PageContext) -> Vec<PaginationItem> {
    let total = ctx.total_pages;
    if total == 0 {
        return Vec::new();
    }
    if total == 1 {
        return vec![PaginationItem::Page(1)];
    }

    let page = ctx.page.clamp(1, total);
    let sibling = ctx.sibling_count;
    let boundary = ctx.boundary_count.max(1);

    let left_sibling = page.saturating_sub(sibling).max(1);
    let right_sibling = page.saturating_add(sibling).min(total);
    let total_page_numbers = (sibling * 2 + 3 + boundary * 2).min(total);

    if total <= total_page_numbers {
        return range_pages(1, total);
    }

    let item_count = total_page_numbers.saturating_sub(1 + boundary);
    let show_left = left_sibling > 1 + boundary + 1 && left_sibling.abs_diff(1) > boundary + 1;
    let show_right =
        right_sibling < total.saturating_sub(boundary + 1) && total.abs_diff(right_sibling) > boundary + 1;

    let mut pages: Vec<PageSlot> = Vec::with_capacity(total_page_numbers);

    if !show_left && show_right {
        pages.extend(range_slots(1, item_count));
        pages.push(PageSlot::Ellipsis);
        pages.extend(range_slots(total.saturating_sub(boundary) + 1, total));
    } else if show_left && !show_right {
        pages.extend(range_slots(1, boundary));
        pages.push(PageSlot::Ellipsis);
        pages.extend(range_slots(total.saturating_sub(item_count) + 1, total));
    } else if show_left && show_right {
        pages.extend(range_slots(1, boundary));
        pages.push(PageSlot::Ellipsis);
        pages.extend(range_slots(left_sibling, right_sibling));
        pages.push(PageSlot::Ellipsis);
        pages.extend(range_slots(total.saturating_sub(boundary) + 1, total));
    } else {
        pages.extend(range_slots(1, total));
    }

    collapse_single_page_ellipsis(&mut pages, total);
    pages
        .into_iter()
        .map(|slot| match slot {
            PageSlot::Page(value) => PaginationItem::Page(value),
            PageSlot::Ellipsis => PaginationItem::Ellipsis,
        })
        .collect()
}

#[derive(Clone, Copy)]
enum PageSlot {
    Page(usize),
    Ellipsis,
}

fn range_pages(start: usize, end: usize) -> Vec<PaginationItem> {
    (start..=end).map(PaginationItem::Page).collect()
}

fn range_slots(start: usize, end: usize) -> impl Iterator<Item = PageSlot> {
    (start..=end).map(PageSlot::Page)
}

fn collapse_single_page_ellipsis(pages: &mut [PageSlot], total: usize) {
    for index in 0..pages.len() {
        if !matches!(pages[index], PageSlot::Ellipsis) {
            continue;
        }
        let prev = match pages.get(index.wrapping_sub(1)) {
            Some(PageSlot::Page(value)) => *value,
            _ => 0,
        };
        let next = match pages.get(index + 1) {
            Some(PageSlot::Page(value)) => *value,
            _ => total.saturating_add(1),
        };
        if next.saturating_sub(prev) == 2 {
            pages[index] = PageSlot::Page(prev + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            [
                PaginationItem::Page(1),
                PaginationItem::Page(2),
                PaginationItem::Page(3),
                PaginationItem::Page(4),
                PaginationItem::Page(5),
            ]
        );
    }

    #[test]
    fn page_items_inserts_ellipsis_windows() {
        assert_eq!(
            page_items(PageContext::new(5, 10)),
            [
                PaginationItem::Page(1),
                PaginationItem::Ellipsis,
                PaginationItem::Page(4),
                PaginationItem::Page(5),
                PaginationItem::Page(6),
                PaginationItem::Ellipsis,
                PaginationItem::Page(10),
            ]
        );
    }

    #[test]
    fn page_items_respects_sibling_count() {
        let items = page_items(PageContext::new(10, 20).sibling_count(2));
        assert!(items.contains(&PaginationItem::Page(8)));
        assert!(items.contains(&PaginationItem::Page(12)));
        assert!(items.contains(&PaginationItem::Ellipsis));
    }
}
