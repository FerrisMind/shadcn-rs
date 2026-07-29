//! Behavioral tests for the slider component.

use crate::iced_compat::widget::canvas;
use crate::iced_compat::{Color, Element, Length, Point, Rectangle, Size, mouse, touch};
use shadcn_common::{AccentColor, StyleId};

use super::geometry;
use super::style::resolve_style;
use super::types::{SliderState, SliderStatus};
use super::*;

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Changed(f32),
    ChangedValues(Vec<f32>),
    Released,
}

/// Horizontal track of 100 px plus the ring reserve on both ends.
fn horizontal_layout(theme: &Theme) -> (geometry::Metrics, Rectangle) {
    let metrics = geometry::resolve_metrics(theme);
    let bounds = Size::new(100.0 + metrics.ring_width * 2.0, metrics.cross_size());

    (
        metrics,
        geometry::track_rect(bounds, metrics, SliderOrientation::Horizontal),
    )
}

fn style_for<'a>(slider: &Slider<'a, Message>, status: SliderStatus) -> SliderStyle {
    let metrics = geometry::resolve_metrics(slider.theme);

    resolve_style(slider, metrics, status, 3.0, 8.0)
}

#[test]
fn builder_defaults_match_the_web_component() {
    let theme = Theme::light();
    let slider = Slider::<Message>::new(&theme);

    assert_eq!(slider.values, vec![0.0]);
    assert_eq!(slider.min, 0.0);
    assert_eq!(slider.max, 100.0);
    assert_eq!(slider.step, 1.0);
    assert_eq!(slider.orientation, SliderOrientation::Horizontal);
    assert!(!slider.disabled);
    assert!(!slider.focused);
    assert!(!slider.is_interactive());
    assert!(std::ptr::eq(slider.theme, &theme));
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let slider = Slider::<Message>::new(&theme)
        .values(vec![10.0, 40.0])
        .range(0.0..=50.0)
        .step(5.0)
        .orientation(SliderOrientation::Vertical)
        .disabled(true)
        .focused(true)
        .color(AccentColor::Blue)
        .radius(SliderRadius::None)
        .thumb_radius(SliderRadius::Full)
        .width(Length::Fixed(24.0))
        .height(Length::Fixed(200.0));

    assert_eq!(slider.values, vec![10.0, 40.0]);
    assert_eq!((slider.min, slider.max), (0.0, 50.0));
    assert_eq!(slider.step, 5.0);
    assert_eq!(slider.orientation, SliderOrientation::Vertical);
    assert!(slider.disabled);
    assert!(slider.focused);
    assert_eq!(slider.color, Some(AccentColor::Blue));
    assert_eq!(slider.radius, Some(SliderRadius::None));
    assert_eq!(slider.thumb_radius, Some(SliderRadius::Full));
    assert_eq!(slider.width, Some(Length::Fixed(24.0)));
    assert_eq!(slider.height, Some(Length::Fixed(200.0)));
}

#[test]
fn ranges_are_validated_instead_of_dividing_by_zero() {
    let theme = Theme::light();

    let reversed = Slider::<Message>::new(&theme).range(80.0..=20.0);
    assert_eq!((reversed.min, reversed.max), (20.0, 80.0));

    let empty = Slider::<Message>::new(&theme).range(5.0..=5.0);
    assert_eq!((empty.min, empty.max), (5.0, 6.0));

    let non_finite = Slider::<Message>::new(&theme).range(f32::NAN..=f32::INFINITY);
    assert_eq!((non_finite.min, non_finite.max), (0.0, 100.0));

    let bounds = Slider::<Message>::new(&theme).min(10.0).max(20.0);
    assert_eq!((bounds.min, bounds.max), (10.0, 20.0));
}

