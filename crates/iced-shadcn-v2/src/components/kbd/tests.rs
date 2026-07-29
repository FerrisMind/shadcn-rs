//! Behavioral tests for the kbd component.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::iced_compat::widget::{container, text};
use crate::iced_compat::{Background, Color, Element, Length};
use shadcn_common::StyleId;
use twill_core::prelude::{Padding, PaddingValue, PaddingVar, Spacing};

use super::geometry;
use super::style;
use super::*;
use crate::theme::Theme;

#[derive(Clone, Debug)]
enum Message {}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let kbd: Kbd<'_, Message> = Kbd::text("B", &theme)
        .surface(KbdSurface::Tooltip)
        .radius(KbdRadius::Large)
        .min_width(30.0)
        .text_size(14.0);

    assert!(matches!(kbd.content, KbdContent::Label(_)));
    assert_eq!(kbd.surface, KbdSurface::Tooltip);
    assert_eq!(kbd.radius, Some(KbdRadius::Large));
    assert_eq!(kbd.min_width, Some(30.0));
    assert_eq!(kbd.text_size, Some(14.0));
    assert!(std::ptr::eq(kbd.theme, &theme));
}

#[test]
fn text_and_generic_kbds_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = Kbd::new(container(text("↑")), &theme).into();
    let _: Element<'_, Message> = Kbd::text("Ctrl", &theme).into();
}

#[test]
fn icons_compose_with_labels() {
    let theme = Theme::light();
    let kbd: Kbd<'_, Message> = Kbd::text("K", &theme)
        .icon_start(text("⌘"))
        .icon_end(text("↵"));

    assert!(kbd.icon_start.is_some());
    assert!(kbd.icon_end.is_some());

    let _: Element<'_, Message> = kbd.into();
}

#[test]
fn surface_mapping_matches_expected_rules() {
    let theme = Theme::light();

    let default_style = style::resolve_container_style(&theme, KbdSurface::Default, None);
    assert_eq!(
        default_style.background,
        Some(Background::Color(theme.palette.muted))
    );
    assert_eq!(
        default_style.text_color,
        Some(theme.palette.muted_foreground)
    );

    let input_group_style = style::resolve_container_style(&theme, KbdSurface::InputGroup, None);
    assert_eq!(
        input_group_style.background,
        Some(Background::Color(theme.palette.input))
    );
}

#[test]
fn tooltip_surface_uses_translucent_background_chip() {
    for (theme, expected_alpha) in [(Theme::light(), 0.20), (Theme::dark(), 0.10)] {
        let resolved = style::resolve_container_style(&theme, KbdSurface::Tooltip, None);

        let background = match resolved.background {
            Some(Background::Color(color)) => color,
            other => panic!("expected solid fill, got {other:?}"),
        };

        assert!((background.a - expected_alpha).abs() < f32::EPSILON);
        assert_eq!(resolved.text_color, Some(theme.palette.background));
    }
}

