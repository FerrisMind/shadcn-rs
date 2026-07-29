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
fn background_foreground_uses_the_theme_token() {
    use twill_core::prelude::theme::SemanticColor;

    for theme in [Theme::light(), Theme::dark()] {
        assert_eq!(
            theme.semantic_foreground(SemanticColor::Background),
            theme.palette.foreground,
        );
    }
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
    assert_send_sync::<crate::AlertRadius>();
    assert_send_sync::<crate::AlertVariant>();
    assert_send_sync::<crate::AvatarRadius>();
    assert_send_sync::<crate::AvatarSize>();
    assert_send_sync::<crate::BadgeBuildError>();
    assert_send_sync::<crate::BadgeRadius>();
    assert_send_sync::<crate::BadgeVariant>();
    assert_send_sync::<crate::ButtonBuildError>();
    assert_send_sync::<crate::ButtonRadius>();
    assert_send_sync::<crate::ButtonSize>();
    assert_send_sync::<crate::ButtonVariant>();
    assert_send_sync::<crate::CardBorder>();
    assert_send_sync::<crate::CardFooterAlignment>();
    assert_send_sync::<crate::CardFooterDirection>();
    assert_send_sync::<crate::CardRadius>();
    assert_send_sync::<crate::CardSize>();
    assert_send_sync::<crate::CheckboxConfig>();
    assert_send_sync::<crate::CheckboxSize>();
    assert_send_sync::<crate::CheckboxState>();
    assert_send_sync::<crate::CheckboxVariant>();
    assert_send_sync::<crate::InputBuildError>();
    assert_send_sync::<crate::InputRadius>();
    assert_send_sync::<crate::InputSize>();
    assert_send_sync::<crate::KbdBuildError>();
    assert_send_sync::<crate::KbdRadius>();
    assert_send_sync::<crate::KbdSurface>();
    assert_send_sync::<crate::LabelContext>();
    assert_send_sync::<crate::ProgressOrientation>();
    assert_send_sync::<crate::ProgressRadius>();
    assert_send_sync::<crate::ProgressSize>();
    assert_send_sync::<crate::ProgressVariant>();
    assert_send_sync::<crate::ScrollAreaAnchor>();
    assert_send_sync::<crate::ScrollAreaBuildError>();
    assert_send_sync::<crate::ScrollAreaOrientation>();
    assert_send_sync::<crate::ScrollAreaRadius>();
    assert_send_sync::<crate::ScrollAreaScrollbar>();
    assert_send_sync::<crate::SeparatorOrientation>();
    assert_send_sync::<crate::SkeletonAnimation>();
    assert_send_sync::<crate::SkeletonFill>();
    assert_send_sync::<crate::SkeletonRadius>();
    assert_send_sync::<crate::SkeletonShape>();
    assert_send_sync::<crate::SliderOrientation>();
    assert_send_sync::<crate::SliderRadius>();
    assert_send_sync::<crate::SliderStyle>();
    assert_send_sync::<crate::SpinnerSize>();
    assert_send_sync::<crate::SpinnerVariant>();
    assert_send_sync::<crate::SwitchRadius>();
    assert_send_sync::<crate::SwitchSize>();
    assert_send_sync::<crate::SwitchStyle>();
    assert_send_sync::<crate::ToggleRadius>();
    assert_send_sync::<crate::ToggleSize>();
    assert_send_sync::<crate::ToggleVariant>();
    assert_send_sync::<crate::TypographyVariant>();
}

/// C-COMMON-TRAITS: `Theme::default()` matches the documented light theme.
#[test]
fn default_theme_is_light() {
    let theme = Theme::default();
    assert_eq!(theme, Theme::light());
    assert!(!theme.is_dark());
}