#[test]
fn unrepresentable_range_widths_fall_back_to_the_default_range() {
    let theme = Theme::light();

    // Finite bounds whose span overflows to infinity.
    let overflowing = Slider::<Message>::new(&theme).range(-f32::MAX..=f32::MAX);
    assert_eq!((overflowing.min, overflowing.max), (0.0, 100.0));

    // Empty range that cannot be widened: `f32::MAX + 1.0 == f32::MAX`.
    let saturated = Slider::<Message>::new(&theme).range(f32::MAX..=f32::MAX);
    assert_eq!((saturated.min, saturated.max), (0.0, 100.0));

    let negative_saturated = Slider::<Message>::new(&theme).range(-f32::MAX..=-f32::MAX);
    assert_eq!(
        (negative_saturated.min, negative_saturated.max),
        (0.0, 100.0)
    );
}

#[test]
fn non_finite_values_fall_back_to_the_lower_bound() {
    let theme = Theme::light();

    assert_eq!(
        Slider::<Message>::new(&theme).value(f32::NAN).values,
        vec![0.0]
    );
    assert_eq!(
        Slider::<Message>::new(&theme)
            .values(vec![f32::INFINITY, 30.0])
            .values,
        vec![0.0, 30.0],
    );
}

#[test]
fn values_snap_onto_the_step_grid_and_stay_in_range() {
    assert_eq!(geometry::snap(12.0, 0.0, 100.0, 5.0), 10.0);
    assert_eq!(geometry::snap(13.0, 0.0, 100.0, 5.0), 15.0);
    assert_eq!(geometry::snap(-10.0, 0.0, 100.0, 5.0), 0.0);
    assert_eq!(geometry::snap(140.0, 0.0, 100.0, 5.0), 100.0);
    // A non-positive or non-finite step keeps the slider continuous.
    assert_eq!(geometry::snap(12.3, 0.0, 100.0, 0.0), 12.3);
    assert_eq!(geometry::snap(12.3, 0.0, 100.0, f32::NAN), 12.3);
}

#[test]
fn fractions_are_clamped_and_never_divide_by_zero() {
    assert_eq!(geometry::fraction(25.0, 0.0, 100.0), 0.25);
    assert_eq!(geometry::fraction(-5.0, 0.0, 100.0), 0.0);
    assert_eq!(geometry::fraction(120.0, 0.0, 100.0), 1.0);
    assert_eq!(geometry::fraction(5.0, 5.0, 5.0), 0.0);
    assert_eq!(geometry::fraction(f32::NAN, 0.0, 100.0), 0.0);
}

#[test]
fn painted_geometry_snaps_values_and_rotates_vertical_thumbs() {
    let theme = Theme::light().with_style(StyleId::Luma);
    let metrics = geometry::resolve_metrics(&theme);

    assert!((geometry::snapped_fraction(12.0, 0.0, 100.0, 5.0) - 0.1).abs() < f32::EPSILON);
    assert_eq!(
        geometry::thumb_size(metrics, SliderOrientation::Horizontal),
        Size::new(metrics.thumb_length, metrics.thumb_thickness),
    );
    assert_eq!(
        geometry::thumb_size(metrics, SliderOrientation::Vertical),
        Size::new(metrics.thumb_thickness, metrics.thumb_length),
    );
}

#[test]
fn widget_reserves_room_for_the_thumb_ring() {
    for style in StyleId::ALL {
        let theme = Theme::light().with_style(style);
        let metrics = geometry::resolve_metrics(&theme);

        assert_eq!(
            metrics.cross_size(),
            metrics.thumb_thickness + metrics.ring_width * 2.0,
        );
        assert!(metrics.thumb_thickness > metrics.track_thickness);

        let (metrics, track) = horizontal_layout(&theme);
        assert_eq!(track.x, metrics.ring_width);
        assert_eq!(track.width, 100.0);
        assert_eq!(track.height, metrics.track_thickness);
    }
}

#[test]
fn default_dimensions_follow_the_orientation() {
    let theme = Theme::light();
    let cross = Length::Fixed(geometry::resolve_metrics(&theme).cross_size());

    let horizontal = geometry::resolved_dimensions(&Slider::<Message>::new(&theme));
    assert_eq!(horizontal, (Length::Fill, cross));

    let vertical = geometry::resolved_dimensions(
        &Slider::<Message>::new(&theme).orientation(SliderOrientation::Vertical),
    );
    assert_eq!(vertical, (cross, Length::Fill));
}

