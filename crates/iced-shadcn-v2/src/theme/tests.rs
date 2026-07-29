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

/// C-SEND-SYNC: theme values and configuration types stay thread-safe.
#[test]
fn theme_and_config_types_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<Theme>();
    assert_send_sync::<super::Palette>();
    assert_send_sync::<crate::ButtonVariant>();
    assert_send_sync::<crate::ButtonSize>();
    assert_send_sync::<crate::ButtonBuildError>();
    assert_send_sync::<crate::BadgeBuildError>();
    assert_send_sync::<crate::InputBuildError>();
    assert_send_sync::<crate::KbdBuildError>();
    assert_send_sync::<crate::ScrollAreaBuildError>();
    assert_send_sync::<crate::ScrollAreaScrollbar>();
    assert_send_sync::<crate::CheckboxConfig>();
    assert_send_sync::<crate::SliderStyle>();
    assert_send_sync::<crate::SwitchStyle>();
}
