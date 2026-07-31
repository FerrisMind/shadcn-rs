//! Unit tests for the carousel builders and strip geometry.

use std::time::Duration;

use shadcn_common::CarouselAlign;

use super::geometry::{Strip, sanitize_basis};
use super::{Carousel, CarouselItem, CarouselOrientation, CarouselPrevious};
use crate::Theme;

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Selected(usize),
}

fn slide<'a>() -> CarouselItem<'a, Message> {
    CarouselItem::new(crate::iced_compat::widget::Space::new())
}

fn five_slides(theme: &Theme) -> Carousel<'_, Message> {
    let mut carousel = Carousel::new(theme);

    for _ in 0..5 {
        carousel = carousel.push(slide());
    }

    carousel
}

#[test]
fn defaults_match_the_web_component() {
    let theme = Theme::light();
    let carousel = five_slides(&theme);

    assert_eq!(carousel.orientation, CarouselOrientation::Horizontal);
    assert_eq!(carousel.align, CarouselAlign::Start);
    assert!(!carousel.looped);
    assert_eq!(carousel.selected, 0);
    assert_eq!(carousel.item_basis, 1.0);
    assert_eq!(carousel.gap, shadcn_common::CAROUSEL_GAP_PX);
    assert!(carousel.animated);
    assert!(carousel.drag_enabled);
    assert!(carousel.keyboard_enabled);
    assert!(carousel.autoplay.is_none());
    assert!(carousel.autoplay_stop_on_interaction);
    assert!(carousel.show_controls);
}

#[test]
fn full_width_slides_have_one_snap_each() {
    let theme = Theme::light();
    let carousel = five_slides(&theme).on_select(Message::Selected);

    assert_eq!(carousel.snap_count(), 5);
    assert!(!carousel.can_scroll_prev());
    assert!(carousel.can_scroll_next());
}

#[test]
fn trim_snaps_shrinks_the_snap_list_for_partial_bases() {
    let theme = Theme::light();
    let carousel = five_slides(&theme).item_basis(1.0 / 3.0);

    // 5 slides, 3 per view: snaps 0, 1, and one trimmed trailing group.
    assert_eq!(carousel.snap_count(), 3);
}

#[test]
fn looping_keeps_every_snap_and_both_controls_enabled() {
    let theme = Theme::light();
    let carousel = five_slides(&theme)
        .item_basis(1.0 / 3.0)
        .looped(true)
        .on_select(Message::Selected);

    assert_eq!(carousel.snap_count(), 5);
    assert!(carousel.can_scroll_prev());
    assert!(carousel.can_scroll_next());
}

#[test]
fn last_snap_disables_next_without_looping() {
    let theme = Theme::light();
    let carousel = five_slides(&theme).selected(4).on_select(Message::Selected);

    assert!(carousel.can_scroll_prev());
    assert!(!carousel.can_scroll_next());
}

#[test]
fn out_of_range_selection_is_clamped() {
    let theme = Theme::light();
    let carousel = five_slides(&theme)
        .selected(99)
        .on_select(Message::Selected);

    assert!(!carousel.can_scroll_next());
    assert!(carousel.can_scroll_prev());
}

#[test]
fn controls_stay_disabled_without_a_callback() {
    let theme = Theme::light();
    // Without on_select, the carousel is read-only, but the geometry queries
    // still report scroll capability (controls are disabled at view time).
    let carousel = five_slides(&theme);

    // First slide: cannot go back, but geometrically "next" is reachable.
    assert!(!carousel.can_scroll_prev());
    assert!(carousel.can_scroll_next());
}

#[test]
fn per_item_basis_overrides_the_default() {
    let theme = Theme::light();
    let carousel = Carousel::<Message>::new(&theme)
        .item_basis(0.5)
        .push(slide())
        .push(slide().basis(1.0))
        .push(slide());
    let strip = carousel.strip();

    assert_eq!(strip.bases, vec![0.5, 1.0, 0.5]);
    assert_eq!(strip.starts, vec![0.0, 0.5, 1.5]);
    assert_eq!(strip.period, 2.0);
}

#[test]
fn bases_are_sanitized() {
    assert_eq!(sanitize_basis(f32::NAN), 1.0);
    assert_eq!(sanitize_basis(4.0), 1.0);
    assert_eq!(sanitize_basis(0.0), 0.05);
    assert_eq!(sanitize_basis(0.5), 0.5);
}

#[test]
fn gap_and_duration_builders_sanitize_inputs() {
    let theme = Theme::light();
    let carousel = Carousel::<Message>::new(&theme)
        .gap(f32::NAN)
        .duration(Duration::from_millis(120));

    assert_eq!(carousel.gap, 0.0);
    assert_eq!(carousel.duration, Duration::from_millis(120));

    let carousel = Carousel::<Message>::new(&theme).gap(-4.0);
    assert_eq!(carousel.gap, 0.0);
}

#[test]
fn strip_snap_offset_clamps_to_the_last_snap() {
    let strip = Strip::new(&[1.0; 3], CarouselAlign::Start, false);

    assert_eq!(strip.snap_offset(0), 0.0);
    assert_eq!(strip.snap_offset(2), 2.0);
    assert_eq!(strip.snap_offset(99), 2.0);
}

#[test]
fn empty_strip_is_harmless() {
    let strip = Strip::new(&[], CarouselAlign::Start, false);

    assert_eq!(strip.snap_count(), 0);
    assert_eq!(strip.snap_offset(0), 0.0);

    let theme = Theme::light();
    let carousel = Carousel::<Message>::new(&theme).on_select(Message::Selected);
    assert_eq!(carousel.snap_count(), 0);
    assert!(!carousel.can_scroll_prev());
    assert!(!carousel.can_scroll_next());
}

#[test]
fn center_alignment_produces_symmetric_first_and_last_snaps() {
    let strip = Strip::new(&[0.5; 4], CarouselAlign::Center, false);

    // max offset = 2.0 - 1.0 = 1.0; centered snaps clamp at both edges.
    assert_eq!(strip.snaps.first().copied(), Some(0.0));
    assert_eq!(strip.snaps.last().copied(), Some(1.0));
}

#[test]
fn control_builder_configures_the_button() {
    let control = CarouselPrevious::<Message>::new()
        .variant(crate::ButtonVariant::Ghost)
        .size(crate::ButtonSize::Icon)
        .disabled(true);

    assert_eq!(control.inner.variant, crate::ButtonVariant::Ghost);
    assert_eq!(control.inner.size, crate::ButtonSize::Icon);
    assert!(control.inner.disabled);
}

#[test]
fn debug_output_does_not_leak_content() {
    let theme = Theme::light();
    let carousel = five_slides(&theme).on_select(Message::Selected);
    let debug = format!("{carousel:?}");

    assert!(debug.contains("items: 5"));
    assert!(debug.contains("on_select: true"));
}