#[test]
fn thumbs_stay_inside_the_track_at_both_ends() {
    let theme = Theme::light();
    let (metrics, track) = horizontal_layout(&theme);

    let start = geometry::thumb_center(track, metrics, SliderOrientation::Horizontal, 0.0);
    let end = geometry::thumb_center(track, metrics, SliderOrientation::Horizontal, 1.0);

    assert_eq!(start.x - metrics.thumb_length / 2.0, track.x);
    assert_eq!(end.x + metrics.thumb_length / 2.0, track.x + track.width);
    assert_eq!(start.y, track.y + track.height / 2.0);
    assert_eq!(
        geometry::travel(track, metrics, SliderOrientation::Horizontal),
        track.width - metrics.thumb_length,
    );
}

#[test]
fn vertical_sliders_grow_upwards() {
    let theme = Theme::light();
    let metrics = geometry::resolve_metrics(&theme);
    let bounds = Size::new(metrics.cross_size(), 100.0 + metrics.ring_width * 2.0);
    let track = geometry::track_rect(bounds, metrics, SliderOrientation::Vertical);

    let min = geometry::thumb_center(track, metrics, SliderOrientation::Vertical, 0.0);
    let max = geometry::thumb_center(track, metrics, SliderOrientation::Vertical, 1.0);

    assert!(max.y < min.y, "the maximum sits at the top edge");
    assert_eq!(min.y + metrics.thumb_length / 2.0, track.y + track.height);
    assert_eq!(max.y - metrics.thumb_length / 2.0, track.y);
}

#[test]
fn cursor_positions_map_back_onto_values() {
    let theme = Theme::light();
    let (metrics, track) = horizontal_layout(&theme);
    let slider = Slider::<Message>::new(&theme).continuous();
    let center = geometry::thumb_center(track, metrics, SliderOrientation::Horizontal, 0.5);

    let midpoint = geometry::value_at(&slider, track, metrics, center);
    assert!((midpoint - 50.0).abs() < 0.01, "midpoint value: {midpoint}");

    let before_start = geometry::value_at(&slider, track, metrics, Point::new(-50.0, 0.0));
    assert_eq!(before_start, 0.0);

    let past_end = geometry::value_at(&slider, track, metrics, Point::new(1_000.0, 0.0));
    assert_eq!(past_end, 100.0);

    // Stepped sliders round the mapped value onto the grid.
    let stepped = Slider::<Message>::new(&theme).step(25.0);
    let off_grid = geometry::thumb_center(track, metrics, SliderOrientation::Horizontal, 0.55);
    assert_eq!(geometry::value_at(&stepped, track, metrics, off_grid), 50.0);
}

#[test]
fn vertical_cursor_positions_are_inverted() {
    let theme = Theme::light();
    let metrics = geometry::resolve_metrics(&theme);
    let bounds = Size::new(metrics.cross_size(), 100.0 + metrics.ring_width * 2.0);
    let track = geometry::track_rect(bounds, metrics, SliderOrientation::Vertical);
    let slider = Slider::<Message>::new(&theme)
        .orientation(SliderOrientation::Vertical)
        .continuous();

    let top = geometry::thumb_center(track, metrics, SliderOrientation::Vertical, 1.0);
    let bottom = geometry::thumb_center(track, metrics, SliderOrientation::Vertical, 0.0);

    assert!((geometry::value_at(&slider, track, metrics, top) - 100.0).abs() < 0.01);
    assert!(geometry::value_at(&slider, track, metrics, bottom).abs() < 0.01);
}

