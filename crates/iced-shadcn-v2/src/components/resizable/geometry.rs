//! Resizable spacing, layout math, and padding conversion.

use twill_core::prelude::{Padding, PaddingValue, Spacing};

use super::error::ResizableBuildError;
use super::types::{PaneConstraints, ResizableDirection, ResizableRadius};
use crate::theme::Theme;

/// Layout width of the divider line (`.cn-resizable-handle` → `w-px` / `h-px`).
pub(super) const HANDLE_LAYOUT_PX: f32 = 1.0;

/// Expanded hit target around the divider (`after:w-1` / `after:h-1`).
pub(super) const HANDLE_HIT_PX: f32 = 4.0;

/// Grip icon width (`.cn-resizable-handle-icon` → `w-1`).
pub(super) const GRIP_CROSS_PX: f32 = 4.0;

/// Grip icon length (`.cn-resizable-handle-icon` → `h-6`).
pub(super) const GRIP_MAIN_PX: f32 = 24.0;

pub(super) fn resolve_padding(
    padding: Padding,
) -> Result<crate::iced_compat::Padding, ResizableBuildError> {
    let (top, right, bottom, left) = padding.sides();

    Ok(crate::iced_compat::Padding {
        top: top.map(padding_value_px).transpose()?.unwrap_or(0.0),
        right: right.map(padding_value_px).transpose()?.unwrap_or(0.0),
        bottom: bottom.map(padding_value_px).transpose()?.unwrap_or(0.0),
        left: left.map(padding_value_px).transpose()?.unwrap_or(0.0),
    })
}

fn padding_value_px(value: PaddingValue) -> Result<f32, ResizableBuildError> {
    match value {
        PaddingValue::Scale(scale) => Ok(match scale {
            Spacing::S0 => 0.0,
            Spacing::Px => 1.0,
            Spacing::S0_5 => 2.0,
            Spacing::S1 => 4.0,
            Spacing::S1_5 => 6.0,
            Spacing::S2 => 8.0,
            Spacing::S2_5 => 10.0,
            Spacing::S3 => 12.0,
            Spacing::S3_5 => 14.0,
            Spacing::S4 => 16.0,
            Spacing::S5 => 20.0,
            Spacing::S6 => 24.0,
            Spacing::S7 => 28.0,
            Spacing::S8 => 32.0,
            Spacing::S9 => 36.0,
            Spacing::S10 => 40.0,
            Spacing::S11 => 44.0,
            Spacing::S12 => 48.0,
            Spacing::S14 => 56.0,
            Spacing::S16 => 64.0,
            Spacing::S20 => 80.0,
            Spacing::S24 => 96.0,
            Spacing::S28 => 112.0,
            Spacing::S32 => 128.0,
            Spacing::S36 => 144.0,
            Spacing::S40 => 160.0,
            Spacing::S44 => 176.0,
            Spacing::S48 => 192.0,
            Spacing::S52 => 208.0,
            Spacing::S56 => 224.0,
            Spacing::S60 => 240.0,
            Spacing::S64 => 256.0,
            Spacing::S72 => 288.0,
            Spacing::S80 => 320.0,
            Spacing::S96 => 384.0,
            Spacing::Auto => return Err(ResizableBuildError::UnsupportedPaddingAuto),
        }),
        PaddingValue::Px(px) => Ok(px.max(0.0)),
        PaddingValue::Rem(rem) => Ok((rem * 16.0).max(0.0)),
        PaddingValue::Var(name) => Err(ResizableBuildError::UnsupportedPaddingVariable {
            name: name.as_str(),
        }),
    }
}

/// Clamps a caller-supplied pixel length to a finite, non-negative value.
pub(super) fn normalize_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

/// Clamps a percentage to `0..=100`.
pub(super) fn normalize_percent(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 100.0)
    } else {
        0.0
    }
}