#[test]
fn pack_geometry_matches_cn_kbd_rules() {
    let vega = Theme::light();
    assert_eq!(geometry::control_height(&vega), 20.0);
    assert_eq!(geometry::min_width(&vega), 20.0);
    assert_eq!(geometry::horizontal_padding(&vega), 4.0);
    assert_eq!(geometry::text_size(&vega), 12.0);

    // Luma / Sera: `h-5.5 min-w-5.5 px-1.5`.
    let luma = Theme::light().with_style(StyleId::Luma);
    assert_eq!(geometry::control_height(&luma), 22.0);
    assert_eq!(geometry::min_width(&luma), 22.0);
    assert_eq!(geometry::horizontal_padding(&luma), 6.0);

    // Mira: `text-[0.625rem]`.
    let mira = Theme::light().with_style(StyleId::Mira);
    assert_eq!(geometry::text_size(&mira), 10.0);
    assert_eq!(geometry::control_height(&mira), 20.0);
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
    let kbd: Kbd<'_, Message> = Kbd::text("B", &theme)
        .padding(Padding::individual(
            Spacing::S0_5,
            Spacing::S2,
            Spacing::S0_5,
            Spacing::S2,
        ))
        .expect("scale padding is supported");

    assert_eq!(
        kbd.padding,
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
    let error = Kbd::<Message>::text("B", &theme)
        .padding(Padding::individual_value(
            PaddingValue::Var(PaddingVar::new("--kbd-padding")),
            PaddingValue::Px(2.0),
            PaddingValue::Px(3.0),
            PaddingValue::Px(4.0),
        ))
        .expect_err("padding variables are unsupported");

    assert_eq!(
        error,
        KbdBuildError::UnsupportedPaddingVariable {
            name: "--kbd-padding"
        }
    );
    assert!(error.to_string().contains("--kbd-padding"));
}

#[test]
fn padding_auto_returns_a_descriptive_error() {
    let theme = Theme::light();
    let error = Kbd::<Message>::text("B", &theme)
        .padding(Padding::all(Spacing::Auto))
        .expect_err("auto padding is unsupported");

    assert_eq!(error, KbdBuildError::UnsupportedPaddingAuto);
    assert!(error.to_string().contains("auto"));
}

#[test]
fn locked_style_packs_default_to_no_radius() {
    let lyra = Theme::light().with_style(StyleId::Lyra);
    assert_eq!(style::effective_radius(&lyra, None), KbdRadius::None);

    let sera = Theme::light().with_style(StyleId::Sera);
    assert_eq!(style::effective_radius(&sera, None), KbdRadius::None);

    let vega = Theme::light().with_style(StyleId::Vega);
    assert_eq!(style::effective_radius(&vega, None), KbdRadius::Small);

    // Explicit radius wins even on locked packs.
    assert_eq!(
        style::effective_radius(&lyra, Some(KbdRadius::Full)),
        KbdRadius::Full
    );
}

#[test]
fn min_width_and_text_size_are_clamped() {
    let theme = Theme::light();
    let kbd: Kbd<'_, Message> = Kbd::text("B", &theme).min_width(-4.0).text_size(0.0);

    assert_eq!(kbd.min_width, Some(0.0));
    assert_eq!(kbd.text_size, Some(1.0));
}

#[test]
fn dimensions_and_style_override_are_configurable() {
    let theme = Theme::light();
    let kbd: Kbd<'_, Message> = Kbd::text("B", &theme)
        .width(Length::Fixed(120.0))
        .height(Length::Fixed(24.0))
        .style_override(|mut style| {
            style.text_color = Some(Color::from_rgb(1.0, 0.0, 1.0));
            style
        });

    assert_eq!(kbd.width, Length::Fixed(120.0));
    assert_eq!(kbd.height, Some(Length::Fixed(24.0)));
    assert!(kbd.style_override.is_some());

    let _: Element<'_, Message> = kbd.into();
}

#[test]
fn all_surfaces_resolve_in_light_and_dark_themes() {
    for theme in [Theme::light(), Theme::dark()] {
        for surface in [
            KbdSurface::Default,
            KbdSurface::Tooltip,
            KbdSurface::InputGroup,
        ] {
            let resolved = style::resolve_container_style(&theme, surface, None);
            assert!(resolved.text_color.is_some_and(|color| color.a.is_finite()));
            assert!(resolved.background.is_some());
        }
    }
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let kbd = Kbd::<NoDebugMessage>::text("B", &theme);
    let debug = format!("{kbd:?}");

    assert!(debug.contains("Kbd"));
    assert!(debug.contains("label"));
}

#[test]
fn configuration_enums_support_hashing_and_expected_order() {
    fn hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    let _ = hash(&KbdSurface::Default);
    let _ = hash(&KbdRadius::Full);
    assert!(KbdRadius::None < KbdRadius::Full);
    assert_eq!(KbdRadius::default(), KbdRadius::Small);
    assert_eq!(KbdSurface::default(), KbdSurface::Default);
}

#[test]
fn group_collects_children_and_clamps_spacing() {
    let theme = Theme::light();

    let group: KbdGroup<'_, Message> = KbdGroup::new()
        .push(Kbd::text("Ctrl", &theme))
        .push(Kbd::text("Shift", &theme))
        .push(text("then"))
        .spacing(-1.0);

    assert_eq!(group.children.len(), 3);
    assert_eq!(group.spacing, 0.0);

    let debug = format!("{group:?}");
    assert!(debug.contains("KbdGroup"));

    let _: Element<'_, Message> = group.into();
}

#[test]
fn group_supports_with_children_extend_and_default() {
    let theme = Theme::light();

    let group: KbdGroup<'_, Message> =
        KbdGroup::with_children([Kbd::text("⌘", &theme).into(), Kbd::text("K", &theme).into()])
            .extend([text("or").into(), Kbd::text("Esc", &theme).into()]);

    assert_eq!(group.children.len(), 4);
    assert_eq!(group.spacing, geometry::DEFAULT_KBD_GAP);

    let empty: KbdGroup<'_, Message> = KbdGroup::default();
    assert_eq!(empty.children.len(), 0);
}
