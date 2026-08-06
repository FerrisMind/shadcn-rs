//! Behavioral tests for the switch component.

use std::time::Duration;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::{Color, Element, Point, Rectangle, Size, mouse, touch};
use shadcn_common::{AccentColor, ControlSize, StyleId};

use super::geometry::{self, Metrics};
use super::style::resolve_style;
use super::types::{SwitchState, SwitchStatus};
use super::*;
use crate::theme::Theme;

#[derive(Debug, Clone)]
enum Message {
    Toggled(bool),
    Pressed,
}

fn status(checked: bool) -> SwitchStatus {
    SwitchStatus {
        checked,
        ..SwitchStatus::default()
    }
}

fn style_for<'a>(switch: &Switch<'a, Message>, status: SwitchStatus) -> SwitchStyle {
    let metrics = geometry::resolve_metrics(switch.theme, switch.size);
    let track_radius = geometry::radius_px(switch, metrics.track);
    let thumb_radius = geometry::radius_px(switch, metrics.thumb);

    resolve_style(switch, metrics, status, track_radius, thumb_radius)
}

#[test]
fn builder_defaults_match_the_web_component() {
    let theme = Theme::light();
    let switch = Switch::<Message>::new(&theme);

    assert!(!switch.checked);
    assert!(!switch.disabled);
    assert!(!switch.focused);
    assert!(!switch.invalid);
    assert_eq!(switch.size, SwitchSize::Default);
    assert_eq!(switch.size, SwitchSize::default());
    assert!(switch.animated);
    assert_eq!(switch.duration, Duration::from_millis(150));
    assert!(switch.on_toggle.is_none());
    assert!(std::ptr::eq(switch.theme, &theme));
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let switch = Switch::<Message>::new(&theme)
        .checked(true)
        .size(SwitchSize::Sm)
        .disabled(true)
        .focused(true)
        .invalid(true)
        .color(AccentColor::Blue)
        .radius(SwitchRadius::None)
        .animated(false)
        .duration_ms(400);

    assert!(switch.checked);
    assert_eq!(switch.size, SwitchSize::Sm);
    assert!(switch.disabled);
    assert!(switch.focused);
    assert!(switch.invalid);
    assert_eq!(switch.color, Some(AccentColor::Blue));
    assert_eq!(switch.radius, Some(SwitchRadius::None));
    assert!(!switch.animated);
    assert_eq!(switch.duration, Duration::from_millis(400));
}

#[test]
fn duration_is_clamped_to_at_least_one_millisecond() {
    let theme = Theme::light();
    let switch = Switch::<Message>::new(&theme).duration(Duration::ZERO);

    assert_eq!(switch.duration, Duration::from_millis(1));
}

#[test]
fn explicit_colors_and_accents_are_mutually_exclusive() {
    let theme = Theme::light();

    let explicit = Switch::<Message>::new(&theme)
        .color(AccentColor::Blue)
        .checked_color(Color::BLACK);
    assert_eq!(explicit.checked_color, Some(Color::BLACK));
    assert_eq!(explicit.color, None);

    let accent = Switch::<Message>::new(&theme)
        .checked_color(Color::BLACK)
        .color(AccentColor::Blue);
    assert_eq!(accent.color, Some(AccentColor::Blue));
    assert_eq!(accent.checked_color, None);

    let reset = accent.theme_primary();
    assert_eq!(reset.color, None);
    assert_eq!(reset.checked_color, None);
}

#[test]
fn callbacks_and_press_messages_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = Switch::new(&theme).on_toggle(Message::Toggled).into();
    let _: Element<'_, Message> = Switch::new(&theme).on_change(Message::Toggled).into();
    let _: Element<'_, Message> = Switch::new(&theme).on_press(Message::Pressed).into();
    let _: Element<'_, Message> = switch(Switch::new(&theme)).into();

    assert!(
        Switch::<Message>::new(&theme)
            .on_press_maybe(None)
            .on_toggle
            .is_none()
    );
    assert!(
        Switch::new(&theme)
            .on_press_maybe(Some(Message::Pressed))
            .on_toggle
            .is_some()
    );
}

