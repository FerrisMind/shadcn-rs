use std::cmp::Ordering;
use std::collections::HashSet;
use std::rc::Rc;

use iced::alignment::Horizontal;
use iced::widget::{Id, column, container, row, text};
use iced::{Alignment, Element, Length};

use crate::button::{ButtonProps, ButtonSize, ButtonVariant, button, button_content};
use crate::checkbox::{CheckboxProps, CheckboxState, checkbox};
use crate::dropdown_menu::{
    DropdownMenuCheckboxItem, DropdownMenuEntry, DropdownMenuItemProps, DropdownMenuProps,
    dropdown_menu,
};
use crate::input::{InputProps, InputVariant, input};
use crate::pagination::{
    PaginationProps, pagination, pagination_ellipsis, pagination_link, pagination_next,
    pagination_previous,
};
use crate::table::{
    TableCellProps, TableProps, TableRowProps, table, table_body, table_cell, table_head,
    table_header, table_row,
};
use crate::theme::Theme;

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

impl std::fmt::Display for SortValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortValue::Str(value) => write!(f, "{value}"),
            SortValue::Num(value) => write!(f, "{value}"),
            SortValue::Bool(value) => write!(f, "{value}"),
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
pub struct DataTableColumn<'a, T, Message> {
    pub id: String,
    pub label: String,
    pub header: String,
    pub cell: Box<dyn Fn(&T) -> Element<'a, Message> + 'a>,
    pub sort_value: Option<Box<dyn Fn(&T) -> SortValue + 'a>>,
    pub filter_value: Option<Box<dyn Fn(&T) -> String + 'a>>,
    pub hideable: bool,
    pub width: Option<f32>,
    pub align: DataTableAlign,
}

impl<'a, T, Message> DataTableColumn<'a, T, Message> {
    pub fn new(
        id: impl Into<String>,
        header: impl Into<String>,
        cell: impl Fn(&T) -> Element<'a, Message> + 'a,
    ) -> Self {
        let label = header.into();
        Self {
            id: id.into(),
            label: label.clone(),
            header: label,
            cell: Box::new(cell),
            sort_value: None,
            filter_value: None,
            hideable: true,
            width: None,
            align: DataTableAlign::Left,
        }
    }

