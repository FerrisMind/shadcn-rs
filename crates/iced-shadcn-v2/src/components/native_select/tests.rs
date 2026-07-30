//! Behavioral tests for the native-select component.

use std::collections::hash_map::DefaultHasher;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use crate::iced_compat::{Element, Length, Pixels};
use crate::theme::Theme;
use shadcn_common::StyleId;

use super::render::{
    flatten_items, next_matching_index, next_selectable_index, selected_index, typeahead_match,
};
use super::style;
use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Message {
    Changed(String),
    Opened,
    Closed,
}

#[test]
fn builder_preserves_the_controlled_native_select_contract() {
    let theme = Theme::light();
    let select: NativeSelect<'_, String, Message> = NativeSelect::with_options(
        &theme,
        [
            NativeSelectOption::new("apple".to_owned(), "Apple"),
            NativeSelectOption::new("banana".to_owned(), "Banana").disabled(true),
        ],
        Some("apple".to_owned()),
    )
    .placeholder("Choose a fruit")
    .size(NativeSelectSize::Sm)
    .radius(NativeSelectRadius::Large)
    .width(Length::Fixed(240.0))
    .menu_height(Length::Fixed(180.0))
    .text_size(Pixels::from(13.0))
    .id("fruit-select")
    .disabled(true)
    .invalid(true)
    .on_select(Message::Changed)
    .on_open(Message::Opened)
    .on_close(Message::Closed)
    .style_override(|style, _| style);

    assert_eq!(select.len(), 2);
    assert!(!select.is_empty());
    assert_eq!(select.selected.as_deref(), Some("apple"));
    assert_eq!(select.placeholder.as_deref(), Some("Choose a fruit"));
    assert_eq!(select.size, NativeSelectSize::Sm);
    assert_eq!(select.radius, Some(NativeSelectRadius::Large));
    assert_eq!(select.width, Length::Fixed(240.0));
    assert_eq!(select.menu_height, Length::Fixed(180.0));
    assert_eq!(select.text_size, Some(Pixels::from(13.0)));
    assert!(select.id.is_some());
    assert!(select.disabled);
    assert!(select.invalid);
    assert!(select.on_select.is_some());
    assert!(select.on_open.is_some());
    assert!(select.on_close.is_some());
    assert!(select.style_override.is_some());
    assert!(std::ptr::eq(select.theme, &theme));

    let callback = select.on_select.as_ref().expect("selection callback");
    assert_eq!(
        callback("pear".to_owned()),
        Message::Changed("pear".to_owned())
    );
}

#[test]
fn plain_values_use_their_display_text_as_labels() {
    let theme = Theme::light();
    let select = NativeSelect::<_, Message>::new(&theme, [1_u32, 2_u32, 3_u32], Some(2_u32));
    let entries = flatten_items(select.items);

    assert!(matches!(entries[1], super::render::Entry::Option { ref label, .. } if label == "2"));
}

#[test]
fn groups_flatten_with_headings_indentation_and_inherited_disabled_state() {
    let items = [
        NativeSelectItem::option(NativeSelectOption::new("top", "Top")),
        NativeSelectItem::opt_group(
            NativeSelectOptGroup::new("Disabled group")
                .disabled(true)
                .push(NativeSelectOption::new("one", "One")),
        ),
        NativeSelectItem::opt_group(
            NativeSelectOptGroup::new("Enabled group")
                .push(NativeSelectOption::new("two", "Two"))
                .push(NativeSelectOption::new("three", "Three").disabled(true)),
        ),
    ];
    let entries = flatten_items(items.into_iter().collect());

    assert_eq!(entries.len(), 6);
    assert!(matches!(
        entries[0],
        super::render::Entry::Option {
            indented: false,
            ..
        }
    ));
    assert!(
        matches!(entries[1], super::render::Entry::Group { ref label } if label == "Disabled group")
    );
    assert!(matches!(
        entries[2],
        super::render::Entry::Option {
            disabled: true,
            indented: true,
            ..
        }
    ));
    assert!(
        matches!(entries[3], super::render::Entry::Group { ref label } if label == "Enabled group")
    );
    assert!(matches!(
        entries[4],
        super::render::Entry::Option {
            disabled: false,
            indented: true,
            ..
        }
    ));
    assert!(matches!(
        entries[5],
        super::render::Entry::Option {
            disabled: true,
            indented: true,
            ..
        }
    ));
}

