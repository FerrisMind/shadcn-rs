//! Configuration types used by the collapsible component.

/// Axis a [`super::Collapsible`] lays its slots on and reveals content along.
///
/// The web component is a plain `div`, so the axis is expressed with utility
/// classes there (`flex-col` vs `flex`); here it is a typed knob that also
/// selects which dimension of [`super::CollapsibleContent`] animates — the
/// counterpart of bits-ui's `--bits-collapsible-content-height` and
/// `--bits-collapsible-content-width` custom properties.
///
/// ```rust
/// use iced_shadcn_v2::CollapsibleOrientation;
///
/// assert_eq!(
///     CollapsibleOrientation::default(),
///     CollapsibleOrientation::Vertical
/// );
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CollapsibleOrientation {
    /// Slots stack in a column; the content animates its height.
    #[default]
    Vertical,
    /// Slots sit in a row; the content animates its width.
    Horizontal,
}

/// Cross-axis alignment of the slots inside a [`super::Collapsible`].
///
/// Mirrors the `items-*` utility the shadcn examples put on the root element
/// (the settings example uses `items-start`).
///
/// ```rust
/// use iced_shadcn_v2::CollapsibleAlignment;
///
/// assert_eq!(CollapsibleAlignment::default(), CollapsibleAlignment::Start);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CollapsibleAlignment {
    /// Slots hug the leading edge (`items-start`).
    #[default]
    Start,
    /// Slots are centered on the cross axis (`items-center`).
    Center,
    /// Slots hug the trailing edge (`items-end`).
    End,
}

/// Open/closed state of a collapsible, mirroring the web `data-state` attribute.
///
/// ```rust
/// use iced_shadcn_v2::CollapsibleState;
///
/// assert_eq!(CollapsibleState::from(true), CollapsibleState::Open);
/// assert!(!CollapsibleState::Open.toggled().is_open());
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CollapsibleState {
    /// The panel is collapsed (`data-state="closed"`).
    #[default]
    Closed,
    /// The panel is expanded (`data-state="open"`).
    Open,
}

impl CollapsibleState {
    /// Whether the panel is expanded.
    pub const fn is_open(self) -> bool {
        matches!(self, Self::Open)
    }

    /// The state a trigger press moves to.
    pub const fn toggled(self) -> Self {
        match self {
            Self::Closed => Self::Open,
            Self::Open => Self::Closed,
        }
    }
}

impl From<bool> for CollapsibleState {
    fn from(open: bool) -> Self {
        if open { Self::Open } else { Self::Closed }
    }
}

impl From<CollapsibleState> for bool {
    fn from(state: CollapsibleState) -> Self {
        state.is_open()
    }
}

/// Chevron painted inside a [`super::CollapsibleTrigger`].
///
/// shadcn-svelte leaves the glyph to the call site and rotates it with
/// `group-data-[state=open]:rotate-90`; this enum ships the same two rotations
/// as a themed canvas glyph so a trigger needs no icon font.
///
/// ```rust
/// use iced_shadcn_v2::CollapsibleIndicator;
///
/// assert_eq!(CollapsibleIndicator::default(), CollapsibleIndicator::Chevron);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CollapsibleIndicator {
    /// Right-pointing chevron that rotates a quarter turn when open.
    #[default]
    Chevron,
    /// Down-pointing chevron that rotates a half turn when open.
    ChevronDown,
}

impl CollapsibleIndicator {
    /// Rotation applied at the fully open state, in radians.
    pub(super) const fn open_angle(self) -> f32 {
        match self {
            Self::Chevron => std::f32::consts::FRAC_PI_2,
            Self::ChevronDown => std::f32::consts::PI,
        }
    }
}

/// Side of the trigger label a [`CollapsibleIndicator`] is painted on.
///
/// ```rust
/// use iced_shadcn_v2::CollapsibleIndicatorPlacement;
///
/// assert_eq!(
///     CollapsibleIndicatorPlacement::default(),
///     CollapsibleIndicatorPlacement::Leading
/// );
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CollapsibleIndicatorPlacement {
    /// Before the label, as in the shadcn file-tree example.
    #[default]
    Leading,
    /// After the label, as in a settings row.
    Trailing,
}

/// Timing curve of the reveal transition.
///
/// ```rust
/// use iced_shadcn_v2::CollapsibleEasing;
///
/// assert_eq!(CollapsibleEasing::default(), CollapsibleEasing::EaseInOut);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CollapsibleEasing {
    /// Constant speed.
    Linear,
    /// Fast start, decelerating finish (`ease-out`).
    EaseOut,
    /// Smoothstep, matching the CSS `ease-in-out` feel of the web component.
    #[default]
    EaseInOut,
}

impl CollapsibleEasing {
    /// Maps linear time in `0.0..=1.0` onto eased progress in `0.0..=1.0`.
    #[cfg(test)]
    pub(super) fn apply(self, time: f32) -> f32 {
        let time = if time.is_finite() {
            time.clamp(0.0, 1.0)
        } else {
            0.0
        };

        match self {
            Self::Linear => time,
            Self::EaseOut => 1.0 - (1.0 - time) * (1.0 - time),
            Self::EaseInOut => time * time * (3.0 - 2.0 * time),
        }
    }
}
