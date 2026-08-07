//! Measurement validation and text geometry for [`super::TreeView`].

use super::{TreeViewBuildError, TreeViewMeasurement};

/// Validates a strictly positive logical-pixel measurement.
pub(super) fn positive(
    value: f32,
    measurement: TreeViewMeasurement,
) -> Result<f32, TreeViewBuildError> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(TreeViewBuildError::InvalidMeasurement(measurement))
    }
}

/// Validates a non-negative logical-pixel measurement.
pub(super) fn non_negative(
    value: f32,
    measurement: TreeViewMeasurement,
) -> Result<f32, TreeViewBuildError> {
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(TreeViewBuildError::InvalidMeasurement(measurement))
    }
}

/// Converts an available width into a conservative character budget.
pub(super) fn max_chars_for_width(width: f32, text_size: f32) -> usize {
    if !width.is_finite() || width <= 0.0 || !text_size.is_finite() || text_size <= 0.0 {
        return 1;
    }

    (width / (text_size * 0.56)).floor().max(1.0) as usize
}

/// Normalizes control characters and applies the configured width limit.
pub(super) fn label_for_width(label: &str, max_width: Option<f32>, text_size: f32) -> String {
    let normalized = label
        .chars()
        .map(|character| match character {
            '\r' | '\n' | '\t' => ' ',
            _ => character,
        })
        .collect::<String>();

    max_width.map_or(normalized.clone(), |width| {
        shadcn_common::truncate_tree_label(&normalized, max_chars_for_width(width, text_size))
    })
}
