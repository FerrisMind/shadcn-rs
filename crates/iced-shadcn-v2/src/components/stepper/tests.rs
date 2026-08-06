use super::geometry::{
    StepperMetrics, next_step, normalize_min_px, normalize_padding, normalize_px, previous_step,
    resolve_length, resolve_step, state_for_step,
};
use super::{Stepper, StepperItemState, StepperOrientation};
use crate::iced_compat::{Length, Padding};
use crate::theme::Theme;

#[test]
fn orientation_reports_vertical_axis() {
    assert!(!StepperOrientation::Horizontal.is_vertical());
    assert!(StepperOrientation::Vertical.is_vertical());
}

#[test]
fn item_state_tracks_one_indexed_active_step() {
    assert_eq!(state_for_step(1, 2), StepperItemState::Completed,);
    assert_eq!(state_for_step(2, 2), StepperItemState::Active);
    assert_eq!(state_for_step(3, 2), StepperItemState::Inactive);
}

#[test]
fn step_bounds_are_safe_for_empty_and_out_of_range_values() {
    assert_eq!(resolve_step(0, 0), 1);
    assert_eq!(resolve_step(0, 3), 1);
    assert_eq!(resolve_step(9, 3), 3);
    assert_eq!(next_step(1, 3), Some(2));
    assert_eq!(next_step(3, 3), None);
    assert_eq!(previous_step(1), None);
    assert_eq!(previous_step(3), Some(2));
}

#[test]
fn invalid_numeric_overrides_are_normalized() {
    assert_eq!(normalize_px(-2.0), 0.0);
    assert_eq!(normalize_px(f32::NAN), 0.0);
    assert_eq!(normalize_min_px(0.0), 1.0);
    assert_eq!(normalize_min_px(f32::INFINITY), 1.0);

    let padding = normalize_padding(Padding {
        top: -1.0,
        right: 2.0,
        bottom: f32::NAN,
        left: f32::INFINITY,
    });
    assert_eq!(padding.top, 0.0);
    assert_eq!(padding.right, 2.0);
    assert_eq!(padding.bottom, 0.0);
    assert_eq!(padding.left, 0.0);
}

#[test]
fn length_resolution_respects_minimum_and_maximum() {
    assert_eq!(resolve_length(Length::Fixed(4.0), 20.0, 8.0, 40.0), 8.0);
    assert_eq!(resolve_length(Length::Fixed(80.0), 20.0, 8.0, 40.0), 40.0);
    assert_eq!(resolve_length(Length::Shrink, 20.0, 8.0, 40.0), 20.0);
    assert_eq!(resolve_length(Length::Fill, 20.0, 8.0, 40.0), 40.0);
}

#[test]
fn metrics_match_the_reference_defaults() {
    let theme = Theme::light();
    let metrics = StepperMetrics::for_theme(&theme);

    assert_eq!(metrics.indicator_size, 28.0);
    assert_eq!(metrics.indicator_ring, 3.0);
    assert_eq!(metrics.separator_top, 12.0);
    assert_eq!(metrics.separator_left, 12.0);
    assert_eq!(metrics.separator_thickness, 4.0);
    assert_eq!(metrics.vertical_gap, theme.style.spacing_unit_px * 2.0);
    assert_eq!(
        metrics.vertical_trigger_gap,
        theme.style.spacing_unit_px * 4.0
    );
    assert_eq!(metrics.title_size, 18.0);
    assert_eq!(metrics.description_size, 14.0);
}

#[test]
fn builder_exposes_controlled_step_helpers() {
    let theme = Theme::light();
    let stepper: Stepper<'_, ()> = Stepper::new(&theme).step(3);

    assert!(stepper.is_empty());
    assert_eq!(stepper.len(), 0);
    assert_eq!(stepper.active_step(), 1);
    assert_eq!(stepper.next_step(), None);
    assert_eq!(stepper.previous_step(), None);
    assert!(!stepper.can_increment());
    assert!(!stepper.can_decrement());
}
