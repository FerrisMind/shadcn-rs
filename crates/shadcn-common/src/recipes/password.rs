//! Password geometry from shadcn-svelte-extras `Password`.
//!
//! The extras component hard-codes Tailwind utilities (`flex flex-col gap-2`,
//! `size-9`, `pr-9` / `pr-[4.5rem]`, `h-[6px]`, score fill colors) rather than
//! pack-specific `.cn-*` tables, so this recipe is intentionally pack-invariant
//! — [`StyleId`] is accepted for API symmetry with other recipes.
//!
//! Pack-specific look comes from the **shared** [`crate::style::StylePack`] on
//! the app theme: Input, Toggle, and Button (via CopyButton) resolve Rhea/Nova/…
//! through `theme.style_id()`. Pass that same theme into every password part —
//! do not invent a separate Password style table.

use crate::style::StyleId;

use super::ComponentRadius;

/// Root column gap (`gap-2` → 8 px).
pub const PASSWORD_ROOT_GAP_PX: f32 = 8.0;
/// Absolute action hit target (`size-9` → 36 px).
pub const PASSWORD_ACTION_SIZE_PX: f32 = 36.0;
/// Lucide eye / eye-off glyph size (`size-4` → 16 px).
pub const PASSWORD_ACTION_ICON_PX: f32 = 16.0;
/// Narrower toggle width when copy is also mounted (`max-w-6` → 24 px).
pub const PASSWORD_TOGGLE_COMPACT_WIDTH_PX: f32 = 24.0;
/// End padding when one of toggle/copy is mounted (`pr-9` → 36 px).
pub const PASSWORD_END_PAD_ONE_PX: f32 = 36.0;
/// End padding when both toggle and copy are mounted (`pr-[4.5rem]` → 72 px).
pub const PASSWORD_END_PAD_BOTH_PX: f32 = 72.0;
/// Strength meter height (`h-[6px]`).
pub const PASSWORD_STRENGTH_HEIGHT_PX: f32 = 6.0;
/// Gap between strength segment rings (`gap-1` → 4 px).
pub const PASSWORD_STRENGTH_GAP_PX: f32 = 4.0;
/// Segment divider ring width (`ring-3` → 3 px).
pub const PASSWORD_STRENGTH_RING_PX: f32 = 3.0;
/// Number of strength segments (score 0–4 mapped onto 4 bars).
pub const PASSWORD_STRENGTH_SEGMENTS: u8 = 4;
/// Fill width transition (`duration-500`).
pub const PASSWORD_STRENGTH_TRANSITION_MS: u32 = 500;
/// Default `minScore` on the web Root (`3`).
pub const PASSWORD_DEFAULT_MIN_SCORE: u8 = 3;

/// Tailwind `red-500` used for scores 0–1.
pub const PASSWORD_SCORE_RED_RGB: (f32, f32, f32) = (239.0 / 255.0, 68.0 / 255.0, 68.0 / 255.0);
/// Tailwind `yellow-500` used for scores 2–3.
pub const PASSWORD_SCORE_YELLOW_RGB: (f32, f32, f32) = (234.0 / 255.0, 179.0 / 255.0, 8.0 / 255.0);
/// Tailwind `green-500` used for score 4.
pub const PASSWORD_SCORE_GREEN_RGB: (f32, f32, f32) = (34.0 / 255.0, 197.0 / 255.0, 94.0 / 255.0);

/// Geometry and interaction tokens for one password instance.
///
/// ```rust
/// use shadcn_common::{StyleId, password_recipe};
///
/// let recipe = password_recipe(StyleId::Vega);
/// assert_eq!(recipe.action_size_px, 36.0);
/// assert_eq!(recipe.strength_height_px, 6.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PasswordRecipe {
    /// Gap between root children (`gap-2`).
    pub root_gap_px: f32,
    /// Absolute action button edge length (`size-9`).
    pub action_size_px: f32,
    /// Icon glyph size inside actions (`size-4`).
    pub action_icon_px: f32,
    /// Compact toggle width when copy is also present (`max-w-6`).
    pub toggle_compact_width_px: f32,
    /// Input end padding with one trailing action (`pr-9`).
    pub end_pad_one_px: f32,
    /// Input end padding with toggle + copy (`pr-[4.5rem]`).
    pub end_pad_both_px: f32,
    /// Strength meter track height.
    pub strength_height_px: f32,
    /// Gap painted between strength segment rings.
    pub strength_gap_px: f32,
    /// Strength segment ring width (`ring-3`).
    pub strength_ring_px: f32,
    /// Strength track / segment corner radius (`rounded-full`).
    pub strength_radius: ComponentRadius,
    /// Fill transition duration in milliseconds.
    pub strength_transition_ms: u32,
    /// Default minimum acceptable zxcvbn score.
    pub default_min_score: u8,
}

