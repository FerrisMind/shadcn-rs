//! Data-table component: headless `chorale-core` state rendered via the
//! existing `Table` component with sort/filter/pagination/selection chrome.
//!
//! The application owns a [`chorale_core::TableState`] and passes it into
//! the [`DataTable`] builder each frame. User interactions produce messages
//! that the app handles in `update()` by calling chorale-core transition
//! functions (e.g. `toggle_sort`, `set_filter`, `set_page`) and storing the
//! new state.
//!
//! ```rust,no_run
//! use chorale_core::{ColumnDef, ColumnId, CellValue, TableState, SortAction, toggle_sort, set_page};
//! use iced::Element;
//! use iced_shadcn_v2::{DataTable, Theme};
//!
//! #[derive(Clone)]
//! struct Payment { id: String, amount: f64, status: String, email: String }
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Sort(ColumnId, SortAction),
//!     Page(usize),
//! }
//!
//! fn view<'a>(theme: &'a Theme, state: &'a TableState<Payment>) -> Element<'a, Message> {
//!     DataTable::new(theme, state)
//!         .on_sort(Message::Sort)
//!         .on_page(Message::Page)
//!         .into()
//! }
//! ```

mod render;

#[cfg(test)]
mod tests;

use std::fmt;

use chorale_core::{ColumnId, FilterValue, RowId, SortAction, TableState};

use crate::iced_compat::Element;
use crate::theme::Theme;

/// Builder-first data-table rendering `chorale-core::TableState`.
///
/// Pass `&TableState<TRow>` each frame; wire callbacks so your app's
/// `update()` applies chorale-core transitions and stores the new state.
#[must_use = "builders do nothing unless turned into an iced Element"]
#[allow(clippy::type_complexity)]
pub struct DataTable<'a, TRow: Clone + 'static, Message> {
    theme: &'a Theme,
    state: &'a TableState<TRow>,
    sortable: bool,
    filterable: bool,
    paginated: bool,
    selectable: bool,
    column_visibility: bool,
    page_sizes: &'a [usize],
    empty_message: String,
    filter_placeholder: String,
    on_sort: Option<Box<dyn Fn(ColumnId, SortAction) -> Message + 'a>>,
    on_filter: Option<Box<dyn Fn(ColumnId, Option<FilterValue>) -> Message + 'a>>,
    on_global_filter: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_page: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    on_page_size: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    on_select: Option<Box<dyn Fn(RowId, bool) -> Message + 'a>>,
    on_select_all: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    on_column_visibility: Option<Box<dyn Fn(ColumnId, bool) -> Message + 'a>>,
}

impl<TRow: Clone + 'static, Message> fmt::Debug for DataTable<'_, TRow, Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataTable")
            .field("sortable", &self.sortable)
            .field("filterable", &self.filterable)
            .field("paginated", &self.paginated)
            .field("selectable", &self.selectable)
            .field("column_visibility", &self.column_visibility)
            .field("on_sort", &self.on_sort.is_some())
            .field("on_page", &self.on_page.is_some())
            .field("on_select", &self.on_select.is_some())
            .finish_non_exhaustive()
    }
}

impl<'a, TRow, Message> DataTable<'a, TRow, Message>
where
    TRow: Clone + 'static,
{
    /// Creates a data-table rendering the given state.
    pub fn new(theme: &'a Theme, state: &'a TableState<TRow>) -> Self {
        Self {
            theme,
            state,
            sortable: true,
            filterable: true,
            paginated: true,
            selectable: true,
            column_visibility: true,
            page_sizes: shadcn_common::data_table::DATA_TABLE_DEFAULT_PAGE_SIZES,
            empty_message: "No results.".to_owned(),
            filter_placeholder: "Filter emails...".to_owned(),
            on_sort: None,
            on_filter: None,
            on_global_filter: None,
            on_page: None,
            on_page_size: None,
            on_select: None,
            on_select_all: None,
            on_column_visibility: None,
        }
    }

    /// Show sort direction indicators on column headers.
    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    /// Show the global filter input above the table.
    pub fn filterable(mut self, filterable: bool) -> Self {
        self.filterable = filterable;
        self
    }

    /// Show pagination controls below the table.
    pub fn paginated(mut self, paginated: bool) -> Self {
        self.paginated = paginated;
        self
    }

    /// Show row-selection checkboxes.
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    /// Show the column-visibility dropdown.
    pub fn column_visibility(mut self, enabled: bool) -> Self {
        self.column_visibility = enabled;
        self
    }

    /// Override the page-size options.
    pub fn page_sizes(mut self, sizes: &'a [usize]) -> Self {
        self.page_sizes = sizes;
        self
    }

    /// Override the empty-state message.
    pub fn empty_message(mut self, message: impl Into<String>) -> Self {
        self.empty_message = message.into();
        self
    }

    /// Override the filter input placeholder.
    pub fn filter_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.filter_placeholder = placeholder.into();
        self
    }

    /// Callback when a column header is clicked for sorting.
    pub fn on_sort(mut self, callback: impl Fn(ColumnId, SortAction) -> Message + 'a) -> Self {
        self.on_sort = Some(Box::new(callback));
        self
    }

    /// Callback when a per-column filter changes.
    pub fn on_filter(
        mut self,
        callback: impl Fn(ColumnId, Option<FilterValue>) -> Message + 'a,
    ) -> Self {
        self.on_filter = Some(Box::new(callback));
        self
    }

    /// Callback when the global filter text changes.
    pub fn on_global_filter(mut self, callback: impl Fn(String) -> Message + 'a) -> Self {
        self.on_global_filter = Some(Box::new(callback));
        self
    }

    /// Callback when the page number changes.
    pub fn on_page(mut self, callback: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_page = Some(Box::new(callback));
        self
    }

    /// Callback when the page size changes.
    pub fn on_page_size(mut self, callback: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_page_size = Some(Box::new(callback));
        self
    }

    /// Callback when a single row's selection checkbox is toggled.
    pub fn on_select(mut self, callback: impl Fn(RowId, bool) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(callback));
        self
    }

    /// Callback when the header "select all" checkbox is toggled.
    pub fn on_select_all(mut self, callback: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_select_all = Some(Box::new(callback));
        self
    }

    /// Callback when a column's visibility is toggled.
    pub fn on_column_visibility(
        mut self,
        callback: impl Fn(ColumnId, bool) -> Message + 'a,
    ) -> Self {
        self.on_column_visibility = Some(Box::new(callback));
        self
    }
}

/// Convenience: creates a [`DataTable`].
pub fn data_table<'a, TRow, Message>(
    theme: &'a Theme,
    state: &'a TableState<TRow>,
) -> DataTable<'a, TRow, Message>
where
    TRow: Clone + 'static,
{
    DataTable::new(theme, state)
}

impl<'a, TRow, Message> From<DataTable<'a, TRow, Message>> for Element<'a, Message>
where
    TRow: Clone + 'static,
    Message: Clone + 'a,
{
    fn from(dt: DataTable<'a, TRow, Message>) -> Self {
        render::build_data_table(dt)
    }
}