#[test]
fn pressing_the_track_picks_the_closest_thumb() {
    let theme = Theme::light();
    let (metrics, track) = horizontal_layout(&theme);
    let slider = Slider::<Message>::new(&theme).values(vec![20.0, 80.0]);

    let near_first = geometry::thumb_center(track, metrics, SliderOrientation::Horizontal, 0.3);
    assert_eq!(
        geometry::closest_thumb(&slider, track, metrics, near_first),
        Some(0),
    );

    let near_second = geometry::thumb_center(track, metrics, SliderOrientation::Horizontal, 0.7);
    assert_eq!(
        geometry::closest_thumb(&slider, track, metrics, near_second),
        Some(1),
    );

    // Stacked thumbs release the one that can follow the cursor.
    let stacked = Slider::<Message>::new(&theme).values(vec![50.0, 50.0]);
    let right = geometry::thumb_center(track, metrics, SliderOrientation::Horizontal, 0.9);
    assert_eq!(
        geometry::closest_thumb(&stacked, track, metrics, right),
        Some(1),
    );
    let left = geometry::thumb_center(track, metrics, SliderOrientation::Horizontal, 0.1);
    assert_eq!(
        geometry::closest_thumb(&stacked, track, metrics, left),
        Some(0),
    );

    let empty = Slider::<Message>::new(&theme).values(Vec::new());
    assert_eq!(geometry::closest_thumb(&empty, track, metrics, right), None);
}

#[test]
fn touch_positions_drive_slider_without_a_mouse_cursor() {
    let theme = Theme::light();
    let slider = Slider::new(&theme).on_change(Message::Changed);
    let (metrics, track) = horizontal_layout(&theme);
    let bounds = Rectangle::new(
        Point::new(20.0, 30.0),
        Size::new(100.0 + metrics.ring_width * 2.0, metrics.cross_size()),
    );
    let midpoint = geometry::thumb_center(track, metrics, SliderOrientation::Horizontal, 0.5);
    let end = geometry::thumb_center(track, metrics, SliderOrientation::Horizontal, 1.0);
    let finger = touch::Finger(7);
    let mut state = SliderState::default();

    let pressed = canvas::Event::Touch(touch::Event::FingerPressed {
        id: finger,
        position: Point::new(bounds.x + midpoint.x, bounds.y + midpoint.y),
    });
    let action = <Slider<'_, Message> as canvas::Program<Message>>::update(
        &slider,
        &mut state,
        &pressed,
        bounds,
        mouse::Cursor::Unavailable,
    )
    .expect("touch press should publish a value");
    let (message, _, _) = action.into_inner();
    assert!(matches!(message, Some(Message::Changed(value)) if (value - 50.0).abs() < 0.01));
    assert_eq!(state.dragging, Some(0));
    assert_eq!(state.active_finger, Some(finger));

    let moved = canvas::Event::Touch(touch::Event::FingerMoved {
        id: finger,
        position: Point::new(bounds.x + end.x, bounds.y + end.y),
    });
    let action = <Slider<'_, Message> as canvas::Program<Message>>::update(
        &slider,
        &mut state,
        &moved,
        bounds,
        mouse::Cursor::Unavailable,
    )
    .expect("touch move should publish a value");
    let (message, _, _) = action.into_inner();
    assert!(matches!(message, Some(Message::Changed(value)) if (value - 100.0).abs() < 0.01));
}

#[test]
fn hit_testing_only_reports_thumbs_under_the_cursor() {
    let theme = Theme::light();
    let (metrics, track) = horizontal_layout(&theme);
    let slider = Slider::<Message>::new(&theme).values(vec![0.0, 100.0]);

    let first = geometry::thumb_center(track, metrics, SliderOrientation::Horizontal, 0.0);
    assert_eq!(geometry::thumb_at(&slider, track, metrics, first), Some(0));

    let last = geometry::thumb_center(track, metrics, SliderOrientation::Horizontal, 1.0);
    assert_eq!(geometry::thumb_at(&slider, track, metrics, last), Some(1));

    let middle = geometry::thumb_center(track, metrics, SliderOrientation::Horizontal, 0.5);
    assert_eq!(geometry::thumb_at(&slider, track, metrics, middle), None);
}

#[test]
fn thumbs_cannot_cross_their_neighbors() {
    let theme = Theme::light();
    let slider = Slider::<Message>::new(&theme).values(vec![20.0, 50.0, 80.0]);

    assert_eq!(slider.clamp_to_neighbors(1, 95.0), 80.0);
    assert_eq!(slider.clamp_to_neighbors(1, 5.0), 20.0);
    assert_eq!(slider.clamp_to_neighbors(1, 60.0), 60.0);
    // The outer thumbs are bounded by the range itself.
    assert_eq!(slider.clamp_to_neighbors(0, -10.0), 0.0);
    assert_eq!(slider.clamp_to_neighbors(2, 150.0), 100.0);
}

