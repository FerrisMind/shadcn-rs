//! Behavioral tests for the input component.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::iced_compat::widget::text_input as text_input_widget;
use crate::iced_compat::{Color, Element, Length};
use shadcn_common::AccentColor;
use twill_core::prelude::theme::SemanticColor;
use twill_core::prelude::{Padding, PaddingValue, PaddingVar, Spacing};

use super::geometry;
use super::style;
use super::*;
use crate::theme::Theme;

#[derive(Clone, Debug)]
enum Message {
    Changed(String),
    Submitted,
}

const FOCUSED: text_input_widget::Status = text_input_widget::Status::Focused { is_hovered: false };

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let input: Input<'_, Message> = Input::new(&theme)
        .value("hello")
        .placeholder("Type here")
        .size(InputSize::Lg)
        .radius(InputRadius::Full)
        .color(AccentColor::Blue)
        .secure(true)
        .disabled(true)
        .invalid(true)
        .on_input(Message::Changed)
        .on_submit(Message::Submitted)
        .on_paste(Message::Changed)
        .style_override(|style, _| style);

    assert_eq!(input.value.as_ref(), "hello");
    assert_eq!(input.placeholder.as_ref(), "Type here");
    assert_eq!(input.size, InputSize::Lg);
    assert_eq!(input.radius, Some(InputRadius::Full));
    assert_eq!(input.color, Some(AccentColor::Blue));
    assert!(input.secure);
    assert!(input.disabled);
    assert!(input.invalid);
    assert!(input.on_input.is_some());
    assert!(input.on_submit.is_some());
    assert!(input.on_paste.is_some());
    assert!(input.style_override.is_some());
    assert!(std::ptr::eq(input.theme, &theme));

    let callback = input.on_input.as_ref().expect("on_input was set");
    assert!(matches!(callback("x".to_owned()), Message::Changed(text) if text == "x"));
}

#[test]
fn builder_and_helper_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = Input::new(&theme)
        .placeholder("Email")
        .on_input(Message::Changed)
        .into();

    let _: Element<'_, Message> = input("Email", "user@example.com", &theme)
        .on_input(Message::Changed)
        .into();
}

#[test]
fn default_width_fills_like_w_full() {
    let theme = Theme::light();
    let input = Input::<Message>::new(&theme);

    assert_eq!(input.width, Length::Fill);
}

#[test]
fn control_heights_match_the_pack_ladder() {
    let vega = Theme::light();
    assert_eq!(InputSize::Sm.control_height(&vega), 32.0);
    assert_eq!(InputSize::Default.control_height(&vega), 36.0);
    assert_eq!(InputSize::Lg.control_height(&vega), 40.0);

    // `.cn-input` heights: Mira h-7, Sera h-10.
    let mira = Theme::light().with_style(shadcn_common::StyleId::Mira);
    assert_eq!(InputSize::Default.control_height(&mira), 28.0);

    let sera = Theme::light().with_style(shadcn_common::StyleId::Sera);
    assert_eq!(InputSize::Default.control_height(&sera), 40.0);
}

#[test]
fn default_padding_reproduces_the_control_height() {
    for style_id in [
        shadcn_common::StyleId::Vega,
        shadcn_common::StyleId::Nova,
        shadcn_common::StyleId::Mira,
        shadcn_common::StyleId::Sera,
        shadcn_common::StyleId::Rhea,
    ] {
        let theme = Theme::light().with_style(style_id);
        let text_size = style::pack_text_size(&theme);
        let padding = geometry::default_padding(&theme, InputSize::Default, text_size);
        let height = padding.top + padding.bottom + geometry::line_height_px(text_size);

        assert_eq!(height, InputSize::Default.control_height(&theme));
        assert_eq!(padding.left, padding.right);
        // Optical nudge shifts py upward; total vertical pad is unchanged.
        assert!(padding.bottom >= padding.top);
    }
}

#[test]
fn padding_maps_all_four_sides() {
    let padding = Padding::individual_value(
        PaddingValue::Px(1.0),
        PaddingValue::Px(2.0),
        PaddingValue::Px(3.0),
        PaddingValue::Px(4.0),
    );

    let resolved = geometry::resolve_padding(padding).expect("pixel padding is supported");

    assert_eq!(resolved.top, 1.0);
    assert_eq!(resolved.right, 2.0);
    assert_eq!(resolved.bottom, 3.0);
    assert_eq!(resolved.left, 4.0);
}

