//! Behavioral tests for the button component.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::iced_compat::widget::{button as button_widget, container};
use crate::iced_compat::{Color, Element, Length};
use shadcn_common::AccentColor;
use twill_core::prelude::{Padding, PaddingValue, PaddingVar, Spacing};

use super::geometry;
use super::style;
use super::*;
use crate::theme::Theme;

#[derive(Clone, Debug)]
enum Message {
    Pressed,
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let button: Button<'_, Message> = Button::text("Save", &theme)
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::Lg)
        .radius(ButtonRadius::Large)
        .color(AccentColor::Blue)
        .loading(true)
        .disabled(true);

    assert!(matches!(button.content, ButtonContent::Label(_)));
    assert_eq!(button.variant, ButtonVariant::Outline);
    assert_eq!(button.size, ButtonSize::Lg);
    assert_eq!(button.radius, Some(ButtonRadius::Large));
    assert_eq!(button.color, Some(AccentColor::Blue));
    assert!(button.loading);
    assert!(button.disabled);
    assert!(std::ptr::eq(button.theme, &theme));
}

#[test]
fn text_and_generic_buttons_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = Button::new(container("Custom"), &theme)
        .on_press(Message::Pressed)
        .into();

    let _: Element<'_, Message> = Button::text("Save", &theme)
        .on_press(Message::Pressed)
        .into();
}

#[test]
fn disabled_style_uses_muted_surface() {
    let style = style::resolve_button_style(
        &Theme::light(),
        ButtonVariant::Default,
        None,
        Some(AccentColor::Blue),
        true,
        button_widget::Status::Disabled,
    );

    assert!(style.background.is_some());
    assert_eq!(style.border.width, 1.0);
}

