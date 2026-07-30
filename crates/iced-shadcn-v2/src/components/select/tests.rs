//! Behavioral tests for the select component.

use crate::iced_compat::{Element, Length};
use shadcn_common::{SelectMode, StyleId};

use super::style;
use super::types::Row;
use super::*;
use crate::theme::Theme;

#[derive(Clone, Debug, PartialEq)]
enum Message {
    Picked(&'static str),
    Changed(SelectSelection<&'static str>),
    Opened,
    Closed,
}

fn fruits(theme: &Theme) -> Select<'_, &'static str, Message> {
    Select::new(theme)
        .placeholder("Select a fruit")
        .item(("apple", "Apple"))
        .item(SelectItem::new("grapes", "Grapes").disabled(true))
        .item(("banana", "Banana"))
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let select = fruits(&theme)
        .size(SelectSize::Sm)
        .radius(SelectRadius::Full)
        .width(Length::Fixed(200.0))
        .text_size(13.0)
        .selected("banana")
        .disabled(true)
        .invalid(true)
        .deselectable(false)
        .on_select(Message::Picked)
        .on_selection_change(Message::Changed)
        .on_open(Message::Opened)
        .on_close(Message::Closed)
        .style_override(|style, _| style);

    assert_eq!(select.rows.len(), 3);
    assert_eq!(select.placeholder.as_deref(), Some("Select a fruit"));
    assert_eq!(select.size, SelectSize::Sm);
    assert_eq!(select.radius, Some(SelectRadius::Full));
    assert_eq!(select.width, Length::Fixed(200.0));
    assert_eq!(
        select.max_height,
        shadcn_common::SELECT_CONTENT_MAX_HEIGHT_PX
    );
    assert_eq!(select.text_size, Some(13.0));
    assert_eq!(select.selection, SelectSelection::Single(Some("banana")));
    assert!(select.disabled);
    assert!(select.invalid);
    assert!(!select.deselectable);
    assert!(select.on_select.is_some());
    assert!(select.on_selection_change.is_some());
    assert_eq!(select.on_open, Some(Message::Opened));
    assert_eq!(select.on_close, Some(Message::Closed));
    assert!(select.style_override.is_some());
    assert!(std::ptr::eq(select.theme, &theme));

    let callback = select.on_select.as_ref().expect("on_select was set");
    assert_eq!(callback("apple"), Message::Picked("apple"));
}

#[test]
fn builder_and_helper_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = fruits(&theme).on_select(Message::Picked).into();

    let _: Element<'_, Message> = select("Select a fruit", &theme)
        .items([("apple", "Apple"), ("banana", "Banana")])
        .on_select(Message::Picked)
        .into();
}

#[test]
fn groups_labels_and_separators_flatten() {
    let theme = Theme::light();
    let select: Select<'_, &str, Message> = Select::new(&theme)
        .group(
            SelectGroup::new("Fruits")
                .item(("apple", "Apple"))
                .items([("banana", "Banana")]),
        )
        .separator()
        .label("Extras")
        .item(("kiwi", "Kiwi"));

    assert!(matches!(&select.rows[0], Row::Label { text } if text == "Fruits"));
    assert!(matches!(&select.rows[1], Row::Option { label, .. } if label == "Apple"));
    assert!(matches!(&select.rows[3], Row::Separator));
    assert!(matches!(&select.rows[4], Row::Label { text } if text == "Extras"));
    assert_eq!(select.rows.len(), 6);
}

#[test]
fn disabled_item_is_not_selectable() {
    let theme = Theme::light();
    let select = fruits(&theme);

    assert!(select.rows[0].is_selectable());
    assert!(!select.rows[1].is_selectable());
    assert!(select.rows[2].is_selectable());
}

#[test]
fn selection_toggle_matches_bits_ui() {
    let single = SelectSelection::Single(Some("apple"));
    assert_eq!(
        single.clone().toggled(SelectMode::Single, &"apple", true),
        SelectSelection::Single(None)
    );
    assert_eq!(
        single.toggled(SelectMode::Single, &"banana", true),
        SelectSelection::Single(Some("banana"))
    );

    let multiple = SelectSelection::multiple(["apple", "banana"]);
    assert_eq!(
        multiple.toggled(SelectMode::Multiple, &"apple", true),
        SelectSelection::Multiple(vec!["banana"])
    );
}

#[test]
fn default_width_shrinks_like_w_fit() {
    let theme = Theme::light();
    let select: Select<'_, &str, Message> = Select::new(&theme);

    assert_eq!(select.width, Length::Shrink);
}

#[test]
fn control_heights_match_the_pack_ladder() {
    let vega = Theme::light();
    assert_eq!(SelectSize::Sm.control_height(&vega), 32.0);
    assert_eq!(SelectSize::Default.control_height(&vega), 36.0);

    let mira = Theme::light().with_style(StyleId::Mira);
    assert_eq!(SelectSize::Default.control_height(&mira), 28.0);

    let sera = Theme::light().with_style(StyleId::Sera);
    assert_eq!(SelectSize::Default.control_height(&sera), 40.0);
}

#[test]
fn trigger_style_marks_invalid_and_opened() {
    let theme = Theme::light();
    let active = style::resolve_trigger_style(
        &theme,
        SelectSize::Default,
        None,
        false,
        false,
        SelectStatus::Active,
    );
    let opened = style::resolve_trigger_style(
        &theme,
        SelectSize::Default,
        None,
        false,
        false,
        SelectStatus::Opened,
    );
    let invalid = style::resolve_trigger_style(
        &theme,
        SelectSize::Default,
        None,
        true,
        false,
        SelectStatus::Opened,
    );

    assert_ne!(active.border_color, opened.border_color);
    assert_ne!(opened.border_color, invalid.border_color);
}

#[test]
fn content_style_uses_popover_surface() {
    let theme = Theme::light();
    let content = style::resolve_content_style(&theme);

    assert_eq!(content.background, theme.palette.popover);
    assert_eq!(content.text_color, theme.palette.popover_foreground);
    assert!(content.radius >= 0.0);
}

#[test]
fn item_and_group_accessors_report_configuration() {
    let item = SelectItem::new("apple", "Apple").disabled(true);
    assert_eq!(item.value(), &"apple");
    assert_eq!(item.label(), "Apple");
    assert!(item.is_disabled());

    let group = SelectGroup::<&str>::new("Fruits");
    assert_eq!(group.label(), Some("Fruits"));
    assert!(group.is_empty());
    assert_eq!(group.item(("apple", "Apple")).len(), 1);
}

#[test]
fn max_height_overrides_default_content_cap() {
    let theme = Theme::light();
    let select = fruits(&theme).max_height(300.0);
    assert_eq!(select.max_height, 300.0);
}
