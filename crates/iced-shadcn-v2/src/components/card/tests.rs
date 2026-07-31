//! Behavioral tests for the card component.

use crate::iced_compat::{Element, Length};
use shadcn_common::StyleId;

use super::geometry::{
    default_spacing, description_metrics, header_gap, radius_px, small_spacing, title_metrics,
};
use super::style::{
    default_footer_background, footer_has_border, resolve_footer_style, resolve_header_style,
    resolve_root_style, root_ring,
};
use super::*;
use crate::theme::Theme;

#[test]
fn style_pack_spacing_matches_card_css() {
    let expected = [
        (StyleId::Vega, 24.0, 16.0),
        (StyleId::Nova, 16.0, 12.0),
        (StyleId::Maia, 24.0, 16.0),
        (StyleId::Lyra, 16.0, 12.0),
        (StyleId::Mira, 16.0, 12.0),
        (StyleId::Luma, 24.0, 16.0),
        (StyleId::Sera, 24.0, 20.0),
        (StyleId::Rhea, 20.0, 16.0),
    ];

    for (style, default, small) in expected {
        let theme = Theme::light().with_style(style);
        assert_eq!(default_spacing(&theme), default, "{style:?} default");
        assert_eq!(small_spacing(&theme), small, "{style:?} small");
    }
}

#[test]
fn style_pack_card_radii_match_source_css() {
    let expected = [
        (StyleId::Vega, 14.0),
        (StyleId::Nova, 14.0),
        (StyleId::Maia, 18.0),
        (StyleId::Lyra, 0.0),
        (StyleId::Mira, 10.0),
        (StyleId::Luma, 26.0),
        (StyleId::Sera, 0.0),
        (StyleId::Rhea, 24.0),
    ];

    for (style, expected_radius) in expected {
        let theme = Theme::light().with_style(style);
        assert_eq!(
            radius_px(&theme, CardRadius::Theme),
            expected_radius,
            "{style:?}"
        );
    }
}

#[test]
fn typography_matches_style_pack_card_rules() {
    let nova = Theme::light().with_style(StyleId::Nova);
    assert_eq!(title_metrics(&nova, CardSize::Default).size_px, 16.0);
    assert_eq!(title_metrics(&nova, CardSize::Sm).size_px, 14.0);
    assert_eq!(title_metrics(&nova, CardSize::Default).line_height_px, 22.0);

    let sera = Theme::light().with_style(StyleId::Sera);
    let title = title_metrics(&sera, CardSize::Default);
    assert_eq!(title.size_px, 18.0);
    assert!(title.semibold);
    assert!(title.uppercase);
    assert_eq!(title.tracking_em, 0.05);

    let mira = Theme::light().with_style(StyleId::Mira);
    assert_eq!(description_metrics(&mira).size_px, 12.0);
    assert_eq!(description_metrics(&mira).line_height_px, 18.0);
    assert_eq!(header_gap(&sera), 6.0);
}

#[test]
fn nova_and_lyra_footer_edges_match_source_rules() {
    for style in [StyleId::Nova, StyleId::Lyra] {
        let theme = Theme::light().with_style(style);
        assert!(footer_has_border(&theme, CardBorder::Theme));
    }

    let vega = Theme::light().with_style(StyleId::Vega);
    assert!(!footer_has_border(&vega, CardBorder::Theme));
    assert!(footer_has_border(&vega, CardBorder::Present));
    assert!(!footer_has_border(&vega, CardBorder::None));

    let nova = Theme::light().with_style(StyleId::Nova);
    let footer_background = default_footer_background(&nova).expect("Nova footer background");
    assert!((footer_background.a - 0.5).abs() < f32::EPSILON);
}

