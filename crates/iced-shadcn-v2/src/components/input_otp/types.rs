//! Configuration, status, and style types for the input-otp component.

use crate::iced_compat::Color;

/// Character filter applied to typed and pasted text, mirroring the
/// `pattern` prop of the web component (`REGEXP_ONLY_DIGITS`,
/// `REGEXP_ONLY_CHARS`, `REGEXP_ONLY_DIGITS_AND_CHARS` from `bits-ui`).
///
/// ```rust
/// use iced_shadcn_v2::InputOtpPattern;
///
/// assert_eq!(InputOtpPattern::default(), InputOtpPattern::Any);
/// assert!(InputOtpPattern::Digits.accepts('7'));
/// assert!(!InputOtpPattern::Digits.accepts('a'));
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputOtpPattern {
    /// Any non-control character (web default: no `pattern` prop).
    #[default]
    Any,
    /// ASCII digits only (`REGEXP_ONLY_DIGITS`).
    Digits,
    /// ASCII letters only (`REGEXP_ONLY_CHARS`).
    Chars,
    /// ASCII letters and digits (`REGEXP_ONLY_DIGITS_AND_CHARS`).
    DigitsAndChars,
}

impl InputOtpPattern {
    /// Whether the pattern accepts `character`.
    ///
    /// ```rust
    /// use iced_shadcn_v2::InputOtpPattern;
    ///
    /// assert!(InputOtpPattern::DigitsAndChars.accepts('x'));
    /// assert!(!InputOtpPattern::Chars.accepts('1'));
    /// ```
    pub fn accepts(self, character: char) -> bool {
        match self {
            Self::Any => !character.is_control(),
            Self::Digits => character.is_ascii_digit(),
            Self::Chars => character.is_ascii_alphabetic(),
            Self::DigitsAndChars => character.is_ascii_alphanumeric(),
        }
    }
}

/// Border radius preset for the outer corners of an [`super::InputOtp`]
/// group (`first:rounded-l-* last:rounded-r-*` on the web slots).
///
/// When no radius is set, the default follows the active style pack's
/// `.cn-input-otp-slot` corner treatment (`rounded-md` on Vega, pill on
/// Maia/Luma, square on Lyra/Sera, …).
///
/// ```rust
/// use iced_shadcn_v2::InputOtpRadius;
///
/// assert!(InputOtpRadius::None < InputOtpRadius::Full);
/// assert_eq!(InputOtpRadius::default(), InputOtpRadius::Medium);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputOtpRadius {
    /// No corner radius.
    None,
    /// Small corner radius.
    Small,
    /// Medium corner radius.
    #[default]
    Medium,
    /// Large corner radius.
    Large,
    /// Fully rounded (pill) outer corners.
    Full,
}

/// Interaction state an [`super::InputOtp`] is styled for.
///
/// ```rust
/// use iced_shadcn_v2::InputOtpStatus;
///
/// let status = InputOtpStatus::default();
/// assert!(!status.focused && !status.disabled && !status.invalid);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct InputOtpStatus {
    /// Whether the control owns keyboard focus.
    pub focused: bool,
    /// Whether the cursor is over the control.
    pub hovered: bool,
    /// Whether interaction is suppressed.
    pub disabled: bool,
    /// Whether the value was marked `aria-invalid`.
    pub invalid: bool,
}

/// Resolved colors and geometry an [`super::InputOtp`] paints for one status.
///
/// The ring is painted around the active slot only, approximating the web
/// `data-[active=true]:ring-*` halo; on Sera ([`Self::underline_only`]) the
/// active treatment recolors the underline instead.
///
/// ```rust
/// use iced::Color;
/// use iced_shadcn_v2::{InputOtp, InputOtpStyle, Theme};
///
/// # #[derive(Debug, Clone)]
/// # enum Message {}
/// let theme = Theme::light();
/// let otp = InputOtp::<Message>::new(&theme).style_override(|style, _status| InputOtpStyle {
///     caret: Color::BLACK,
///     ..style
/// });
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InputOtpStyle {
    /// Slot fill (`bg-input/N` of the pack).
    pub slot_background: Color,
    /// Resting slot border (`border-input`, `destructive` when invalid).
    pub slot_border: Color,
    /// Entered character color.
    pub slot_text: Color,
    /// Border of the active slot (`data-[active=true]:border-ring`).
    pub active_border: Color,
    /// Ring color painted around the active slot, alpha already applied.
    pub ring: Color,
    /// Ring width in logical pixels (`ring-1`/`ring-2`/`ring-3`).
    pub ring_width: f32,
    /// Fake caret color (`bg-foreground` caret line).
    pub caret: Color,
    /// Separator (minus icon) color between groups.
    pub separator: Color,
    /// Outer corner radius of each group in logical pixels.
    pub radius: f32,
    /// Sera's underline-only borders (`border-b-input` on transparent slots).
    pub underline_only: bool,
}