#[test]
fn change_actions_publish_the_configured_callback() {
    let theme = Theme::light();

    let single = Slider::new(&theme).value(10.0).on_change(Message::Changed);
    let (message, _, _) = single.change_action(0, 30.0).into_inner();
    assert_eq!(message, Some(Message::Changed(30.0)));

    let multiple = Slider::new(&theme)
        .values(vec![10.0, 60.0])
        .on_change_values(Message::ChangedValues);
    let (message, _, _) = multiple.change_action(0, 30.0).into_inner();
    assert_eq!(message, Some(Message::ChangedValues(vec![30.0, 60.0])));

    // Neighbors still clamp the published value.
    let (message, _, _) = multiple.change_action(0, 90.0).into_inner();
    assert_eq!(message, Some(Message::ChangedValues(vec![60.0, 60.0])));

    // An unchanged value and a missing callback publish nothing.
    let (message, _, _) = multiple.change_action(0, 10.0).into_inner();
    assert_eq!(message, None);
    let (message, _, _) = Slider::<Message>::new(&theme)
        .value(10.0)
        .change_action(0, 30.0)
        .into_inner();
    assert_eq!(message, None);
    let (message, _, _) = single.change_action(7, 30.0).into_inner();
    assert_eq!(message, None);
}

#[test]
fn range_fractions_span_single_and_multiple_thumbs() {
    let theme = Theme::light();

    let single = Slider::<Message>::new(&theme).value(25.0);
    assert_eq!(single.range_fractions(), Some((0.25, 0.25)));

    let multiple = Slider::<Message>::new(&theme).values(vec![75.0, 25.0]);
    assert_eq!(multiple.range_fractions(), Some((0.25, 0.75)));

    let empty = Slider::<Message>::new(&theme).values(Vec::new());
    assert_eq!(empty.range_fractions(), None);
}

#[test]
fn radius_presets_stay_within_the_painted_box() {
    let theme = Theme::light();
    let track = Size::new(120.0, 6.0);

    assert_eq!(geometry::radius_px(&theme, SliderRadius::Full, track), 3.0);
    assert_eq!(geometry::radius_px(&theme, SliderRadius::None, track), 0.0);
    assert_eq!(
        geometry::radius_px(&theme, SliderRadius::Custom(999.0), track),
        3.0,
    );
    assert_eq!(
        geometry::radius_px(&theme, SliderRadius::Custom(f32::NAN), track),
        0.0,
    );
}

#[test]
fn default_radii_follow_the_style_pack() {
    assert_eq!(
        geometry::default_track_radius(&Theme::light().with_style(StyleId::Lyra)),
        SliderRadius::None,
    );
    assert_eq!(
        geometry::default_thumb_radius(&Theme::light().with_style(StyleId::Vega)),
        SliderRadius::Full,
    );
    assert_eq!(
        geometry::default_thumb_radius(&Theme::light().with_style(StyleId::Mira)),
        SliderRadius::Medium,
    );
}

#[test]
fn colors_follow_the_style_pack_recipe() {
    let theme = Theme::light();
    let vega = style_for(&Slider::<Message>::new(&theme), SliderStatus::default());
    assert_eq!(vega.range, theme.palette.primary);
    assert_eq!(
        vega.track,
        theme.semantic_color(crate::SemanticColor::Muted)
    );
    assert_eq!(vega.thumb, Color::WHITE);
    // Vega draws a `border-primary` hairline around the thumb.
    assert_eq!(vega.thumb_border, theme.palette.primary);
    assert_eq!(vega.thumb_border_width, 1.0);

    // Sera fills the thumb with `primary` and drops the border entirely.
    let sera_theme = Theme::light().with_style(StyleId::Sera);
    let sera = style_for(
        &Slider::<Message>::new(&sera_theme),
        SliderStatus::default(),
    );
    assert_eq!(sera.thumb, sera_theme.palette.primary);
    assert_eq!(sera.thumb_border_width, 0.0);
    // `bg-input/50`.
    let input = sera_theme.semantic_color(crate::SemanticColor::Input);
    assert!((sera.track.a - input.a * 0.5).abs() < f32::EPSILON);
}

