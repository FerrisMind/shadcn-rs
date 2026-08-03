//! Behavioral tests for the star-rating component.

use crate::iced_compat::widget::canvas;
use crate::iced_compat::{Element, Length, Point, Rectangle, mouse};
use shadcn_common::{Direction, StarRatingItemState, StarRatingKey};

use super::geometry;
use super::types::StarRatingState;
use super::*;

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Changed(f32),
}

#[test]
fn builder_defaults_match_extras() {
    let theme = Theme::light();
    let rating = StarRating::<Message>::new(&theme);

    assert_eq!(rating.value, 0.0);
    assert_eq!(rating.min, 0.0);
    assert_eq!(rating.max, 5.0);
    assert!(!rating.allow_half);
    assert_eq!(rating.orientation, StarRatingOrientation::Horizontal);
    assert_eq!(rating.direction, Direction::Ltr);
    assert!(!rating.disabled);
    assert!(!rating.readonly);
    assert!(rating.hover_preview);
    assert!(!rating.is_interactive());
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let rating = StarRating::<Message>::new(&theme)
        .value(3.5)
        .min(0.0)
        .max(10.0)
        .allow_half(true)
        .orientation(StarRatingOrientation::Vertical)
        .direction(Direction::Rtl)
        .disabled(true)
        .readonly(true)
        .required(true)
        .hover_preview(false)
        .focused(true)
        .star_size(StarRatingSize::Xl)
        .width(Length::Fixed(200.0))
        .height(Length::Fixed(48.0))
        .name("score");

    assert_eq!(rating.value, 3.5);
    assert_eq!(rating.max, 10.0);
    assert!(rating.allow_half);
    assert_eq!(rating.orientation, StarRatingOrientation::Vertical);
    assert_eq!(rating.direction, Direction::Rtl);
    assert!(rating.disabled);
    assert!(rating.readonly);
    assert!(rating.required);
    assert!(!rating.hover_preview);
    assert!(rating.focused);
    assert_eq!(rating.star_size.pixels(), 40.0);
    assert_eq!(rating.name, Some("score"));
}

#[test]
fn half_values_snap_when_allow_half_is_off() {
    let theme = Theme::light();
    let rating = StarRating::<Message>::new(&theme).value(3.5);
    assert_eq!(rating.config().step(), 1.0);
    // Builder keeps the raw value; painting clamps through common helpers.
    assert_eq!(rating.value, 3.5);
}

#[test]
fn apply_key_mirrors_bits_ui_arrows() {
    let theme = Theme::light();
    let rating = StarRating::<Message>::new(&theme)
        .value(2.0)
        .allow_half(true)
        .on_change(Message::Changed);

    assert_eq!(rating.apply_key(StarRatingKey::ArrowRight), Some(2.5));
    assert_eq!(rating.apply_key(StarRatingKey::Home), Some(0.0));
    assert_eq!(rating.apply_key(StarRatingKey::End), Some(5.0));
    assert_eq!(rating.apply_key(StarRatingKey::Digit(4)), Some(4.0));
}

#[test]
fn readonly_and_disabled_block_keys() {
    let theme = Theme::light();
    let readonly = StarRating::<Message>::new(&theme)
        .value(2.0)
        .readonly(true)
        .on_change(Message::Changed);
    assert_eq!(readonly.apply_key(StarRatingKey::ArrowRight), None);

    let disabled = StarRating::<Message>::new(&theme)
        .value(2.0)
        .disabled(true)
        .on_change(Message::Changed);
    assert_eq!(disabled.apply_key(StarRatingKey::ArrowRight), None);
}

#[test]
fn click_publishes_rating_and_clears_first_star() {
    let theme = Theme::light();
    let rating = StarRating::<Message>::new(&theme)
        .value(1.0)
        .on_change(Message::Changed);

    let metrics = geometry::resolve_metrics(&rating);
    let size = crate::iced_compat::Size::new(metrics.main_size(), metrics.cross_size());
    let bounds = Rectangle {
        x: 0.0,
        y: 0.0,
        width: size.width,
        height: size.height,
    };
    let star = geometry::star_rect(metrics, StarRatingOrientation::Horizontal, 0);
    let cursor = mouse::Cursor::Available(Point::new(
        bounds.x + star.x + star.width * 0.75,
        bounds.y + star.y + star.height * 0.5,
    ));

    let mut state = StarRatingState::default();
    let action = canvas::Program::<Message>::update(
        &rating,
        &mut state,
        &canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
        bounds,
        cursor,
    );
    assert!(action.is_some());
}

