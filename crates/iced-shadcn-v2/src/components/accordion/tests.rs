//! Behavioral tests for the accordion component.

use crate::iced_compat::widget::text;
use crate::iced_compat::{Background, Element};
use crate::theme::Theme;
use crate::{
    Accordion, AccordionBuildError, AccordionContent, AccordionHeaderLevel, AccordionItem,
    AccordionLoop, AccordionOrientation, AccordionTrigger, AccordionType, AccordionValue,
};
use shadcn_common::StyleId;
use twill_core::prelude::theme::SemanticColor;
use twill_core::prelude::{Padding, PaddingValue, PaddingVar, Spacing};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Changed(AccordionValue),
    Single(Option<String>),
    Multiple(Vec<String>),
    Pressed,
}

#[test]
fn defaults_match_the_source_root_contract() {
    let theme = Theme::light();
    let accordion = Accordion::<Message>::new(&theme);

    assert_eq!(accordion.selection_type(), AccordionType::Single);
    assert_eq!(accordion.selected_value(), &AccordionValue::Single(None));
    assert_eq!(accordion.orientation, AccordionOrientation::Vertical);
    assert_eq!(accordion.loop_navigation, AccordionLoop::Enabled);
    assert!(accordion.animated);
    assert_eq!(accordion.duration, super::DEFAULT_TRANSITION);
    assert!(accordion.is_empty());
}

#[test]
fn style_pack_surfaces_match_accordion_css_defaults() {
    let vega = Theme::light().with_style(StyleId::Vega);
    assert!(!super::geometry::default_root_bordered(&vega));
    assert_eq!(super::geometry::default_root_radius(&vega), 0.0);

    let mira = Theme::light().with_style(StyleId::Mira);
    assert!(super::geometry::default_root_bordered(&mira));
    assert_eq!(
        super::geometry::default_root_radius(&mira),
        mira.radius_scale().md_px
    );

    let maia = Theme::light().with_style(StyleId::Maia);
    assert!(super::geometry::default_root_bordered(&maia));
    assert_eq!(
        super::geometry::default_root_radius(&maia),
        maia.radius_scale().xxl_px
    );
    assert_eq!(Accordion::<Message>::new(&maia).bordered, None);
    assert_eq!(
        Accordion::<Message>::new(&maia).bordered(false).bordered,
        Some(false)
    );

    let open = super::style::resolve_item_surface(&maia, None, false, None, true);
    let closed = super::style::resolve_item_surface(&maia, None, false, None, false);
    let custom =
        super::style::resolve_item_surface(&maia, Some(SemanticColor::Card), false, None, true);

    assert!(closed.background.is_none());
    assert!(matches!(custom.background, Some(Background::Color(_))));
    match open.background {
        Some(Background::Color(color)) => {
            let muted = maia.semantic_color(SemanticColor::Muted);
            assert!((color.a - muted.a * 0.5).abs() < f32::EPSILON);
        }
        _ => panic!("open Maia item should use the muted surface"),
    }
}

#[test]
fn values_preserve_order_and_remove_duplicates() {
    let value = AccordionValue::multiple(["one", "two", "one"]);

    assert_eq!(value.as_multiple(), ["one".to_owned(), "two".to_owned()]);
    assert!(value.is_open("two"));
    assert!(!value.is_open("three"));
}

#[test]
fn toggling_supports_single_and_multiple_modes() {
    let single = AccordionValue::single(Some("one"));
    assert_eq!(
        single.toggled(AccordionType::Single, "one"),
        AccordionValue::Single(None)
    );
    assert_eq!(
        single.toggled(AccordionType::Single, "two"),
        AccordionValue::Single(Some("two".to_owned()))
    );

    let multiple = AccordionValue::multiple(["one"]);
    assert_eq!(
        multiple.toggled(AccordionType::Multiple, "two"),
        AccordionValue::Multiple(vec!["one".to_owned(), "two".to_owned()])
    );
    assert_eq!(
        multiple.toggled(AccordionType::Multiple, "one"),
        AccordionValue::Multiple(Vec::new())
    );
}

#[test]
fn changing_mode_normalizes_the_controlled_value() {
    let theme = Theme::light();
    let accordion = Accordion::<Message>::new(&theme)
        .values(["one", "two"])
        .single();

    assert_eq!(accordion.selection_type(), AccordionType::Single);
    assert_eq!(accordion.selected_value().as_single(), Some("one"));
}

#[test]
fn padding_errors_are_reported_without_panicking() {
    let theme = Theme::light();

    let error = Accordion::<Message>::new(&theme)
        .padding(Padding::all(Spacing::Auto))
        .expect_err("auto padding has no iced equivalent");
    assert_eq!(error, AccordionBuildError::UnsupportedPaddingAuto);

    let error = AccordionContent::<Message>::new(&theme)
        .padding(Padding::individual_value(
            PaddingValue::Var(PaddingVar::new("--accordion-padding")),
            PaddingValue::Px(2.0),
            PaddingValue::Px(3.0),
            PaddingValue::Px(4.0),
        ))
        .expect_err("variable padding has no iced equivalent");
    assert_eq!(
        error,
        AccordionBuildError::UnsupportedPaddingVariable {
            name: "--accordion-padding"
        }
    );

    let error = AccordionTrigger::<Message>::text("Trigger", &theme)
        .padding(Padding::all(Spacing::Auto))
        .expect_err("auto trigger padding has no iced equivalent");
    assert_eq!(error, AccordionBuildError::UnsupportedPaddingAuto);
}

