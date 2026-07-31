//! Rendering for the data-table: composes Table + Input + Pagination chrome.

use std::rc::Rc;

use chorale_core::{CellValue, RenderRow, SortAction, SortDirection, visible_view};

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::checkbox::{Checkbox, CheckboxState};
use crate::components::input::Input;
use crate::components::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};
use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::{column, container, row, text as iced_text};
use crate::iced_compat::{Element, Length};

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
        column_visibility: _column_visibility,
        page_sizes: _page_sizes,
        empty_message,
        filter_placeholder,
        on_sort,
        on_filter: _on_filter,
        on_global_filter,
        on_page,
        on_page_size: _on_page_size,
        on_select,
        on_select_all,
        on_column_visibility: _on_column_visibility,
    } = dt;

    let palette = theme.palette;

    let mut sections: Vec<Element<'a, Message>> = Vec::new();

    // ── Toolbar: filter input ──────────────────────────────────────────────
    if filterable {
        let filter_input: Element<'a, Message> = if let Some(callback) = on_global_filter {
            Input::new(theme)
                .placeholder(filter_placeholder.clone())
                .width(Length::Fixed(250.0))
                .on_input(callback)
                .into()
        } else {
            Input::new(theme)
                .placeholder(filter_placeholder.clone())
                .width(Length::Fixed(250.0))
                .into()
        };

        sections.push(
            container(filter_input)
                .width(Length::Fill)
                .padding(4)
                .into(),
        );
    }

    // ── Table ──────────────────────────────────────────────────────────────
    let visible_cols: Vec<_> = state
        .columns
        .iter()
        .filter(|col| state.is_column_visible(col.id))
        .collect();

    // Header row
    let mut header_row = TableRow::new(theme);

    // Selection checkbox header
    if selectable {
        let all_selected = !state.selection.is_empty() && state.selection.len() == state.rows.len();
        let some_selected = !state.selection.is_empty() && !all_selected;

        let checkbox_state = if all_selected {
            CheckboxState::Checked
        } else if some_selected {
            CheckboxState::Indeterminate
        } else {
            CheckboxState::Unchecked
        };

        let mut cb = Checkbox::new(theme).state(checkbox_state);
        if let Some(callback) = on_select_all {
            cb = cb
                .on_change(move |new_state| callback(matches!(new_state, CheckboxState::Checked)));
        }

        header_row = header_row.head(TableHead::new(cb, theme).width(Length::Fixed(40.0)));
    }

    for col in &visible_cols {
        let header_text = col.header.clone();
        let col_id = col.id;

        let sort_state = state
            .sort
            .iter()
            .find(|s| s.column == col_id)
            .map(|s| s.direction);

        let header_content: Element<'a, Message> = if sortable && col.sortable {
            let arrow = match sort_state {
                Some(SortDirection::Asc) => " \u{2191}",
                Some(SortDirection::Desc) => " \u{2193}",
                None => "",
            };

            let label = format!("{header_text}{arrow}");

            if let Some(ref callback) = on_sort {
                let cb = callback;
                Button::text(label, theme)
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Sm)
                    .on_press(cb(col_id, SortAction::Replace))
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

        header_row = header_row.head(TableHead::new(header_content, theme));
    }

    let header = TableHeader::new(theme).push(header_row);

    // Body
    let view = visible_view(state);
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
                let mut cb = Checkbox::new(theme).state(cb_state);
                if let Some(ref callback) = on_select_rc {
                    let cb_rc = Rc::clone(callback);
                    cb = cb.on_change(move |new_state| {
                        cb_rc(rid, matches!(new_state, CheckboxState::Checked))
                    });
                }
                table_row = table_row.cell(TableCell::new(cb, theme));
            }

            // Data cells
            for col in &visible_cols {
                let value = (col.accessor)(row_data);
                let cell_text = format_cell_value(&value);

                table_row = table_row.cell(TableCell::text(cell_text, theme));
            }

            body = body.push(table_row);
        }
    }

    let table = Table::new(theme).header(header).body(body);
    sections.push(table.into());

    // ── Footer: selection count + pagination ───────────────────────────────
    if paginated || selectable {
        let mut footer_children: Vec<Element<'a, Message>> = Vec::new();

        // Selection count
        if selectable {
            let selected = state.selection.len();
            let total = state.rows.len();
            let count_text = format!("{selected} of {total} row(s) selected.");
            footer_children.push(
                iced_text(count_text)
                    .size(13)
                    .font(iced_font(theme.font_pack().sans))
                    .color(palette.muted_foreground)
                    .into(),
            );
        }

        // Pagination buttons
        if paginated {
            let current_page = state.page;
            let total_pages = state.total_pages();

            let mut pagination_row: Vec<Element<'a, Message>> = Vec::new();

            // Page info
            pagination_row.push(
                iced_text(format!(
                    "Page {} of {}",
                    current_page + 1,
                    total_pages.max(1)
                ))
                .size(13)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.foreground)
                .into(),
            );

            // Prev button
            let can_prev = current_page > 0;
            let mut prev_btn = Button::text("\u{2039}", theme)
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::IconSm);
            if can_prev && let Some(ref callback) = on_page {
                prev_btn = prev_btn.on_press(callback(current_page.saturating_sub(1)));
            }
            pagination_row.push(prev_btn.into());

            // Next button
            let can_next = current_page + 1 < total_pages;
            let mut next_btn = Button::text("\u{203a}", theme)
                .variant(ButtonVariant::Outline)
                .size(ButtonSize::IconSm);
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
