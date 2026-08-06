//! Behavioral tests for the resizable component.

use crate::iced_compat::widget::text;
use crate::iced_compat::{Element, Length};

use super::*;
use crate::theme::Theme;
use twill_core::prelude::{Padding, PaddingValue, PaddingVar, Spacing};

#[derive(Clone, Debug, PartialEq)]
enum Message {
    Layout(ResizableLayout),
}

#[test]
fn group_defaults_match_the_web_primitive() {
    let theme = Theme::light();
    let group = ResizablePaneGroup::<Message>::new(&theme);

    assert_eq!(group.direction, ResizableDirection::Horizontal);
    assert!(group.sizes.is_none());
    assert!(group.slots.is_empty());
    assert_eq!(group.width, Length::Fill);
    assert_eq!(group.height, Length::Fill);
    assert!(group.padding.is_none());
    assert!(!group.bordered);
    assert_eq!(group.radius, ResizableRadius::Medium);
}

#[test]
fn pane_constraints_default_to_fifty_fifty() {
    let pane = ResizablePane::<Message>::new(text("Pane"));
    assert!((pane.constraints.default_size - 50.0).abs() < f32::EPSILON);
    assert!((pane.constraints.min_size).abs() < f32::EPSILON);
    assert!((pane.constraints.max_size - 100.0).abs() < f32::EPSILON);
}

#[test]
fn layout_normalizes_to_one_hundred() {
    let layout = ResizableLayout::new(vec![25.0, 25.0, 50.0]);
    let sum: f32 = layout.as_slice().iter().sum();
    assert!((sum - 100.0).abs() < 0.01);
}

#[test]
fn invalid_slot_sequence_is_rejected() {
    let theme = Theme::light();

    assert!(
        ResizablePaneGroup::<Message>::new(&theme)
            .handle(ResizableHandle::new())
            .into_element()
            .is_err(),
        "handle cannot lead"
    );

    assert!(
        ResizablePaneGroup::<Message>::new(&theme)
            .pane(ResizablePane::new(text("A")))
            .pane(ResizablePane::new(text("B")))
            .into_element()
            .is_err(),
        "missing handle"
    );
}

#[test]
fn horizontal_pair_builds_successfully() {
    let theme = Theme::light();

    let element: Element<'_, Message> = ResizablePaneGroup::new(&theme)
        .direction(ResizableDirection::Horizontal)
        .sizes_slice(&[25.0, 75.0])
        .pane(ResizablePane::new(text("Sidebar")).default_size(25.0))
        .handle(ResizableHandle::new())
        .pane(ResizablePane::new(text("Content")).default_size(75.0))
        .width(Length::Fixed(400.0))
        .height(Length::Fixed(200.0))
        .bordered(true)
        .radius(ResizableRadius::Large)
        .on_layout_change(Message::Layout)
        .into_element()
        .expect("valid horizontal group");

    assert!(std::mem::size_of_val(&element) > 0);
}

#[test]
fn resize_pair_respects_min_size() {
    let constraints = [
        PaneConstraints {
            default_size: 30.0,
            min_size: 20.0,
            max_size: 100.0,
            ..PaneConstraints::default()
        },
        PaneConstraints {
            default_size: 70.0,
            min_size: 30.0,
            max_size: 100.0,
            ..PaneConstraints::default()
        },
    ];

    let mut sizes = vec![30.0, 70.0];
    assert!(geometry::resize_pair(
        &mut sizes,
        &constraints,
        0,
        -500.0,
        400.0
    ));
    assert!(sizes[0] >= 20.0);
    assert!(sizes[1] >= 30.0);
}

#[test]
fn resize_pair_respects_opposite_min_and_preserves_total() {
    let constraints = [
        PaneConstraints {
            default_size: 30.0,
            min_size: 20.0,
            max_size: 100.0,
            ..PaneConstraints::default()
        },
        PaneConstraints {
            default_size: 70.0,
            min_size: 30.0,
            max_size: 100.0,
            ..PaneConstraints::default()
        },
    ];

    let mut sizes = vec![30.0, 70.0];
    assert!(geometry::resize_pair(
        &mut sizes,
        &constraints,
        0,
        500.0,
        400.0
    ));
    assert!((sizes[0] - 70.0).abs() < 0.01);
    assert!((sizes[1] - 30.0).abs() < 0.01);

    let total: f32 = sizes.iter().sum();
    assert!((total - 100.0).abs() < 0.01);
}

#[test]
fn unsupported_padding_is_reported_instead_of_panicking() {
    let theme = Theme::light();

    let error = ResizablePaneGroup::<Message>::new(&theme)
        .padding(Padding::all(Spacing::Auto))
        .expect_err("auto padding has no iced equivalent");
    assert_eq!(error, ResizableBuildError::UnsupportedPaddingAuto);

    let error = ResizablePaneGroup::<Message>::new(&theme)
        .padding(Padding::individual_value(
            PaddingValue::Var(PaddingVar::new("--resizable-padding")),
            PaddingValue::Px(2.0),
            PaddingValue::Px(3.0),
            PaddingValue::Px(4.0),
        ))
        .expect_err("variable padding has no iced equivalent");
    assert!(matches!(
        error,
        ResizableBuildError::UnsupportedPaddingVariable { .. }
    ));
}

use super::geometry;
use super::types::PaneConstraints;
