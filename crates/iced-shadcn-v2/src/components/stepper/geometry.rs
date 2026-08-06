//! Numeric geometry for the stepper component.

use crate::iced_compat::{Length, Padding};
use crate::theme::Theme;

/// Layout metrics derived from the active style pack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct StepperMetrics {
    pub(super) indicator_size: f32,
    pub(super) indicator_ring: f32,
    pub(super) separator_top: f32,
    pub(super) separator_left: f32,
    pub(super) separator_thickness: f32,
    pub(super) vertical_gap: f32,
    pub(super) vertical_trigger_gap: f32,
    pub(super) title_size: f32,
    pub(super) title_line_height: f32,
    pub(super) description_size: f32,
    pub(super) description_line_height: f32,
    pub(super) title_weight: shadcn_common::FontWeight,
}

impl StepperMetrics {
    pub(super) fn for_theme(theme: &Theme) -> Self {
        let spacing = theme.style.spacing_unit_px.max(1.0);
        let compact = matches!(
            theme.style.id,
            shadcn_common::StyleId::Lyra | shadcn_common::StyleId::Mira
        );

        Self {
            indicator_size: 28.0,
            indicator_ring: 3.0,
            separator_top: 12.0,
            separator_left: 12.0,
            separator_thickness: 4.0,
            vertical_gap: spacing * 2.0,
            vertical_trigger_gap: spacing * 4.0,
            title_size: if compact { 16.0 } else { 18.0 },
            title_line_height: if compact { 24.0 } else { 28.0 },
            description_size: if compact { 12.0 } else { 14.0 },
            description_line_height: if compact { 18.0 } else { 20.0 },
            title_weight: shadcn_common::FontWeight::Medium,
        }
    }
}

pub(super) fn normalize_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

pub(super) fn normalize_min_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(1.0)
    } else {
        1.0
    }
}

pub(super) fn normalize_padding(padding: Padding) -> Padding {
    Padding {
        top: normalize_px(padding.top),
        right: normalize_px(padding.right),
        bottom: normalize_px(padding.bottom),
        left: normalize_px(padding.left),
    }
}

pub(super) fn resolve_length(length: Length, natural: f32, min: f32, max: f32) -> f32 {
    let max = max.max(min);
    match length {
        Length::Fixed(value) => normalize_px(value).clamp(min, max),
        Length::Fill | Length::FillPortion(_) => max,
        Length::Shrink => natural.clamp(min, max),
    }
}

pub(super) fn state_for_step(step: usize, active_step: usize) -> super::StepperItemState {
    if step < active_step {
        super::StepperItemState::Completed
    } else if step == active_step {
        super::StepperItemState::Active
    } else {
        super::StepperItemState::Inactive
    }
}

pub(super) fn resolve_step(requested: usize, item_count: usize) -> usize {
    if item_count == 0 {
        return 1;
    }

    requested.clamp(1, item_count)
}

pub(super) fn next_step(current: usize, item_count: usize) -> Option<usize> {
    (item_count > 0 && current < item_count).then_some(current + 1)
}

pub(super) fn previous_step(current: usize) -> Option<usize> {
    (current > 1).then_some(current - 1)
}
