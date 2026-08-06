//! Behavioral tests for the avatar component.

use crate::iced_compat::widget::text;
use crate::iced_compat::{Color, Element, Length};
use shadcn_common::{StyleId, ThemeMode};

use super::geometry;
use super::style;
use super::*;
use crate::theme::Theme;

struct NoDebugMessage;

#[test]
fn avatar_defaults_match_the_source_root() {
    let theme = Theme::light();
    let avatar = Avatar::<NoDebugMessage>::new(&theme);

    assert_eq!(avatar.size, AvatarSize::Default);
    assert_eq!(avatar.radius, AvatarRadius::Theme);
    assert!(avatar.width.is_none());
    assert!(avatar.height.is_none());
    assert!(avatar.image.is_none());
    assert!(avatar.fallback.is_none());
    assert!(avatar.badge.is_none());
    assert!(std::ptr::eq(avatar.theme, &theme));
}

#[test]
fn builder_supports_image_fallback_badge_and_root_overrides() {
    let theme = Theme::light();
    let avatar = Avatar::new(&theme)
        .size(AvatarSize::Custom(48.0))
        .radius(AvatarRadius::Large)
        .width(Length::Fixed(52.0))
        .height(Length::Fixed(48.0))
        .image(AvatarImage::from_rgba(1, 1, [255, 0, 0, 255]))
        .fallback(AvatarFallback::text("CN", &theme))
        .badge(AvatarBadge::text("+", &theme))
        .style_override(|mut style| {
            style.text_color = Some(Color::from_rgb(1.0, 0.0, 1.0));
            style
        });

    assert_eq!(avatar.size, AvatarSize::Custom(48.0));
    assert_eq!(avatar.radius, AvatarRadius::Large);
    assert_eq!(avatar.width, Some(Length::Fixed(52.0)));
    assert_eq!(avatar.height, Some(Length::Fixed(48.0)));
    assert!(avatar.image.is_some());
    assert!(avatar.fallback.is_some());
    assert!(avatar.badge.is_some());

    let _: Element<'_, NoDebugMessage> = avatar.into();
}

#[test]
fn all_public_slot_types_convert_to_elements_without_message_debug() {
    let theme = Theme::light();

    let _: Element<'_, NoDebugMessage> = AvatarFallback::text("CN", &theme).into();
    let _: Element<'_, NoDebugMessage> = AvatarBadge::dot(&theme).into();
    let _: Element<'_, NoDebugMessage> = AvatarBadge::icon(text("+"), &theme).into();
    let _: Element<'_, NoDebugMessage> = AvatarGroupCount::text("+3", &theme).into();
    let _: Element<'_, NoDebugMessage> = AvatarGroupCount::icon(text("+"), &theme).into();
    let _: Element<'_, NoDebugMessage> = AvatarGroup::new(&theme).into();

    assert!(format!("{:?}", Avatar::<NoDebugMessage>::new(&theme)).contains("Avatar"));
    assert!(
        format!("{:?}", AvatarFallback::<NoDebugMessage>::text("CN", &theme))
            .contains("AvatarFallback")
    );
    assert!(format!("{:?}", AvatarBadge::<NoDebugMessage>::dot(&theme)).contains("AvatarBadge"));
    assert!(
        format!(
            "{:?}",
            AvatarGroupCount::<NoDebugMessage>::text("+3", &theme)
        )
        .contains("AvatarGroupCount")
    );
    assert!(format!("{:?}", AvatarGroup::<NoDebugMessage>::new(&theme)).contains("AvatarGroup"));
}

#[test]
fn image_sources_accept_path_bytes_and_rgba_handles() {
    let path = AvatarImage::from_path("avatars/user.png");
    let bytes = AvatarImage::from_bytes([137, 80, 78, 71]);
    let rgba = AvatarImage::from_rgba(1, 1, [0, 0, 0, 255]);

    assert!(format!("{path:?}").contains("Path"));
    assert!(format!("{bytes:?}").contains("Bytes"));
    assert!(format!("{rgba:?}").contains("Pixels"));
}

