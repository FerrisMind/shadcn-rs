//! Behavioral tests for the spinner component.

use std::time::Duration;

use super::render::{AI_LOADER_SEGMENTS, AI_LOADER_VIEWBOX};
use super::*;
use crate::theme::Theme;

#[test]
fn spinner_defaults_to_ai_loader_without_internal_animation() {
    let theme = Theme::light();
    let spinner = Spinner::new(&theme);
    assert_eq!(spinner.variant, SpinnerVariant::AiLoaderIcon);
    assert_eq!(spinner.variant, SpinnerVariant::default());
    assert!(!spinner.animated);
    assert_eq!(spinner.duration, Duration::from_millis(1000));
}

#[test]
fn spinner_duration_is_clamped_to_at_least_one_millisecond() {
    let theme = Theme::light();
    let spinner = Spinner::new(&theme).duration(Duration::ZERO);
    assert_eq!(spinner.duration, Duration::from_millis(1));
}

#[test]
fn spinner_progress_compatibility_uses_external_progress_when_not_animated() {
    let theme = Theme::light();
    let spinner = Spinner::new(&theme).progress(0.37);
    let state = SpinnerState::default();
    assert!((spinner.resolved_progress(&state) - 0.37).abs() < f32::EPSILON);
}

#[test]
fn spinner_progress_normalizes_non_finite_values() {
    let theme = Theme::light();

    assert_eq!(Spinner::new(&theme).progress(f32::NAN).progress, 0.0);
    assert_eq!(Spinner::new(&theme).progress(f32::INFINITY).progress, 0.0);
    assert_eq!(
        Spinner::new(&theme).progress(f32::NEG_INFINITY).progress,
        0.0
    );
}

#[test]
fn spinner_amplitudes_are_clamped_and_nan_becomes_silence() {
    let theme = Theme::light();
    let spinner =
        Spinner::new(&theme).amplitudes([f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -0.5, 1.5]);

    assert_eq!(spinner.amplitudes, Some([0.0, 0.0, 0.0, 0.0, 1.0]));
}

#[test]
fn ai_loader_segments_match_reference_contract() {
    assert_eq!(AI_LOADER_SEGMENTS.len(), 10);
    let expected_alpha = [1.0, 0.5, 0.9, 0.1, 0.4, 0.6, 0.2, 0.7, 0.3, 0.8];
    for (index, (_, _, alpha)) in AI_LOADER_SEGMENTS.iter().copied().enumerate() {
        assert!((alpha - expected_alpha[index]).abs() < 1e-6);
    }
}

#[test]
fn ai_loader_segment_scaling_stays_within_bounds() {
    let size = 16.0;
    let scale = size / AI_LOADER_VIEWBOX;
    for (start, end, _) in AI_LOADER_SEGMENTS {
        for (x, y) in [start, end] {
            let sx = x * scale;
            let sy = y * scale;
            assert!((0.0..=size).contains(&sx));
            assert!((0.0..=size).contains(&sy));
        }
    }
}
