//! Configuration types used by the kbd component.

/// Surface a [`super::Kbd`] is rendered on.
///
/// The shadcn-svelte `cn-kbd` restyles itself contextually
/// (`in-data-[slot=tooltip-content]:*`, `in-data-[slot=input-group]:*`).
/// iced has no CSS ancestor selectors, so the host surface is declared
/// explicitly on the builder instead.
///
/// ```rust
/// use iced_shadcn_v2::KbdSurface;
///
/// assert_eq!(KbdSurface::default(), KbdSurface::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KbdSurface {
    /// Regular content surface (`bg-muted text-muted-foreground`).
    #[default]
    Default,
    /// Inside a tooltip bubble (`bg-background/20 text-background`,
    /// dark: `bg-background/10`).
    Tooltip,
    /// Inside an input group addon (`bg-input`).
    InputGroup,
}

/// Border radius preset for a [`super::Kbd`].
///
/// When unset on the builder, kbds default to the small style-pack radius —
/// which reproduces the per-style web values (`rounded-sm` for Vega/Nova/Maia,
/// `rounded-lg` for Luma/Rhea, `rounded-xs` for Mira) — except for style packs
/// that lock radius to none (Lyra / Sera).
///
/// ```rust
/// use iced_shadcn_v2::KbdRadius;
///
/// assert!(KbdRadius::None < KbdRadius::Full);
/// assert_eq!(KbdRadius::default(), KbdRadius::Small);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KbdRadius {
    /// No corner radius.
    None,
    /// Small corner radius.
    #[default]
    Small,
    /// Medium corner radius.
    Medium,
    /// Large corner radius.
    Large,
    /// Fully rounded (pill) corners.
    Full,
}