#[test]
fn padding_variable_returns_a_descriptive_error() {
    let theme = Theme::light();
    let error = Input::<Message>::new(&theme)
        .padding(Padding::individual_value(
            PaddingValue::Var(PaddingVar::new("--input-padding")),
            PaddingValue::Px(2.0),
            PaddingValue::Px(3.0),
            PaddingValue::Px(4.0),
        ))
        .expect_err("padding variables are unsupported");

    assert_eq!(
        error,
        InputBuildError::UnsupportedPaddingVariable {
            name: "--input-padding"
        }
    );
    assert!(error.to_string().contains("--input-padding"));
}

#[test]
fn padding_auto_returns_a_descriptive_error() {
    let theme = Theme::light();
    let error = Input::<Message>::new(&theme)
        .padding(Padding::all(Spacing::Auto))
        .expect_err("auto padding is unsupported");

    assert_eq!(error, InputBuildError::UnsupportedPaddingAuto);
    assert!(error.to_string().contains("auto"));
}

#[test]
fn base_style_uses_the_input_border() {
    let theme = Theme::light();
    let style = style::resolve_input_style(
        &theme,
        None,
        None,
        false,
        false,
        text_input_widget::Status::Active,
    );

    assert_eq!(style.border.width, 1.0);
    assert_eq!(
        style.border.color,
        theme.semantic_color(SemanticColor::Input)
    );
    assert_eq!(style.value, theme.semantic_color(SemanticColor::Foreground));
    assert_eq!(
        style.placeholder,
        theme.semantic_color(SemanticColor::MutedForeground)
    );
}

#[test]
fn focused_style_recolors_the_border_with_ring() {
    let theme = Theme::light();
    let style = style::resolve_input_style(&theme, None, None, false, false, FOCUSED);

    assert_eq!(
        style.border.color,
        theme.semantic_color(SemanticColor::Ring)
    );
}

#[test]
fn invalid_outranks_the_focus_treatment() {
    let theme = Theme::light();
    let style = style::resolve_input_style(&theme, None, None, true, false, FOCUSED);

    assert_eq!(
        style.border.color,
        theme.semantic_color(SemanticColor::Destructive)
    );

    // `dark:aria-invalid:border-destructive/50`.
    let dark = Theme::dark();
    let dark_style = style::resolve_input_style(&dark, None, None, true, false, FOCUSED);
    let destructive = dark.semantic_color(SemanticColor::Destructive);
    assert!((dark_style.border.color.a - destructive.a * 0.5).abs() < f32::EPSILON);
}

#[test]
fn disabled_style_halves_the_text_opacity() {
    let theme = Theme::light();
    let base = style::resolve_input_style(
        &theme,
        None,
        None,
        false,
        false,
        text_input_widget::Status::Active,
    );
    let disabled = style::resolve_input_style(
        &theme,
        None,
        None,
        false,
        true,
        text_input_widget::Status::Disabled,
    );

    assert!((disabled.value.a - base.value.a * 0.5).abs() < f32::EPSILON);
    assert!((disabled.placeholder.a - base.placeholder.a * 0.5).abs() < f32::EPSILON);
}

#[test]
fn missing_on_input_without_disabled_keeps_the_resting_look() {
    // A display-only input (no `on_input`) reports `Status::Disabled` in iced
    // but must not gray out unless `disabled` was requested.
    let theme = Theme::light();
    let style = style::resolve_input_style(
        &theme,
        None,
        None,
        false,
        false,
        text_input_widget::Status::Disabled,
    );

    assert_eq!(style.value, theme.semantic_color(SemanticColor::Foreground));
}

