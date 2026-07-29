//! Behavioral tests for the collapsible component.

use std::time::Duration;

use crate::iced_compat::widget::text;
use crate::iced_compat::{Element, Length, Size, time};

use super::render::{Animation, Transition, revealed_size};
use super::*;
use crate::theme::Theme;
use twill_core::prelude::{Padding, PaddingValue, PaddingVar, Spacing};

#[derive(Clone, Debug, PartialEq)]
enum Message {
    Toggled(bool),
}

fn animation(animated: bool) -> Animation {
    Animation {
        animated,
        duration: Duration::from_millis(200),
        easing: CollapsibleEasing::default(),
    }
}

#[test]
fn root_defaults_match_the_web_primitive() {
    let theme = Theme::light();
    let root = Collapsible::<Message>::new(&theme);

    assert!(!root.is_open());
    assert!(!root.disabled);
    assert_eq!(root.orientation, CollapsibleOrientation::Vertical);
    assert_eq!(root.align, CollapsibleAlignment::Start);
    assert!(root.spacing.is_none());
    assert!(root.padding.is_none());
    assert!(root.slots.is_empty());
    assert!(root.animated);
    assert_eq!(root.duration, DEFAULT_TRANSITION);
    assert_eq!(root.easing, CollapsibleEasing::EaseInOut);
    assert!(root.on_open_change.is_none());
    assert!(root.style_override.is_none());
}

#[test]
fn state_round_trips_through_bool() {
    assert_eq!(CollapsibleState::from(true), CollapsibleState::Open);
    assert_eq!(CollapsibleState::from(false), CollapsibleState::Closed);
    assert!(bool::from(CollapsibleState::Open));
    assert!(!bool::from(CollapsibleState::Closed));
    assert_eq!(CollapsibleState::Closed.toggled(), CollapsibleState::Open);
    assert_eq!(CollapsibleState::Open.toggled(), CollapsibleState::Closed);
}

#[test]
fn state_setter_matches_open_setter() {
    let theme = Theme::light();

    assert!(
        Collapsible::<Message>::new(&theme)
            .state(CollapsibleState::Open)
            .is_open()
    );
    assert!(
        !Collapsible::<Message>::new(&theme)
            .state(CollapsibleState::Closed)
            .is_open()
    );
}

#[test]
fn spacing_and_radius_reject_hostile_values() {
    let theme = Theme::light();

    let root = Collapsible::<Message>::new(&theme)
        .spacing(f32::NAN)
        .radius(-8.0);
    assert_eq!(root.spacing, Some(0.0));
    assert_eq!(root.surface.radius, Some(0.0));

    let content = CollapsibleContent::<Message>::new(&theme)
        .spacing(f32::INFINITY)
        .radius(f32::NEG_INFINITY);
    assert_eq!(content.spacing, Some(0.0));
    assert_eq!(content.surface.radius, Some(0.0));
}

#[test]
fn duration_is_clamped_to_at_least_one_millisecond() {
    let theme = Theme::light();

    let root = Collapsible::<Message>::new(&theme).duration(Duration::ZERO);
    assert_eq!(root.duration, Duration::from_millis(1));

    let root = Collapsible::<Message>::new(&theme).duration_ms(350);
    assert_eq!(root.duration, Duration::from_millis(350));
}

#[test]
fn unsupported_padding_is_reported_instead_of_panicking() {
    let theme = Theme::light();

    let error = Collapsible::<Message>::new(&theme)
        .padding(Padding::all(Spacing::Auto))
        .expect_err("auto padding has no iced equivalent");
    assert_eq!(error, CollapsibleBuildError::UnsupportedPaddingAuto);

    let error = CollapsibleContent::<Message>::new(&theme)
        .padding(Padding::individual_value(
            PaddingValue::Var(PaddingVar::new("--collapsible-padding")),
            PaddingValue::Px(2.0),
            PaddingValue::Px(3.0),
            PaddingValue::Px(4.0),
        ))
        .expect_err("variable padding has no iced equivalent");
    assert_eq!(
        error,
        CollapsibleBuildError::UnsupportedPaddingVariable {
            name: "--collapsible-padding"
        }
    );
    assert!(error.to_string().contains("--collapsible-padding"));
}

#[test]
fn supported_padding_is_converted_to_pixels() {
    let theme = Theme::light();

    let content = CollapsibleContent::<Message>::new(&theme)
        .padding(Padding::all(Spacing::S4))
        .expect("scale padding is supported");
    let padding = content.padding.expect("padding is stored");

    assert!((padding.top - 16.0).abs() < f32::EPSILON);
    assert!((padding.left - 16.0).abs() < f32::EPSILON);
}

