//! Behavioral tests for the hover-card component.

use shadcn_common::{
    FloatingAlign, FloatingConfig, FloatingPadding, FloatingRect, FloatingSide, FloatingSticky,
    PopoverShadow, StyleId, compute_floating, hover_card_recipe,
};

use super::style;
use super::types::HoverCardState;
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
    let hover_card: HoverCard<'_, Message> =
        HoverCard::new(container("@sveltejs"), container("Body"), &theme)
            .side(HoverCardSide::Top)
            .align(HoverCardAlign::Start)
            .side_offset(10.0)
            .align_offset(2.0)
            .open_delay(Duration::from_millis(100))
            .close_delay(Duration::from_millis(50))
            .width(320.0)
            .duration(Duration::from_millis(200))
            .animated(false)
            .disabled(true)
            .open(true)
            .default_open(true)
            .on_open_change(Message::OpenChanged)
            .avoid_collisions(false)
            .collision_padding(16.0)
            .sticky(FloatingSticky::Always)
            .hide_when_detached(true)
            .close_on_click_outside(false)
            .close_on_escape(false);

    assert!(matches!(hover_card.content, HoverCardContent::Element(_)));
    assert_eq!(hover_card.side, HoverCardSide::Top);
    assert_eq!(hover_card.align, HoverCardAlign::Start);
    assert_eq!(hover_card.side_offset, 10.0);
    assert_eq!(hover_card.align_offset, 2.0);
    assert_eq!(hover_card.open_delay, Duration::from_millis(100));
    assert_eq!(hover_card.close_delay, Duration::from_millis(50));
    assert_eq!(hover_card.width, Some(320.0));
    assert_eq!(hover_card.duration, Duration::from_millis(200));
    assert!(!hover_card.animated);
    assert!(hover_card.disabled);
    assert_eq!(hover_card.open, Some(true));
    assert!(hover_card.default_open);
    assert!(hover_card.on_open_change.is_some());
    assert!(!hover_card.avoid_collisions);
    assert_eq!(hover_card.collision_padding, FloatingPadding::all(16.0));
    assert_eq!(hover_card.sticky, FloatingSticky::Always);
    assert!(hover_card.hide_when_detached);
    assert!(!hover_card.close_on_click_outside);
    assert!(!hover_card.close_on_escape);
    assert!(std::ptr::eq(hover_card.theme, &theme));
}

#[test]
fn defaults_match_shadcn_svelte() {
    let theme = Theme::light();
    let hover_card: HoverCard<'_, Message> =
        HoverCard::text(container("@sveltejs"), "Body", &theme);

    // side = bottom, align = center, sideOffset = 4, openDelay = 700,
    // closeDelay = 300, w-64, dismissable.
    assert_eq!(hover_card.side, HoverCardSide::Bottom);
    assert_eq!(hover_card.align, HoverCardAlign::Center);
    assert_eq!(hover_card.side_offset, 4.0);
    assert_eq!(hover_card.align_offset, 0.0);
    assert_eq!(
        hover_card.open_delay,
        Duration::from_millis(HOVER_CARD_OPEN_DELAY_MS)
    );
    assert_eq!(
        hover_card.close_delay,
        Duration::from_millis(HOVER_CARD_CLOSE_DELAY_MS)
    );
    assert_eq!(hover_card.width, None);
    assert_eq!(
        hover_card.duration,
        Duration::from_millis(HOVER_CARD_ANIMATION_MS)
    );
    assert!(hover_card.animated);
    assert!(hover_card.avoid_collisions);
    assert_eq!(hover_card.open, None);
    assert!(!hover_card.default_open);
    assert!(!hover_card.disabled);
    assert!(hover_card.close_on_click_outside);
    assert!(hover_card.close_on_escape);
}

#[test]
fn text_and_generic_hover_cards_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = HoverCard::text(
        container("@sveltejs"),
        "Cybernetically enhanced web apps.",
        &theme,
    )
    .into();

    let _: Element<'_, Message> =
        HoverCard::new(container("@sveltejs"), container("Profile card"), &theme)
            .open(true)
            .on_open_change(Message::OpenChanged)
            .into();

    let _ = Message::Pressed;
    assert!(Message::OpenChanged(true).is_open());
    assert!(!Message::OpenChanged(false).is_open());
}

