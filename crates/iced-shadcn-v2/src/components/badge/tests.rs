//! Behavioral tests for the badge component.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::iced_compat::widget::{button as button_widget, container, text};
use crate::iced_compat::{Color, Element, Length};
use shadcn_common::{AccentColor, StyleId};
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
    let badge: Badge<'_, Message> = Badge::text("New", &theme)
        .variant(BadgeVariant::Outline)
        .radius(BadgeRadius::Large)
        .color(AccentColor::Blue)
        .loading(true)
        .disabled(true);

    assert!(matches!(badge.content, BadgeContent::Label(_)));
    assert_eq!(badge.variant, BadgeVariant::Outline);
    assert_eq!(badge.radius, Some(BadgeRadius::Large));
    assert_eq!(badge.color, Some(AccentColor::Blue));
    assert!(badge.loading);
    assert!(badge.disabled);
    assert!(std::ptr::eq(badge.theme, &theme));
}

#[test]
fn text_and_generic_badges_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = Badge::new(container(text("Custom")), &theme).into();

    let _: Element<'_, Message> = Badge::text("New", &theme).on_press(Message::Pressed).into();
}

#[test]
fn icons_and_loading_compose() {
    let theme = Theme::light();
    let badge: Badge<'_, Message> = Badge::text("Sync", &theme)
        .icon_start(text("*"))
        .icon_end(text(">"))
        .loading(true);

    assert!(badge.icon_start.is_some());
    assert!(badge.icon_end.is_some());
    assert!(badge.loading);

    let _: Element<'_, Message> = badge.into();
}

#[test]
fn variant_mapping_matches_expected_surface_rules() {
    let theme = Theme::light();

    let default_style = style::resolve_container_style(
        &theme,
        BadgeVariant::Default,
        None,
        Some(AccentColor::Blue),
    );
    assert!(default_style.background.is_some());
    assert_eq!(default_style.border.width, 0.0);

    let outline_style = style::resolve_container_style(
        &theme,
        BadgeVariant::Outline,
        None,
        Some(AccentColor::Blue),
    );
    assert_eq!(outline_style.border.width, 1.0);

    let link_style =
        style::resolve_container_style(&theme, BadgeVariant::Link, None, Some(AccentColor::Blue));
    assert!(link_style.background.is_none());
}

#[test]
fn interactive_hover_softens_default_fill() {
    let theme = Theme::light();
    let active = style::resolve_button_style(
        &theme,
        BadgeVariant::Default,
        None,
        None,
        false,
        button_widget::Status::Active,
    );
    let hovered = style::resolve_button_style(
        &theme,
        BadgeVariant::Default,
        None,
        None,
        false,
        button_widget::Status::Hovered,
    );

    let active_bg = match active.background {
        Some(crate::iced_compat::Background::Color(color)) => color,
        _ => panic!("expected solid fill"),
    };
    let hovered_bg = match hovered.background {
        Some(crate::iced_compat::Background::Color(color)) => color,
        _ => panic!("expected solid fill"),
    };

    assert!((active_bg.a - 1.0).abs() < f32::EPSILON);
    assert!((hovered_bg.a - 0.80).abs() < f32::EPSILON);
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
    let badge: Badge<'_, Message> = Badge::text("New", &theme)
        .padding(Padding::individual(
            Spacing::S0_5,
            Spacing::S2,
            Spacing::S0_5,
            Spacing::S2,
        ))
        .expect("scale padding is supported");

    assert_eq!(
        badge.padding,
        Some(crate::iced_compat::Padding {
            top: 2.0,
            right: 8.0,
            bottom: 2.0,
            left: 8.0,
        })
    );
}

#[test]
fn padding_variable_returns_a_descriptive_error() {
    let theme = Theme::light();
    let error = Badge::<Message>::text("New", &theme)
        .padding(Padding::individual_value(
            PaddingValue::Var(PaddingVar::new("--badge-padding")),
            PaddingValue::Px(2.0),
            PaddingValue::Px(3.0),
            PaddingValue::Px(4.0),
        ))
        .expect_err("padding variables are unsupported");

    assert_eq!(
        error,
        BadgeBuildError::UnsupportedPaddingVariable {
            name: "--badge-padding"
        }
    );
    assert!(error.to_string().contains("--badge-padding"));
}

