//! Recipe smoke tests aligned with shadcn-svelte style CSS.

use shadcn_common::{
    ComponentRadius, ControlSize, FontWeight, LabelContext, StyleId, StylePack, badge_recipe,
    button_size, button_type, label_recipe, skeleton_default_radius,
};

#[test]
fn label_vega_and_sera_match_css() {
    let vega = label_recipe(StyleId::Vega, LabelContext::Field);
    assert_eq!(vega.typography.size_px, 14.0);
    assert_eq!(vega.typography.weight, FontWeight::Medium);

    let sera = label_recipe(StyleId::Sera, LabelContext::Field);
    assert!(sera.typography.uppercase);
    assert_eq!(sera.typography.weight, FontWeight::Semibold);

    let peer = label_recipe(StyleId::Sera, LabelContext::AdjacentControl);
    assert!(!peer.typography.uppercase);
    assert_eq!(peer.typography.size_px, 14.0);
    assert!((peer.typography.line_height_px - 20.0).abs() < f32::EPSILON);
}

#[test]
fn button_heights_follow_style_packs() {
    assert_eq!(button_size(StyleId::Vega, ControlSize::Md).height_px, 36.0);
    assert_eq!(button_size(StyleId::Nova, ControlSize::Md).height_px, 32.0);
    assert_eq!(button_size(StyleId::Mira, ControlSize::Md).height_px, 28.0);
    assert_eq!(button_size(StyleId::Sera, ControlSize::Md).height_px, 40.0);
    assert_eq!(button_size(StyleId::Sera, ControlSize::Lg).height_px, 44.0);
    assert_eq!(button_size(StyleId::Rhea, ControlSize::Md).height_px, 32.0);

    assert_eq!(
        StylePack::SERA.control_height_md_px,
        button_size(StyleId::Sera, ControlSize::Md).height_px
    );
}

#[test]
fn button_sera_is_uppercase_semibold() {
    let ty = button_type(StyleId::Sera);
    assert!(ty.typography.uppercase);
    assert_eq!(ty.typography.weight, FontWeight::Semibold);
    assert_eq!(ty.default_radius, ComponentRadius::None);
}

#[test]
fn badge_sera_has_no_fixed_height() {
    let sera = badge_recipe(StyleId::Sera);
    assert!(sera.height_px.is_none());
    assert!(sera.typography.uppercase);
    assert_eq!(sera.default_radius, ComponentRadius::None);

    let vega = badge_recipe(StyleId::Vega);
    assert_eq!(vega.height_px, Some(20.0));
    assert_eq!(vega.default_radius, ComponentRadius::S4xl);
}

#[test]
fn skeleton_radius_matches_css() {
    assert_eq!(
        skeleton_default_radius(StyleId::Lyra),
        ComponentRadius::None
    );
    assert_eq!(
        skeleton_default_radius(StyleId::Sera),
        ComponentRadius::None
    );
    assert_eq!(skeleton_default_radius(StyleId::Vega), ComponentRadius::Md);
    assert_eq!(skeleton_default_radius(StyleId::Maia), ComponentRadius::Xl);
}
