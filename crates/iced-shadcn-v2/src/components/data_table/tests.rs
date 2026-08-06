//! Behavioral checks for the data-table builder.

use chorale_core::{CellValue, ColumnDef, ColumnId, RowId, TableState};
use shadcn_common::AccentColor;

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
fn composed_component_styles_are_configurable() {
    let theme = Theme::light();
    let state = mock_state();
    let dt: DataTable<'_, MockRow, ()> = DataTable::new(&theme, &state)
        .filter_input_size(InputSize::Lg)
        .filter_input_radius(InputRadius::Full)
        .filter_input_color(AccentColor::Blue)
        .sort_button_variant(ButtonVariant::Secondary)
        .sort_button_size(ButtonSize::Lg)
        .sort_button_radius(ButtonRadius::Large)
        .sort_button_color(AccentColor::Rose)
        .columns_button_variant(ButtonVariant::Soft)
        .columns_button_size(ButtonSize::Sm)
        .columns_button_radius(ButtonRadius::Small)
        .columns_button_color(AccentColor::Amber)
        .pagination_button_variant(ButtonVariant::Ghost)
        .pagination_button_size(ButtonSize::Default)
        .pagination_button_radius(ButtonRadius::Medium)
        .pagination_button_color(AccentColor::Emerald)
        .checkbox_variant(CheckboxVariant::Classic)
        .checkbox_size(CheckboxSize::Sm)
        .table_min_width(720.0);

    assert_eq!(dt.filter_input_size, InputSize::Lg);
    assert_eq!(dt.filter_input_radius, Some(InputRadius::Full));
    assert_eq!(dt.filter_input_color, Some(AccentColor::Blue));
    assert_eq!(dt.sort_button_variant, ButtonVariant::Secondary);
    assert_eq!(dt.sort_button_size, ButtonSize::Lg);
    assert_eq!(dt.sort_button_radius, Some(ButtonRadius::Large));
    assert_eq!(dt.sort_button_color, Some(AccentColor::Rose));
    assert_eq!(dt.columns_button_variant, ButtonVariant::Soft);
    assert_eq!(dt.columns_button_size, ButtonSize::Sm);
    assert_eq!(dt.columns_button_radius, Some(ButtonRadius::Small));
    assert_eq!(dt.columns_button_color, Some(AccentColor::Amber));
    assert_eq!(dt.pagination_button_variant, ButtonVariant::Ghost);
    assert_eq!(dt.pagination_button_size, ButtonSize::Default);
    assert_eq!(dt.pagination_button_radius, Some(ButtonRadius::Medium));
    assert_eq!(dt.pagination_button_color, Some(AccentColor::Emerald));
    assert_eq!(dt.checkbox_variant, CheckboxVariant::Classic);
    assert_eq!(dt.checkbox_size, CheckboxSize::Sm);
    assert_eq!(dt.table_min_width, 720.0);
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