#[test]
fn padding_auto_returns_a_descriptive_error() {
    let theme = Theme::light();
    let error = Badge::<Message>::text("New", &theme)
        .padding(Padding::all(Spacing::Auto))
        .expect_err("auto padding is unsupported");

    assert_eq!(error, BadgeBuildError::UnsupportedPaddingAuto);
    assert!(error.to_string().contains("auto"));
}

#[test]
fn default_padding_tightens_for_icons() {
    let theme = Theme::light();
    let plain = geometry::default_padding(&theme, false, false);
    assert_eq!(plain.top, 0.0);
    assert_eq!(plain.bottom, 0.0);
    assert_eq!(plain.left, 8.0);
    assert_eq!(plain.right, 8.0);

    let with_start = geometry::default_padding(&theme, true, false);
    assert_eq!(with_start.left, 6.0);
    assert_eq!(with_start.right, 8.0);

    let with_end = geometry::default_padding(&theme, false, true);
    assert_eq!(with_end.left, 8.0);
    assert_eq!(with_end.right, 6.0);
}

#[test]
fn badge_recipe_comes_from_shadcn_common() {
    let theme = Theme::light();
    assert_eq!(theme.style.badge().height_px, Some(20.0));
    assert_eq!(theme.style.badge().typography.size_px, 12.0);

    let sera = Theme::light().with_style(StyleId::Sera);
    assert!(sera.style.badge().height_px.is_none());
    assert!(sera.style.badge().typography.uppercase);
}

#[test]
fn locked_style_packs_default_to_no_radius() {
    let lyra = Theme::light().with_style(StyleId::Lyra);
    assert_eq!(style::effective_radius(&lyra, None), BadgeRadius::None);

    let vega = Theme::light().with_style(StyleId::Vega);
    assert_eq!(style::effective_radius(&vega, None), BadgeRadius::Large);
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let badge = Badge::<NoDebugMessage>::text("New", &theme);
    let debug = format!("{badge:?}");

    assert!(debug.contains("Badge"));
    assert!(debug.contains("label"));
}

#[test]
fn configuration_enums_support_hashing_and_expected_order() {
    fn hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    let _ = hash(&BadgeVariant::Default);
    let _ = hash(&BadgeRadius::Full);
    assert!(BadgeRadius::None < BadgeRadius::Full);
}

#[test]
fn tone_is_an_alias_for_color() {
    let theme = Theme::light();
    let badge: Badge<'_, Message> = Badge::text("New", &theme).tone(AccentColor::Blue);

    assert_eq!(badge.color, Some(AccentColor::Blue));
}

#[test]
fn dimensions_and_style_override_are_configurable() {
    let theme = Theme::light();
    let badge = Badge::text("New", &theme)
        .loading(true)
        .disabled(true)
        .width(Length::Fixed(120.0))
        .height(Length::Fixed(24.0))
        .style_override(|mut style| {
            style.text_color = Some(Color::from_rgb(1.0, 0.0, 1.0));
            style
        });

    assert!(badge.loading);
    assert!(badge.disabled);
    assert_eq!(badge.width, Length::Fixed(120.0));
    assert_eq!(badge.height, Some(Length::Fixed(24.0)));
    assert!(badge.style_override.is_some());

    let _: Element<'_, Message> = badge.into();
}

#[test]
fn all_variants_resolve_in_light_and_dark_themes() {
    for theme in [Theme::light(), Theme::dark()] {
        for variant in [
            BadgeVariant::Default,
            BadgeVariant::Destructive,
            BadgeVariant::Outline,
            BadgeVariant::Secondary,
            BadgeVariant::Ghost,
            BadgeVariant::Link,
        ] {
            let container_style =
                style::resolve_container_style(&theme, variant, None, Some(AccentColor::Blue));
            assert!(
                container_style
                    .text_color
                    .is_some_and(|color| color.a.is_finite())
            );

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