#[test]
fn touch_press_uses_the_touch_position_without_a_mouse_cursor() {
    let theme = Theme::light();
    let switch = Switch::new(&theme).on_toggle(Message::Toggled);
    let metrics = geometry::resolve_metrics(&theme, SwitchSize::Default);
    let bounds = Rectangle::new(Point::new(12.0, 18.0), metrics.track);
    let event = canvas::Event::Touch(touch::Event::FingerPressed {
        id: touch::Finger(3),
        position: Point::new(
            bounds.x + bounds.width / 2.0,
            bounds.y + bounds.height / 2.0,
        ),
    });

    let action = <Switch<'_, Message> as canvas::Program<Message>>::update(
        &switch,
        &mut super::types::SwitchState::default(),
        &event,
        bounds,
        mouse::Cursor::Unavailable,
    )
    .expect("touch press inside the switch should publish");
    let (message, _, _) = action.into_inner();

    assert!(matches!(message, Some(Message::Toggled(true))));
}

#[test]
fn toggle_callback_receives_the_next_state_and_press_ignores_it() {
    let theme = Theme::light();

    let toggled = Switch::new(&theme)
        .checked(false)
        .on_toggle(Message::Toggled);
    let on_toggle = toggled.on_toggle.as_ref().expect("callback is stored");
    assert!(matches!(on_toggle(true), Message::Toggled(true)));
    assert!(matches!(on_toggle(false), Message::Toggled(false)));

    let pressed = Switch::new(&theme).on_press(Message::Pressed);
    let on_press = pressed.on_toggle.as_ref().expect("callback is stored");
    assert!(matches!(on_press(true), Message::Pressed));
}

#[test]
fn every_style_pack_keeps_the_thumb_inside_the_track() {
    for style in StyleId::ALL {
        let theme = Theme::light().with_style(style);

        for size in [
            SwitchSize::Sm,
            SwitchSize::Default,
            SwitchSize::Custom(28.0),
        ] {
            let metrics = geometry::resolve_metrics(&theme, size);
            let checked_offset = geometry::thumb_offset(metrics, 1.0);

            assert!(metrics.thumb.width <= metrics.track.width);
            assert!(metrics.thumb.height <= metrics.track.height);
            assert!(
                checked_offset + metrics.thumb.width <= metrics.track.width + 0.001,
                "{style:?} {size:?} thumb overflows the track",
            );
            assert!(geometry::thumb_offset(metrics, 0.0) < checked_offset);
        }
    }
}

#[test]
fn widget_bounds_reserve_room_for_the_ring() {
    let theme = Theme::light();
    let metrics = geometry::resolve_metrics(&theme, SwitchSize::Default);
    let bounds = metrics.bounds();

    assert_eq!(bounds.width, metrics.track.width + metrics.ring_width * 2.0);
    assert_eq!(
        bounds.height,
        metrics.track.height + metrics.ring_width * 2.0
    );
}

#[test]
fn custom_size_scales_the_default_footprint() {
    let theme = Theme::light();
    let default = geometry::resolve_metrics(&theme, SwitchSize::Default);
    let doubled = geometry::resolve_metrics(&theme, SwitchSize::Custom(default.track.height * 2.0));

    assert!((doubled.track.width - default.track.width * 2.0).abs() < 0.001);
    assert!((doubled.thumb.width - default.thumb.width * 2.0).abs() < 0.001);
    assert!((doubled.thumb_travel - default.thumb_travel * 2.0).abs() < 0.001);
}

#[test]
fn non_finite_custom_size_falls_back_to_a_visible_footprint() {
    let theme = Theme::light();
    let metrics = geometry::resolve_metrics(&theme, SwitchSize::Custom(f32::NAN));

    assert!(metrics.track.width > 0.0);
    assert!(metrics.track.height > 0.0);
}

#[test]
fn radius_presets_stay_within_the_track() {
    let theme = Theme::light();
    let track = Size::new(32.0, 18.0);

    let pill = Switch::<Message>::new(&theme).radius(SwitchRadius::Full);
    assert_eq!(geometry::radius_px(&pill, track), 9.0);

    let square = Switch::<Message>::new(&theme).radius(SwitchRadius::None);
    assert_eq!(geometry::radius_px(&square, track), 0.0);

    let oversized = Switch::<Message>::new(&theme).radius(SwitchRadius::Custom(999.0));
    assert_eq!(geometry::radius_px(&oversized, track), 9.0);

    let invalid = Switch::<Message>::new(&theme).radius(SwitchRadius::Custom(f32::NAN));
    assert_eq!(geometry::radius_px(&invalid, track), 0.0);
}

