//! Backend-agnostic scalar transition state.
//!
//! The primitive stores only values, timing, and easing. A backend owns frame
//! scheduling and decides when to call [`TransitionValue::advance`]. This is
//! enough for switches, collapsible reveals, tooltips, and determinate
//! progress without coupling the shared crate to a renderer.

use std::time::{Duration, Instant};

/// Easing curves used by shadcn component transitions.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Easing {
    /// Constant speed.
    Linear,
    /// Fast start with a decelerating finish.
    EaseOut,
    /// Smoothstep ease-in/ease-out.
    #[default]
    EaseInOut,
}

impl Easing {
    /// Maps linear progress in `0.0..=1.0` to an eased progress value.
    #[must_use]
    pub fn apply(self, progress: f32) -> f32 {
        let progress = if progress.is_finite() {
            progress.clamp(0.0, 1.0)
        } else {
            0.0
        };

        match self {
            Self::Linear => progress,
            Self::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            Self::EaseInOut => progress * progress * (3.0 - 2.0 * progress),
        }
    }
}

/// State for a scalar value moving between controlled targets.
///
/// The value is initialized on the first call to [`Self::advance`]. When no
/// transition is running, [`Self::displayed`] returns the caller's fallback
/// target so controlled widgets remain correct even if no redraw was emitted
/// between two views.
#[derive(Debug, Clone, Copy, Default)]
#[must_use = "a transition value has no effect until advanced or displayed"]
pub struct TransitionValue {
    initialized: bool,
    current: f32,
    from: f32,
    to: f32,
    start: Option<Instant>,
}

impl TransitionValue {
    /// Creates an uninitialized transition at zero.
    pub const fn new() -> Self {
        Self {
            initialized: false,
            current: 0.0,
            from: 0.0,
            to: 0.0,
            start: None,
        }
    }

    /// Resets the transition to `value` without starting an animation.
    pub fn reset(&mut self, value: f32) {
        let value = sanitize(value);
        self.initialized = true;
        self.current = value;
        self.from = value;
        self.to = value;
        self.start = None;
    }

    /// Advances the transition towards `target` for the frame at `now`.
    ///
    /// A zero duration behaves like a disabled animation. Changing the target
    /// while an animation is active starts the new segment at the currently
    /// displayed value, avoiding jumps in reversals.
    pub fn advance(
        &mut self,
        target: f32,
        animated: bool,
        duration: Duration,
        easing: Easing,
        now: Instant,
    ) {
        let target = sanitize(target);

        if !self.initialized {
            self.reset(target);
            return;
        }

        if self.to != target {
            if animated && !duration.is_zero() {
                self.from = self.current;
                self.to = target;
                self.start = Some(now);
            } else {
                self.reset(target);
            }
        }

        if (!animated || duration.is_zero()) && self.start.is_some() {
            self.reset(target);
            return;
        }

        let Some(start) = self.start else {
            return;
        };

        let progress = (now.saturating_duration_since(start).as_secs_f32()
            / duration.as_secs_f32())
        .clamp(0.0, 1.0);
        self.current = self.from + (self.to - self.from) * easing.apply(progress);

        if progress >= 1.0 {
            self.reset(target);
        }
    }

    /// Returns the value currently driven by an active transition, or the
    /// supplied controlled target when the transition is idle.
    #[must_use]
    pub fn displayed(&self, fallback_target: f32) -> f32 {
        if self.start.is_some() {
            self.current
        } else {
            sanitize(fallback_target)
        }
    }

    /// Returns the interpolated value, including the last settled value.
    #[must_use]
    pub const fn current(&self) -> f32 {
        self.current
    }

    /// Returns whether the first target has been observed.
    #[must_use]
    pub const fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Returns whether another frame is needed to finish the transition.
    #[must_use]
    pub const fn is_running(&self) -> bool {
        self.start.is_some()
    }
}

fn sanitize(value: f32) -> f32 {
    if value.is_finite() { value } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_frame_adopts_target_without_animating() {
        let now = Instant::now();
        let mut transition = TransitionValue::default();

        transition.advance(
            1.0,
            true,
            Duration::from_millis(100),
            Easing::EaseInOut,
            now,
        );

        assert!(transition.is_initialized());
        assert!(!transition.is_running());
        assert_eq!(transition.displayed(1.0), 1.0);
    }

    #[test]
    fn target_changes_interpolate_and_settle() {
        let start = Instant::now();
        let duration = Duration::from_millis(100);
        let mut transition = TransitionValue::default();

        transition.advance(0.0, true, duration, Easing::EaseInOut, start);
        transition.advance(1.0, true, duration, Easing::EaseInOut, start);
        assert!(transition.is_running());

        transition.advance(
            1.0,
            true,
            duration,
            Easing::EaseInOut,
            start + Duration::from_millis(50),
        );
        assert!(transition.displayed(1.0) > 0.0);
        assert!(transition.displayed(1.0) < 1.0);

        transition.advance(1.0, true, duration, Easing::EaseInOut, start + duration);
        assert!(!transition.is_running());
        assert_eq!(transition.displayed(1.0), 1.0);
    }

    #[test]
    fn disabling_animation_snaps_even_mid_transition() {
        let start = Instant::now();
        let duration = Duration::from_millis(100);
        let mut transition = TransitionValue::default();

        transition.advance(0.0, true, duration, Easing::EaseInOut, start);
        transition.advance(1.0, true, duration, Easing::EaseInOut, start);
        transition.advance(
            1.0,
            false,
            duration,
            Easing::EaseInOut,
            start + Duration::from_millis(10),
        );

        assert!(!transition.is_running());
        assert_eq!(transition.displayed(1.0), 1.0);
    }

    #[test]
    fn reversing_starts_from_the_current_value() {
        let start = Instant::now();
        let duration = Duration::from_millis(100);
        let mut transition = TransitionValue::default();

        transition.advance(0.0, true, duration, Easing::Linear, start);
        transition.advance(1.0, true, duration, Easing::Linear, start);
        transition.advance(
            1.0,
            true,
            duration,
            Easing::Linear,
            start + Duration::from_millis(40),
        );
        let before_reverse = transition.displayed(1.0);

        transition.advance(
            0.0,
            true,
            duration,
            Easing::Linear,
            start + Duration::from_millis(40),
        );
        assert_eq!(transition.current(), before_reverse);
    }
}
