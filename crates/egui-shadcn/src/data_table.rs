//! Data table component with sorting, filtering, selection, and pagination.

use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::checkbox::{CheckboxCycle, CheckboxProps, CheckboxState, checkbox_with_props};
use crate::dropdown_menu::{
    DropdownMenuCheckboxItemProps, DropdownMenuProps, DropdownMenuTriggerProps, dropdown_menu,
    dropdown_menu_checkbox_item, dropdown_menu_trigger,
};
use crate::input::Input;
use crate::pagination::{
    PaginationLinkProps, PaginationProps, pagination, pagination_content, pagination_ellipsis,
    pagination_item, pagination_link, pagination_next, pagination_previous,
};
use crate::table::{
    TableCellProps, TableProps, TableRowProps, table, table_body, table_cell, table_head,
    table_header, table_row,
};
use crate::theme::Theme;
use egui::{Align, Direction, Id, Label, Layout, RichText, Sense, Ui, WidgetText};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SortValue {
    Str(String),
    Num(f64),
    Bool(bool),
}

impl fmt::Display for SortValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortValue::Str(value) => write!(f, "{value}"),
            SortValue::Num(value) => write!(f, "{value}"),
            SortValue::Bool(value) => write!(f, "{value}"),
        }
    }
}

impl SortValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (SortValue::Num(a), SortValue::Num(b)) => a.total_cmp(b),
            (SortValue::Bool(a), SortValue::Bool(b)) => a.cmp(b),
            (SortValue::Str(a), SortValue::Str(b)) => a.to_lowercase().cmp(&b.to_lowercase()),
            _ => self
                .to_string()
                .to_lowercase()
                .cmp(&other.to_string().to_lowercase()),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DataTableAlign {
    #[default]
    Left,
    Center,
    Right,
}

#[allow(clippy::type_complexity)]
pub struct DataTableColumn<'a, T> {
    pub id: String,
    pub label: String,
    pub header: WidgetText,
    pub cell: Box<dyn Fn(&mut Ui, &T) + 'a>,
    pub sort_value: Option<Box<dyn Fn(&T) -> SortValue + 'a>>,
    pub filter_value: Option<Box<dyn Fn(&T) -> String + 'a>>,
    pub hideable: bool,
    pub width: Option<f32>,
    pub align: DataTableAlign,
}

impl<'a, T> DataTableColumn<'a, T> {
    pub fn new(
        id: impl Into<String>,
        header: impl Into<String>,
        cell: impl Fn(&mut Ui, &T) + 'a,
    ) -> Self {
        let label = header.into();
        let header_text = WidgetText::from(RichText::new(label.clone()).strong());
        Self {
            id: id.into(),
            label,
            header: header_text,
            cell: Box::new(cell),
            sort_value: None,
            filter_value: None,
            hideable: true,
            width: None,
            align: DataTableAlign::Left,
        }
    }

    pub fn header(mut self, header: impl Into<WidgetText>) -> Self {
        self.header = header.into();
        self
    }

    pub fn sort_by(mut self, sort_value: impl Fn(&T) -> SortValue + 'a) -> Self {
        self.sort_value = Some(Box::new(sort_value));
        self
    }

    pub fn filter_by(mut self, filter_value: impl Fn(&T) -> String + 'a) -> Self {
        self.filter_value = Some(Box::new(filter_value));
        self
    }

    pub fn hideable(mut self, hideable: bool) -> Self {
        self.hideable = hideable;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn align(mut self, align: DataTableAlign) -> Self {
        self.align = align;
        self
    }
}

#[allow(clippy::type_complexity)]
pub struct DataTableProps<'a, T> {
    pub id_source: Id,
    pub columns: Vec<DataTableColumn<'a, T>>,
    pub data: &'a [T],
    pub page_size: usize,
    pub filter_placeholder: &'a str,
    pub filter_fn: Option<Box<dyn Fn(&T, &str) -> bool + 'a>>,
    pub enable_selection: bool,
    pub show_column_toggle: bool,
}

impl<'a, T> DataTableProps<'a, T> {
    pub fn new(id_source: Id, columns: Vec<DataTableColumn<'a, T>>, data: &'a [T]) -> Self {
        Self {
            id_source,
            columns,
            data,
            page_size: 10,
            filter_placeholder: "Filter...",
            filter_fn: None,
            enable_selection: true,
            show_column_toggle: true,
        }
    }