#[test]
fn default_radius_follows_the_style_pack() {
    assert_eq!(
        geometry::default_radius(&Theme::light().with_style(StyleId::Sera)),
        SwitchRadius::None,
    );
    assert_eq!(
        geometry::default_radius(&Theme::light().with_style(StyleId::Vega)),
        SwitchRadius::Full,
    );
}

#[test]
fn track_and_thumb_colors_follow_the_checked_state() {
    let theme = Theme::light();
    let switch = Switch::<Message>::new(&theme);

    let checked = style_for(&switch, status(true));
    assert_eq!(checked.track, theme.palette.primary);
    assert_eq!(checked.thumb, theme.palette.background);
    assert_eq!(checked.border, checked.track);

    let unchecked = style_for(&switch, status(false));
    assert_eq!(unchecked.track, theme.palette.input);
    assert_eq!(unchecked.ring, None);
}

#[test]
fn dark_mode_swaps_the_thumb_and_dims_the_unchecked_track() {
    let theme = Theme::dark();
    let switch = Switch::<Message>::new(&theme);

    let unchecked = style_for(&switch, status(false));
    assert!((unchecked.track.a - theme.palette.input.a * 0.8).abs() < f32::EPSILON);
    assert_eq!(unchecked.thumb, theme.palette.foreground);

    let checked = style_for(&switch, status(true));
    assert_eq!(checked.thumb, theme.palette.primary_foreground);
}

#[test]
fn accent_overlay_and_explicit_colors_win_over_theme_tokens() {
    let theme = Theme::light();

    let accent = Switch::<Message>::new(&theme).color(AccentColor::Blue);
    assert_ne!(
        style_for(&accent, status(true)).track,
        theme.palette.primary
    );

    let explicit = Switch::<Message>::new(&theme)
        .checked_color(Color::BLACK)
        .track_color(Color::WHITE)
        .thumb_color(Color::from_rgb(1.0, 0.0, 0.0));
    assert_eq!(style_for(&explicit, status(true)).track, Color::BLACK);
    assert_eq!(style_for(&explicit, status(false)).track, Color::WHITE);
    assert_eq!(
        style_for(&explicit, status(true)).thumb,
        Color::from_rgb(1.0, 0.0, 0.0)
    );
}

#[test]
fn disabled_switches_are_painted_at_half_opacity() {
    let theme = Theme::light();
    let switch = Switch::<Message>::new(&theme);

    let enabled = style_for(&switch, status(true));
    let disabled = style_for(
        &switch,
        SwitchStatus {
            checked: true,
            disabled: true,
            ..SwitchStatus::default()
        },
    );

    assert!((disabled.track.a - enabled.track.a * 0.5).abs() < f32::EPSILON);
    assert!((disabled.thumb.a - enabled.thumb.a * 0.5).abs() < f32::EPSILON);
}

#[test]
fn focus_and_invalid_states_paint_a_ring() {
    let theme = Theme::light();
    let switch = Switch::<Message>::new(&theme);

    let focused = style_for(
        &switch,
        SwitchStatus {
            focused: true,
            ..SwitchStatus::default()
        },
    );
    let ring = focused.ring.expect("focused switches paint a ring");
    assert!(focused.ring_width > 0.0);
    assert!((ring.a - theme.style.switch().ring_opacity).abs() < f32::EPSILON);

    let invalid = style_for(
        &switch,
        SwitchStatus {
            focused: true,
            invalid: true,
            ..SwitchStatus::default()
        },
    );
    let invalid_ring = invalid.ring.expect("invalid switches paint a ring");
    assert_ne!(invalid_ring, ring);
    assert_eq!(invalid.border, theme.palette.destructive);
}

