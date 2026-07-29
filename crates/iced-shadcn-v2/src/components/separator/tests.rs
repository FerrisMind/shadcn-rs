//! Behavioral tests for the separator component.

use crate::iced_compat::{Color, Length};

use super::*;
use crate::theme::Theme;

#[test]
fn separator_defaults_match_shadcn_reference() {
    let theme = Theme::light();
    let rule = Separator::new(&theme);

    assert_eq!(rule.orientation, SeparatorOrientation::Horizontal);
    assert_eq!(rule.orientation, SeparatorOrientation::default());
    assert!((rule.thickness - 1.0).abs() < f32::EPSILON);
    assert!(rule.radius.abs() < f32::EPSILON);
    assert_eq!(rule.length, Length::Fill);
    assert!(!rule.decorative);
    assert_eq!(rule.color, theme.palette.border);
}

#[test]
fn separator_thickness_is_clamped_to_at_least_one_pixel() {
    let rule = Separator::from_color(Color::BLACK).thickness(0.0);
    assert!((rule.thickness - 1.0).abs() < f32::EPSILON);

    let rule = Separator::from_color(Color::BLACK).thickness(-4.0);
    assert!((rule.thickness - 1.0).abs() < f32::EPSILON);
}

#[test]
fn horizontal_separator_fills_width_with_fixed_height() {
    let rule = Separator::from_color(Color::BLACK);
    let (width, height) = rule.resolved_axes();

    assert_eq!(width, Length::Fill);
    assert_eq!(height, Length::Fixed(1.0));
}

#[test]
fn vertical_separator_fills_height_with_fixed_width() {
    let rule = Separator::from_color(Color::BLACK)
        .orientation(SeparatorOrientation::Vertical)
        .thickness(2.0);
    let (width, height) = rule.resolved_axes();

    assert_eq!(width, Length::Fixed(2.0));
    assert_eq!(height, Length::Fill);
}

#[test]
fn separator_length_override_applies_to_main_axis() {
    let rule = Separator::from_color(Color::BLACK).length(Length::Fixed(120.0));
    assert_eq!(
        rule.resolved_axes(),
        (Length::Fixed(120.0), Length::Fixed(1.0))
    );

    let rule = rule.orientation(SeparatorOrientation::Vertical);
    assert_eq!(
        rule.resolved_axes(),
        (Length::Fixed(1.0), Length::Fixed(120.0))
    );
}

#[test]
fn separator_radius_is_clamped_to_at_least_zero() {
    let rule = Separator::from_color(Color::BLACK).radius(-3.0);
    assert!(rule.radius.abs() < f32::EPSILON);

    let rule = Separator::from_color(Color::BLACK)
        .thickness(8.0)
        .radius(4.0);
    assert!((rule.radius - 4.0).abs() < f32::EPSILON);
}

#[test]
fn separator_color_override_beats_theme_border() {
    let theme = Theme::light();
    let rule = Separator::new(&theme).color(Color::WHITE);
    assert_eq!(rule.color, Color::WHITE);
}
