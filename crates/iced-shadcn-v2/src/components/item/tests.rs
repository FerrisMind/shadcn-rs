//! Behavioral tests for the item component.

use crate::iced_compat::{Background, Color, Element, Length};
use shadcn_common::{FontWeight, StyleId};

use super::geometry::{
    content_gap, description_metrics, group_gap, media_image_radius_px, media_image_size_px,
    radius_px, separator_margin_y, size_metrics, title_metrics,
};
use super::style::{muted_background, resolve_hover_style, resolve_root_style, to_button_style};
use super::*;
use crate::theme::Theme;

#[test]
fn style_pack_size_metrics_match_item_css() {
    // (style, size, gap, padding_x, padding_y)
    let expected = [
        (StyleId::Vega, ItemSize::Default, 14.0, 16.0, 14.0),
        (StyleId::Vega, ItemSize::Sm, 10.0, 12.0, 10.0),
        (StyleId::Vega, ItemSize::Xs, 8.0, 10.0, 8.0),
        (StyleId::Nova, ItemSize::Default, 10.0, 12.0, 10.0),
        (StyleId::Lyra, ItemSize::Sm, 10.0, 12.0, 10.0),
        (StyleId::Mira, ItemSize::Xs, 10.0, 10.0, 8.0),
        (StyleId::Maia, ItemSize::Sm, 14.0, 14.0, 12.0),
        (StyleId::Luma, ItemSize::Default, 14.0, 16.0, 14.0),
        (StyleId::Sera, ItemSize::Xs, 10.0, 12.0, 10.0),
        (StyleId::Rhea, ItemSize::Xs, 8.0, 10.0, 8.0),
    ];

    for (style, size, gap, padding_x, padding_y) in expected {
        let theme = Theme::light().with_style(style);
        let metrics = size_metrics(&theme, size);
        assert_eq!(metrics.gap, gap, "{style:?} {size:?} gap");
        assert_eq!(metrics.padding_x, padding_x, "{style:?} {size:?} px");
        assert_eq!(metrics.padding_y, padding_y, "{style:?} {size:?} py");
    }
}

#[test]
fn style_pack_item_radii_match_source_css() {
    let expected = [
        (StyleId::Vega, 8.0),
        (StyleId::Nova, 10.0),
        (StyleId::Maia, 18.0),
        (StyleId::Lyra, 0.0),
        (StyleId::Mira, 8.0),
        (StyleId::Luma, 18.0),
        (StyleId::Sera, 0.0),
        (StyleId::Rhea, 18.0),
    ];

    for (style, expected_radius) in expected {
        let theme = Theme::light().with_style(style);
        assert_eq!(
            radius_px(&theme, ItemRadius::Theme),
            expected_radius,
            "{style:?}"
        );
    }
}

#[test]
fn media_image_geometry_matches_source_css() {
    let vega = Theme::light().with_style(StyleId::Vega);
    assert_eq!(media_image_size_px(&vega, ItemSize::Default), 40.0);
    assert_eq!(media_image_size_px(&vega, ItemSize::Sm), 32.0);
    assert_eq!(media_image_size_px(&vega, ItemSize::Xs), 24.0);

    // Mira renders default-density thumbnails at size-8.
    let mira = Theme::light().with_style(StyleId::Mira);
    assert_eq!(media_image_size_px(&mira, ItemSize::Default), 32.0);

    // Sharp packs keep square thumbnails; Luma steps down for xs.
    let sera = Theme::light().with_style(StyleId::Sera);
    assert_eq!(media_image_radius_px(&sera, ItemSize::Default), 0.0);
    let luma = Theme::light().with_style(StyleId::Luma);
    assert!(
        media_image_radius_px(&luma, ItemSize::Xs)
            < media_image_radius_px(&luma, ItemSize::Default)
    );
}

