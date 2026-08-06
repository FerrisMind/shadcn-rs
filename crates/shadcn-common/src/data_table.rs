//! Backend-agnostic data-table adapter wrapping `chorale-core`.
//!
//! Re-exports the headless table engine types and provides shadcn-specific
//! defaults (page sizes, label text) so iced and egui backends share one
//! configuration layer.

pub use chorale_core::Labels as DataTableLabels;

/// Default page-size options matching the shadcn-svelte data-table demo.
pub const DATA_TABLE_DEFAULT_PAGE_SIZES: &[usize] = &[10, 20, 30, 40, 50];

/// Default page size (rows per page).
pub const DATA_TABLE_DEFAULT_PAGE_SIZE: usize = 10;

/// Filter input debounce in milliseconds (for apps that throttle).
pub const DATA_TABLE_FILTER_DEBOUNCE_MS: u64 = 300;

/// Creates [`DataTableLabels`] with shadcn-svelte-style English text.
///
/// Overrides the chorale-core defaults to match the shadcn data-table demo
/// wording ("No results.", page arrows as icons, etc.). The struct is
/// `#[non_exhaustive]` so we mutate `Labels::default()`.
///
/// ```rust
/// use shadcn_common::data_table::shadcn_labels;
///
/// let labels = shadcn_labels();
/// assert_eq!(labels.filter_placeholder, "Filter emails...");
/// ```
#[must_use]
pub fn shadcn_labels() -> DataTableLabels {
    let mut labels = DataTableLabels::default();
    labels.filter_placeholder = "Filter emails...".into();
    labels.no_rows_label = "No results.".into();
    labels.previous_page_label = "\u{2039}".into();
    labels.next_page_label = "\u{203a}".into();
    labels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_page_sizes_are_reasonable() {
        assert_eq!(DATA_TABLE_DEFAULT_PAGE_SIZES, &[10, 20, 30, 40, 50]);
        assert_eq!(DATA_TABLE_DEFAULT_PAGE_SIZE, 10);
    }

    #[test]
    fn shadcn_labels_has_correct_wording() {
        let labels = shadcn_labels();
        assert_eq!(labels.filter_placeholder, "Filter emails...");
        assert_eq!(labels.no_rows_label, "No results.");
        let page_text = (labels.page_count)(2, 5);
        assert_eq!(page_text, "2 of 5");
    }
}
