//! Behavioral tests for the progress component.

use std::time::Duration;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::{Color, Element, Length, Rectangle, mouse, time, window};
use shadcn_common::{AccentColor, Easing, StyleId};

use super::geometry::{default_height, default_radius, display_ratio, normalized_ratio, radius_px};
use super::style::resolve_visual;
use super::*;
use crate::theme::Theme;

#[test]
fn defaults_match_shadcn_svelte_progress() {
    let theme = Theme::light();
    let progress = Progress::new(&theme);

    assert_eq!(progress.value, None);
    assert_eq!(progress.max, 100.0);
    assert_eq!(progress.variant, ProgressVariant::Surface);
    assert_eq!(progress.orientation, ProgressOrientation::Horizontal);
    assert_eq!(progress.size, ProgressSize::Default);
    assert_eq!(progress.radius, None);
    assert_eq!(progress.width, None);
    assert_eq!(progress.height, None);
    assert_eq!(default_height(&theme), 6.0);
    assert_eq!(default_radius(&theme), ProgressRadius::Full);
}

#[test]
fn style_pack_progress_geometry_matches_source_css() {
    let expected = [
        (StyleId::Vega, 6.0, ProgressRadius::Full),
        (StyleId::Nova, 4.0, ProgressRadius::Full),
        (StyleId::Maia, 12.0, ProgressRadius::Custom(26.0)),
        (StyleId::Lyra, 4.0, ProgressRadius::None),
        (StyleId::Mira, 4.0, ProgressRadius::Medium),
        (StyleId::Luma, 12.0, ProgressRadius::Full),
        (StyleId::Sera, 2.0, ProgressRadius::None),
        (StyleId::Rhea, 8.0, ProgressRadius::Custom(18.0)),
    ];

    for (style, height, radius) in expected {
        let theme = Theme::light().with_style(style);
        assert_eq!(default_height(&theme), height);
        assert_eq!(default_radius(&theme), radius);
    }
}

#[test]
fn values_are_clamped_and_invalid_max_is_safe() {
    assert_eq!(normalized_ratio(Some(-10.0), 100.0), 0.0);
    assert_eq!(normalized_ratio(Some(50.0), 100.0), 0.5);
    assert_eq!(normalized_ratio(Some(150.0), 100.0), 1.0);
    assert_eq!(normalized_ratio(Some(f32::NAN), 100.0), 0.0);
    assert_eq!(normalized_ratio(Some(50.0), 0.0), 0.0);
    assert_eq!(normalized_ratio(None, 100.0), 0.0);

    let theme = Theme::light();
    let progress = Progress::new(&theme).value(f32::NAN).max(-1.0);
    assert_eq!(progress.value, Some(0.0));
    assert_eq!(progress.max, 1.0);
}

#[test]
fn builder_exposes_svelte_and_iced_capabilities() {
    let theme = Theme::dark();
    let progress = Progress::new(&theme)
        .value(66.0)
        .max(120.0)
        .size(ProgressSize::Custom(10.0))
        .variant(ProgressVariant::Soft)
        .orientation(ProgressOrientation::Vertical)
        .radius(ProgressRadius::Custom(3.0))
        .color(AccentColor::Blue)
        .track_color(Color::from_rgb(0.1, 0.2, 0.3))
        .high_contrast(true)
        .animated(false)
        .width(Length::Fixed(18.0))
        .height(Length::Fixed(160.0))
        .duration(Duration::from_millis(240));

    assert_eq!(progress.value, Some(66.0));
    assert_eq!(progress.max, 120.0);
    assert_eq!(progress.size, ProgressSize::Custom(10.0));
    assert_eq!(progress.variant, ProgressVariant::Soft);
    assert_eq!(progress.orientation, ProgressOrientation::Vertical);
    assert_eq!(progress.radius, Some(ProgressRadius::Custom(3.0)));
    assert_eq!(progress.color, Some(AccentColor::Blue));
    assert_eq!(progress.track_color, Some(Color::from_rgb(0.1, 0.2, 0.3)));
    assert!(progress.high_contrast);
    assert!(!progress.animated);
    assert_eq!(progress.width, Some(Length::Fixed(18.0)));
    assert_eq!(progress.height, Some(Length::Fixed(160.0)));
    assert_eq!(progress.transition_duration, Duration::from_millis(240));
    assert_eq!(progress.indeterminate_duration, Duration::from_millis(240));
}