#[test]
fn typography_matches_style_pack_item_rules() {
    let vega = Theme::light().with_style(StyleId::Vega);
    let title = title_metrics(&vega);
    assert_eq!(title.size_px, 14.0);
    assert_eq!(title.weight, FontWeight::Medium);
    assert!(!title.uppercase);

    let sera = Theme::light().with_style(StyleId::Sera);
    let title = title_metrics(&sera);
    assert_eq!(title.size_px, 12.0);
    assert_eq!(title.weight, FontWeight::Semibold);
    assert!(title.uppercase);

    // Vega and Nova descriptions drop to text-xs on xs items.
    assert_eq!(description_metrics(&vega, ItemSize::Default).size_px, 14.0);
    assert_eq!(description_metrics(&vega, ItemSize::Xs).size_px, 12.0);

    let lyra = Theme::light().with_style(StyleId::Lyra);
    assert_eq!(description_metrics(&lyra, ItemSize::Default).size_px, 12.0);
    assert_eq!(
        description_metrics(&lyra, ItemSize::Default).line_height_px,
        19.5
    );
}

#[test]
fn content_gap_collapses_on_xs_items() {
    let vega = Theme::light().with_style(StyleId::Vega);
    assert_eq!(content_gap(&vega, ItemSize::Default), 4.0);
    assert_eq!(content_gap(&vega, ItemSize::Xs), 0.0);

    let sera = Theme::light().with_style(StyleId::Sera);
    assert_eq!(content_gap(&sera, ItemSize::Xs), 2.0);
}

#[test]
fn variant_styles_use_theme_tokens() {
    let theme = Theme::light();

    let default = resolve_root_style(&theme, ItemVariant::Default, ItemRadius::Theme);
    assert_eq!(default.background, None);
    assert_eq!(default.border.color, Color::TRANSPARENT);
    assert_eq!(default.border.width, 1.0);

    let outline = resolve_root_style(&theme, ItemVariant::Outline, ItemRadius::Theme);
    assert_eq!(outline.border.color, theme.palette.border);

    let muted = resolve_root_style(&theme, ItemVariant::Muted, ItemRadius::Theme);
    let expected = muted_background(&theme);
    assert_eq!(muted.background, Some(Background::Color(expected)));
    assert!((expected.a - theme.palette.muted.a * 0.5).abs() < f32::EPSILON);
}

#[test]
fn hover_style_paints_the_muted_surface() {
    let theme = Theme::light();
    let hovered = resolve_hover_style(&theme, ItemVariant::Outline, ItemRadius::Theme);
    assert_eq!(
        hovered.background,
        Some(Background::Color(theme.palette.muted))
    );
    // Hover keeps the outline border.
    assert_eq!(hovered.border.color, theme.palette.border);

    let button = to_button_style(hovered, theme.palette.foreground);
    assert_eq!(
        button.background,
        Some(Background::Color(theme.palette.muted))
    );
    assert_eq!(button.text_color, theme.palette.foreground);

    // Fully transparent backgrounds are dropped instead of painted.
    let resting = resolve_root_style(&theme, ItemVariant::Default, ItemRadius::Theme);
    assert_eq!(
        to_button_style(resting, theme.palette.foreground).background,
        None
    );
}

#[test]
fn group_gap_follows_the_densest_item() {
    assert_eq!(group_gap(false, false), 16.0);
    assert_eq!(group_gap(true, false), 10.0);
    // The xs rule wins over sm, matching the source cascade order.
    assert_eq!(group_gap(true, true), 8.0);

    let theme = Theme::light();
    let group = ItemGroup::<()>::new(&theme)
        .push(Item::new(&theme).size(ItemSize::Sm))
        .push(Item::new(&theme).size(ItemSize::Xs));
    assert!(group.has_sm);
    assert!(group.has_xs);
    assert_eq!(separator_margin_y(), 8.0);
}

#[test]
fn invalid_custom_values_are_safe() {
    let theme = Theme::light();
    assert_eq!(radius_px(&theme, ItemRadius::Custom(f32::NAN)), 0.0);
    assert_eq!(radius_px(&theme, ItemRadius::Custom(-5.0)), 0.0);

    let item = Item::<()>::new(&theme)
        .spacing(f32::NAN)
        .padding_x(-4.0)
        .padding_y(f32::INFINITY);
    assert_eq!(item.spacing, Some(0.0));
    assert_eq!(item.padding_x, Some(0.0));
    assert_eq!(item.padding_y, Some(0.0));

    let media = ItemMedia::<()>::new(&theme)
        .image_size(f32::NAN)
        .image_radius(-3.0);
    assert_eq!(media.image_size, Some(0.0));
    assert_eq!(media.image_radius, Some(0.0));
}

