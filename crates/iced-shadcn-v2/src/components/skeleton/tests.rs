//! Behavioral tests for the skeleton component.

use std::time::Duration;

use crate::iced_compat::{Element, Length};
use twill_core::prelude::theme::SemanticColor;

use super::geometry::radius_px;
use super::*;
use crate::theme::Theme;

#[test]
fn defaults_match_shadcn_svelte_skeleton() {
    let theme = Theme::light();
    let skeleton = Skeleton::new(&theme);

    assert_eq!(skeleton.animation, SkeletonAnimation::Pulse);
    assert_eq!(
        skeleton.shape,
        SkeletonShape::Rounded(SkeletonRadius::Medium)
    );
    assert_eq!(skeleton.fill, SkeletonFill::Semantic(SemanticColor::Muted));
    assert_eq!(skeleton.width, Length::Fill);
    assert_eq!(skeleton.height, Length::Fixed(16.0));
    assert_eq!(skeleton.duration, Duration::from_secs(2));
}

#[test]
fn sera_and_lyra_default_to_no_radius() {
    let sera = Theme::light().with_style(shadcn_common::StyleId::Sera);
    assert_eq!(
        Skeleton::new(&sera).shape,
        SkeletonShape::Rounded(SkeletonRadius::None)
    );

    let lyra = Theme::light().with_style(shadcn_common::StyleId::Lyra);
    assert_eq!(
        Skeleton::new(&lyra).shape,
        SkeletonShape::Rounded(SkeletonRadius::None)
    );
}

#[test]
fn builder_supports_common_skeleton_shapes_and_dimensions() {
    let theme = Theme::light();
    let skeleton = Skeleton::new(&theme)
        .size(Length::Fixed(48.0))
        .circle()
        .animation(SkeletonAnimation::Static)
        .color(SemanticColor::Accent);

    assert_eq!(skeleton.width, Length::Fixed(48.0));
    assert_eq!(skeleton.height, Length::Fixed(48.0));
    assert_eq!(skeleton.shape, SkeletonShape::Circle);
    assert_eq!(skeleton.animation, SkeletonAnimation::Static);
    assert_eq!(skeleton.fill, SkeletonFill::Semantic(SemanticColor::Accent));
}

#[test]
fn custom_values_are_normalized() {
    let theme = Theme::light();
    let skeleton = Skeleton::new(&theme)
        .duration(Duration::ZERO)
        .radius(SkeletonRadius::Custom(-4.0));

    assert_eq!(skeleton.duration, Duration::from_millis(1));
    assert_eq!(
        radius_px(
            &theme,
            skeleton.shape,
            crate::iced_compat::Size::new(40.0, 20.0)
        ),
        0.0
    );
}

#[test]
fn radius_presets_follow_theme_and_shape_bounds() {
    let theme = Theme::light();
    let size = crate::iced_compat::Size::new(48.0, 20.0);

    assert_eq!(
        radius_px(&theme, SkeletonShape::Rounded(SkeletonRadius::None), size),
        0.0
    );
    assert_eq!(radius_px(&theme, SkeletonShape::Circle, size), 10.0);
    assert_eq!(
        radius_px(&theme, SkeletonShape::Rounded(SkeletonRadius::Full), size),
        10.0
    );
    assert!(radius_px(&theme, SkeletonShape::Rounded(SkeletonRadius::Large), size) <= 10.0);
}

#[test]
fn custom_color_and_canvas_conversion_are_available() {
    let theme = Theme::dark();
    let skeleton =
        Skeleton::new(&theme).custom_color(crate::iced_compat::Color::from_rgb(0.1, 0.2, 0.3));
    assert!(format!("{skeleton:?}").contains("Skeleton"));
    let _: Element<'_, ()> = skeleton.into();
}