#[test]
fn selection_navigation_skips_groups_and_disabled_options() {
    let entries = flatten_items(vec![
        NativeSelectItem::opt_group(
            NativeSelectOptGroup::new("Group")
                .push(NativeSelectOption::new("disabled", "Disabled").disabled(true))
                .push(NativeSelectOption::new("enabled", "Enabled")),
        ),
        NativeSelectItem::option(NativeSelectOption::new("last", "Last")),
    ]);

    assert_eq!(selected_index(&entries, Some(&"enabled")), Some(2));
    assert_eq!(next_selectable_index(&entries, None, true), Some(2));
    assert_eq!(next_selectable_index(&entries, Some(2), true), Some(3));
    assert_eq!(next_selectable_index(&entries, Some(3), false), Some(2));
    assert_eq!(next_selectable_index(&entries, Some(2), false), None);
}

#[test]
fn typeahead_navigation_matches_labels_and_wraps() {
    let entries = flatten_items(vec![
        NativeSelectItem::option(NativeSelectOption::new("one", "One")),
        NativeSelectItem::option(NativeSelectOption::new("ops", "Operations")),
        NativeSelectItem::option(NativeSelectOption::new("other", "Other")),
    ]);

    assert_eq!(next_matching_index(&entries, None, "op"), Some(1));
    assert_eq!(next_matching_index(&entries, Some(1), "o"), Some(2));
    assert_eq!(next_matching_index(&entries, Some(2), "o"), Some(0));
    assert_eq!(next_matching_index(&entries, None, "missing"), None);
}

#[test]
fn typeahead_keeps_a_matching_multi_character_prefix() {
    let entries = flatten_items(vec![
        NativeSelectItem::option(NativeSelectOption::new("one", "One")),
        NativeSelectItem::option(NativeSelectOption::new("ops", "Operations")),
        NativeSelectItem::option(NativeSelectOption::new("other", "Other")),
    ]);

    let (first, buffer) = typeahead_match(&entries, None, "", "o");
    assert_eq!(first, Some(0));
    assert_eq!(buffer, "o");

    let (second, buffer) = typeahead_match(&entries, first, &buffer, "p");
    assert_eq!(second, Some(1));
    assert_eq!(buffer, "op");
}

#[test]
fn option_and_group_builders_expose_source_flags() {
    let option = NativeSelectOption::new("value", "Visible").disabled(true);
    assert_eq!(option.value(), &"value");
    assert_eq!(option.label(), "Visible");
    assert!(option.is_disabled());
    assert_eq!(format!("{option}"), "Visible");

    let group =
        NativeSelectOptGroup::with_options("Fruits", [NativeSelectOption::new("apple", "Apple")])
            .disabled(true);
    assert_eq!(group.label(), "Fruits");
    assert_eq!(group.options().len(), 1);
    assert!(group.is_disabled());
}

