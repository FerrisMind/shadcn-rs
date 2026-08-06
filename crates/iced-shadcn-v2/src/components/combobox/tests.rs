//! Unit tests for the combobox builder and composed selection helpers.

use crate::iced_compat::{Element, Length};
use shadcn_common::SelectMode;

use super::render::{mark_selection, selected_text};
use super::{Combobox, ComboboxGroup, ComboboxItem, ComboboxSelection};
use crate::theme::Theme;

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Query(String),
    Selection(ComboboxSelection<&'static str>),
    Open(bool),
    Picked(&'static str),
}

fn frameworks(theme: &Theme) -> Combobox<'_, &'static str, Message> {
    Combobox::new(theme)
        .item(("svelte", "Svelte"))
        .item(ComboboxItem::new("react", "React").disabled(true))
        .group(ComboboxGroup::new("More").item(("vue", "Vue")))
}

#[test]
fn defaults_match_composed_combobox() {
    let theme = Theme::light();
    let combobox = Combobox::<&str, Message>::new(&theme);

    assert_eq!(combobox.placeholder, "Select an option...");
    assert_eq!(combobox.search_placeholder, "Search...");
    assert_eq!(combobox.trigger_width, Length::Shrink);
    assert_eq!(combobox.command_max_height, 288.0);
    assert!(combobox.command_should_filter);
    assert!(combobox.command_show_search_icon);
    assert!(!combobox.command_show_border);
    assert!(!combobox.command_show_shadow);
    assert!(!combobox.deselectable);
    assert!(combobox.empty.is_some());
}

#[test]
fn builders_cover_selection_query_and_composed_style_knobs() {
    let theme = Theme::light();
    let combobox = frameworks(&theme)
        .width(Length::Fixed(200.0))
        .content_width(220.0)
        .command_width(Length::Fill)
        .max_height(240.0)
        .query("sv")
        .selection(ComboboxSelection::single(Some("svelte")))
        .select_type(SelectMode::Single)
        .invalid(true)
        .disabled(false)
        .open(true)
        .on_query_change(Message::Query)
        .on_selection_change(Message::Selection)
        .on_open_change(Message::Open)
        .on_select(Message::Picked);

    assert_eq!(combobox.trigger_width, Length::Fixed(200.0));
    assert_eq!(combobox.popover_width, Some(220.0));
    assert_eq!(combobox.command_max_height, 240.0);
    assert_eq!(combobox.query, "sv");
    assert_eq!(
        combobox.selection,
        ComboboxSelection::Single(Some("svelte"))
    );
    assert!(combobox.invalid);
    assert!(combobox.open == Some(true));
    assert!(combobox.on_query_change.is_some());
    assert!(combobox.on_selection_change.is_some());
    assert!(combobox.on_select.is_some());
    assert!(combobox.on_open_change.is_some());
}

#[test]
fn selected_labels_walk_nested_groups_and_multiple_values() {
    let rows = vec![
        super::CommandEntry::Item(ComboboxItem::new("one", "One")),
        super::CommandEntry::Group(
            ComboboxGroup::new("More")
                .item(("two", "Two"))
                .item(("three", "Three")),
        ),
    ];

    assert_eq!(
        selected_text(&rows, &ComboboxSelection::single(Some("two")), "Choose"),
        "Two"
    );
    assert_eq!(
        selected_text(
            &rows,
            &ComboboxSelection::multiple(["one", "three"]),
            "Choose",
        ),
        "2 selected"
    );
    assert_eq!(
        selected_text(&rows, &ComboboxSelection::single(Some("missing")), "Choose"),
        "Choose"
    );
}

#[test]
fn marking_selection_adds_transparent_leading_check_slots() {
    let mut rows = vec![super::CommandEntry::Group(
        ComboboxGroup::new("Frameworks")
            .item(("svelte", "Svelte"))
            .item(("react", "React")),
    )];

    mark_selection(&mut rows, &ComboboxSelection::single(Some("react")));

    let super::CommandEntry::Group(group) = &rows[0] else {
        panic!("expected group")
    };
    let super::CommandEntry::Item(selected) = &group.entries[1] else {
        panic!("expected selected item")
    };
    let super::CommandEntry::Item(unselected) = &group.entries[0] else {
        panic!("expected unselected item")
    };

    assert!(selected.checked);
    assert!(selected.leading_check);
    assert!(!unselected.checked);
    assert!(unselected.leading_check);
}

#[test]
fn converts_to_an_iced_element() {
    let theme = Theme::light();
    let _: Element<'_, Message> = frameworks(&theme)
        .selected("svelte")
        .on_select(Message::Picked)
        .into();
}