#[test]
fn content_defaults_to_force_mount_and_supports_find_parity() {
    let theme = Theme::light();
    let content = AccordionContent::<Message>::text("Details", &theme).hidden_until_found(true);

    assert!(!content.force_mount);
    assert!(content.hidden_until_found);
    assert_eq!(content.children.len(), 1);
}

#[test]
fn item_builder_keeps_composition_and_explicit_values() {
    let theme = Theme::light();
    let item = AccordionItem::<Message>::new(&theme)
        .value("account")
        .trigger(AccordionTrigger::text("Account", &theme).level(AccordionHeaderLevel::Two))
        .content(AccordionContent::text("Account details", &theme));

    assert_eq!(item.item_value(), Some("account"));
    assert!(item.trigger.is_some());
    assert!(item.content.is_some());
    assert_eq!(item.trigger.as_ref().expect("trigger").level.number(), 2);
}

#[test]
fn callbacks_can_be_selected_by_value_shape() {
    let theme = Theme::light();
    let accordion = Accordion::<Message>::new(&theme)
        .on_change_single(Message::Single)
        .on_change_multiple(Message::Multiple)
        .on_value_change(Message::Changed);

    assert!(accordion.on_value_change.is_some());
    assert!(accordion.on_press.is_none());

    let callback = accordion.on_value_change.as_ref().expect("value callback");
    assert_eq!(
        callback(AccordionValue::Single(Some("one".to_owned()))),
        Message::Changed(AccordionValue::Single(Some("one".to_owned())))
    );
}

#[test]
fn root_press_messages_clear_value_callbacks() {
    let theme = Theme::light();
    let accordion = Accordion::<Message>::new(&theme)
        .on_value_change(Message::Changed)
        .on_press(Message::Pressed);

    assert!(accordion.on_value_change.is_none());
    assert!(accordion.on_press.is_some());
}

#[test]
fn navigation_skips_disabled_triggers_and_honors_loop_policy() {
    let theme = Theme::light();
    let accordion = Accordion::<Message>::new(&theme)
        .push(AccordionItem::text("one", "One", "First", &theme))
        .push(AccordionItem::text("two", "Two", "Second", &theme).disabled(true))
        .push(AccordionItem::text("three", "Three", "Third", &theme));

    assert_eq!(
        accordion.next_trigger_value(Some("one")),
        Some("three".to_owned())
    );
    assert_eq!(
        accordion.previous_trigger_value(Some("three")),
        Some("one".to_owned())
    );
    assert_eq!(
        accordion.next_trigger_value(Some("three")),
        Some("one".to_owned())
    );

    let no_loop = accordion.loop_navigation(AccordionLoop::Disabled);
    assert_eq!(
        no_loop.next_trigger_value(Some("three")),
        Some("three".to_owned())
    );
    assert_eq!(
        no_loop.previous_trigger_value(Some("one")),
        Some("one".to_owned())
    );
}

#[test]
fn navigation_uses_generated_values_for_items_without_explicit_values() {
    let theme = Theme::light();
    let accordion = Accordion::<Message>::new(&theme)
        .push(
            AccordionItem::new(&theme)
                .trigger(AccordionTrigger::text("First", &theme))
                .content(AccordionContent::text("First content", &theme)),
        )
        .push(
            AccordionItem::new(&theme)
                .trigger(AccordionTrigger::text("Second", &theme))
                .content(AccordionContent::text("Second content", &theme)),
        );

    assert_eq!(
        accordion.next_trigger_value(Some("item-1")),
        Some("item-2".to_owned())
    );
}

#[test]
fn builders_convert_to_elements() {
    let theme = Theme::light();
    let _: Element<'_, Message> = Accordion::new(&theme)
        .value("item-1")
        .push(AccordionItem::text(
            "item-1",
            "Product information",
            "The content is visible when the item is open.",
            &theme,
        ))
        .on_value_change(Message::Changed)
        .into();

    let _: Element<'_, Message> = super::accordion(
        Accordion::<Message>::new(&theme)
            .multiple()
            .values(["item-1", "item-2"])
            .orientation(AccordionOrientation::Horizontal)
            .loop_navigation(AccordionLoop::Disabled),
    );

    let _: Element<'_, Message> = AccordionContent::with_children(
        &theme,
        [Element::from(text("a")), Element::from(text("b"))],
    )
    .into();
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let accordion = Accordion::<NoDebugMessage>::new(&theme).push(
        AccordionItem::new(&theme)
            .trigger(AccordionTrigger::text("Details", &theme))
            .content(AccordionContent::text("Content", &theme)),
    );

    assert!(format!("{accordion:?}").contains("Accordion"));
}
