//! Geometry calculations for checkbox track and indicator.

use crate::components::checkbox::types::CheckboxSize;
use crate::theme::Theme;

/// Returns the size of the checkbox track in pixels.
pub fn track_size(size: CheckboxSize) -> f32 {
    match size {
        CheckboxSize::Xs => 16.0,
        CheckboxSize::Sm => 20.0,
        CheckboxSize::Md => 24.0,
        CheckboxSize::Lg => 28.0,
    }
}

/// Returns the corner radius for the track from the active style pack.
///
/// Matches `.cn-checkbox` in `style-*.css` (`rounded-[4px]` / `[5px]` /
/// `[6px]` / `none`). The reference does not vary radius by size.
pub fn track_radius(theme: &Theme, _size: CheckboxSize) -> f32 {
    theme.style.checkbox().radius_px
}

/// Returns padding around the track (for centering indicator).
pub fn track_padding(size: CheckboxSize) -> f32 {
    match size {
        CheckboxSize::Xs => 1.5,
        CheckboxSize::Sm => 2.0,
        CheckboxSize::Md => 3.0,
        CheckboxSize::Lg => 4.0,
    }
}