#[test]
fn indeterminate_and_value_maybe_are_reversible() {
    let theme = Theme::light();
    let progress = Progress::new(&theme).value(25.0).value_maybe(None);
    assert_eq!(progress.value, None);

    let progress = progress.value_maybe(Some(75.0));
    assert_eq!(progress.value, Some(75.0));

    let progress = progress.indeterminate().value(10.0).theme_primary();
    assert_eq!(progress.value, Some(10.0));
    assert_eq!(progress.color, None);
    assert_eq!(progress.custom_indicator_color, None);
}

#[test]
fn radius_is_capped_to_the_track_bounds() {
    let theme = Theme::light();
    let progress = Progress::new(&theme).radius(ProgressRadius::Full);
    assert_eq!(radius_px(&theme, &progress, 100.0, 8.0), 4.0);

    let progress = Progress::new(&theme).radius(ProgressRadius::Custom(-4.0));
    assert_eq!(radius_px(&theme, &progress, 100.0, 8.0), 0.0);
}

#[test]
fn custom_colors_override_theme_resolution() {
    let theme = Theme::light();
    let custom_indicator = Color::from_rgb(0.1, 0.2, 0.3);
    let custom_track = Color::from_rgb(0.3, 0.2, 0.1);
    let progress = Progress::new(&theme)
        .custom_color(custom_indicator)
        .track_color(custom_track);
    let visual = resolve_visual(&progress, &theme);

    assert_eq!(visual.indicator, custom_indicator);
    assert_eq!(visual.track, custom_track);
}

#[test]
fn progress_converts_to_canvas_and_element() {
    let theme = Theme::light();
    let progress = Progress::new(&theme).value(33.0);
    let _ = progress.into_canvas::<()>();

    let _: Element<'_, ()> = Progress::new(&theme).value(33.0).into();
}

#[test]
fn public_debug_is_non_empty() {
    let theme = Theme::light();
    let debug = format!("{:?}", Progress::new(&theme));
    assert!(debug.contains("Progress"));
}

#[test]
fn display_ratio_tracks_live_value_when_idle() {
    // No active transition: the painted ratio must follow `value`/`max`
    // immediately, so moving a slider updates a resting bar instead of it
    // staying frozen on a stale transition value.
    let idle = ProgressState::default();
    assert_eq!(display_ratio(&idle, Some(30.0), 100.0), 0.30);
    assert_eq!(display_ratio(&idle, Some(90.0), 100.0), 0.90);

    // Active transition: the shared scalar value drives the animation.
    let now = crate::iced_compat::time::Instant::now();
    let mut animating = ProgressState::default();
    animating.transition.reset(0.66);
    animating
        .transition
        .advance(0.9, true, Duration::from_millis(100), Easing::Linear, now);
    assert_eq!(display_ratio(&animating, Some(30.0), 100.0), 0.66);
}

#[test]
fn settled_determinate_progress_does_not_schedule_redraws() {
    let theme = Theme::light();
    let progress = Progress::new(&theme).value(50.0).animated(true);
    let mut state = ProgressState::default();
    let now = time::Instant::now();
    let event = canvas::Event::Window(window::Event::RedrawRequested(now));

    let action = <Progress<'_> as canvas::Program<()>>::update(
        &progress,
        &mut state,
        &event,
        Rectangle::with_size(crate::iced_compat::Size::new(100.0, 6.0)),
        mouse::Cursor::Unavailable,
    );

    assert!(action.is_none());
    assert!(!state.transition.is_running());
}