#[test]
fn style_override_patches_the_resolved_style() {
    let theme = Theme::light();
    let switch = Switch::<Message>::new(&theme).style_override(|style, _status| SwitchStyle {
        thumb: Color::BLACK,
        ..style
    });

    let patched = switch.style_override.as_ref().expect("override is stored")(
        style_for(&switch, status(true)),
        status(true),
    );

    assert_eq!(patched.thumb, Color::BLACK);
}

#[test]
fn geometry_matches_the_shared_style_pack_recipe() {
    let theme = Theme::light();
    let recipe = theme.style.switch_size(ControlSize::Md);
    let metrics = geometry::resolve_metrics(&theme, SwitchSize::Default);

    assert_eq!(metrics.track.width, recipe.track_width_px);
    assert_eq!(metrics.track.height, recipe.track_height_px);
    assert_eq!(metrics.thumb.width, recipe.thumb_width_px);
    assert_eq!(metrics.thumb_travel, recipe.thumb_travel_px);
    assert_eq!(metrics.border_width, theme.style.switch().border_width_px);
}

#[test]
fn animation_eases_the_thumb_and_settles_on_the_target() {
    let theme = Theme::light();
    let switch = Switch::<Message>::new(&theme).duration(Duration::from_millis(100));
    let mut state = SwitchState::default();
    let start = crate::iced_compat::time::Instant::now();

    // The first frame adopts the current state without animating.
    switch.advance(&mut state, start);
    assert_eq!(switch.position(&state), 0.0);
    assert!(!state.transition.is_running());

    let checked = Switch::<Message>::new(&theme)
        .checked(true)
        .duration(Duration::from_millis(100));
    checked.advance(&mut state, start);
    assert!(state.transition.is_running());

    checked.advance(&mut state, start + Duration::from_millis(50));
    let midway = checked.position(&state);
    assert!(midway > 0.0 && midway < 1.0, "midway position: {midway}");

    checked.advance(&mut state, start + Duration::from_millis(150));
    assert_eq!(checked.position(&state), 1.0);
    assert!(!state.transition.is_running());
}

#[test]
fn disabled_animation_snaps_to_the_target() {
    let theme = Theme::light();
    let mut state = SwitchState::default();
    let now = crate::iced_compat::time::Instant::now();

    Switch::<Message>::new(&theme)
        .animated(false)
        .advance(&mut state, now);
    let checked = Switch::<Message>::new(&theme).checked(true).animated(false);
    checked.advance(&mut state, now);

    assert!(!state.transition.is_running());
    assert_eq!(checked.position(&state), 1.0);
}

#[test]
fn disabling_animation_mid_transition_snaps_to_the_target() {
    let theme = Theme::light();
    let mut state = SwitchState::default();
    let start = crate::iced_compat::time::Instant::now();

    Switch::<Message>::new(&theme).advance(&mut state, start);

    // Start an animated transition towards `checked = true`.
    let animated = Switch::<Message>::new(&theme)
        .checked(true)
        .duration(Duration::from_millis(100));
    animated.advance(&mut state, start);
    assert!(state.transition.is_running());

    // The next view disables animation while `checked` stays the same: the
    // stale transition must be dropped and the thumb snapped to the target.
    let snapped = Switch::<Message>::new(&theme).checked(true).animated(false);
    snapped.advance(&mut state, start + Duration::from_millis(10));

    assert!(!state.transition.is_running());
    assert_eq!(snapped.position(&state), 1.0);
}

#[test]
fn position_at_rest_is_derived_from_the_controlled_state() {
    let theme = Theme::light();
    let state = SwitchState::default();

    assert_eq!(Switch::<Message>::new(&theme).position(&state), 0.0);
    assert_eq!(
        Switch::<Message>::new(&theme)
            .checked(true)
            .position(&state),
        1.0
    );
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let switch = Switch::<NoDebugMessage>::new(&theme).checked(true);
    let debug = format!("{switch:?}");

    assert!(debug.contains("Switch"));
    assert!(debug.contains("checked: true"));
}

#[test]
fn metrics_are_comparable_and_copyable() {
    let theme = Theme::light();
    let metrics: Metrics = geometry::resolve_metrics(&theme, SwitchSize::Default);

    assert_eq!(
        metrics,
        geometry::resolve_metrics(&theme, SwitchSize::Default)
    );
}