#[test]
fn easing_stays_inside_the_unit_range() {
    for easing in [
        CollapsibleEasing::Linear,
        CollapsibleEasing::EaseOut,
        CollapsibleEasing::EaseInOut,
    ] {
        assert!(easing.apply(0.0).abs() < f32::EPSILON);
        assert!((easing.apply(1.0) - 1.0).abs() < f32::EPSILON);
        assert!((0.0..=1.0).contains(&easing.apply(0.5)));
        // A poisoned time value must not leak a non-finite progress.
        assert!(easing.apply(f32::NAN).abs() < f32::EPSILON);
    }
}

#[test]
fn revealed_size_scales_only_the_animated_axis() {
    let natural = Size::new(200.0, 80.0);

    let closed = revealed_size(natural, CollapsibleOrientation::Vertical, 0.0);
    assert!((closed.width - 200.0).abs() < f32::EPSILON);
    assert!(closed.height.abs() < f32::EPSILON);

    let half = revealed_size(natural, CollapsibleOrientation::Vertical, 0.5);
    assert!((half.height - 40.0).abs() < f32::EPSILON);

    let open = revealed_size(natural, CollapsibleOrientation::Horizontal, 1.0);
    assert!((open.width - 200.0).abs() < f32::EPSILON);
    assert!((open.height - 80.0).abs() < f32::EPSILON);

    // Hostile progress collapses instead of producing a NaN layout.
    let poisoned = revealed_size(natural, CollapsibleOrientation::Vertical, f32::NAN);
    assert!(poisoned.height.abs() < f32::EPSILON);
}

#[test]
fn first_frame_snaps_to_the_initial_state() {
    let mut transition = Transition::default();
    let now = time::Instant::now();

    transition.advance(true, animation(true), now);

    assert!(!transition.is_running());
    assert!((transition.progress(true) - 1.0).abs() < f32::EPSILON);
}

#[test]
fn opening_runs_a_transition_and_settles_at_the_target() {
    let mut transition = Transition::default();
    let start = time::Instant::now();
    let animation = animation(true);

    transition.advance(false, animation, start);
    transition.advance(true, animation, start);
    assert!(transition.is_running());

    transition.advance(true, animation, start + Duration::from_millis(100));
    let midway = transition.progress(true);
    assert!(midway > 0.0 && midway < 1.0);

    transition.advance(true, animation, start + Duration::from_millis(200));
    assert!(!transition.is_running());
    assert!((transition.progress(true) - 1.0).abs() < f32::EPSILON);
}

#[test]
fn disabled_animation_snaps_without_a_transition() {
    let mut transition = Transition::default();
    let now = time::Instant::now();

    transition.advance(false, animation(false), now);
    transition.advance(true, animation(false), now);

    assert!(!transition.is_running());
    assert!((transition.progress(true) - 1.0).abs() < f32::EPSILON);
}

#[test]
fn disabling_animation_mid_transition_snaps_on_the_next_frame() {
    let mut transition = Transition::default();
    let start = time::Instant::now();

    transition.advance(false, animation(true), start);
    transition.advance(true, animation(true), start);
    assert!(transition.is_running());

    transition.advance(true, animation(false), start + Duration::from_millis(20));
    assert!(!transition.is_running());
    assert!((transition.progress(true) - 1.0).abs() < f32::EPSILON);
}

#[test]
fn state_change_without_frames_still_paints_the_target() {
    let transition = Transition::default();

    assert!(transition.progress(true) > 0.0);
    assert!(transition.progress(false).abs() < f32::EPSILON);
}

#[test]
fn trigger_defaults_to_a_ghost_button_without_an_indicator() {
    let theme = Theme::light();
    let trigger = CollapsibleTrigger::<Message>::text("src", &theme);

    assert_eq!(trigger.variant, ButtonVariant::Ghost);
    assert_eq!(trigger.size, ButtonSize::Default);
    assert!(trigger.indicator.is_none());
    assert_eq!(
        trigger.indicator_placement,
        CollapsibleIndicatorPlacement::Leading
    );
    assert!(trigger.on_press.is_none());
}

#[test]
fn chevron_trigger_is_a_square_icon_button() {
    let theme = Theme::light();
    let trigger = CollapsibleTrigger::<Message>::chevron(&theme);

    assert_eq!(trigger.size, ButtonSize::Icon);
    assert_eq!(trigger.indicator, Some(CollapsibleIndicator::Chevron));
    assert!(trigger.size.is_icon());
}

