//! Rendering for the data-table: composes Table + Input + Pagination chrome.

use std::rc::Rc;

use chorale_core::{
    Alignment as ColumnAlignment, CellValue, RenderRow, SortAction, SortDirection,
    filtered_sorted_pairs, visible_view,
};
use shadcn_common::AccentColor;

use crate::components::button::{Button, ButtonRadius, ButtonSize, ButtonVariant};
use crate::components::checkbox::{Checkbox, CheckboxState};
use crate::components::dropdown_menu::{DropdownMenuCheckboxItem, dropdown_menu};
use crate::components::input::Input;
use crate::components::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};
use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::{Space, column, container, row, text as iced_text};
use crate::iced_compat::{Border, Element, Length};

use crate::fonts::iced_font;

use super::DataTable;

pub(super) fn build_data_table<'a, TRow, Message>(
    dt: DataTable<'a, TRow, Message>,
) -> Element<'a, Message>
where
    TRow: Clone + 'static,
    Message: Clone + 'a,
{
    let DataTable {
        theme,
        state,
        sortable,
        filterable,
        paginated,
        selectable,
        column_visibility,
        page_sizes: _page_sizes,
        empty_message,
        filter_placeholder,
        filter_value,
        filter_input_size,
        filter_input_radius,
        filter_input_color,
        sort_button_variant,
        sort_button_size,
        sort_button_radius,
        sort_button_color,
        columns_button_variant,
        columns_button_size,
        columns_button_radius,
        columns_button_color,
        pagination_button_variant,
        pagination_button_size,
        pagination_button_radius,
        pagination_button_color,
        checkbox_variant,
        checkbox_size,
        table_width,
        table_min_width,
        on_sort,
        on_filter: _on_filter,
        on_global_filter,
        on_page,
        on_page_size: _on_page_size,
        on_select,
        on_select_all,
        on_column_visibility,
    } = dt;

    let palette = theme.palette;

    let mut sections: Vec<Element<'a, Message>> = Vec::new();

    // ── Toolbar: filter input + column visibility ─────────────────────────
    if filterable || column_visibility {
        let mut toolbar = row![]
            .spacing(8)
            .align_y(Vertical::Center)
            .width(Length::Fill);

        if filterable {
            let mut filter = Input::new(theme)
                .value(filter_value.clone())
                .placeholder(filter_placeholder.clone())
                .size(filter_input_size)
                .width(Length::Fixed(250.0));
            if let Some(radius) = filter_input_radius {
                filter = filter.radius(radius);
            }
            if let Some(color) = filter_input_color {
                filter = filter.color(color);
            }
            if let Some(callback) = on_global_filter {
                filter = filter.on_input(callback);
            }
            let filter_input: Element<'a, Message> = filter.into();
            toolbar = toolbar.push(filter_input);
        }

        if column_visibility {
            let columns_trigger = apply_button_style(
                Button::text("Columns", theme),
                columns_button_variant,
                columns_button_size,
                columns_button_radius,
                columns_button_color,
            );
            let mut columns_menu = dropdown_menu("Columns", theme)
                .trigger(columns_trigger)
                .width(180.0);
            for column in &state.columns {
                let visible = state.is_column_visible(column.id);
                let on_toggle = on_column_visibility
                    .as_ref()
                    .map(|callback| callback(column.id, !visible));
                columns_menu = columns_menu.checkbox_item(
                    DropdownMenuCheckboxItem::new(column.header.clone(), visible)
                        .on_toggle_maybe(on_toggle),
                );
            }
            toolbar = toolbar.push(Space::new().width(Length::Fill));
            toolbar = toolbar.push(columns_menu);
        }

        sections.push(container(toolbar).width(Length::Fill).padding(4).into());
    }

    // ── Table ──────────────────────────────────────────────────────────────
    let visible_cols: Vec<_> = state
        .columns
        .iter()
        .filter(|col| state.is_column_visible(col.id))
        .collect();

    let view = visible_view(state);
    let visible_ids: Vec<_> = view
        .iter()
        .filter_map(|row| match row {
            RenderRow::Data { id, .. } => Some(*id),
            _ => None,
        })
        .collect();

    // Header row
    let mut header_row = TableRow::new(theme);

    // Selection checkbox header
    if selectable {
        let all_selected = !visible_ids.is_empty()
            && visible_ids
                .iter()
                .all(|row_id| state.selection.contains(row_id));
        let some_selected = visible_ids
            .iter()
            .any(|row_id| state.selection.contains(row_id))
            && !all_selected;

        let checkbox_state = if all_selected {
            CheckboxState::Checked
        } else if some_selected {
            CheckboxState::Indeterminate
        } else {
            CheckboxState::Unchecked
        };

        let mut cb = Checkbox::new(theme)
            .variant(checkbox_variant)
            .size(checkbox_size)
            .state(checkbox_state);
        if let Some(callback) = on_select_all {
            cb = cb
                .on_change(move |new_state| callback(matches!(new_state, CheckboxState::Checked)));
        }

        header_row = header_row.head(TableHead::new(cb, theme).width(Length::Fixed(40.0)));
    }

    for col in &visible_cols {
        let header_text = col.header.clone();
        let col_id = col.id;
        let alignment = table_alignment(col.alignment);

        let sort_state = state
            .sort
            .iter()
            .find(|s| s.column == col_id)
            .map(|s| s.direction);

        let header_content: Element<'a, Message> = if sortable && col.sortable {
            let arrow = match sort_state {
                Some(SortDirection::Asc) => " \u{2191}",
                Some(SortDirection::Desc) => " \u{2193}",
                None => " \u{2195}",
            };

            let label = format!("{header_text}{arrow}");

            if let Some(ref callback) = on_sort {
                apply_button_style(
                    Button::text(label, theme),
                    sort_button_variant,
                    sort_button_size,
                    sort_button_radius,
                    sort_button_color,
                )
                .on_press(callback(col_id, SortAction::Replace))
                .into()
            } else {
                iced_text(label)
                    .size(14)
                    .font(iced_font(theme.font_pack().sans))
                    .into()
            }
        } else {
            iced_text(header_text)
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .into()
        };

        header_row = header_row.head(TableHead::new(header_content, theme).align_x(alignment));
    }

    let header = TableHeader::new(theme).push(header_row);

    // Body
    let mut body = TableBody::new(theme);

    // Wrap on_select in Rc for sharing across row closures.
    let on_select_rc: Option<Rc<dyn Fn(chorale_core::RowId, bool) -> Message + 'a>> =
        on_select.map(|f| Rc::from(f) as Rc<dyn Fn(chorale_core::RowId, bool) -> Message + 'a>);

    if view.is_empty() {
        let empty_row = TableRow::new(theme).cell(
            TableCell::new(
                container(
                    iced_text(empty_message.clone())
                        .size(14)
                        .font(iced_font(theme.font_pack().sans))
                        .color(palette.muted_foreground),
                )
                .width(Length::Fill)
                .align_x(Horizontal::Center)
                .padding(24),
                theme,
            )
            .span(visible_cols.len() + usize::from(selectable)),
        );
        body = body.push(empty_row);
    } else {
        for render_row in &view {
            let (row_id, row_data) = match render_row {
                RenderRow::Data { id, row } => (id, row),
                _ => continue,
            };
            let mut table_row = TableRow::new(theme);

            // Selection checkbox
            if selectable {
                let is_selected = state.selection.contains(row_id);
                let cb_state = if is_selected {
                    CheckboxState::Checked
                } else {
                    CheckboxState::Unchecked
                };

                let rid = *row_id;
                let mut cb = Checkbox::new(theme)
                    .variant(checkbox_variant)
                    .size(checkbox_size)
                    .state(cb_state);
                if let Some(ref callback) = on_select_rc {
                    let cb_rc = Rc::clone(callback);
                    cb = cb.on_change(move |new_state| {
                        cb_rc(rid, matches!(new_state, CheckboxState::Checked))
                    });
                }
                table_row = table_row.cell(TableCell::new(cb, theme).width(Length::Fixed(40.0)));
            }

            // Data cells
            for col in &visible_cols {
                let value = (col.accessor)(row_data);
                let cell_text = format_cell_value(&value);

                table_row = table_row.cell(
                    TableCell::text(cell_text, theme).align_x(table_alignment(col.alignment)),
                );
            }

            body = body.push(table_row);
        }
    }

    let mut column_widths = Vec::with_capacity(visible_cols.len() + usize::from(selectable));
    if selectable {
        column_widths.push(Length::Fixed(40.0));
    }
    column_widths.extend(std::iter::repeat_n(Length::Fill, visible_cols.len()));

    let table = Table::new(theme)
        .width(table_width)
        .min_width(table_min_width)
        .column_widths(column_widths)
        .header(header)
        .body(body);
    let table_radius = theme.radius_scale().md_px;
    sections.push(
        container(table)
            .width(Length::Fill)
            .style(move |_| container::Style {
                border: Border {
                    color: palette.border,
                    width: 1.0,
                    radius: table_radius.into(),
                },
                ..container::Style::default()
            })
            .into(),
    );

    // ── Footer: selection count + pagination ───────────────────────────────
    if paginated || selectable {
        let mut footer_children: Vec<Element<'a, Message>> = Vec::new();

        // Selection count
        if selectable {
            let selected = filtered_sorted_pairs(state)
                .iter()
                .filter(|(row_id, _)| state.selection.contains(row_id))
                .count();
            let total = state.filtered_row_count();
            let count_text = format!("{selected} of {total} row(s) selected.");
            footer_children.push(
                iced_text(count_text)
                    .size(13)
                    .font(iced_font(theme.font_pack().sans))
                    .color(palette.muted_foreground)
                    .width(Length::Fill)
                    .into(),
            );
        }

        // Pagination buttons
        if paginated {
            let current_page = state.page;
            let total_pages = state.total_pages();

            let mut pagination_row: Vec<Element<'a, Message>> = Vec::new();

            // Prev button
            let can_prev = current_page > 0;
            let mut prev_btn = apply_button_style(
                Button::text("Previous", theme),
                pagination_button_variant,
                pagination_button_size,
                pagination_button_radius,
                pagination_button_color,
            )
            .disabled(!can_prev || on_page.is_none());
            if can_prev && let Some(ref callback) = on_page {
                prev_btn = prev_btn.on_press(callback(current_page.saturating_sub(1)));
            }
            pagination_row.push(prev_btn.into());

            // Next button
            let can_next = current_page + 1 < total_pages;
            let mut next_btn = apply_button_style(
                Button::text("Next", theme),
                pagination_button_variant,
                pagination_button_size,
                pagination_button_radius,
                pagination_button_color,
            )
            .disabled(!can_next || on_page.is_none());
            if can_next && let Some(ref callback) = on_page {
                next_btn = next_btn.on_press(callback(current_page + 1));
            }
            pagination_row.push(next_btn.into());

            footer_children.push(
                row(pagination_row)
                    .spacing(8)
                    .align_y(Vertical::Center)
                    .into(),
            );
        }

        sections.push(
            row(footer_children)
                .spacing(16)
                .align_y(Vertical::Center)
                .width(Length::Fill)
                .padding(8)
                .into(),
        );
    }

    column(sections).spacing(8).width(Length::Fill).into()
}

fn table_alignment(alignment: ColumnAlignment) -> Horizontal {
    match alignment {
        ColumnAlignment::Left => Horizontal::Left,
        ColumnAlignment::Center => Horizontal::Center,
        ColumnAlignment::Right => Horizontal::Right,
        _ => Horizontal::Left,
    }
}

fn apply_button_style<'a, Message>(
    mut button: Button<'a, Message>,
    variant: ButtonVariant,
    size: ButtonSize,
    radius: Option<ButtonRadius>,
    color: Option<AccentColor>,
) -> Button<'a, Message> {
    button = button.variant(variant).size(size);
    if let Some(radius) = radius {
        button = button.radius(radius);
    }
    if let Some(color) = color {
        button = button.color(color);
    }
    button
}

fn format_cell_value(value: &CellValue) -> String {
    match value {
        CellValue::Text(s) => s.clone(),
        CellValue::Integer(n) => n.to_string(),
        CellValue::Float(f) => format!("{f:.2}"),
        CellValue::Boolean(b) => if *b { "Yes" } else { "No" }.to_owned(),
        CellValue::Date(d) => d.to_string(),
        CellValue::DateTime(dt) => dt.to_string(),
        CellValue::Empty => String::new(),
        _ => String::new(),
    }
}
