//! Backend-agnostic password field state and zxcvbn strength estimation.
//!
//! Ports the pure behaviour of shadcn-svelte-extras `Password` (Root / Input /
//! Strength / ToggleVisibility / Copy) so iced and egui share one source of
//! truth for visibility, strength scoring, and invalidation.

use std::fmt;

use crate::recipes::{PASSWORD_DEFAULT_MIN_SCORE, password_end_padding_px};

/// zxcvbn strength score (`0`–`4`), matching `@zxcvbn-ts/core`.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum PasswordScore {
    /// Too guessable (`≤ 10³` guesses).
    #[default]
    Zero = 0,
    /// Very guessable (`≤ 10⁶` guesses).
    One = 1,
    /// Somewhat guessable (`≤ 10⁸` guesses).
    Two = 2,
    /// Safely unguessable (`≤ 10¹⁰` guesses).
    Three = 3,
    /// Very unguessable (`> 10¹⁰` guesses).
    Four = 4,
}

impl PasswordScore {
    /// All valid scores in ascending order.
    pub const ALL: [Self; 5] = [Self::Zero, Self::One, Self::Two, Self::Three, Self::Four];

    /// Converts a `0..=4` integer into a score, clamping out-of-range values.
    #[must_use]
    pub const fn from_u8_saturating(value: u8) -> Self {
        match value {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            _ => Self::Four,
        }
    }

    /// Numeric score used by the strength meter fill (`score / 4`).
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Short English label used by the extras strength demo.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Zero => "Poor",
            Self::One => "Weak",
            Self::Two => "Average",
            Self::Three => "Strong",
            Self::Four => "Secure",
        }
    }
}

impl From<PasswordScore> for u8 {
    fn from(score: PasswordScore) -> Self {
        score.as_u8()
    }
}

impl TryFrom<u8> for PasswordScore {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            _ => Err("password score must be in the range 0-4"),
        }
    }
}

impl fmt::Display for PasswordScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

/// Result of estimating password strength with zxcvbn.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PasswordStrength {
    score: PasswordScore,
    guesses_log10: f64,
    warning: Option<String>,
    suggestions: Vec<String>,
}

impl PasswordStrength {
    /// Creates a strength result from already-computed fields.
    #[must_use]
    pub fn new(
        score: PasswordScore,
        guesses_log10: f64,
        warning: Option<String>,
        suggestions: Vec<String>,
    ) -> Self {
        Self {
            score,
            guesses_log10,
            warning,
            suggestions,
        }
    }

    /// Overall score (`0`–`4`).
    #[must_use]
    pub const fn score(&self) -> PasswordScore {
        self.score
    }

    /// Base-10 logarithm of estimated guesses to crack.
    #[must_use]
    pub const fn guesses_log10(&self) -> f64 {
        self.guesses_log10
    }

    /// Primary warning from zxcvbn feedback, when present.
    #[must_use]
    pub fn warning(&self) -> Option<&str> {
        self.warning.as_deref()
    }

    /// Improvement suggestions from zxcvbn feedback.
    #[must_use]
    pub fn suggestions(&self) -> &[String] {
        &self.suggestions
    }
}

/// Estimates password strength with the Dropbox zxcvbn algorithm.
///
/// Empty passwords return score `0` with no feedback, matching the web
/// component which clears strength when the value is empty.
#[must_use]
pub fn estimate_password_strength(password: &str) -> PasswordStrength {
    if password.is_empty() {
        return PasswordStrength::new(PasswordScore::Zero, 0.0, None, Vec::new());
    }

    let entropy = zxcvbn::zxcvbn(password, &[]);
    let score = PasswordScore::from_u8_saturating(u8::from(entropy.score()));
    let feedback = entropy.feedback();
    let warning = feedback
        .and_then(|feedback| feedback.warning())
        .map(|warning| warning.to_string());
    let suggestions = feedback
        .map(|feedback| {
            feedback
                .suggestions()
                .iter()
                .map(|suggestion| suggestion.to_string())
                .collect()
        })
        .unwrap_or_default();

    PasswordStrength::new(score, entropy.guesses_log10(), warning, suggestions)
}

