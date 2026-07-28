//! Geometry calculations for checkbox track and indicator.

use crate::components::checkbox::types::CheckboxSize;

/// Returns the size of the checkbox track in pixels.
pub fn track_size(size: CheckboxSize) -> f32 {
    match size {
        CheckboxSize::Sm => 20.0,
        CheckboxSize::Md => 24.0,
        CheckboxSize::Lg => 28.0,
    }
}

/// Returns the corner radius for the track.
pub fn track_radius(size: CheckboxSize) -> f32 {
    match size {
        CheckboxSize::Sm => 4.0,
        CheckboxSize::Md => 6.0,
        CheckboxSize::Lg => 8.0,
    }
}

/// Returns padding around the track (for centering indicator).
pub fn track_padding(size: CheckboxSize) -> f32 {
    match size {
        CheckboxSize::Sm => 2.0,
        CheckboxSize::Md => 3.0,
        CheckboxSize::Lg => 4.0,
    }
}
