//! Style / radius defaults matching shadcn-svelte create customizer.

use shadcn_common::{RadiusId, ResolvedTheme, StyleId, StylePack};

#[test]
fn lyra_and_sera_default_picker_resolves_to_zero_rem() {
    for style in [StyleId::Lyra, StyleId::Sera] {
        assert!(style.locks_radius());
        assert_eq!(style.default_radius_id(), RadiusId::Default);
        assert_eq!(style.default_radius_rem(), 0.0);
        assert_eq!(style.pack().radius.base_rem, 0.0);
        assert_eq!(
            style.pack().twill_radius_sm,
            twill_core::tokens::BorderRadius::None
        );
        assert_eq!(
            RadiusId::Default.resolved_rem(style.default_radius_rem()),
            0.0
        );
    }
}

#[test]
fn unlocked_styles_use_base_color_radius_when_default() {
    for style in [
        StyleId::Vega,
        StyleId::Nova,
        StyleId::Maia,
        StyleId::Mira,
        StyleId::Luma,
        StyleId::Rhea,
    ] {
        assert!(!style.locks_radius());
        assert_eq!(style.default_radius_id(), RadiusId::Default);
        assert_eq!(style.default_radius_rem(), 0.625);
        assert!((style.pack().radius.base_rem - 0.625).abs() < f32::EPSILON);
    }
}

#[test]
fn switching_to_sera_keeps_default_picker_and_zero_rem() {
    let theme = ResolvedTheme::default()
        .with_radius(RadiusId::Medium)
        .with_style(StyleId::Sera);
    assert_eq!(theme.radius_id(), RadiusId::Default);
    assert_eq!(theme.style_pack().radius.base_rem, 0.0);
}

#[test]
fn leaving_sera_restores_default_rem_not_stuck_none() {
    let theme = ResolvedTheme::default()
        .with_style(StyleId::Sera)
        .with_style(StyleId::Vega);
    assert_eq!(theme.radius_id(), RadiusId::Default);
    assert!((theme.style_pack().radius.base_rem - 0.625).abs() < f32::EPSILON);
}

#[test]
fn rhea_rejects_large_radius() {
    assert!(StyleId::Rhea.disallows_large_radius());
    let theme = ResolvedTheme::default()
        .with_style(StyleId::Rhea)
        .with_radius(RadiusId::Large);
    assert_eq!(theme.radius_id(), RadiusId::Default);
    assert!((theme.style_pack().radius.base_rem - 0.625).abs() < f32::EPSILON);
}

#[test]
fn style_preset_fonts_match_shadcn_presets() {
    assert_eq!(StylePack::VEGA.font_pack.sans.as_str(), "geist");
    assert_eq!(StylePack::NOVA.font_pack.sans.as_str(), "inter");
    assert_eq!(StylePack::LYRA.font_pack.sans.as_str(), "jetbrains-mono");
    assert_eq!(
        StylePack::SERA.font_pack.heading.as_str(),
        "instrument-serif"
    );
}

#[test]
fn radius_picker_values_match_shadcn_radii() {
    assert_eq!(RadiusId::None.rem(), Some(0.0));
    assert_eq!(RadiusId::Small.rem(), Some(0.45));
    assert_eq!(RadiusId::Medium.rem(), Some(0.625));
    assert_eq!(RadiusId::Large.rem(), Some(0.875));
    assert_eq!(RadiusId::Default.rem(), None);
}
