//! Behavioral tests for the tooltip component.

use shadcn_common::{
    FloatingAlign, FloatingConfig, FloatingPadding, FloatingRect, FloatingSide, FloatingSticky,
    StyleId, compute_floating, tooltip_recipe,
};

use super::style;
use super::types::TooltipState;
use super::*;
use crate::iced_compat::widget::container;
use crate::iced_compat::{Element, time::Duration};
use crate::theme::Theme;

#[derive(Clone, Debug)]
enum Message {
    Pressed,
    OpenChanged(bool),
}

impl Message {
    fn is_open(&self) -> bool {
        matches!(self, Self::OpenChanged(true))
    }
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let tooltip: Tooltip<'_, Message> = Tooltip::text(container("Hover"), "Add to library", &theme)
        .side(TooltipSide::Right)
        .align(TooltipAlign::Start)
        .side_offset(4.0)
        .align_offset(2.0)
        .delay(Duration::from_millis(700))
        .duration(Duration::from_millis(200))
        .animated(false)
        .disabled(true)
        .open(true)
        .on_open_change(Message::OpenChanged)
        .arrow(false)
        .avoid_collisions(false)
        .collision_padding(16.0)
        .sticky(FloatingSticky::Always)
        .hide_when_detached(true)
        .arrow_padding(4.0)
        .max_width(200.0);

    assert!(matches!(tooltip.content, TooltipContent::Label(_)));
    assert_eq!(tooltip.side, TooltipSide::Right);
    assert_eq!(tooltip.align, TooltipAlign::Start);
    assert_eq!(tooltip.side_offset, 4.0);
    assert_eq!(tooltip.align_offset, 2.0);
    assert_eq!(tooltip.delay, Duration::from_millis(700));
    assert_eq!(tooltip.duration, Duration::from_millis(200));
    assert!(!tooltip.animated);
    assert!(tooltip.disabled);
    assert_eq!(tooltip.open, Some(true));
    assert!(tooltip.on_open_change.is_some());
    assert!(!tooltip.arrow);
    assert!(!tooltip.avoid_collisions);
    assert_eq!(tooltip.collision_padding, FloatingPadding::all(16.0));
    assert_eq!(tooltip.sticky, FloatingSticky::Always);
    assert!(tooltip.hide_when_detached);
    assert_eq!(tooltip.arrow_padding, Some(4.0));
    assert_eq!(tooltip.max_width, Some(200.0));
    assert!(std::ptr::eq(tooltip.theme, &theme));
}

#[test]
fn defaults_match_shadcn_svelte() {
    let theme = Theme::light();
    let tooltip: Tooltip<'_, Message> = Tooltip::text(container("Hover"), "Tip", &theme);

    // Provider delayDuration = 0, side = top, align = center, arrow shown.
    assert_eq!(tooltip.side, TooltipSide::Top);
    assert_eq!(tooltip.align, TooltipAlign::Center);
    assert_eq!(tooltip.side_offset, 0.0);
    assert_eq!(tooltip.delay, Duration::ZERO);
    assert!(tooltip.animated);
    assert!(tooltip.arrow);
    assert!(tooltip.avoid_collisions);
    assert_eq!(tooltip.open, None);
    assert!(!tooltip.disabled);
}

#[test]
fn text_and_generic_tooltips_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> =
        Tooltip::text(container("Hover"), "Add to library", &theme).into();

    let _: Element<'_, Message> = Tooltip::new(container("Hover"), container("Custom"), &theme)
        .open(true)
        .on_open_change(Message::OpenChanged)
        .into();

    let _ = Message::Pressed;
    assert!(Message::OpenChanged(true).is_open());
    assert!(!Message::OpenChanged(false).is_open());
}

#[test]
fn style_uses_swapped_foreground_background_pair() {
    let theme = Theme::light();
    let resolved = style::resolve_style(&theme);

    assert_eq!(resolved.background, theme.palette.foreground);
    assert_eq!(resolved.text_color, theme.palette.background);
    assert!(resolved.arrow_size > 0.0);
}

#[test]
fn style_override_patches_resolved_style() {
    let theme = Theme::light();
    let tooltip: Tooltip<'_, Message> = Tooltip::text(container("Hover"), "Tip", &theme)
        .style_override(|style| TooltipStyle {
            radius: 0.0,
            ..style
        });

    let resolved =
        (tooltip.style_override.as_ref().expect("override set"))(style::resolve_style(&theme));
    assert_eq!(resolved.radius, 0.0);
}

#[test]
fn recipe_tracks_style_pack_tokens() {
    // Sharp styles drop both radii; the base geometry stays constant.
    let lyra = tooltip_recipe(StyleId::Lyra);
    assert_eq!(lyra.arrow_radius_px, 0.0);

    let vega = tooltip_recipe(StyleId::Vega);
    assert_eq!(vega.pad_x_px, 12.0);
    assert_eq!(vega.pad_y_px, 6.0);
    assert_eq!(vega.max_width_px, 320.0);
    assert_eq!(vega.arrow_size_px, 10.0);
    assert_eq!(vega.typography.size_px, 12.0);
}

#[test]
fn state_visibility_follows_open_and_transition() {
    let mut state = TooltipState::default();
    assert!(!state.is_visible());

    state.open = true;
    assert!(state.is_visible());

    state.open = false;
    state.transition.reset(0.4);
    assert!(state.is_visible());

    state.transition.reset(0.0);
    assert!(!state.is_visible());
}

#[test]
fn side_and_align_map_to_floating_tokens() {
    assert_eq!(TooltipSide::Top.to_floating(), FloatingSide::Top);
    assert_eq!(TooltipSide::Right.to_floating(), FloatingSide::Right);
    assert_eq!(TooltipSide::Bottom.to_floating(), FloatingSide::Bottom);
    assert_eq!(TooltipSide::Left.to_floating(), FloatingSide::Left);
    assert_eq!(TooltipAlign::Start.to_floating(), FloatingAlign::Start);
    assert_eq!(TooltipAlign::Center.to_floating(), FloatingAlign::Center);
    assert_eq!(TooltipAlign::End.to_floating(), FloatingAlign::End);
}

#[test]
fn floating_pipeline_places_bubble_above_trigger() {
    let anchor = FloatingRect::new(300.0, 200.0, 100.0, 36.0);
    let boundary = FloatingRect::new(0.0, 0.0, 800.0, 600.0);
    let config = FloatingConfig::default().side_offset(5.0);

    let placement = compute_floating(anchor, 120.0, 28.0, boundary, &config);

    assert_eq!(placement.side, FloatingSide::Top);
    assert_eq!(placement.y, 200.0 - 5.0 - 28.0);
    assert_eq!(placement.arrow, anchor.center_x() - placement.x);
}