    pub fn page_size(mut self, page_size: usize) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn filter_placeholder(mut self, placeholder: &'a str) -> Self {
        self.filter_placeholder = placeholder;
        self
    }

    pub fn filter_fn(mut self, filter_fn: impl Fn(&T, &str) -> bool + 'a) -> Self {
        self.filter_fn = Some(Box::new(filter_fn));
        self
    }

    pub fn enable_selection(mut self, enable: bool) -> Self {
        self.enable_selection = enable;
        self
    }

    pub fn show_column_toggle(mut self, show: bool) -> Self {
        self.show_column_toggle = show;
        self
    }
}

#[derive(Clone, Debug, Default)]
struct DataTableState {
    page: usize,
    page_size: usize,
    filter: String,
    sort: Option<(usize, SortDirection)>,
    column_visibility: Vec<bool>,
    selected: HashSet<usize>,
}

#[derive(Clone, Debug)]
pub struct DataTableResponse {
    pub selected: Vec<usize>,
    pub filtered_rows: usize,
    pub total_rows: usize,
    pub page: usize,
    pub page_count: usize,
}

enum PageItem {
    Page(usize),
    Ellipsis,
}

fn pagination_items(current: usize, total: usize) -> Vec<PageItem> {
    if total <= 7 {
        return (1..=total).map(PageItem::Page).collect();
    }

    let mut items = Vec::new();
    items.push(PageItem::Page(1));

    let mut start = current.saturating_sub(1).max(2);
    let mut end = (current + 1).min(total.saturating_sub(1));

    if current <= 3 {
        start = 2;
        end = 4;
    } else if current >= total.saturating_sub(2) {
        start = total.saturating_sub(3);
        end = total.saturating_sub(1);
    }

    if start > 2 {
        items.push(PageItem::Ellipsis);
    }

    for page in start..=end {
        items.push(PageItem::Page(page));
    }

    if end < total.saturating_sub(1) {
        items.push(PageItem::Ellipsis);
    }

    items.push(PageItem::Page(total));
    items
}

