use super::*;
use crate::components::button::{ButtonRadius, ButtonVariant};
use crate::components::input::InputSize;
use crate::iced_compat::widget::text_editor;
use crate::iced_compat::{Element, Length};
use crate::theme::Theme;
use shadcn_common::StyleId;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Message {
    Changed(String),
    Pressed,
}

fn theme() -> Theme {
    Theme::default()
}

#[test]
fn root_defaults_to_fill_and_accepts_composable_items() {
    let theme = theme();
    let group = InputGroup::<Message>::new(&theme)
        .push_input(
            Input::new(&theme)
                .placeholder("Search")
                .on_input(Message::Changed),
        )
        .push_addon(
            InputGroupAddon::empty(&theme)
                .align(InputGroupAddonAlign::InlineEnd)
                .push(InputGroupButton::text("Go", &theme).on_press(Message::Pressed)),
        );

    assert_eq!(group.width, Length::Fill);
    assert_eq!(group.height, Length::Shrink);
    assert_eq!(group.items.len(), 2);
}

#[test]
fn props_and_public_builders_preserve_requested_options() {
    let theme = theme();
    let props = InputGroupProps::new()
        .radius(InputGroupRadius::Large)
        .invalid(true)
        .disabled(true);
    let group = InputGroup::<Message>::with_props(&theme, props)
        .width(Length::Fixed(320.0))
        .height(Length::Fixed(48.0))
        .aria_label("query");

    assert_eq!(group.radius, Some(InputGroupRadius::Large));
    assert!(group.invalid);
    assert!(group.disabled);
    assert_eq!(group.width, Length::Fixed(320.0));
    assert_eq!(group.height, Length::Fixed(48.0));
    assert_eq!(group.aria_label.as_deref(), Some("query"));

    let button = InputGroupButton::text("Save", &theme)
        .variant(ButtonVariant::Secondary)
        .size(InputGroupButtonSize::Sm)
        .radius(ButtonRadius::Small)
        .disabled(true);
    let _: Element<'_, Message> = button.into();

    let textarea = InputGroupTextareaProps::new()
        .size(InputSize::Lg)
        .rows(3)
        .max_rows(5)
        .resize(InputGroupTextareaResize::Vertical)
        .padding([10.0, 12.0])
        .max_len(100);
    assert_eq!(textarea.size, InputSize::Lg);
    assert_eq!(textarea.rows, Some(3));
    assert_eq!(textarea.max_rows, Some(5));
    assert_eq!(textarea.resize, InputGroupTextareaResize::Vertical);
    assert_eq!(textarea.padding, Some([10.0, 12.0]));
    assert_eq!(textarea.max_len, Some(100));
}

#[test]
fn display_tokens_match_web_component_names() {
    assert_eq!(InputGroupAddonAlign::InlineEnd.to_string(), "inline-end");
    assert_eq!(InputGroupButtonSize::IconXs.to_string(), "icon-xs");
    assert_eq!(InputGroupRadius::Medium.to_string(), "medium");
    assert_eq!(InputGroupTextareaResize::Both.to_string(), "both");
}

#[test]
fn textarea_action_helper_honors_read_only_and_max_length() {
    let mut content = text_editor::Content::with_text("abc");
    let props = InputGroupTextareaProps::new().max_len(3);

    assert!(!input_group_textarea_apply_action(
        &mut content,
        text_editor::Action::Edit(text_editor::Edit::Insert('d')),
        props,
    ));
    assert_eq!(content.text(), "abc");

    let props = props.read_only(true);
    assert!(!input_group_textarea_apply_action(
        &mut content,
        text_editor::Action::Edit(text_editor::Edit::Backspace),
        props,
    ));
    assert_eq!(content.text(), "abc");
}

#[test]
fn builders_convert_to_elements() {
    let theme = theme();
    let _: Element<'_, Message> = InputGroupText::text("units", &theme).into();
    let _: Element<'_, Message> = InputGroupAddon::text("kg", &theme).into();
    let _: Element<'_, Message> = input_group_text("ms", &theme);
    let _: Element<'_, Message> = input_group_button(
        "Run",
        Some(Message::Pressed),
        InputGroupButtonProps::new(),
        &theme,
    );
}

#[test]
fn addon_geometry_matches_the_nova_css_slots() {
    let theme = Theme::light().with_style(StyleId::Nova);

    let inline_start = style::addon_padding(&theme, InputGroupAddonAlign::InlineStart);
    assert_eq!(inline_start.top, 6.0);
    assert_eq!(inline_start.right, 0.0);
    assert_eq!(inline_start.bottom, 6.0);
    assert_eq!(inline_start.left, 8.0);

    let inline_end = style::addon_padding(&theme, InputGroupAddonAlign::InlineEnd);
    assert_eq!(inline_end.right, 8.0);
    assert_eq!(inline_end.left, 0.0);

    let block_end = style::addon_padding(&theme, InputGroupAddonAlign::BlockEnd);
    assert_eq!(block_end.top, 6.0);
    assert_eq!(block_end.right, 10.0);
    assert_eq!(block_end.bottom, 8.0);
    assert_eq!(block_end.left, 10.0);

    assert_eq!(style::addon_spacing(&theme), 8.0);
    assert_eq!(
        style::addon_spacing(&Theme::light().with_style(StyleId::Mira)),
        4.0
    );
}

#[test]
fn sera_uses_a_bottom_border_for_resting_focus_and_invalid_states() {
    let theme = Theme::light().with_style(StyleId::Sera);
    let input = theme.semantic_color(twill_core::prelude::theme::SemanticColor::Input);
    let ring = theme.semantic_color(twill_core::prelude::theme::SemanticColor::Ring);
    let destructive = theme.semantic_color(twill_core::prelude::theme::SemanticColor::Destructive);

    assert!(style::uses_bottom_border(&theme));
    assert_eq!(
        style::resolve_group_style(&theme, None, false, false, false)
            .border
            .color,
        input
    );
    assert_eq!(
        style::resolve_group_style(&theme, None, false, false, true)
            .border
            .color,
        ring
    );
    assert_eq!(
        style::resolve_group_style(&theme, None, true, false, false)
            .border
            .color,
        destructive
    );
}

#[test]
fn control_state_is_carried_into_the_group_layout_metadata() {
    let theme = theme();
    let item: InputGroupItem<'_, Message> = Input::new(&theme).invalid(true).disabled(true).into();

    let ItemKind::Input(input) = item.kind else {
        panic!("input must be represented as a control item");
    };

    assert!(input.is_invalid());
    assert!(input.is_disabled());
}
