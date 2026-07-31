//! Behavioral checks for the data-table builder.

use chorale_core::{CellValue, ColumnDef, ColumnId, RowId, TableState};

use super::*;
use crate::theme::Theme;

#[derive(Clone)]
struct MockRow {
    name: String,
    amount: f64,
}

fn mock_columns() -> Vec<ColumnDef<MockRow>> {
    vec![
        ColumnDef::new(ColumnId("name"), "Name", |row: &MockRow| {
            CellValue::Text(row.name.clone())
        }),
        ColumnDef::new(ColumnId("amount"), "Amount", |row: &MockRow| {
            CellValue::Float(row.amount)
        })
        .sortable(),
    ]
}

fn mock_state() -> TableState<MockRow> {
    let rows = vec![
        (
            RowId::new(),
            MockRow {
                name: "Alice".into(),
                amount: 100.0,
            },
        ),
        (
            RowId::new(),
            MockRow {
                name: "Bob".into(),
                amount: 200.0,
            },
        ),
    ];
    TableState::new(rows, mock_columns())
}

#[test]
fn defaults_are_all_enabled() {
    let theme = Theme::light();
    let state = mock_state();
    let dt: DataTable<'_, MockRow, ()> = DataTable::new(&theme, &state);

    assert!(dt.sortable);
    assert!(dt.filterable);
    assert!(dt.paginated);
    assert!(dt.selectable);
    assert!(dt.column_visibility);
}

#[test]
fn feature_toggles_work() {
    let theme = Theme::light();
    let state = mock_state();
    let dt: DataTable<'_, MockRow, ()> = DataTable::new(&theme, &state)
        .sortable(false)
        .filterable(false)
        .paginated(false)
        .selectable(false)
        .column_visibility(false);

    assert!(!dt.sortable);
    assert!(!dt.filterable);
    assert!(!dt.paginated);
    assert!(!dt.selectable);
    assert!(!dt.column_visibility);
}

#[test]
fn converts_to_element() {
    let theme = Theme::light();
    let state = mock_state();
    let _: crate::iced_compat::Element<'_, ()> = DataTable::new(&theme, &state)
        .on_sort(|_col, _action| ())
        .on_page(|_page| ())
        .into();
}

#[test]
fn convenience_helper_works() {
    let theme = Theme::light();
    let state = mock_state();
    let _: DataTable<'_, MockRow, ()> = data_table(&theme, &state);
}