#[test]
fn variant_mapping_matches_expected_surface_rules() {
    let theme = Theme::light();

    let default_style = style::resolve_button_style(
        &theme,
        ButtonVariant::Default,
        None,
        Some(AccentColor::Blue),
        false,
        button_widget::Status::Active,
    );
    assert!(default_style.background.is_some());
    assert_eq!(default_style.border.width, 0.0);

    let outline_style = style::resolve_button_style(
        &theme,
        ButtonVariant::Outline,
        None,
        Some(AccentColor::Blue),
        false,
        button_widget::Status::Active,
    );
    assert_eq!(outline_style.border.width, 1.0);

    let link_style = style::resolve_button_style(
        &theme,
        ButtonVariant::Link,
        None,
        Some(AccentColor::Blue),
        false,
        button_widget::Status::Active,
    );
    assert!(link_style.background.is_none());
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
fn padding_builder_stores_resolved_padding() {
    let theme = Theme::light();
    let button: Button<'_, Message> = Button::text("Save", &theme)
        .padding(Padding::individual(
            Spacing::S1,
            Spacing::S2,
            Spacing::S3,
            Spacing::S4,
        ))
        .expect("scale padding is supported");

    assert_eq!(
        button.padding,
        Some(crate::iced_compat::Padding {
            top: 4.0,
            right: 8.0,
            bottom: 12.0,
            left: 16.0,
        })
    );
}

#[test]
fn padding_variable_returns_a_descriptive_error() {
    let theme = Theme::light();
    let error = Button::<Message>::text("Save", &theme)
        .padding(Padding::individual_value(
            PaddingValue::Var(PaddingVar::new("--button-padding")),
            PaddingValue::Px(2.0),
            PaddingValue::Px(3.0),
            PaddingValue::Px(4.0),
        ))
        .expect_err("padding variables are unsupported");

    assert_eq!(
        error,
        ButtonBuildError::UnsupportedPaddingVariable {
            name: "--button-padding"
        }
    );
    assert!(error.to_string().contains("--button-padding"));
}

#[test]
fn padding_auto_returns_a_descriptive_error() {
    let theme = Theme::light();
    let error = Button::<Message>::text("Save", &theme)
        .padding(Padding::all(Spacing::Auto))
        .expect_err("auto padding is unsupported");

    assert_eq!(error, ButtonBuildError::UnsupportedPaddingAuto);
    assert!(error.to_string().contains("auto"));
}

#[test]
fn icon_button_uses_custom_fixed_height_for_both_dimensions() {
    let resolved =
        geometry::resolve_button_width(Length::Shrink, Length::Fixed(72.0), false, true, 36.0);

    assert_eq!(resolved, Length::Fixed(72.0));
}

#[test]
fn button_sizes_match_style_pack_recipes() {
    let vega = Theme::light();
    assert_eq!(ButtonSize::Xs.control_height(&vega), 24.0);
    assert_eq!(ButtonSize::Sm.control_height(&vega), 32.0);
    assert_eq!(ButtonSize::Default.control_height(&vega), 36.0);
    assert_eq!(ButtonSize::Lg.control_height(&vega), 40.0);

    let nova = Theme::light().with_style(shadcn_common::StyleId::Nova);
    assert_eq!(ButtonSize::Default.control_height(&nova), 32.0);

    let sera = Theme::light().with_style(shadcn_common::StyleId::Sera);
    assert_eq!(ButtonSize::Default.control_height(&sera), 40.0);
    assert_eq!(ButtonSize::Lg.control_height(&sera), 44.0);

    assert!(ButtonSize::Icon.is_icon());
    assert!(!ButtonSize::Default.is_icon());
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let button = Button::<NoDebugMessage>::text("Save", &theme);
    let debug = format!("{button:?}");

    assert!(debug.contains("Button"));
    assert!(debug.contains("label"));
}

#[test]
fn configuration_enums_support_hashing_and_expected_order() {
    fn hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    let _ = hash(&ButtonVariant::Default);
    let _ = hash(&ButtonSize::Default);
    let _ = hash(&ButtonRadius::Medium);
    assert!(ButtonSize::Icon.is_icon());
    assert!(ButtonRadius::None < ButtonRadius::Full);
}

#[test]
fn tone_is_an_alias_for_color() {
    let theme = Theme::light();
    let button: Button<'_, Message> = Button::text("Save", &theme).tone(AccentColor::Blue);

    assert_eq!(button.color, Some(AccentColor::Blue));
}

#[test]
fn states_dimensions_and_style_override_are_configurable() {
    let theme = Theme::light();
    let button = Button::text("Save", &theme)
        .loading(true)
        .disabled(true)
        .full_width()
        .width(Length::Fixed(240.0))
        .height(Length::Fixed(48.0))
        .style_override(|mut style, _| {
            style.text_color = Color::from_rgb(1.0, 0.0, 1.0);
            style
        })
        .on_press(Message::Pressed);

    assert!(button.loading);
    assert!(button.disabled);
    assert!(button.full_width);
    assert_eq!(button.width, Length::Fixed(240.0));
    assert_eq!(button.height, Some(Length::Fixed(48.0)));
    assert!(button.style_override.is_some());

    let _ = button.into_button();
}

#[test]
fn all_variants_resolve_in_light_and_dark_themes() {
    for theme in [Theme::light(), Theme::dark()] {
        for variant in [
            ButtonVariant::Default,
            ButtonVariant::Destructive,
            ButtonVariant::Outline,
            ButtonVariant::Secondary,
            ButtonVariant::Ghost,
            ButtonVariant::Link,
            ButtonVariant::Soft,
            ButtonVariant::Surface,
        ] {
            for status in [
                button_widget::Status::Active,
                button_widget::Status::Hovered,
                button_widget::Status::Pressed,
                button_widget::Status::Disabled,
            ] {
                let style = style::resolve_button_style(
                    &theme,
                    variant,
                    None,
                    Some(AccentColor::Blue),
                    status == button_widget::Status::Disabled,
                    status,
                );
                assert!(style.text_color.a.is_finite());
            }
        }
    }
}
