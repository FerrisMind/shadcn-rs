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
use shadcn_common::AccentColor;

use crate::components::button::{ButtonRadius, ButtonSize, ButtonVariant};
use crate::components::checkbox::{CheckboxSize, CheckboxVariant};
use crate::components::input::{InputRadius, InputSize};
use crate::iced_compat::{Element, Length};
use crate::theme::Theme;

/// Builder-first data-table rendering `chorale-core::TableState`.
///
/// Pass `&TableState<TRow>` each frame; wire callbacks so your app's
/// `update()` applies chorale-core transitions and stores the new state.
/// Styling is forwarded to the composed [`crate::Input`], [`crate::Button`],
/// [`crate::Checkbox`], and [`crate::Table`] builders through the
/// `filter_input_*`, `sort_button_*`, `columns_button_*`,
/// `pagination_button_*`, and `checkbox_*` methods.
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
    filter_value: String,
    filter_input_size: InputSize,
    filter_input_radius: Option<InputRadius>,
    filter_input_color: Option<AccentColor>,
    sort_button_variant: ButtonVariant,
    sort_button_size: ButtonSize,
    sort_button_radius: Option<ButtonRadius>,
    sort_button_color: Option<AccentColor>,
    columns_button_variant: ButtonVariant,
    columns_button_size: ButtonSize,
    columns_button_radius: Option<ButtonRadius>,
    columns_button_color: Option<AccentColor>,
    pagination_button_variant: ButtonVariant,
    pagination_button_size: ButtonSize,
    pagination_button_radius: Option<ButtonRadius>,
    pagination_button_color: Option<AccentColor>,
    checkbox_variant: CheckboxVariant,
    checkbox_size: CheckboxSize,
    table_width: Length,
    table_min_width: f32,
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
            .field("filter_input_size", &self.filter_input_size)
            .field("sort_button_variant", &self.sort_button_variant)
            .field("sort_button_size", &self.sort_button_size)
            .field("columns_button_variant", &self.columns_button_variant)
            .field("columns_button_size", &self.columns_button_size)
            .field("pagination_button_variant", &self.pagination_button_variant)
            .field("pagination_button_size", &self.pagination_button_size)
            .field("checkbox_variant", &self.checkbox_variant)
            .field("checkbox_size", &self.checkbox_size)
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
            filter_value: String::new(),
            filter_input_size: InputSize::Default,
            filter_input_radius: None,
            filter_input_color: None,
            sort_button_variant: ButtonVariant::Ghost,
            sort_button_size: ButtonSize::Default,
            sort_button_radius: None,
            sort_button_color: None,
            columns_button_variant: ButtonVariant::Outline,
            columns_button_size: ButtonSize::Default,
            columns_button_radius: None,
            columns_button_color: None,
            pagination_button_variant: ButtonVariant::Outline,
            pagination_button_size: ButtonSize::Sm,
            pagination_button_radius: None,
            pagination_button_color: None,
            checkbox_variant: CheckboxVariant::Surface,
            checkbox_size: CheckboxSize::Xs,
            table_width: Length::Fill,
            table_min_width: 0.0,
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

    /// Set the controlled value shown in the global filter input.
    pub fn filter_value(mut self, value: impl Into<String>) -> Self {
        self.filter_value = value.into();
        self
    }

    /// Sets the [`InputSize`] used by the composed global filter input.
    pub fn filter_input_size(mut self, size: InputSize) -> Self {
        self.filter_input_size = size;
        self
    }

    /// Sets an explicit [`InputRadius`] for the composed global filter input.
    ///
    /// When this is not called, the active theme's input radius is preserved.
    pub fn filter_input_radius(mut self, radius: InputRadius) -> Self {
        self.filter_input_radius = Some(radius);
        self
    }

    /// Applies an accent overlay to the composed global filter input.
    pub fn filter_input_color(mut self, color: AccentColor) -> Self {
        self.filter_input_color = Some(color);
        self
    }

    /// Sets the [`ButtonVariant`] used by sortable column-header buttons.
    pub fn sort_button_variant(mut self, variant: ButtonVariant) -> Self {
        self.sort_button_variant = variant;
        self
    }

    /// Sets the [`ButtonSize`] used by sortable column-header buttons.
    pub fn sort_button_size(mut self, size: ButtonSize) -> Self {
        self.sort_button_size = size;
        self
    }

    /// Sets an explicit [`ButtonRadius`] for sortable column-header buttons.
    pub fn sort_button_radius(mut self, radius: ButtonRadius) -> Self {
        self.sort_button_radius = Some(radius);
        self
    }

    /// Applies an accent overlay to sortable column-header buttons.
    pub fn sort_button_color(mut self, color: AccentColor) -> Self {
        self.sort_button_color = Some(color);
        self
    }

    /// Sets the [`ButtonVariant`] used by the column-visibility trigger.
    pub fn columns_button_variant(mut self, variant: ButtonVariant) -> Self {
        self.columns_button_variant = variant;
        self
    }

    /// Sets the [`ButtonSize`] used by the column-visibility trigger.
    pub fn columns_button_size(mut self, size: ButtonSize) -> Self {
        self.columns_button_size = size;
        self
    }

    /// Sets an explicit [`ButtonRadius`] for the column-visibility trigger.
    pub fn columns_button_radius(mut self, radius: ButtonRadius) -> Self {
        self.columns_button_radius = Some(radius);
        self
    }

    /// Applies an accent overlay to the column-visibility trigger.
    pub fn columns_button_color(mut self, color: AccentColor) -> Self {
        self.columns_button_color = Some(color);
        self
    }

    /// Sets the [`ButtonVariant`] used by pagination controls.
    pub fn pagination_button_variant(mut self, variant: ButtonVariant) -> Self {
        self.pagination_button_variant = variant;
        self
    }

    /// Sets the [`ButtonSize`] used by pagination controls.
    pub fn pagination_button_size(mut self, size: ButtonSize) -> Self {
        self.pagination_button_size = size;
        self
    }

    /// Sets an explicit [`ButtonRadius`] for pagination controls.
    pub fn pagination_button_radius(mut self, radius: ButtonRadius) -> Self {
        self.pagination_button_radius = Some(radius);
        self
    }

    /// Applies an accent overlay to pagination controls.
    pub fn pagination_button_color(mut self, color: AccentColor) -> Self {
        self.pagination_button_color = Some(color);
        self
    }

    /// Sets the [`CheckboxVariant`] used by selection checkboxes.
    pub fn checkbox_variant(mut self, variant: CheckboxVariant) -> Self {
        self.checkbox_variant = variant;
        self
    }

    /// Sets the [`CheckboxSize`] used by selection checkboxes.
    pub fn checkbox_size(mut self, size: CheckboxSize) -> Self {
        self.checkbox_size = size;
        self
    }

    /// Sets the outer width passed to the composed [`crate::Table`].
    pub fn table_width(mut self, width: impl Into<Length>) -> Self {
        self.table_width = width.into();
        self
    }

    /// Sets the minimum scrollable width passed to the composed [`crate::Table`].
    pub fn table_min_width(mut self, width: f32) -> Self {
        self.table_min_width = width;
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