#[test]
fn root_visual_uses_card_tokens_ring_and_style_shadow() {
    let theme = Theme::light().with_style(StyleId::Vega);
    let visual = resolve_root_style(&theme, CardRadius::Theme);

    assert_eq!(
        visual.background,
        Some(crate::iced_compat::Background::Color(theme.palette.card))
    );
    assert_eq!(visual.text_color, Some(theme.palette.card_foreground));
    assert_eq!(visual.border.width, 0.0);
    assert_eq!(visual.border.color, crate::iced_compat::Color::TRANSPARENT);
    let (ring_color, ring_width) = root_ring(&theme);
    assert_eq!(ring_width, 1.0);
    assert!((ring_color.a - 0.1).abs() < f32::EPSILON);
    assert!(visual.shadow.blur_radius > 0.0);

    let sera = Theme::light().with_style(StyleId::Sera);
    let (ring_color, _) = root_ring(&sera);
    assert!((ring_color.a - 0.05).abs() < f32::EPSILON);
}

#[test]
fn explicit_section_borders_use_the_theme_border_token() {
    let theme = Theme::light().with_style(StyleId::Nova);

    assert_eq!(
        resolve_header_style(&theme, CardRadius::Theme).border.color,
        theme.palette.border
    );
    assert_eq!(
        resolve_footer_style(&theme, CardRadius::Theme, None)
            .border
            .color,
        theme.palette.border
    );
}

#[test]
fn invalid_custom_values_are_safe() {
    let theme = Theme::light();
    assert_eq!(radius_px(&theme, CardRadius::Custom(f32::NAN)), 0.0);
    assert_eq!(radius_px(&theme, CardRadius::Custom(-5.0)), 0.0);

    let card = Card::<()>::new(&theme)
        .spacing(f32::NAN)
        .top_padding(-4.0)
        .bottom_padding(f32::INFINITY);
    assert_eq!(card.spacing, Some(0.0));
    assert_eq!(card.top_padding, Some(0.0));
    assert_eq!(card.bottom_padding, Some(0.0));
}

#[test]
fn builder_composes_all_slots_and_iced_overrides() {
    let theme = Theme::dark().with_style(StyleId::Nova);
    let card = Card::new(&theme)
        .size(CardSize::Sm)
        .spacing(20.0)
        .radius(CardRadius::Custom(9.0))
        .width(Length::Fixed(360.0))
        .height(Length::Shrink)
        .top_padding(0.0)
        .style_override(|mut style| {
            style.border.width = 2.0;
            style
        })
        .header(
            CardHeader::new(&theme)
                .title(CardTitle::text("Title", &theme))
                .description(CardDescription::text("Description", &theme))
                .action(CardAction::new(crate::iced_compat::widget::text("Action")))
                .border_bottom(),
        )
        .content(CardContent::with_content(
            crate::iced_compat::widget::text("Body"),
            &theme,
        ))
        .footer(
            CardFooter::new(&theme)
                .column()
                .spacing(8.0)
                .border_top()
                .push(crate::iced_compat::widget::text("Footer")),
        )
        .push(crate::iced_compat::widget::text("Arbitrary child"));

    let debug = format!("{card:?}");
    assert!(debug.contains("Card"));
    let _: Element<'_, ()> = card.into();
}

#[test]
fn public_builders_have_non_empty_debug_output() {
    let theme = Theme::light();
    assert!(format!("{:?}", Card::<()>::new(&theme)).contains("Card"));
    assert!(format!("{:?}", CardHeader::<()>::new(&theme)).contains("CardHeader"));
    assert!(format!("{:?}", CardContent::<()>::new(&theme)).contains("CardContent"));
    assert!(format!("{:?}", CardFooter::<()>::new(&theme)).contains("CardFooter"));
    assert!(
        format!(
            "{:?}",
            CardAction::<()>::new(crate::iced_compat::widget::text("a"))
        )
        .contains("CardAction")
    );
    assert!(format!("{:?}", CardTitle::<()>::text("a", &theme)).contains("CardTitle"));
    assert!(format!("{:?}", CardDescription::<()>::text("a", &theme)).contains("CardDescription"));
}
