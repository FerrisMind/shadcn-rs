//! Behavioral tests for the alert component.

use crate::iced_compat::widget::{container, text};
use crate::iced_compat::{Color, Element, Length, Padding};
use shadcn_common::{StyleId, ThemeMode};

use super::geometry;
use super::style;
use super::*;
use crate::theme::Theme;

struct NoDebugMessage;

#[test]
fn alert_defaults_to_the_default_variant_and_theme_geometry() {
    let theme = Theme::light();
    let alert: Alert<'_, NoDebugMessage> = Alert::new(&theme);

    assert_eq!(alert.variant, AlertVariant::Default);
    assert_eq!(alert.radius, AlertRadius::Theme);
    assert_eq!(alert.width, Length::Fill);
    assert_eq!(alert.height, Length::Shrink);
    assert!(alert.icon.is_none());
    assert!(alert.items.is_empty());
    assert!(alert.action.is_none());
    assert!(std::ptr::eq(alert.theme, &theme));
}

#[test]
fn builder_supports_all_source_slots_and_arbitrary_children() {
    let theme = Theme::light();
    let alert = Alert::new(&theme)
        .variant(AlertVariant::Destructive)
        .radius(AlertRadius::Large)
        .width(Length::Fixed(420.0))
        .height(Length::Shrink)
        .padding(Padding::new(12.0))
        .spacing(6.0)
        .icon(text("!"))
        .title(AlertTitle::text("Payment failed", &theme))
        .description(AlertDescription::new(container(text("Try again.")), &theme))
        .action(AlertAction::new(text("Retry")))
        .push(text("More details"));

    assert_eq!(alert.variant, AlertVariant::Destructive);
    assert_eq!(alert.radius, AlertRadius::Large);
    assert_eq!(alert.width, Length::Fixed(420.0));
    assert_eq!(alert.padding, Some(Padding::new(12.0)));
    assert_eq!(alert.spacing, Some(6.0));
    assert!(alert.icon.is_some());
    assert_eq!(alert.items.len(), 3);
    assert!(alert.action.is_some());

    let _: Element<'_, NoDebugMessage> = alert.into();
}

#[test]
fn debug_does_not_require_message_debug() {
    let theme = Theme::light();
    let alert = Alert::<NoDebugMessage>::new(&theme)
        .icon(text("!"))
        .title(AlertTitle::text("Heads up", &theme));

    let debug = format!("{alert:?}");

    assert!(debug.contains("Alert"));
    assert!(debug.contains("items"));
}

#[test]
fn title_and_description_wrappers_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, NoDebugMessage> = AlertTitle::text("Title", &theme).into();
    let _: Element<'_, NoDebugMessage> = AlertDescription::text("Description", &theme).into();
    let _: Element<'_, NoDebugMessage> = AlertAction::new(text("Action")).into();
}

#[test]
fn metrics_follow_the_source_style_css() {
    let cases = [
        (StyleId::Vega, 16.0, 12.0, 10.0, 16.0, 14.0),
        (StyleId::Nova, 10.0, 8.0, 8.0, 16.0, 14.0),
        (StyleId::Maia, 16.0, 12.0, 10.0, 16.0, 14.0),
        (StyleId::Lyra, 10.0, 8.0, 8.0, 16.0, 12.0),
        (StyleId::Mira, 8.0, 6.0, 6.0, 14.0, 12.0),
        (StyleId::Luma, 16.0, 12.0, 10.0, 16.0, 14.0),
        (StyleId::Sera, 16.0, 12.0, 10.0, 16.0, 14.0),
        (StyleId::Rhea, 16.0, 12.0, 10.0, 16.0, 14.0),
    ];

    for (style_id, padding_x, padding_y, icon_gap, icon_size, title_size) in cases {
        let theme = Theme::light().with_style(style_id);
        let metrics = geometry::metrics(&theme);

        assert_eq!(metrics.padding_x_px, padding_x, "{style_id:?}");
        assert_eq!(metrics.padding_y_px, padding_y, "{style_id:?}");
        assert_eq!(metrics.icon_gap_px, icon_gap, "{style_id:?}");
        assert_eq!(metrics.icon_size_px, icon_size, "{style_id:?}");
        assert_eq!(metrics.title_size_px, title_size, "{style_id:?}");
    }
}

#[test]
fn theme_radius_matches_source_shape_families() {
    let vega = Theme::light().with_style(StyleId::Vega);
    let lyra = Theme::light().with_style(StyleId::Lyra);
    let luma = Theme::light().with_style(StyleId::Luma);
    let rhea = Theme::light().with_style(StyleId::Rhea);

    assert_eq!(geometry::radius_px(&vega, AlertRadius::Theme), 10.0);
    assert_eq!(geometry::radius_px(&lyra, AlertRadius::Theme), 0.0);
    assert_eq!(geometry::radius_px(&luma, AlertRadius::Theme), 18.0);
    assert_eq!(geometry::radius_px(&rhea, AlertRadius::Theme), 18.0);
    assert_eq!(geometry::radius_px(&vega, AlertRadius::Custom(-4.0)), 0.0);
    assert_eq!(
        geometry::radius_px(&vega, AlertRadius::Custom(f32::NAN)),
        0.0
    );
}

#[test]
fn variant_colors_preserve_destructive_description_contrast() {
    for mode in [ThemeMode::Light, ThemeMode::Dark] {
        let theme = Theme::light().with_mode(mode);
        let regular = style::resolve_root_style(&theme, AlertVariant::Default, AlertRadius::Theme);
        let destructive =
            style::resolve_root_style(&theme, AlertVariant::Destructive, AlertRadius::Theme);

        assert_eq!(regular.text_color, Some(theme.palette.card_foreground));
        assert_eq!(destructive.text_color, Some(theme.palette.destructive));
        assert_eq!(regular.border.width, 1.0);
        assert_eq!(destructive.border.width, 1.0);

        let description = style::description_color(&theme, AlertVariant::Destructive);
        assert!((description.a - 0.9).abs() < f32::EPSILON);
    }
}

#[test]
fn sera_gets_the_semantic_leading_rail() {
    let sera = Theme::light().with_style(StyleId::Sera);
    let vega = Theme::light().with_style(StyleId::Vega);

    assert_eq!(
        style::accent_bar_color(&sera, AlertVariant::Default),
        Some(sera.palette.foreground)
    );
    assert_eq!(
        style::accent_bar_color(&sera, AlertVariant::Destructive),
        Some(sera.palette.destructive)
    );
    assert_eq!(style::accent_bar_color(&vega, AlertVariant::Default), None);
}

#[test]
fn padding_and_text_metrics_normalize_invalid_values() {
    let padding = geometry::normalize_padding(Padding {
        top: -1.0,
        right: f32::NAN,
        bottom: f32::INFINITY,
        left: 4.0,
    });

    assert_eq!(padding.top, 0.0);
    assert_eq!(padding.right, 0.0);
    assert_eq!(padding.bottom, 0.0);
    assert_eq!(padding.left, 4.0);
    assert_eq!(geometry::normalize_min_px(0.0), 1.0);
    assert_eq!(geometry::normalize_min_px(f32::NAN), 1.0);
}

#[test]
fn style_override_is_stored_without_forcing_message_debug() {
    let theme = Theme::light();
    let alert = Alert::<NoDebugMessage>::new(&theme).style_override(|mut style| {
        style.text_color = Some(Color::from_rgb(1.0, 0.0, 1.0));
        style
    });

    assert!(alert.style_override.is_some());
}