/// Controlled state for a password field suite.
///
/// The application owns this value and feeds updates through
/// [`password_reduce`]. Trailing-action mount flags drive input end padding
/// the same way the Svelte context tracks `toggleMounted` / `copyMounted`.
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PasswordState {
    value: String,
    hidden: bool,
    min_score: PasswordScore,
    tainted: bool,
    toggle_mounted: bool,
    copy_mounted: bool,
    strength_mounted: bool,
    strength: Option<PasswordStrength>,
}

impl fmt::Debug for PasswordState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PasswordState")
            .field("value", &"<redacted>")
            .field("value_len", &self.value.len())
            .field("hidden", &self.hidden)
            .field("min_score", &self.min_score)
            .field("tainted", &self.tainted)
            .field("toggle_mounted", &self.toggle_mounted)
            .field("copy_mounted", &self.copy_mounted)
            .field("strength_mounted", &self.strength_mounted)
            .field("strength", &self.strength)
            .finish()
    }
}

impl Default for PasswordState {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordState {
    /// Creates an empty, hidden password with the web default `minScore` of 3.
    #[must_use]
    pub fn new() -> Self {
        Self {
            value: String::new(),
            hidden: true,
            min_score: PasswordScore::from_u8_saturating(PASSWORD_DEFAULT_MIN_SCORE),
            tainted: false,
            toggle_mounted: false,
            copy_mounted: false,
            strength_mounted: false,
            strength: None,
        }
    }

    /// Current password value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Whether the input should mask its value (`type="password"`).
    #[must_use]
    pub const fn hidden(&self) -> bool {
        self.hidden
    }

    /// Minimum acceptable strength score.
    #[must_use]
    pub const fn min_score(&self) -> PasswordScore {
        self.min_score
    }

    /// Whether the user has edited the value at least once.
    #[must_use]
    pub const fn tainted(&self) -> bool {
        self.tainted
    }

    /// Whether a visibility toggle is composed into the input.
    #[must_use]
    pub const fn toggle_mounted(&self) -> bool {
        self.toggle_mounted
    }

    /// Whether a copy action is composed into the input.
    #[must_use]
    pub const fn copy_mounted(&self) -> bool {
        self.copy_mounted
    }

    /// Whether a strength meter is composed under the input.
    #[must_use]
    pub const fn strength_mounted(&self) -> bool {
        self.strength_mounted
    }

    /// Latest strength estimate, when the meter is mounted and the value is non-empty.
    #[must_use]
    pub fn strength(&self) -> Option<&PasswordStrength> {
        self.strength.as_ref()
    }

    /// Current score for the meter fill (`0` when unknown / empty).
    #[must_use]
    pub fn score(&self) -> PasswordScore {
        self.strength
            .as_ref()
            .map(PasswordStrength::score)
            .unwrap_or(PasswordScore::Zero)
    }

    /// Input end padding driven by mounted trailing actions.
    #[must_use]
    pub const fn end_padding_px(&self) -> f32 {
        password_end_padding_px(self.toggle_mounted, self.copy_mounted)
    }

    /// Whether the input should be marked `aria-invalid`.
    ///
    /// Matches the web rule: strength is mounted, the field is tainted, the
    /// value is non-empty, and the score is below [`Self::min_score`].
    #[must_use]
    pub fn is_invalid(&self) -> bool {
        if !self.strength_mounted || !self.tainted || self.value.is_empty() {
            return false;
        }
        self.score() < self.min_score
    }

    /// Sets the initial value without marking the field tainted.
    #[must_use]
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        if self.strength_mounted {
            self.refresh_strength();
        }
        self
    }

    /// Sets whether the value starts hidden.
    #[must_use]
    pub const fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Sets the minimum acceptable score (`0`–`4`).
    #[must_use]
    pub const fn with_min_score(mut self, min_score: PasswordScore) -> Self {
        self.min_score = min_score;
        self
    }

    /// Marks the visibility toggle as composed (affects end padding).
    #[must_use]
    pub const fn with_toggle_mounted(mut self, mounted: bool) -> Self {
        self.toggle_mounted = mounted;
        self
    }

    /// Marks the copy action as composed (affects end padding).
    #[must_use]
    pub const fn with_copy_mounted(mut self, mounted: bool) -> Self {
        self.copy_mounted = mounted;
        self
    }