impl Default for PasswordRecipe {
    fn default() -> Self {
        password_recipe(StyleId::Vega)
    }
}

/// Resolves password tokens.
///
/// `style` is accepted for API symmetry but unused — the extras markup is the
/// same across Vega…Rhea packs.
#[must_use]
pub const fn password_recipe(style: StyleId) -> PasswordRecipe {
    let _ = style;
    PasswordRecipe {
        root_gap_px: PASSWORD_ROOT_GAP_PX,
        action_size_px: PASSWORD_ACTION_SIZE_PX,
        action_icon_px: PASSWORD_ACTION_ICON_PX,
        toggle_compact_width_px: PASSWORD_TOGGLE_COMPACT_WIDTH_PX,
        end_pad_one_px: PASSWORD_END_PAD_ONE_PX,
        end_pad_both_px: PASSWORD_END_PAD_BOTH_PX,
        strength_height_px: PASSWORD_STRENGTH_HEIGHT_PX,
        strength_gap_px: PASSWORD_STRENGTH_GAP_PX,
        strength_ring_px: PASSWORD_STRENGTH_RING_PX,
        strength_radius: ComponentRadius::Full,
        strength_transition_ms: PASSWORD_STRENGTH_TRANSITION_MS,
        default_min_score: PASSWORD_DEFAULT_MIN_SCORE,
    }
}

/// RGB fill for a zxcvbn score (0–4), matching the extras `tv()` map.
#[must_use]
pub const fn password_score_rgb(score: u8) -> (f32, f32, f32) {
    match score {
        0 | 1 => PASSWORD_SCORE_RED_RGB,
        2 | 3 => PASSWORD_SCORE_YELLOW_RGB,
        _ => PASSWORD_SCORE_GREEN_RGB,
    }
}

/// End padding in px for the password input given which trailing actions are mounted.
#[must_use]
pub const fn password_end_padding_px(toggle_mounted: bool, copy_mounted: bool) -> f32 {
    match (toggle_mounted, copy_mounted) {
        (true, true) => PASSWORD_END_PAD_BOTH_PX,
        (true, false) | (false, true) => PASSWORD_END_PAD_ONE_PX,
        (false, false) => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_matches_extras_tailwind() {
        for style in StyleId::ALL {
            let recipe = password_recipe(style);
            assert_eq!(recipe.root_gap_px, 8.0);
            assert_eq!(recipe.action_size_px, 36.0);
            assert_eq!(recipe.action_icon_px, 16.0);
            assert_eq!(recipe.toggle_compact_width_px, 24.0);
            assert_eq!(recipe.end_pad_one_px, 36.0);
            assert_eq!(recipe.end_pad_both_px, 72.0);
            assert_eq!(recipe.strength_height_px, 6.0);
            assert_eq!(recipe.default_min_score, 3);
            assert_eq!(recipe.strength_radius, ComponentRadius::Full);
        }
    }

    #[test]
    fn end_padding_matches_pr_classes() {
        assert_eq!(password_end_padding_px(false, false), 0.0);
        assert_eq!(password_end_padding_px(true, false), 36.0);
        assert_eq!(password_end_padding_px(false, true), 36.0);
        assert_eq!(password_end_padding_px(true, true), 72.0);
    }

    #[test]
    fn score_colors_match_tailwind_500() {
        assert_eq!(password_score_rgb(0), PASSWORD_SCORE_RED_RGB);
        assert_eq!(password_score_rgb(1), PASSWORD_SCORE_RED_RGB);
        assert_eq!(password_score_rgb(2), PASSWORD_SCORE_YELLOW_RGB);
        assert_eq!(password_score_rgb(3), PASSWORD_SCORE_YELLOW_RGB);
        assert_eq!(password_score_rgb(4), PASSWORD_SCORE_GREEN_RGB);
    }
}