pub fn data_table<'a, T>(
    ui: &mut Ui,
    theme: &Theme,
    props: DataTableProps<'a, T>,
) -> DataTableResponse
where
    T: 'a,
{
    let state_id = ui.make_persistent_id(props.id_source);
    let mut state = ui
        .ctx()
        .data(|data| data.get_temp::<DataTableState>(state_id))
        .unwrap_or_default();

    if state.page == 0 {
        state.page = 1;
    }

    if state.page_size != props.page_size && props.page_size > 0 {
        state.page_size = props.page_size;
        state.page = 1;
    } else if state.page_size == 0 {
        state.page_size = 10;
    }

    if state.column_visibility.len() != props.columns.len() {
        state.column_visibility = vec![true; props.columns.len()];
    }

    state.selected.retain(|index| *index < props.data.len());

    let mut filter_changed = false;
    ui.horizontal(|ui| {
        let filter_response = Input::new(state_id.with("filter"))
            .placeholder(props.filter_placeholder)
            .width(240.0)
            .show(ui, theme, &mut state.filter);
        filter_changed = filter_response.changed();

        if props.show_column_toggle {
            ui.add_space(12.0);
            let trigger = dropdown_menu_trigger(
                ui,
                DropdownMenuTriggerProps::new(state_id.with("columns-trigger")),
                |ui| {
                    Button::new("Columns")
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Sm)
                        .show(ui, theme)
                },
            );

            let visible_count = state
                .column_visibility
                .iter()
                .filter(|visible| **visible)
                .count();
            let _ = dropdown_menu(
                ui,
                theme,
                DropdownMenuProps::new(&trigger.response),
                |menu_ui| {
                    for (index, column) in props.columns.iter().enumerate() {
                        if !column.hideable {
                            continue;
                        }
                        let is_visible = state.column_visibility[index];
                        let disabled = is_visible && visible_count == 1;
                        let response = dropdown_menu_checkbox_item(
                            menu_ui,
                            theme,
                            DropdownMenuCheckboxItemProps::new(&column.label, is_visible)
                                .disabled(disabled),
                        );
                        if response.clicked() && !disabled {
                            state.column_visibility[index] = !is_visible;
                        }
                    }
                },
            );
        }
    });
    if filter_changed {
        state.page = 1;
    }

    let mut indices: Vec<usize> = (0..props.data.len()).collect();
    let filter_query = state.filter.trim();
    if !filter_query.is_empty() || props.filter_fn.is_some() {
        let query_lower = filter_query.to_lowercase();
        let has_column_filters = props
            .columns
            .iter()
            .any(|column| column.filter_value.is_some());
        indices.retain(|index| {
            let row = &props.data[*index];
            if let Some(filter_fn) = props.filter_fn.as_ref() {
                return filter_fn(row, filter_query);
            }
            if filter_query.is_empty() {
                return true;
            }
            if !has_column_filters {
                return true;
            }
            props.columns.iter().any(|column| {
                column.filter_value.as_ref().is_some_and(|filter_value| {
                    let value = filter_value(row);
                    value.to_lowercase().contains(&query_lower)
                })
            })
        });
    }
    if let Some(((_, direction), sort_fn)) = state.sort.and_then(|s| {
        props
            .columns
            .get(s.0)
            .and_then(|c| c.sort_value.as_ref())
            .map(|f| (s, f))
    }) {
        indices.sort_by(|a, b| {
            let a_val = sort_fn(&props.data[*a]);
            let b_val = sort_fn(&props.data[*b]);
            let ordering = a_val.cmp(&b_val);
            match direction {
                SortDirection::Asc => ordering,
                SortDirection::Desc => ordering.reverse(),
            }
        });
    }

    let total_rows = indices.len();
    let page_size = state.page_size.max(1);
    let total_pages = total_rows.div_ceil(page_size);
    let total_pages = total_pages.max(1);
    if state.page > total_pages {
        state.page = total_pages;
    }
    let start = (state.page - 1) * page_size;
    let end = (start + page_size).min(total_rows);
    let page_indices = if start < end {
        indices[start..end].to_vec()
    } else {
        Vec::new()
    };

    table(ui, theme, TableProps::new(), |ui, ctx| {
        table_header(ui, ctx, |ui| {
            table_row(
                ui,
                ctx,
                TableRowProps::new("header").hoverable(false),
                |ui| {
                    if props.enable_selection {
                        let selected_on_page = page_indices
                            .iter()
                            .filter(|index| state.selected.contains(index))
                            .count();
                        let mut header_state = if page_indices.is_empty() {
                            CheckboxState::Unchecked
                        } else if selected_on_page == page_indices.len() {
                            CheckboxState::Checked
                        } else if selected_on_page == 0 {
                            CheckboxState::Unchecked
                        } else {
                            CheckboxState::Indeterminate
                        };
                        let mut clicked = false;
                        let enabled = !page_indices.is_empty();
                        table_head(ui, ctx, TableCellProps::new().checkbox(true), |cell_ui| {
                            let response = checkbox_with_props(
                                cell_ui,
                                theme,
                                &mut header_state,
                                "",
                                CheckboxProps::default()
                                    .with_cycle(CheckboxCycle::Binary)
                                    .with_enabled(enabled),
                            );
                            clicked = response.clicked();
                        });
                        if clicked {
                            if selected_on_page == page_indices.len() {
                                for index in page_indices.iter() {
                                    state.selected.remove(index);
                                }
                            } else {
                                for index in page_indices.iter().copied() {
                                    state.selected.insert(index);
                                }
                            }
                        }
                    }
                    for (index, column) in props.columns.iter().enumerate() {
                        if !state.column_visibility.get(index).copied().unwrap_or(true) {
                            continue;
                        }
                        let sortable = column.sort_value.is_some();
                        let current_sort = state.sort.and_then(|(sorted_index, dir)| {
                            if sorted_index == index {
                                Some(dir)
                            } else {
                                None
                            }
                        });
                        let indicator = match current_sort {
                            Some(SortDirection::Asc) => Some("^"),
                            Some(SortDirection::Desc) => Some("v"),
                            None => None,
                        };
                        let mut clicked = false;
                        table_head(ui, ctx, TableCellProps::new(), |cell_ui| {
                            if let Some(width) = column.width {
                                cell_ui.set_min_width(width);
                            }
                            let layout = match column.align {
                                DataTableAlign::Left => Layout::left_to_right(Align::Center),
                                DataTableAlign::Center => {
                                    Layout::centered_and_justified(Direction::LeftToRight)
                                }
                                DataTableAlign::Right => Layout::right_to_left(Align::Center),
                            };
                            cell_ui.with_layout(layout, |inner_ui| {
                                let mut response = inner_ui.add(
                                    Label::new(column.header.clone()).sense(if sortable {
                                        Sense::click()
                                    } else {
                                        Sense::hover()
                                    }),
                                );
                                if sortable {
                                    response =
                                        response.on_hover_cursor(egui::CursorIcon::PointingHand);
                                }
                                clicked = response.clicked();
                                if let Some(indicator) = indicator {
                                    inner_ui.label(
                                        RichText::new(indicator)
                                            .color(theme.palette.muted_foreground)
                                            .size(12.0),
                                    );
                                }
                            });
                        });
                        if clicked && sortable {
                            state.sort = match state.sort {
                                Some((sorted_index, SortDirection::Asc))
                                    if sorted_index == index =>
                                {
                                    Some((index, SortDirection::Desc))
                                }
                                Some((sorted_index, SortDirection::Desc))
                                    if sorted_index == index =>
                                {
                                    None
                                }
                                _ => Some((index, SortDirection::Asc)),
                            };
                            state.page = 1;
                        }
                    }
                },
            );
        });

        table_body(ui, ctx, |ui| {
            if indices.is_empty() {
                table_row(ui, ctx, TableRowProps::new("empty"), |row_ui| {
                    table_cell(row_ui, ctx, TableCellProps::new().fill(true), |cell_ui| {
                        cell_ui.label("No results.");
                    });
                });
                return;
            }

            for index in page_indices.iter().copied() {
                let row = &props.data[index];
                let is_selected = state.selected.contains(&index);
                table_row(
                    ui,
                    ctx,
                    TableRowProps::new(index).selected(is_selected),
                    |row_ui| {
                        if props.enable_selection {
                            let mut row_state = CheckboxState::from(is_selected);
                            let response = table_cell(
                                row_ui,
                                ctx,
                                TableCellProps::new().checkbox(true),
                                |cell_ui| {
                                    checkbox_with_props(
                                        cell_ui,
                                        theme,
                                        &mut row_state,
                                        "",
                                        CheckboxProps::default(),
                                    )
                                },
                            );
                            if response.clicked() {
                                if is_selected {
                                    state.selected.remove(&index);
                                } else {
                                    state.selected.insert(index);
                                }
                            }
                        }
                        for (col_index, column) in props.columns.iter().enumerate() {
                            if !state
                                .column_visibility
                                .get(col_index)
                                .copied()
                                .unwrap_or(true)
                            {
                                continue;
                            }
                            table_cell(row_ui, ctx, TableCellProps::new(), |cell_ui| {
                                if let Some(width) = column.width {
                                    cell_ui.set_min_width(width);
                                }
                                let layout = match column.align {
                                    DataTableAlign::Left => Layout::left_to_right(Align::Center),
                                    DataTableAlign::Center => {
                                        Layout::centered_and_justified(Direction::LeftToRight)
                                    }
                                    DataTableAlign::Right => Layout::right_to_left(Align::Center),
                                };
                                cell_ui.with_layout(layout, |inner_ui| {
                                    (column.cell)(inner_ui, row);
                                });
                            });
                        }
                    },
                );
            }
        });
    });

    if total_pages > 1 {
        ui.add_space(12.0);
        pagination(
            ui,
            PaginationProps::new(total_pages, &mut state.page),
            |ui, props| {
                pagination_content(ui, |ui| {
                    pagination_item(ui, |ui| pagination_previous(ui, theme, props));
                    for item in pagination_items(*props.current_page, total_pages) {
                        match item {
                            PageItem::Page(page) => {
                                pagination_item(ui, |ui| {
                                    pagination_link(
                                        ui,
                                        theme,
                                        props,
                                        PaginationLinkProps::new(page, page.to_string())
                                            .active(page == *props.current_page),
                                    )
                                });
                            }
                            PageItem::Ellipsis => {
                                pagination_item(ui, |ui| pagination_ellipsis(ui, theme));
                            }
                        }
                    }
                    pagination_item(ui, |ui| pagination_next(ui, theme, props));
                });
            },
        );
    }

    ui.ctx()
        .data_mut(|data| data.insert_temp(state_id, state.clone()));

    DataTableResponse {
        selected: state.selected.iter().copied().collect(),
        filtered_rows: indices.len(),
        total_rows: props.data.len(),
        page: state.page,
        page_count: total_pages,
    }
}