    /// Marks the strength meter as composed and refreshes the estimate.
    #[must_use]
    pub fn with_strength_mounted(mut self, mounted: bool) -> Self {
        self.strength_mounted = mounted;
        if mounted {
            self.refresh_strength();
        } else {
            self.strength = None;
        }
        self
    }

    fn refresh_strength(&mut self) {
        if !self.strength_mounted || self.value.is_empty() {
            self.strength = None;
            return;
        }
        self.strength = Some(estimate_password_strength(&self.value));
    }
}

/// Application actions understood by [`password_reduce`].
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PasswordAction {
    /// Replace the password value and mark the field tainted.
    SetValue(String),
    /// Show or hide the plaintext value.
    SetHidden(bool),
    /// Flip the current visibility.
    ToggleHidden,
    /// Change the minimum acceptable score.
    SetMinScore(PasswordScore),
    /// Compose / unmount the visibility toggle.
    MountToggle(bool),
    /// Compose / unmount the copy action.
    MountCopy(bool),
    /// Compose / unmount the strength meter.
    MountStrength(bool),
    /// Recompute strength for the current value.
    RefreshStrength,
}

/// Reduces one password action into the next controlled state.
#[must_use]
pub fn password_reduce(mut state: PasswordState, action: PasswordAction) -> PasswordState {
    match action {
        PasswordAction::SetValue(value) => {
            if state.value != value {
                state.tainted = true;
                state.value = value;
                state.refresh_strength();
            }
        }
        PasswordAction::SetHidden(hidden) => {
            state.hidden = hidden;
        }
        PasswordAction::ToggleHidden => {
            state.hidden = !state.hidden;
        }
        PasswordAction::SetMinScore(min_score) => {
            state.min_score = min_score;
        }
        PasswordAction::MountToggle(mounted) => {
            state.toggle_mounted = mounted;
        }
        PasswordAction::MountCopy(mounted) => {
            state.copy_mounted = mounted;
        }
        PasswordAction::MountStrength(mounted) => {
            state.strength_mounted = mounted;
            if mounted {
                state.refresh_strength();
            } else {
                state.strength = None;
            }
        }
        PasswordAction::RefreshStrength => {
            state.refresh_strength();
        }
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_strength_is_zero() {
        let strength = estimate_password_strength("");
        assert_eq!(strength.score(), PasswordScore::Zero);
        assert!(strength.warning().is_none());
    }

    #[test]
    fn weak_password_scores_below_default_min() {
        let strength = estimate_password_strength("password");
        assert!(strength.score() < PasswordScore::Three);
    }

    #[test]
    fn strong_password_reaches_min_score() {
        let strength = estimate_password_strength("xK9#mP2$vL7@qR4!");
        assert!(strength.score() >= PasswordScore::Three);
    }

    #[test]
    fn invalid_requires_taint_strength_and_weak_score() {
        let state = PasswordState::new()
            .with_strength_mounted(true)
            .with_value("password");
        assert!(!state.is_invalid());

        let state = password_reduce(state, PasswordAction::SetValue("password1".to_owned()));
        assert!(state.tainted());
        assert!(state.is_invalid());
    }

    #[test]
    fn empty_value_is_not_invalid() {
        let state = password_reduce(
            PasswordState::new().with_strength_mounted(true),
            PasswordAction::SetValue(String::new()),
        );
        assert!(!state.is_invalid());
    }

    #[test]
    fn toggle_hidden_flips_mask() {
        let state = password_reduce(PasswordState::new(), PasswordAction::ToggleHidden);
        assert!(!state.hidden());
    }

    #[test]
    fn debug_redacts_value() {
        let state = PasswordState::new().with_value("super-secret");
        let debug = format!("{state:?}");
        assert!(debug.contains("<redacted>"));
        assert!(!debug.contains("super-secret"));
    }

    #[test]
    fn score_labels_match_extras_demo() {
        assert_eq!(PasswordScore::Zero.label(), "Poor");
        assert_eq!(PasswordScore::One.label(), "Weak");
        assert_eq!(PasswordScore::Two.label(), "Average");
        assert_eq!(PasswordScore::Three.label(), "Strong");
        assert_eq!(PasswordScore::Four.label(), "Secure");
    }
}
