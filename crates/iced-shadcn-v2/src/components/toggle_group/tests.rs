//! Behavioral tests for the toggle-group component.

use crate::iced_compat::Element;
use crate::iced_compat::widget::container;
use crate::{Theme, ToggleGroup, ToggleGroupItem, ToggleGroupSelection, ToggleGroupType};
use shadcn_common::StyleId;

use super::{geometry, style};

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Message {
    Changed(ToggleGroupSelection),
    Single(Option<String>),
    Multiple(Vec<String>),
    Pressed,
}

#[test]
fn defaults_match_the_web_component() {
    let theme = Theme::light();
    let group = ToggleGroup::<Message>::new(&theme);

    assert_eq!(group.selection_type(), ToggleGroupType::Single);
    assert!(group.is_empty());
    assert!(format!("{group:?}").contains("orientation: Horizontal"));
}

#[test]
fn selection_values_are_normalized_and_toggled() {
    let selection = ToggleGroupSelection::multiple(["bold", "italic", "bold"]);
    assert_eq!(
        selection.as_multiple(),
        ["bold".to_owned(), "italic".to_owned()]
    );
    assert!(selection.is_selected("italic"));
    assert!(!selection.is_selected("underline"));

    let removed = selection.toggled(ToggleGroupType::Multiple, "bold");
    assert_eq!(removed.as_multiple(), ["italic".to_owned()]);

    let added = removed.toggled(ToggleGroupType::Multiple, "underline");
    assert_eq!(
        added.as_multiple(),
        ["italic".to_owned(), "underline".to_owned()]
    );

    let single = ToggleGroupSelection::single(Some("bold"));
    assert_eq!(
        single.toggled(ToggleGroupType::Single, "bold"),
        ToggleGroupSelection::Single(None)
    );
}

#[test]
fn builder_supports_single_and_multiple_controlled_values() {
    let theme = Theme::light();
    let single = ToggleGroup::<Message>::new(&theme)
        .value("bold")
        .push(ToggleGroupItem::text("bold", "Bold", &theme));
    assert_eq!(single.selection_type(), ToggleGroupType::Single);

    let optional = ToggleGroup::<Message>::new(&theme).value(Some("bold"));
    assert_eq!(optional.selection_type(), ToggleGroupType::Single);
    let empty = ToggleGroup::<Message>::new(&theme).value(None::<String>);
    assert_eq!(empty.selection_type(), ToggleGroupType::Single);

    let multiple = ToggleGroup::<Message>::new(&theme)
        .values(["bold", "italic"])
        .push(ToggleGroupItem::text("bold", "Bold", &theme))
        .push(ToggleGroupItem::text("italic", "Italic", &theme));
    assert_eq!(multiple.selection_type(), ToggleGroupType::Multiple);
    assert_eq!(multiple.len(), 2);
}

#[test]
fn callbacks_store_the_expected_selection_shape() {
    let theme = Theme::light();
    let group = ToggleGroup::<Message>::new(&theme)
        .on_change(Message::Single)
        .on_change_values(Message::Multiple)
        .on_selection_change(Message::Changed);
    let debug = format!("{group:?}");
    assert!(debug.contains("on_selection_change: true"));
    assert!(debug.contains("on_press: false"));

    let group = ToggleGroup::<Message>::new(&theme).on_press(Message::Pressed);
    assert!(format!("{group:?}").contains("on_press: true"));
}

#[test]
fn items_can_be_rendered_as_elements_and_groups() {
    let theme = Theme::light();
    let _: Element<'_, Message> = ToggleGroupItem::icon("bold", container("B"), &theme).into();
    let _: Element<'_, Message> = ToggleGroup::new(&theme)
        .group_type(ToggleGroupType::Multiple)
        .push(ToggleGroupItem::text("bold", "Bold", &theme))
        .push(ToggleGroupItem::text("italic", "Italic", &theme).disabled(true))
        .into();
}

#[test]
fn spacing_uses_the_style_pack_unit_and_outline_groups_merge_edges() {
    let vega = Theme::light();
    assert_eq!(geometry::spacing_px(&vega, 2.0), 8.0);
    assert_eq!(geometry::spacing_px(&vega, f32::NAN), 0.0);

    assert!(geometry::merged_borders(crate::ToggleVariant::Outline, 0.0));
    assert!(!geometry::merged_borders(
        crate::ToggleVariant::Outline,
        1.0
    ));
    assert!(!geometry::merged_borders(
        crate::ToggleVariant::Default,
        0.0
    ));

    let mira = Theme::light().with_style(StyleId::Mira);
    let outline = style::resolve_group_style(&mira, crate::ToggleVariant::Outline, 0.0, false);
    assert_eq!(outline.border.width, 1.0);
    let spaced = style::resolve_group_style(&mira, crate::ToggleVariant::Outline, 2.0, false);
    assert_eq!(spaced.border.width, 0.0);
}