#[test]
fn accent_overlay_and_explicit_colors_win_over_theme_tokens() {
    let theme = Theme::light();

    let accent = Slider::<Message>::new(&theme).color(AccentColor::Blue);
    assert_ne!(
        style_for(&accent, SliderStatus::default()).range,
        theme.palette.primary,
    );

    let explicit = Slider::<Message>::new(&theme)
        .range_color(Color::BLACK)
        .track_color(Color::WHITE)
        .thumb_color(Color::from_rgb(1.0, 0.0, 0.0));
    let style = style_for(&explicit, SliderStatus::default());
    assert_eq!(style.range, Color::BLACK);
    assert_eq!(style.track, Color::WHITE);
    assert_eq!(style.thumb, Color::from_rgb(1.0, 0.0, 0.0));
}

#[test]
fn disabled_sliders_are_painted_at_half_opacity() {
    let theme = Theme::light();
    let slider = Slider::<Message>::new(&theme);

    let enabled = style_for(&slider, SliderStatus::default());
    let disabled = style_for(
        &slider,
        SliderStatus {
            disabled: true,
            ..SliderStatus::default()
        },
    );

    assert!((disabled.range.a - enabled.range.a * 0.5).abs() < f32::EPSILON);
    assert!((disabled.track.a - enabled.track.a * 0.5).abs() < f32::EPSILON);
    assert!((disabled.ring.a - enabled.ring.a * 0.5).abs() < f32::EPSILON);
}

#[test]
fn ring_alpha_follows_the_pack_and_every_pack_resolves() {
    for style in StyleId::ALL {
        let theme = Theme::light().with_style(style);
        let recipe = theme.style.slider();
        let resolved = style_for(
            &Slider::<Message>::new(&theme),
            SliderStatus {
                hovered: true,
                ..SliderStatus::default()
            },
        );

        assert!((resolved.ring.a - recipe.ring_opacity).abs() < f32::EPSILON);
        assert_eq!(resolved.ring_width, recipe.ring_width_px);
        assert!(resolved.track.a.is_finite());
        assert!(resolved.thumb.a.is_finite());
    }
}

#[test]
fn style_override_patches_the_resolved_style() {
    let theme = Theme::light();
    let slider = Slider::<Message>::new(&theme).style_override(|style, _status| SliderStyle {
        thumb: Color::BLACK,
        ..style
    });

    let patched = slider.style_override.as_ref().expect("override is stored")(
        style_for(&slider, SliderStatus::default()),
        SliderStatus::default(),
    );

    assert_eq!(patched.thumb, Color::BLACK);
}

#[test]
fn callbacks_and_conversions_build_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = Slider::new(&theme).on_change(Message::Changed).into();
    let _: Element<'_, Message> = Slider::new(&theme)
        .values(vec![10.0, 20.0])
        .on_change_values(Message::ChangedValues)
        .on_release(Message::Released)
        .into();
    let _: Element<'_, Message> = slider(Slider::new(&theme)).into();

    let released = Slider::<Message>::new(&theme).on_release(Message::Released);
    let on_release = released.on_release.as_ref().expect("callback is stored");
    assert_eq!(on_release(), Message::Released);
}

#[test]
fn continuous_clears_the_step_and_min_length_matches_the_pack() {
    let theme = Theme::light();

    assert_eq!(Slider::<Message>::new(&theme).continuous().step, 0.0);
    assert_eq!(Slider::<Message>::new(&theme).min_length(), 160.0);
}

#[test]
fn state_defaults_to_no_interaction() {
    let state = SliderState::default();

    assert_eq!(state.dragging, None);
    assert_eq!(state.hovered, None);
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let slider = Slider::<NoDebugMessage>::new(&theme)
        .value(42.0)
        .on_change(|_| NoDebugMessage);
    let debug = format!("{slider:?}");

    assert!(debug.contains("Slider"));
    assert!(debug.contains("42.0"));
    assert!(debug.contains("single"));
}
