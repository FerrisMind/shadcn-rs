//! Configuration types and page-range math for the pagination component.
//!
//! Range helpers live in [`shadcn_common`]; this module keeps iced-facing
//! defaults and the three-argument [`page_items`] signature used by the
//! component API.

pub use shadcn_common::{DEFAULT_SIBLING_COUNT, PageContext, PaginationItem, total_pages};

/// Default number of items per page, matching shadcn-svelte's `perPage`.
pub const DEFAULT_PER_PAGE: usize = 10;

/// Computes the visible page range for `page` of `total_pages`.
///
/// Thin wrapper around [`shadcn_common::page_items`] that preserves the
/// iced/shadcn call shape `(page, total_pages, sibling_count)`.
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
    shadcn_common::page_items(PageContext::new(page, total_pages).sibling_count(sibling_count))
}