    pub fn header(mut self, header: impl Into<String>) -> Self {
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
pub struct DataTableProps<'a, T, Message> {
    pub id_source: Id,
    pub columns: Vec<DataTableColumn<'a, T, Message>>,
    pub data: &'a [T],
    pub page_size: usize,
    pub filter_placeholder: &'a str,
    pub filter_fn: Option<Box<dyn Fn(&T, &str) -> bool + 'a>>,
    pub enable_selection: bool,
    pub show_column_toggle: bool,
}

impl<'a, T, Message> DataTableProps<'a, T, Message> {
    pub fn new(
        id_source: Id,
        columns: Vec<DataTableColumn<'a, T, Message>>,
        data: &'a [T],
    ) -> Self {
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
pub struct DataTableState {
    pub page: usize,
    pub filter: String,
    pub sort: Option<(usize, SortDirection)>,
    pub column_visibility: Vec<bool>,
    pub selected: HashSet<usize>,
}

#[derive(Clone, Debug)]
pub struct DataTableResponse {
    pub selected: Vec<usize>,
    pub filtered_rows: usize,
    pub total_rows: usize,
    pub page: usize,
    pub page_count: usize,
}

#[derive(Clone, Debug)]
pub enum DataTableAction {
    FilterChanged(String),
    SortChanged(Option<(usize, SortDirection)>),
    ToggleColumn(usize),
    ToggleRow(usize),
    ToggleAll(bool),
    PageChanged(usize),
}

pub fn data_table<'a, T, Message: Clone + 'a, F>(
    props: DataTableProps<'a, T, Message>,
    state: &'a DataTableState,
    on_action: Option<F>,
    theme: &'a Theme,
) -> Element<'a, Message>
where
    T: 'a,
    F: Fn(DataTableAction) -> Message + 'a,
{
    let on_action = on_action.map(|f| Rc::new(f) as Rc<dyn Fn(DataTableAction) -> Message + 'a>);
    let has_actions = on_action.is_some();
    let mut column_visibility = if state.column_visibility.is_empty() {
        vec![true; props.columns.len()]
    } else {
        state.column_visibility.clone()
    };
    if column_visibility.len() < props.columns.len() {
        column_visibility.resize(props.columns.len(), true);
    }

    let filtered = filter_rows(
        props.data,
        &props.columns,
        &state.filter,
        props.filter_fn.as_deref(),
    );
    let _total_rows = props.data.len();
    let _filtered_rows = filtered.len();

    let mut rows = filtered;
    if let Some((col_index, direction)) = state.sort
        && let Some(column) = props.columns.get(col_index)
        && let Some(sorter) = column.sort_value.as_ref()
    {
        rows.sort_by(|(_, a), (_, b)| {
            let a_value = sorter(a);
            let b_value = sorter(b);
            match direction {
                SortDirection::Asc => a_value.cmp(&b_value),
                SortDirection::Desc => b_value.cmp(&a_value),
            }
        });
    }

    let page_size = props.page_size.max(1);
    let page_count = rows.len().div_ceil(page_size);
    let page = state.page.clamp(1, page_count.max(1));
    let start = (page - 1) * page_size;
    let end = (start + page_size).min(rows.len());
    let page_rows = rows.get(start..end).unwrap_or(&[]);

    let filter_on_input = on_action.as_ref().map(|f| {
        let f = Rc::clone(f);
        move |value| f(DataTableAction::FilterChanged(value))
    });

    let filter_input = input(
        &state.filter,
        props.filter_placeholder,
        filter_on_input,
        InputProps::new().variant(InputVariant::Surface),
        theme,
    )
    .width(Length::Fixed(240.0));

    let mut controls = row![filter_input].spacing(12).align_y(Alignment::Center);

    if props.show_column_toggle {
        let visible_count = column_visibility.iter().filter(|visible| **visible).count();
        let menu_enabled = has_actions;
        let mut entries: Vec<DropdownMenuEntry<'a, Message>> = Vec::new();

        for (index, column) in props.columns.iter().enumerate() {
            if !column.hideable {
                continue;
            }
            let is_visible = column_visibility[index];
            let disabled = !menu_enabled || (is_visible && visible_count == 1);
            let on_toggle = on_action
                .as_ref()
                .map(|f| f(DataTableAction::ToggleColumn(index)))
                .filter(|_| !disabled);
            let entry = DropdownMenuEntry::CheckboxItem(
                DropdownMenuCheckboxItem::new(column.label.clone(), is_visible, on_toggle)
                    .props(DropdownMenuItemProps::new().disabled(disabled)),
            );
            entries.push(entry);
        }

        if !entries.is_empty() {
            let trigger = button(
                "Columns",
                None,
                ButtonProps::new()
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Size1)
                    .disabled(!menu_enabled),
                theme,
            );
            let menu = dropdown_menu(
                trigger,
                entries,
                DropdownMenuProps::new().width(200).disabled(!menu_enabled),
                theme,
            );
            controls = controls.push(menu);
        }
    }

    let pagination_items = pagination_items(page, page_count);
    let mut items = Vec::new();
    items.push(pagination_previous());
    for item in pagination_items {
        match item {
            PageItem::Page(p) => items.push(pagination_link(p, p.to_string())),
            PageItem::Ellipsis => items.push(pagination_ellipsis()),
        }
    }
    items.push(pagination_next());

    let pagination_control = pagination(
        items,
        PaginationProps::new(page_count.max(1), page),
        on_action.as_ref().map(|f| {
            let f = Rc::clone(f);
            move |value| f(DataTableAction::PageChanged(value))
        }),
        theme,
    );

    let table_element = table(TableProps::default(), theme, |ctx| {
        let mut header_cells: Vec<Element<'a, Message>> = Vec::new();
        if props.enable_selection {
            let all_selected = page_rows
                .iter()
                .all(|(idx, _)| state.selected.contains(idx));
            let any_selected = page_rows
                .iter()
                .any(|(idx, _)| state.selected.contains(idx));
            let header_state = if all_selected {
                CheckboxState::Checked
            } else if any_selected {
                CheckboxState::Indeterminate
            } else {
                CheckboxState::Unchecked
            };
            let on_toggle = on_action.as_ref().map(|f| {
                let f = Rc::clone(f);
                move |next| {
                    f(DataTableAction::ToggleAll(matches!(
                        next,
                        CheckboxState::Checked
                    )))
                }
            });
            header_cells.push(table_head(
                ctx,
                TableCellProps::new().checkbox(true),
                checkbox(header_state, on_toggle, CheckboxProps::new(), theme),
            ));
        }

        for (index, column) in props.columns.iter().enumerate() {
            if !column_visibility[index] {
                continue;
            }
            let sortable = column.sort_value.is_some();
            let indicator = match state.sort {
                Some((current, SortDirection::Asc)) if current == index => Some("▲"),
                Some((current, SortDirection::Desc)) if current == index => Some("▼"),
                _ => None,
            };
            let on_press = on_action.as_ref().filter(|_| sortable).map(|f| {
                let f = Rc::clone(f);
                let next = match state.sort {
                    Some((current, SortDirection::Asc)) if current == index => {
                        Some((index, SortDirection::Desc))
                    }
                    Some((current, SortDirection::Desc)) if current == index => None,
                    _ => Some((index, SortDirection::Asc)),
                };
                f(DataTableAction::SortChanged(next))
            });

            let indicator_element: Element<'a, Message> = if let Some(text_value) = indicator {
                text(text_value).size(10).into()
            } else {
                text("").size(10).into()
            };

            let header_content = row![text(column.header.clone()).size(12), indicator_element]
                .spacing(4)
                .align_y(Alignment::Center);

            let header: Element<'a, Message> = if sortable {
                button_content(
                    header_content,
                    on_press,
                    ButtonProps::new()
                        .variant(ButtonVariant::Ghost)
                        .size(ButtonSize::Size1)
                        .disabled(!has_actions),
                    theme,
                )
                .into()
            } else {
                header_content.into()
            };

            let aligned = align_cell(header, column.align, column.width);
            let cell = table_head(
                ctx,
                TableCellProps::new().fill(column.width.is_none()),
                aligned,
            );
            header_cells.push(cell);
        }

        let header_row = table_row(
            ctx,
            TableRowProps::new(props.id_source.clone()).hoverable(false),
            header_cells,
        );

        let mut body_rows: Vec<Element<'a, Message>> = Vec::new();
        if page_rows.is_empty() {
            let mut empty_cells: Vec<Element<'a, Message>> = Vec::new();
            if props.enable_selection {
                let empty_placeholder: Element<'a, Message> = text("").into();
                empty_cells.push(table_cell(
                    ctx,
                    TableCellProps::new().checkbox(true),
                    empty_placeholder,
                ));
            }
            let empty_text =
                text("No results.")
                    .size(12)
                    .style(move |_t| iced::widget::text::Style {
                        color: Some(theme.palette.muted_foreground),
                    });
            empty_cells.push(table_cell(
                ctx,
                TableCellProps::new().fill(true),
                empty_text,
            ));
            body_rows.push(table_row(
                ctx,
                TableRowProps::new(props.id_source.clone()).hoverable(false),
                empty_cells,
            ));
        } else {
            for (row_index, row) in page_rows.iter() {
                let mut cells: Vec<Element<'a, Message>> = Vec::new();
                if props.enable_selection {
                    let checked = state.selected.contains(row_index);
                    let on_toggle = on_action.as_ref().map(|f| {
                        let f = Rc::clone(f);
                        let idx = *row_index;
                        move |_next| f(DataTableAction::ToggleRow(idx))
                    });
                    cells.push(table_cell(
                        ctx,
                        TableCellProps::new().checkbox(true),
                        checkbox(checked.into(), on_toggle, CheckboxProps::new(), theme),
                    ));
                }

                for (col_index, column) in props.columns.iter().enumerate() {
                    if !column_visibility[col_index] {
                        continue;
                    }
                    let content = (column.cell)(row);
                    let aligned = align_cell(content, column.align, column.width);
                    cells.push(table_cell(
                        ctx,
                        TableCellProps::new().fill(column.width.is_none()),
                        aligned,
                    ));
                }
                let row_element = table_row(
                    ctx,
                    TableRowProps::new(props.id_source.clone())
                        .selected(state.selected.contains(row_index)),
                    cells,
                );
                body_rows.push(row_element);
            }
        }

        column![
            table_header(ctx, header_row),
            table_body(ctx, column(body_rows).spacing(0))
        ]
        .spacing(0)
        .into()
    });