#[test]
fn indicator_can_be_cleared_again() {
    let theme = Theme::light();
    let trigger = CollapsibleTrigger::<Message>::chevron(&theme).indicator_maybe(None);

    assert!(trigger.indicator.is_none());
}

#[test]
fn indicator_rotation_matches_the_web_transform() {
    assert!(
        (CollapsibleIndicator::Chevron.open_angle() - std::f32::consts::FRAC_PI_2).abs()
            < f32::EPSILON
    );
    assert!(
        (CollapsibleIndicator::ChevronDown.open_angle() - std::f32::consts::PI).abs()
            < f32::EPSILON
    );
}

#[test]
fn a_disabled_root_publishes_no_toggle_message() {
    let theme = Theme::light();
    let root = Collapsible::new(&theme)
        .disabled(true)
        .trigger(CollapsibleTrigger::text("src", &theme))
        .on_open_change(Message::Toggled);

    // The disabled root drops the callback before it reaches the trigger, so
    // the built element cannot publish anything.
    let _: Element<'_, Message> = root.into();
}

#[test]
fn open_change_callback_receives_the_next_state() {
    let theme = Theme::light();
    let root = Collapsible::<Message>::new(&theme)
        .open(true)
        .on_open_change(Message::Toggled);
    let callback = root.on_open_change.as_ref().expect("callback is stored");

    assert_eq!(callback(!root.is_open()), Message::Toggled(false));
}

#[test]
fn callback_can_be_cleared_for_read_only_previews() {
    let theme = Theme::light();
    let root = Collapsible::<Message>::new(&theme)
        .on_open_change(Message::Toggled)
        .on_open_change_maybe(None::<fn(bool) -> Message>);

    assert!(root.on_open_change.is_none());
}

#[test]
fn slots_are_kept_in_insertion_order() {
    let theme = Theme::light();
    let root = Collapsible::<Message>::new(&theme)
        .push(text("always visible"))
        .trigger(CollapsibleTrigger::text("src", &theme))
        .content(CollapsibleContent::new(&theme).push(text("utils.ts")));

    assert!(matches!(root.slots[0], CollapsibleSlot::Element(_)));
    assert!(matches!(root.slots[1], CollapsibleSlot::Trigger(_)));
    assert!(matches!(root.slots[2], CollapsibleSlot::Content(_)));
}

#[test]
fn extend_appends_every_child() {
    let theme = Theme::light();
    let content = CollapsibleContent::<Message>::with_children(
        &theme,
        vec![Element::from(text("a")), Element::from(text("b"))],
    )
    .extend(vec![Element::from(text("c"))]);

    assert_eq!(content.children.len(), 3);
}

#[test]
fn builder_converts_to_element_through_both_paths() {
    let theme = Theme::light();

    let from_trait: Element<'_, Message> = Collapsible::new(&theme)
        .open(true)
        .trigger(
            CollapsibleTrigger::text("src", &theme)
                .indicator(CollapsibleIndicator::Chevron)
                .full_width(true),
        )
        .content(CollapsibleContent::new(&theme).push(text("utils.ts")))
        .on_open_change(Message::Toggled)
        .into();
    assert_eq!(from_trait.as_widget().size().width, Length::Fill);

    let from_helper = collapsible(
        Collapsible::<Message>::new(&theme)
            .orientation(CollapsibleOrientation::Horizontal)
            .align(CollapsibleAlignment::Center)
            .content(CollapsibleContent::new(&theme).force_mount(true)),
    );
    assert_eq!(from_helper.as_widget().size().width, Length::Fill);
}

#[test]
fn style_overrides_are_retained() {
    let theme = Theme::light();

    let root = Collapsible::<Message>::new(&theme).style_override(|style| style);
    assert!(root.style_override.is_some());

    let trigger =
        CollapsibleTrigger::<Message>::text("src", &theme).style_override(|style, _status| style);
    assert!(trigger.style_override.is_some());

    let content = CollapsibleContent::<Message>::new(&theme).style_override(|style| style);
    assert!(content.style_override.is_some());
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let root = Collapsible::<NoDebugMessage>::new(&theme)
        .trigger(CollapsibleTrigger::chevron(&theme))
        .content(CollapsibleContent::new(&theme));

    let debug = format!("{root:?}");
    assert!(debug.contains("Collapsible"));
    assert!(debug.contains("slots: 2"));

    let trigger = CollapsibleTrigger::<NoDebugMessage>::chevron(&theme);
    assert!(format!("{trigger:?}").contains("indicator"));

    let content = CollapsibleContent::<NoDebugMessage>::new(&theme);
    assert!(format!("{content:?}").contains("force_mount"));
}