#[test]
fn dark_mode_fills_follow_the_pack_alpha() {
    // Vega: `bg-transparent` in light, `dark:bg-input/30`.
    let light = style::resolve_input_style(
        &Theme::light(),
        None,
        None,
        false,
        false,
        text_input_widget::Status::Active,
    );
    let dark_theme = Theme::dark();
    let dark = style::resolve_input_style(
        &dark_theme,
        None,
        None,
        false,
        false,
        text_input_widget::Status::Active,
    );

    let crate::iced_compat::Background::Color(light_fill) = light.background else {
        panic!("input backgrounds are plain colors");
    };
    let crate::iced_compat::Background::Color(dark_fill) = dark.background else {
        panic!("input backgrounds are plain colors");
    };
    let input = dark_theme.semantic_color(SemanticColor::Input);

    assert!(light_fill.a.abs() < f32::EPSILON);
    assert!((dark_fill.a - input.a * 0.3).abs() < f32::EPSILON);
}

#[test]
fn pack_text_sizes_follow_the_css() {
    assert_eq!(style::pack_text_size(&Theme::light()), 14.0);

    let lyra = Theme::light().with_style(shadcn_common::StyleId::Lyra);
    assert_eq!(style::pack_text_size(&lyra), 12.0);

    let mira = Theme::light().with_style(shadcn_common::StyleId::Mira);
    assert_eq!(style::pack_text_size(&mira), 12.0);
}

#[test]
fn accent_color_recolors_the_focus_ring() {
    let theme = Theme::light();
    let plain = style::resolve_input_style(&theme, None, None, false, false, FOCUSED);
    let accented =
        style::resolve_input_style(&theme, None, Some(AccentColor::Blue), false, false, FOCUSED);

    assert_eq!(
        accented.border.color,
        theme.color_with_accent(AccentColor::Blue, SemanticColor::Primary)
    );
    assert_ne!(plain.border.color, accented.border.color);
}

#[test]
fn all_states_resolve_in_light_and_dark_themes() {
    for theme in [Theme::light(), Theme::dark()] {
        for invalid in [false, true] {
            for disabled in [false, true] {
                for status in [
                    text_input_widget::Status::Active,
                    text_input_widget::Status::Hovered,
                    FOCUSED,
                    text_input_widget::Status::Disabled,
                ] {
                    let style =
                        style::resolve_input_style(&theme, None, None, invalid, disabled, status);
                    assert!(style.value.a.is_finite());
                    assert!(style.border.width.is_finite());
                }
            }
        }
    }
}

#[test]
fn debug_does_not_require_message_debug_and_redacts_secure_values() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let input = Input::<NoDebugMessage>::new(&theme).value("hunter2");
    let debug = format!("{input:?}");
    assert!(debug.contains("Input"));
    assert!(debug.contains("hunter2"));

    let secure = Input::<NoDebugMessage>::new(&theme)
        .value("hunter2")
        .secure(true);
    let debug = format!("{secure:?}");
    assert!(debug.contains("<secure>"));
    assert!(!debug.contains("hunter2"));
}

#[test]
fn configuration_enums_support_hashing_and_expected_order() {
    fn hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    let _ = hash(&InputSize::Default);
    let _ = hash(&InputRadius::Medium);
    assert!(InputRadius::None < InputRadius::Full);
    assert_eq!(InputSize::default(), InputSize::Default);
    assert_eq!(InputRadius::default(), InputRadius::Medium);
}

#[test]
fn tone_is_an_alias_for_color() {
    let theme = Theme::light();
    let input: Input<'_, Message> = Input::new(&theme).tone(AccentColor::Blue);

    assert_eq!(input.color, Some(AccentColor::Blue));
}

#[test]
fn states_dimensions_and_style_override_are_configurable() {
    let theme = Theme::light();
    let input = Input::text_fixture(&theme)
        .width(Length::Fixed(240.0))
        .align_x(crate::iced_compat::alignment::Horizontal::Center)
        .id("email")
        .style_override(|mut style, _| {
            style.value = Color::from_rgb(1.0, 0.0, 1.0);
            style
        })
        .on_input(Message::Changed);

    assert_eq!(input.width, Length::Fixed(240.0));
    assert_eq!(
        input.align_x,
        crate::iced_compat::alignment::Horizontal::Center
    );
    assert!(input.id.is_some());
    assert!(input.style_override.is_some());

    let _ = input.into_text_input();
}

impl<'a> Input<'a, Message> {
    /// Test fixture with a value and placeholder preset.
    fn text_fixture(theme: &'a Theme) -> Self {
        Input::new(theme).value("value").placeholder("placeholder")
    }
}
