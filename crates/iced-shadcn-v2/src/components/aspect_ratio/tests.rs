//! Behavioral tests for the aspect-ratio component.

use crate::iced_compat::advanced::layout;
use crate::iced_compat::widget::text;
use crate::iced_compat::{Color, Element, Length, Size};

use super::render::resolve_bounds;
use super::*;
use crate::theme::Theme;

#[test]
fn aspect_ratio_defaults_match_shadcn_reference() {
    let frame: AspectRatio<'_, ()> = AspectRatio::new(text("content"));

    assert!((frame.ratio - 1.0).abs() < f32::EPSILON);
    assert_eq!(frame.ratio, AspectRatio::<()>::new(text("other")).ratio);
    assert!(frame.background.is_none());
    assert!(frame.radius.abs() < f32::EPSILON);
    assert!(!frame.clip);
    assert!(frame.style_override.is_none());
}

#[test]
fn ratio_is_clamped_to_a_positive_minimum() {
    let frame: AspectRatio<'_, ()> = AspectRatio::new(text("content")).ratio(0.0);
    assert!((frame.resolved_ratio() - MIN_ASPECT_RATIO).abs() < f32::EPSILON);

    let frame: AspectRatio<'_, ()> = AspectRatio::new(text("content")).ratio(-4.0);
    assert!((frame.resolved_ratio() - MIN_ASPECT_RATIO).abs() < f32::EPSILON);

    let frame: AspectRatio<'_, ()> = AspectRatio::new(text("content")).ratio(f32::NAN);
    assert!((frame.resolved_ratio() - MIN_ASPECT_RATIO).abs() < f32::EPSILON);
}

#[test]
fn common_demo_ratios_are_preserved() {
    for ratio in [16.0 / 9.0, 21.0 / 9.0, 1.0, 9.0 / 16.0] {
        let frame: AspectRatio<'_, ()> = AspectRatio::new(text("content")).ratio(ratio);
        assert!((frame.resolved_ratio() - ratio).abs() < f32::EPSILON);
    }
}

#[test]
fn muted_and_color_overrides_apply_background() {
    let theme = Theme::light();

    let muted: AspectRatio<'_, ()> = AspectRatio::new(text("content")).muted(&theme);
    assert_eq!(muted.background, Some(theme.palette.muted));

    let custom: AspectRatio<'_, ()> = AspectRatio::new(text("content")).background(Color::WHITE);
    assert_eq!(custom.background, Some(Color::WHITE));
}

#[test]
fn radius_is_clamped_to_at_least_zero() {
    let frame: AspectRatio<'_, ()> = AspectRatio::new(text("content")).radius(-3.0);
    assert!(frame.radius.abs() < f32::EPSILON);

    let frame: AspectRatio<'_, ()> = AspectRatio::new(text("content")).radius(12.0);
    assert!((frame.radius - 12.0).abs() < f32::EPSILON);
}

#[test]
fn builder_converts_to_element() {
    let theme = Theme::light();
    let _: Element<'_, ()> = AspectRatio::new(text("16:9"))
        .ratio(16.0 / 9.0)
        .muted(&theme)
        .into();

    let _: Element<'_, ()> = aspect_ratio(
        AspectRatio::new(text("1:1"))
            .ratio(1.0)
            .style_override(|style| style),
    );
}

#[test]
fn resolve_bounds_prefers_width_then_shrinks_to_max_height() {
    let limits = layout::Limits::new(Size::new(0.0, 0.0), Size::new(320.0, 180.0));

    let wide = resolve_bounds(16.0 / 9.0, &limits);
    assert!((wide.width - 320.0).abs() < f32::EPSILON);
    assert!((wide.height - 180.0).abs() < f32::EPSILON);

    let tall_limits = layout::Limits::new(Size::new(0.0, 0.0), Size::new(200.0, 400.0));
    let portrait = resolve_bounds(9.0 / 16.0, &tall_limits);
    assert!((portrait.width - 200.0).abs() < f32::EPSILON);
    assert!((portrait.height - (200.0 / (9.0 / 16.0))).abs() < f32::EPSILON);
}

#[test]
fn resolve_bounds_honors_minimum_size() {
    let limits = layout::Limits::new(Size::new(120.0, 80.0), Size::new(320.0, 240.0));

    let bounds = resolve_bounds(1.0, &limits);
    assert!(bounds.width >= 120.0);
    assert!(bounds.height >= 80.0);
}

#[test]
fn clip_flag_is_configurable() {
    let frame: AspectRatio<'_, ()> = AspectRatio::new(text("content")).clip(true);
    assert!(frame.clip);
}

#[test]
fn style_override_is_retained() {
    let frame = AspectRatio::new(text("content")).style_override(|style| style);
    assert!(frame.style_override.is_some());

    let _: Element<'_, ()> = frame.into();
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let frame = AspectRatio::<NoDebugMessage>::new(text("content"));
    let debug = format!("{frame:?}");

    assert!(debug.contains("AspectRatio"));
    assert!(debug.contains("ratio"));
}

#[test]
fn into_element_matches_helper() {
    let config = AspectRatio::new(text("content")).ratio(4.0 / 3.0);
    let from_trait: Element<'_, ()> = config.into();
    let from_helper: Element<'_, ()> =
        aspect_ratio(AspectRatio::new(text("content")).ratio(4.0 / 3.0));

    assert_eq!(from_trait.as_widget().size().width, Length::Fill);
    assert_eq!(from_helper.as_widget().size().width, Length::Fill);
}