#[test]
fn style_recipes_match_native_select_geometry_across_packs() {
    for style_id in [
        StyleId::Vega,
        StyleId::Nova,
        StyleId::Maia,
        StyleId::Lyra,
        StyleId::Mira,
        StyleId::Luma,
        StyleId::Sera,
        StyleId::Rhea,
    ] {
        let theme = Theme::light().with_style(style_id);
        let padding = style::padding(&theme, NativeSelectSize::Default);
        let expected_vertical = match style_id {
            StyleId::Mira => 2.0,
            StyleId::Sera => 8.0,
            _ => 4.0,
        };
        assert_eq!(padding.top, expected_vertical);
        assert_eq!(padding.bottom, expected_vertical);
        assert!(style::radius_px(&theme, None).is_finite());
        assert!(style::icon_size(&theme).is_finite());
        assert!(style::icon_right(&theme).is_finite());
    }

    let maia = Theme::light().with_style(StyleId::Maia);
    assert_eq!(style::icon_right(&maia), 14.0);

    let mira = Theme::light().with_style(StyleId::Mira);
    assert_eq!(style::text_size_for(&mira, NativeSelectSize::Sm), 10.0);
    assert_eq!(style::icon_size_for(&mira, NativeSelectSize::Sm), 12.0);
    assert_eq!(style::focus_ring_px(&mira), 2.0);

    let lyra = Theme::light().with_style(StyleId::Lyra);
    assert_eq!(style::focus_ring_px(&lyra), 1.0);
    assert_eq!(
        style::line_height_px(&lyra, NativeSelectSize::Default, 12.0),
        16.0
    );

    let sera = Theme::light().with_style(StyleId::Sera);
    assert!(style::uses_bottom_border(&sera));
    assert_eq!(style::focus_ring_px(&sera), 0.0);

    let mira = Theme::light().with_style(StyleId::Mira);
    assert_eq!(
        style::line_height_px(&mira, NativeSelectSize::Default, 12.0),
        19.5
    );
    assert_eq!(
        style::line_height_px(&mira, NativeSelectSize::Sm, 10.0),
        19.5
    );
}

#[test]
fn invalid_and_disabled_styles_preserve_source_precedence() {
    let light = Theme::light();
    let invalid = style::resolve(
        &light,
        None,
        true,
        false,
        crate::iced_compat::widget::pick_list::Status::Active,
    );
    assert_eq!(
        invalid.border.color,
        light.semantic_color(crate::SemanticColor::Destructive)
    );

    let dark = Theme::dark();
    let dark_invalid = style::resolve(
        &dark,
        None,
        true,
        false,
        crate::iced_compat::widget::pick_list::Status::Active,
    );
    let destructive = dark.semantic_color(crate::SemanticColor::Destructive);
    assert!((dark_invalid.border.color.a - destructive.a * 0.5).abs() < f32::EPSILON);

    let active = style::resolve(
        &light,
        None,
        false,
        false,
        crate::iced_compat::widget::pick_list::Status::Active,
    );
    let disabled = style::resolve(
        &light,
        None,
        false,
        true,
        crate::iced_compat::widget::pick_list::Status::Active,
    );
    assert!((disabled.text_color.a - active.text_color.a * 0.5).abs() < f32::EPSILON);
    assert!((disabled.placeholder_color.a - active.placeholder_color.a * 0.5).abs() < f32::EPSILON);
}

#[test]
fn builders_convert_to_elements_and_debug_without_message_debug() {
    #[derive(Clone)]
    struct NoDebugMessage;

    let theme = Theme::light();
    let select = NativeSelect::<String, NoDebugMessage>::with_options(
        &theme,
        [NativeSelectOption::new("one".to_owned(), "One")],
        None,
    );
    let debug = format!("{select:?}");
    assert!(debug.contains("NativeSelect"));

    let _: Element<'_, NoDebugMessage> = NativeSelect::new(&theme, ["one"], None).into();
}

#[test]
fn configuration_types_are_ordered_hashable_and_display_bound_is_minimal() {
    fn hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    let _ = hash(&NativeSelectSize::Default);
    let _ = hash(&NativeSelectRadius::Full);
    assert!(NativeSelectSize::Sm < NativeSelectSize::Default);
    assert!(NativeSelectRadius::None < NativeSelectRadius::Full);

    fn display<T: Display>(value: T) -> String {
        value.to_string()
    }
    assert_eq!(display(NativeSelectOption::new((), "Unit")), "Unit");
}