#[test]
fn builder_composes_all_slots_and_iced_overrides() {
    #[derive(Debug, Clone, PartialEq)]
    struct Pressed;

    let theme = Theme::dark().with_style(StyleId::Nova);
    let item = Item::new(&theme)
        .variant(ItemVariant::Muted)
        .size(ItemSize::Sm)
        .radius(ItemRadius::Custom(9.0))
        .width(Length::Fixed(420.0))
        .spacing(12.0)
        .header(ItemHeader::new(&theme).push(crate::iced_compat::widget::text("Header")))
        .media(ItemMedia::icon(
            crate::iced_compat::widget::text("i"),
            &theme,
        ))
        .content(
            ItemContent::new(&theme)
                .title(ItemTitle::text("Title", &theme))
                .description(ItemDescription::text("Description", &theme)),
        )
        .content(ItemContent::new(&theme).push(crate::iced_compat::widget::text("Meta")))
        .actions(ItemActions::new(&theme).push(crate::iced_compat::widget::text("Action")))
        .footer(ItemFooter::new(&theme).push(crate::iced_compat::widget::text("Footer")))
        .push(crate::iced_compat::widget::text("Arbitrary child"))
        .on_press(Pressed)
        .style_override(|mut style| {
            style.border.width = 2.0;
            style
        });

    let debug = format!("{item:?}");
    assert!(debug.contains("Item"));
    let _: Element<'_, Pressed> = item.into();

    let group = ItemGroup::new(&theme)
        .push(Item::new(&theme).content(ItemContent::new(&theme)))
        .separator()
        .push(Item::new(&theme).size(ItemSize::Xs));
    let _: Element<'_, Pressed> = group.into();
}

#[test]
fn public_builders_have_non_empty_debug_output() {
    let theme = Theme::light();
    assert!(format!("{:?}", Item::<()>::new(&theme)).contains("Item"));
    assert!(format!("{:?}", ItemMedia::<()>::new(&theme)).contains("ItemMedia"));
    assert!(format!("{:?}", ItemContent::<()>::new(&theme)).contains("ItemContent"));
    assert!(format!("{:?}", ItemTitle::<()>::text("a", &theme)).contains("ItemTitle"));
    assert!(format!("{:?}", ItemDescription::<()>::text("a", &theme)).contains("ItemDescription"));
    assert!(format!("{:?}", ItemActions::<()>::new(&theme)).contains("ItemActions"));
    assert!(format!("{:?}", ItemHeader::<()>::new(&theme)).contains("ItemHeader"));
    assert!(format!("{:?}", ItemFooter::<()>::new(&theme)).contains("ItemFooter"));
    assert!(format!("{:?}", ItemGroup::<()>::new(&theme)).contains("ItemGroup"));
    assert!(format!("{:?}", ItemSeparator::new(&theme)).contains("ItemSeparator"));
}

#[test]
fn standalone_slots_build_into_elements() {
    let theme = Theme::light();
    let _: Element<'_, ()> =
        ItemMedia::image(crate::iced_compat::widget::text("img"), &theme).into_element();
    let _: Element<'_, ()> = ItemContent::new(&theme)
        .title(ItemTitle::text("Title", &theme))
        .into_element();
    let _: Element<'_, ()> = ItemTitle::text("Title", &theme).into_element();
    let _: Element<'_, ()> = ItemDescription::text("Description", &theme).into_element();
    let _: Element<'_, ()> = ItemActions::new(&theme).into_element();
    let _: Element<'_, ()> = ItemHeader::new(&theme).into_element();
    let _: Element<'_, ()> = ItemFooter::new(&theme).into_element();
    let _: Element<'_, ()> = ItemSeparator::new(&theme).into_element();
}