/// Normalizes `sizes` in place so they sum to `100` when the total is positive.
pub(super) fn normalize_layout(sizes: &mut [f32]) {
    for size in sizes.iter_mut() {
        *size = normalize_percent(*size);
    }

    let total: f32 = sizes.iter().sum();
    if total <= f32::EPSILON {
        if sizes.is_empty() {
            return;
        }

        let even = 100.0 / sizes.len() as f32;
        sizes.fill(even);
        return;
    }

    for size in sizes.iter_mut() {
        *size = (*size / total) * 100.0;
    }
}

/// Builds the initial layout from pane constraints when the app does not supply one.
pub(super) fn default_layout(constraints: &[PaneConstraints]) -> Vec<f32> {
    if constraints.is_empty() {
        return Vec::new();
    }

    let mut sizes: Vec<f32> = constraints
        .iter()
        .map(|pane| {
            if pane.collapsed {
                pane.collapsed_size
            } else {
                pane.default_size
            }
        })
        .collect();

    normalize_layout(&mut sizes);
    sizes
}

/// Frame corner radius in pixels.
pub(super) fn frame_radius_px(theme: &Theme, radius: ResizableRadius) -> f32 {
    match radius {
        ResizableRadius::None => 0.0,
        ResizableRadius::Medium => theme.radius_scale().md_px.max(0.0),
        ResizableRadius::Large => theme.radius_scale().lg_px.max(0.0),
        ResizableRadius::Px(px) => normalize_px(px),
    }
}

/// Grip pill radius (`.cn-resizable-handle-icon` → `rounded-lg`).
pub(super) fn grip_radius_px(theme: &Theme) -> f32 {
    theme.radius_scale().lg_px.max(0.0)
}

/// Available pane area along the split axis after subtracting handle thickness.
pub(super) fn pane_area_px(total: f32, handle_count: usize) -> f32 {
    let handles = handle_count as f32 * HANDLE_LAYOUT_PX;
    (total - handles).max(0.0)
}

/// Expands a handle's visual bounds to the hit target from the shadcn CSS.
pub(super) fn hit_bounds(
    visual: crate::iced_compat::Rectangle,
    direction: ResizableDirection,
) -> crate::iced_compat::Rectangle {
    use crate::iced_compat::Rectangle;

    match direction {
        ResizableDirection::Horizontal => {
            let extra = (HANDLE_HIT_PX - visual.width).max(0.0);
            Rectangle {
                x: visual.x - extra / 2.0,
                width: visual.width + extra,
                ..visual
            }
        }
        ResizableDirection::Vertical => {
            let extra = (HANDLE_HIT_PX - visual.height).max(0.0);
            Rectangle {
                y: visual.y - extra / 2.0,
                height: visual.height + extra,
                ..visual
            }
        }
    }
}

/// Applies a drag delta to the pane pair separated by `handle_index`.
pub(super) fn resize_pair(
    sizes: &mut [f32],
    constraints: &[PaneConstraints],
    handle_index: usize,
    delta_px: f32,
    pane_area_px: f32,
) -> bool {
    if pane_area_px <= f32::EPSILON {
        return false;
    }

    let left = handle_index;
    let right = handle_index + 1;

    if right >= sizes.len() || left >= constraints.len() || right >= constraints.len() {
        return false;
    }

    let delta_pct = (delta_px / pane_area_px) * 100.0;
    if !delta_pct.is_finite() || delta_pct.abs() <= f32::EPSILON {
        return false;
    }

    let left_bounds = effective_bounds(&constraints[left]);
    let right_bounds = effective_bounds(&constraints[right]);
    let left_size = sizes[left];
    let right_size = sizes[right];

    let min_delta = (left_bounds.0 - left_size).max(right_size - right_bounds.1);
    let max_delta = (left_bounds.1 - left_size).min(right_size - right_bounds.0);
    let applied = delta_pct.clamp(min_delta, max_delta);
    if applied.abs() <= f32::EPSILON {
        return false;
    }

    sizes[left] = left_size + applied;
    sizes[right] = right_size - applied;
    true
}

fn effective_bounds(pane: &PaneConstraints) -> (f32, f32) {
    if pane.collapsed {
        (pane.collapsed_size, pane.collapsed_size)
    } else {
        (pane.min_size, pane.max_size)
    }
}
