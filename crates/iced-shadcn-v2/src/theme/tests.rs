//! Behavioral tests for theme resolution and color conversion.

use shadcn_common::AccentColor;
use twill_core::tokens::ColorValue;

use super::Theme;
use super::palette::color_value_to_iced;

#[test]
fn light_neutral_has_bright_background() {
    let theme = Theme::light();
    assert!(theme.palette.background.r > 0.9);
    assert!(!theme.is_dark());
}

#[test]
fn accent_overlay_changes_primary() {
    let base = Theme::light();
    let amber = base.clone().with_accent(Some(AccentColor::Amber));
    assert_ne!(base.palette.primary, amber.palette.primary);
}

#[test]
fn color_value_conversion_preserves_alpha() {
    let value = ColorValue::from_oklch(0.6, 0.1, 200.0).with_alpha(0.5);
    let color = color_value_to_iced(value);
    assert!((color.a - 0.5).abs() < f32::EPSILON);
}