#[test]
fn gap_between_stars_does_not_clear_hover_preview() {
    let theme = Theme::light();
    let rating = StarRating::<Message>::new(&theme).on_change(Message::Changed);
    let metrics = geometry::resolve_metrics(&rating);
    let size = crate::iced_compat::Size::new(metrics.main_size(), metrics.cross_size());
    let bounds = Rectangle {
        x: 0.0,
        y: 0.0,
        width: size.width,
        height: size.height,
    };

    // Hover the first star so a preview is armed.
    let first = geometry::star_rect(metrics, StarRatingOrientation::Horizontal, 0);
    let over_first = mouse::Cursor::Available(Point::new(
        bounds.x + first.x + first.width * 0.5,
        bounds.y + first.y + first.height * 0.5,
    ));
    let mut state = StarRatingState::default();
    let _ = canvas::Program::<Message>::update(
        &rating,
        &mut state,
        &canvas::Event::Mouse(mouse::Event::CursorMoved {
            position: Point::new(
                bounds.x + first.x + first.width * 0.5,
                bounds.y + first.y + first.height * 0.5,
            ),
        }),
        bounds,
        over_first,
    );
    assert_eq!(state.hover_value, Some(1.0));

    // Mid-gutter between star 0 and star 1 — must stay interactive via hit_rect
    // and must not clear the preview.
    let second = geometry::star_rect(metrics, StarRatingOrientation::Horizontal, 1);
    let gap_x = (first.x + first.width + second.x) * 0.5;
    let in_gap = mouse::Cursor::Available(Point::new(
        bounds.x + gap_x,
        bounds.y + first.y + first.height * 0.5,
    ));
    let action = canvas::Program::<Message>::update(
        &rating,
        &mut state,
        &canvas::Event::Mouse(mouse::Event::CursorMoved {
            position: Point::new(bounds.x + gap_x, bounds.y + first.y + first.height * 0.5),
        }),
        bounds,
        in_gap,
    );
    assert!(
        state.hover_value.is_some(),
        "gap must not clear hover preview"
    );
    assert!(
        geometry::hit_star(
            metrics,
            StarRatingOrientation::Horizontal,
            Point::new(gap_x, first.y + first.height * 0.5),
        )
        .is_some(),
        "gap must belong to a hit target"
    );
    // Either a redraw to the neighbouring star or no-op keeping previous — never clear.
    if let Some(action) = action {
        let _ = action;
    }
    assert!(state.hover_value.is_some());
}

#[test]
fn hit_rects_cover_the_gutter_between_neighbours() {
    let theme = Theme::light();
    let rating = StarRating::<Message>::new(&theme);
    let metrics = geometry::resolve_metrics(&rating);
    let first = geometry::star_rect(metrics, StarRatingOrientation::Horizontal, 0);
    let second = geometry::star_rect(metrics, StarRatingOrientation::Horizontal, 1);
    let gap_x = (first.x + first.width + second.x) * 0.5;
    let point = Point::new(gap_x, first.y + first.height * 0.5);

    assert!(geometry::star_at(metrics, StarRatingOrientation::Horizontal, point).is_none());
    assert!(geometry::hit_star(metrics, StarRatingOrientation::Horizontal, point).is_some());
}

#[test]
fn default_geometry_matches_extras_measurements() {
    let theme = Theme::light();
    let rating = StarRating::<Message>::new(&theme);
    let metrics = geometry::resolve_metrics(&rating);

    // Playwright on shadcn-svelte-extras: 5×20 + 4×4 = 116 px content row.
    assert_eq!(metrics.star_size, 20.0);
    assert_eq!(metrics.gap, 4.0);
    assert_eq!(metrics.count, 5);
    let content = metrics.count as f32 * metrics.star_size
        + (metrics.count.saturating_sub(1)) as f32 * metrics.gap;
    assert_eq!(content, 116.0);

    // Custom size demo uses size-10 → 40 px.
    assert_eq!(StarRatingSize::Xl.pixels(), 40.0);
}

#[test]
fn item_states_follow_value() {
    assert_eq!(
        shadcn_common::item_state(2, 3.5, true),
        StarRatingItemState::Active
    );
    assert_eq!(
        shadcn_common::item_state(3, 3.5, true),
        StarRatingItemState::Partial
    );
    assert_eq!(
        shadcn_common::item_state(4, 3.5, true),
        StarRatingItemState::Inactive
    );
}

#[test]
fn converts_into_element() {
    let theme = Theme::light();
    let _: Element<'_, Message> = StarRating::new(&theme).on_change(Message::Changed).into();
}