    column![controls, table_element, pagination_control]
        .spacing(12)
        .into()
}

fn align_cell<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    align: DataTableAlign,
    width: Option<f32>,
) -> Element<'a, Message> {
    let mut wrapper = container(content.into());
    if let Some(width) = width {
        wrapper = wrapper.width(Length::Fixed(width.max(1.0)));
    } else {
        wrapper = wrapper.width(Length::Fill);
    }
    let horizontal = match align {
        DataTableAlign::Left => Horizontal::Left,
        DataTableAlign::Center => Horizontal::Center,
        DataTableAlign::Right => Horizontal::Right,
    };
    wrapper.align_x(horizontal).into()
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

type FilterFn<'a, T> = dyn Fn(&T, &str) -> bool + 'a;

fn filter_rows<'a, T, Message>(
    data: &'a [T],
    columns: &[DataTableColumn<'a, T, Message>],
    filter: &str,
    filter_fn: Option<&FilterFn<'a, T>>,
) -> Vec<(usize, &'a T)> {
    let filter = filter.trim();
    if filter.is_empty() {
        return data.iter().enumerate().collect();
    }
    let filter_lower = filter.to_lowercase();

    let has_column_filters = columns.iter().any(|column| column.filter_value.is_some());
    if !has_column_filters && filter_fn.is_none() {
        return data.iter().enumerate().collect();
    }

    data.iter()
        .enumerate()
        .filter(|(_, row)| {
            if let Some(filter_fn) = filter_fn {
                return filter_fn(row, filter);
            }
            columns.iter().any(|col| {
                if let Some(filter_value) = col.filter_value.as_ref() {
                    filter_value(row).to_lowercase().contains(&filter_lower)
                } else {
                    false
                }
            })
        })
        .collect()
}