#[test]
fn source_geometry_is_preserved() {
    assert_eq!(AvatarSize::Sm.pixels(), 24.0);
    assert_eq!(AvatarSize::Default.pixels(), 32.0);
    assert_eq!(AvatarSize::Lg.pixels(), 40.0);
    assert_eq!(AvatarSize::Custom(48.0).pixels(), 48.0);
    assert_eq!(geometry::badge_size(AvatarSize::Sm), 8.0);
    assert_eq!(geometry::badge_size(AvatarSize::Default), 10.0);
    assert_eq!(geometry::badge_size(AvatarSize::Lg), 12.0);
    assert_eq!(geometry::badge_icon_size(AvatarSize::Sm), None);
    assert_eq!(geometry::badge_icon_size(AvatarSize::Default), Some(8.0));
    assert_eq!(geometry::badge_icon_size(AvatarSize::Lg), Some(8.0));
    assert_eq!(geometry::group_count_icon_size(AvatarSize::Sm), 12.0);
    assert_eq!(geometry::group_count_icon_size(AvatarSize::Default), 16.0);
    assert_eq!(geometry::group_count_icon_size(AvatarSize::Lg), 20.0);

    let fallback_sm = geometry::fallback_metrics(AvatarSize::Sm);
    let fallback_default = geometry::fallback_metrics(AvatarSize::Default);
    assert_eq!(fallback_sm.size_px, 12.0);
    assert_eq!(fallback_default.size_px, 14.0);

    let mira = Theme::light().with_style(StyleId::Mira);
    assert_eq!(
        geometry::group_count_metrics(&mira, AvatarSize::Default).size_px,
        12.0
    );
}

#[test]
fn radius_and_validation_are_safe() {
    let theme = Theme::light();

    assert_eq!(geometry::radius_px(&theme, AvatarRadius::Theme), 9999.0);
    assert_eq!(geometry::radius_px(&theme, AvatarRadius::None), 0.0);
    assert_eq!(geometry::radius_px(&theme, AvatarRadius::Custom(-4.0)), 0.0);
    assert_eq!(
        geometry::radius_px(&theme, AvatarRadius::Custom(f32::NAN)),
        0.0
    );
    assert_eq!(AvatarSize::Custom(f32::NAN).pixels(), 1.0);
    assert_eq!(geometry::normalize_opacity(f32::NAN), 1.0);
    assert_eq!(geometry::normalize_opacity(2.0), 1.0);
    assert_eq!(geometry::normalize_scale(-2.0), 0.0);
}

#[test]
fn semantic_styles_follow_light_and_dark_theme_tokens() {
    for mode in [ThemeMode::Light, ThemeMode::Dark] {
        let theme = Theme::light().with_mode(mode);
        let root = style::resolve_root_style(&theme, AvatarRadius::Theme);
        let fallback = style::resolve_fallback_style(&theme, 9999.0);
        let badge = style::resolve_badge_style(&theme, AvatarSize::Default);
        let count = style::resolve_group_count_style(&theme, 32.0);

        assert_eq!(root.border.width, 1.0);
        assert_eq!(root.border.color, theme.palette.border);
        assert_eq!(
            fallback.background,
            Some(crate::iced_compat::Background::Color(theme.palette.muted))
        );
        assert_eq!(fallback.text_color, Some(theme.palette.muted_foreground));
        assert_eq!(
            badge.background,
            Some(crate::iced_compat::Background::Color(theme.palette.primary))
        );
        assert_eq!(badge.text_color, Some(theme.palette.primary_foreground));
        assert_eq!(count.text_color, Some(theme.palette.muted_foreground));
        assert_eq!(count.border.width, 0.0);
        assert_eq!(
            style::resolve_group_ring_style(&theme, 36.0).border.width,
            2.0
        );
        let badge_ring = style::resolve_badge_ring_style(Color::BLACK, 7.0, 2.0);
        assert_eq!(badge_ring.border.color, Color::BLACK);
        assert_eq!(badge_ring.border.width, 2.0);
    }
}

#[test]
fn group_supports_overlap_arbitrary_items_and_count() {
    let theme = Theme::light();
    let group = AvatarGroup::new(&theme)
        .overlap(10.0)
        .push(Avatar::new(&theme).size(AvatarSize::Sm).fallback_text("CN"))
        .push_element(text("LR"), AvatarSize::Sm)
        .count(AvatarGroupCount::text("+3", &theme));

    assert_eq!(group.overlap, 10.0);
    assert_eq!(group.items.len(), 3);
    assert!(matches!(group.items[2], AvatarGroupItem::Count(_)));

    let _: Element<'_, NoDebugMessage> = group.into();
}

#[test]
fn group_item_rings_keep_the_avatar_shape() {
    let theme = Theme::light();
    let group: AvatarGroup<'_, NoDebugMessage> = AvatarGroup::new(&theme).push_element_with_radius(
        text("LR"),
        AvatarSize::Default,
        AvatarRadius::Medium,
    );

    assert!(matches!(
        group.items.as_slice(),
        [AvatarGroupItem::Element {
            radius: AvatarRadius::Medium,
            ..
        }]
    ));

    let square = style::resolve_group_ring_style(&theme, 2.0);
    assert_eq!(
        square.border.radius,
        crate::iced_compat::border::Radius::from(2.0)
    );
}