#[test]
fn style_uses_popover_pair_with_foreground_ring() {
    let theme = Theme::light();
    let resolved = style::resolve_style(&theme);

    assert_eq!(resolved.background, theme.palette.popover);
    assert_eq!(resolved.text_color, theme.palette.popover_foreground);
    assert_eq!(resolved.border_width, 1.0);
    // `ring-foreground/10`: the foreground color at a tenth of its alpha.
    assert_eq!(
        resolved.border_color,
        theme.palette.foreground.scale_alpha(0.10)
    );
    assert!(resolved.shadow.blur_radius > 0.0);
}

#[test]
fn style_override_patches_resolved_style() {
    let theme = Theme::light();
    let hover_card: HoverCard<'_, Message> =
        HoverCard::text(container("@sveltejs"), "Body", &theme).style_override(|style| {
            HoverCardStyle {
                radius: 0.0,
                ..style
            }
        });

    let resolved =
        (hover_card.style_override.as_ref().expect("override set"))(style::resolve_style(&theme));
    assert_eq!(resolved.radius, 0.0);
}

#[test]
fn recipe_tracks_style_pack_tokens() {
    // Vega: `w-64 rounded-lg p-4 text-sm shadow-md ring-foreground/10`.
    let vega = hover_card_recipe(StyleId::Vega);
    assert_eq!(vega.pad_px, 16.0);
    assert_eq!(vega.width_px, 256.0);
    assert_eq!(vega.ring_alpha, 0.10);
    assert_eq!(vega.shadow, PopoverShadow::MD);
    assert_eq!(vega.typography.size_px, 14.0);

    // Nova: `w-64 rounded-lg p-2.5 text-sm`.
    let nova = hover_card_recipe(StyleId::Nova);
    assert_eq!(nova.pad_px, 10.0);
    assert_eq!(nova.width_px, 256.0);

    // Mira: `w-72 p-2.5 text-xs/relaxed`.
    let mira = hover_card_recipe(StyleId::Mira);
    assert_eq!(mira.width_px, 288.0);
    assert_eq!(mira.typography.size_px, 12.0);
    assert_eq!(mira.typography.line_height_px, 19.5);

    // Maia: `w-72 rounded-2xl shadow-2xl ring-foreground/5`.
    let maia = hover_card_recipe(StyleId::Maia);
    assert_eq!(maia.width_px, 288.0);
    assert_eq!(maia.shadow, PopoverShadow::XXL);
    assert_eq!(maia.ring_alpha, 0.05);

    // Luma: `shadow-lg ring-foreground/5 dark:ring-foreground/10`.
    let luma = hover_card_recipe(StyleId::Luma);
    assert_eq!(luma.shadow, PopoverShadow::LG);
    assert_eq!(luma.ring_alpha, 0.05);
    assert_eq!(luma.ring_alpha_dark, 0.10);
}

#[test]
fn state_visibility_follows_open_and_transition() {
    let mut state = HoverCardState::new(false);
    assert!(!state.is_visible());
    assert!(!state.requested_open);

    state.open = true;
    assert!(state.is_visible());

    state.open = false;
    state.transition.reset(0.4);
    assert!(state.is_visible());

    state.transition.reset(0.0);
    assert!(!state.is_visible());

    // `defaultOpen` seeds the uncontrolled intent only.
    let state = HoverCardState::new(true);
    assert!(state.requested_open);
    assert!(!state.open);
}

#[test]
fn side_and_align_map_to_floating_tokens() {
    assert_eq!(HoverCardSide::Top.to_floating(), FloatingSide::Top);
    assert_eq!(HoverCardSide::Right.to_floating(), FloatingSide::Right);
    assert_eq!(HoverCardSide::Bottom.to_floating(), FloatingSide::Bottom);
    assert_eq!(HoverCardSide::Left.to_floating(), FloatingSide::Left);
    assert_eq!(HoverCardAlign::Start.to_floating(), FloatingAlign::Start);
    assert_eq!(HoverCardAlign::Center.to_floating(), FloatingAlign::Center);
    assert_eq!(HoverCardAlign::End.to_floating(), FloatingAlign::End);
}

#[test]
fn floating_pipeline_places_surface_below_trigger() {
    let anchor = FloatingRect::new(300.0, 200.0, 100.0, 36.0);
    let boundary = FloatingRect::new(0.0, 0.0, 800.0, 600.0);
    let config = FloatingConfig::default()
        .side(FloatingSide::Bottom)
        .side_offset(4.0);

    let placement = compute_floating(anchor, 256.0, 120.0, boundary, &config);

    assert_eq!(placement.side, FloatingSide::Bottom);
    assert_eq!(placement.y, 200.0 + 36.0 + 4.0);
    // align = center on the trigger edge.
    assert_eq!(placement.x, anchor.center_x() - 256.0 / 2.0);
}
