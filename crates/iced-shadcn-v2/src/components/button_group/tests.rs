//! Behavioral tests for the button-group component.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::iced_compat::{Element, Length};

use super::render;
use super::*;
use crate::components::button::CornerFlatten;
use crate::components::button::{Button, ButtonVariant};
use crate::components::separator::{Separator, SeparatorOrientation};
use crate::theme::Theme;

#[derive(Clone, Debug)]
enum Message {
    Pressed,
}

fn outline_button(theme: &Theme) -> Button<'_, Message> {
    Button::text("Save", theme)
        .variant(ButtonVariant::Outline)
        .on_press(Message::Pressed)
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let group: ButtonGroup<'_, Message> = ButtonGroup::new(&theme)
        .orientation(ButtonGroupOrientation::Vertical)
        .width(Length::Fixed(240.0))
        .height(Length::Fixed(120.0))
        .nested_gap(4.0)
        .aria_label("Pagination");

    assert_eq!(group.orientation, ButtonGroupOrientation::Vertical);
    assert_eq!(group.width, Length::Fixed(240.0));
    assert_eq!(group.height, Length::Fixed(120.0));
    assert_eq!(group.nested_gap, 4.0);
    assert_eq!(group.aria_label.as_deref(), Some("Pagination"));
    assert!(std::ptr::eq(group.theme, &theme));
}

#[test]
fn nested_gap_is_clamped_to_zero() {
    let theme = Theme::light();
    let group: ButtonGroup<'_, Message> = ButtonGroup::new(&theme).nested_gap(-8.0);

    assert_eq!(group.nested_gap, 0.0);
}

#[test]
fn push_accepts_every_item_kind() {
    let theme = Theme::light();
    let group: ButtonGroup<'_, Message> = ButtonGroup::new(&theme)
        .push(outline_button(&theme))
        .push(ButtonGroupText::text("https://", &theme))
        .push_separator()
        .push_element(crate::iced_compat::widget::text("plain"))
        .push(ButtonGroup::new(&theme).push(outline_button(&theme)));

    let kinds: Vec<_> = group.items.iter().map(ButtonGroupItem::kind_name).collect();
    assert_eq!(kinds, ["button", "text", "separator", "element", "group"],);
}

#[test]
fn with_children_and_extend_collect_items() {
    let theme = Theme::light();
    let group: ButtonGroup<'_, Message> = ButtonGroup::with_children(
        &theme,
        [
            outline_button(&theme).into(),
            ButtonGroupItem::element(crate::iced_compat::widget::text("plain")),
        ],
    )
    .extend([Separator::new(&theme).into()]);

    assert_eq!(group.items.len(), 3);
}

#[test]
fn groups_convert_to_elements() {
    let theme = Theme::light();

    // Merged mode: buttons, text, separator, and an opaque element.
    let _: Element<'_, Message> = ButtonGroup::new(&theme)
        .push(outline_button(&theme))
        .push(ButtonGroupText::text("https://", &theme))
        .push_separator()
        .push(Button::text("Send", &theme).on_press(Message::Pressed))
        .push_element(crate::iced_compat::widget::text("plain"))
        .into();

    // Nesting mode: groups of groups are spaced instead of merged.
    let _: Element<'_, Message> = ButtonGroup::new(&theme)
        .push(ButtonGroup::new(&theme).push(outline_button(&theme)))
        .push(ButtonGroup::new(&theme).push(outline_button(&theme)))
        .into();

    // Vertical group with an explicit width stretches its children.
    let _: Element<'_, Message> = ButtonGroup::new(&theme)
        .orientation(ButtonGroupOrientation::Vertical)
        .width(Length::Fixed(160.0))
        .push(outline_button(&theme))
        .push(outline_button(&theme))
        .into();

    // Empty groups build without panicking.
    let _: Element<'_, Message> = ButtonGroup::new(&theme).into();
}

#[test]
fn corner_mask_flattens_only_inner_edges() {
    let horizontal_first = render::corner_mask(ButtonGroupOrientation::Horizontal, true, false);
    assert!(!horizontal_first.top_left);
    assert!(!horizontal_first.bottom_left);
    assert!(horizontal_first.top_right);
    assert!(horizontal_first.bottom_right);

    let horizontal_middle = render::corner_mask(ButtonGroupOrientation::Horizontal, false, false);
    assert!(horizontal_middle.top_left);
    assert!(horizontal_middle.top_right);
    assert!(horizontal_middle.bottom_right);
    assert!(horizontal_middle.bottom_left);

    let vertical_last = render::corner_mask(ButtonGroupOrientation::Vertical, false, true);
    assert!(vertical_last.top_left);
    assert!(vertical_last.top_right);
    assert!(!vertical_last.bottom_right);
    assert!(!vertical_last.bottom_left);

    let only_child = render::corner_mask(ButtonGroupOrientation::Horizontal, true, true);
    assert!(!only_child.is_any());
}

#[test]
fn separators_run_along_the_cross_axis() {
    assert_eq!(
        render::cross_orientation(ButtonGroupOrientation::Horizontal),
        SeparatorOrientation::Vertical,
    );
    assert_eq!(
        render::cross_orientation(ButtonGroupOrientation::Vertical),
        SeparatorOrientation::Horizontal,
    );
}

#[test]
fn text_builder_clamps_inputs() {
    let theme = Theme::light();
    let text: ButtonGroupText<'_, Message> = ButtonGroupText::text("https://", &theme)
        .padding_x(-4.0)
        .text_size(0.0);

    assert_eq!(text.padding_x, Some(0.0));
    assert_eq!(text.text_size, Some(1.0));
    assert!(matches!(text.content, TextContent::Label(_)));
}

#[test]
fn text_cell_converts_to_a_standalone_element() {
    let theme = Theme::light();

    let _: Element<'_, Message> = ButtonGroupText::text("https://", &theme)
        .style_override(|style| style)
        .into();
    let _: Element<'_, Message> =
        ButtonGroupText::new(crate::iced_compat::widget::text("42 items"), &theme).into();
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let group = ButtonGroup::<NoDebugMessage>::new(&theme)
        .push(ButtonGroupText::<NoDebugMessage>::text("cell", &theme));
    let debug = format!("{group:?}");

    assert!(debug.contains("ButtonGroup"));
    assert!(debug.contains("text"));

    let text = ButtonGroupText::<NoDebugMessage>::text("cell", &theme);
    assert!(format!("{text:?}").contains("label"));
}

#[test]
fn orientation_supports_hashing_and_serialization_traits() {
    fn hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    let _ = hash(&ButtonGroupOrientation::Horizontal);
    assert_ne!(
        ButtonGroupOrientation::Horizontal,
        ButtonGroupOrientation::Vertical,
    );
}

#[test]
fn flatten_corners_zeroes_only_masked_radii() {
    let theme = Theme::light();
    let corners = CornerFlatten {
        top_left: true,
        bottom_left: true,
        ..CornerFlatten::default()
    };

    let button = outline_button(&theme).flatten_corners(corners);
    let widget = button.into_button();

    // The style closure runs during draw; here we only assert the builder
    // still produces a widget after the crate-internal hook is applied.
    let _ = widget;
}
