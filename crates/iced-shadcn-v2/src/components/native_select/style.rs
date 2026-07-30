//! Mapping of `.cn-native-select` style-pack rules to resolved iced visuals.
//!
//! The web trigger is fully pack-styled while the dropdown is OS-rendered;
//! iced draws both, so the dropdown reuses the popover palette (`bg-popover`,
//! `text-popover-foreground`, accent hover) the way the styled shadcn select
//! does. Like the input, the translucent `focus-visible:ring-*` halo is
//! approximated by recoloring the border with `ring`, and Sera's
//! underline-only border degrades to a full hairline box.

use crate::iced_compat::{Color, Shadow, Vector};

use shadcn_common::{
    AccentColor, ComponentRadius, NATIVE_SELECT_DISABLED_OPACITY, NativeSelectRecipe,
};
use twill_core::prelude::theme::SemanticColor;

use super::types::{NativeSelectRadius, NativeSelectSize};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// `dark:aria-invalid:border-destructive/50`.
const DARK_INVALID_BORDER_ALPHA: f32 = 0.5;
/// Wash of the selected (but not hovered) dropdown row.
const SELECTED_ROW_ALPHA: f32 = 0.5;
/// Text alpha of a disabled dropdown option.
const DISABLED_ROW_ALPHA: f32 = 0.6;
/// Dropdown drop shadow (`shadow-md`).
const MENU_SHADOW_ALPHA: f32 = 0.12;

/// Interaction status the field style is resolved for.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NativeSelectStatus {
    /// Resting field.
    #[default]
    Active,
    /// Cursor over the closed field.
    Hovered,
    /// Dropdown is open (implies the web focus treatment).
    Opened,
    /// Field is disabled.
    Disabled,
}

/// Resolved visuals of the closed select field.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NativeSelectStyle {
    /// Field fill (`bg-input/N` of the pack).
    pub background: Color,
    /// Field border color (`border-input`, `ring` when open, `destructive`
    /// when invalid).
    pub border_color: Color,
    /// Field border width in px.
    pub border_width: f32,
    /// Field corner radius in px.
    pub radius: f32,
    /// Color of the selected-value text.
    pub text_color: Color,
    /// Color of the placeholder text.
    pub placeholder_color: Color,
    /// Color of the chevron icon (`text-muted-foreground`).
    pub icon_color: Color,
}

/// Resolved visuals of the dropdown surface and its rows.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NativeSelectMenuStyle {
    /// Dropdown fill (`bg-popover`).
    pub background: Color,
    /// Dropdown hairline border.
    pub border_color: Color,
    /// Dropdown corner radius in px.
    pub radius: f32,
    /// Dropdown drop shadow.
    pub shadow: Shadow,
    /// Option text (`text-popover-foreground`).
    pub text_color: Color,
    /// `<optgroup>` heading text (`text-muted-foreground`).
    pub group_label_color: Color,
    /// Disabled option text.
    pub disabled_text_color: Color,
    /// Fill of the hovered row (`bg-accent`).
    pub hovered_background: Color,
    /// Text of the hovered row (`text-accent-foreground`).
    pub hovered_text_color: Color,
    /// Wash of the currently selected row.
    pub selected_background: Color,
    /// Corner radius of the row highlight in px (`rounded-sm`).
    pub row_radius: f32,
}

/// `.cn-native-select` numbers of the active pack.
pub(super) fn recipe(theme: &Theme) -> NativeSelectRecipe {
    theme.style.native_select()
}

/// Value text size for `size` (`text-sm`, `data-[size=sm]:text-[0.625rem]`
/// on Mira).
pub(super) fn pack_text_size(theme: &Theme, size: NativeSelectSize) -> f32 {
    let recipe = recipe(theme);

    match size {
        NativeSelectSize::Sm => recipe.text_size_sm_px,
        NativeSelectSize::Default | NativeSelectSize::Lg => recipe.text_size_px,
    }
}

/// Chevron edge length for `size` (`size-4`, `size-3` on Mira sm).
pub(super) fn pack_icon_size(theme: &Theme, size: NativeSelectSize) -> f32 {
    let recipe = recipe(theme);

    match size {
        NativeSelectSize::Sm => recipe.icon_size_sm_px,
        NativeSelectSize::Default | NativeSelectSize::Lg => recipe.icon_size_px,
    }
}

impl NativeSelectSize {
    /// Control height in px from the style pack size ladder.
    pub(super) fn control_height(self, theme: &Theme) -> f32 {
        match self {
            Self::Sm => theme.style.control_height_sm_px,
            Self::Default => theme.style.control_height_md_px,
            Self::Lg => theme.style.control_height_lg_px,
        }
    }
}

pub(super) fn resolve_field_style(
    theme: &Theme,
    size: NativeSelectSize,
    radius: Option<NativeSelectRadius>,
    color: Option<AccentColor>,
    invalid: bool,
    disabled: bool,
    status: NativeSelectStatus,
) -> NativeSelectStyle {
    let pack = recipe(theme);
    let input = theme.semantic_color(SemanticColor::Input);

    let hovered = matches!(
        status,
        NativeSelectStatus::Hovered | NativeSelectStatus::Opened
    ) && !disabled;
    let fill_alpha = if theme.is_dark() {
        if hovered {
            pack.hover_fill_alpha_dark
        } else {
            pack.fill_alpha_dark
        }
    } else {
        pack.fill_alpha_light
    };
    let mut background = with_alpha(input, input.a * fill_alpha);
    let mut border_color = if pack.bordered {
        input
    } else {
        Color::TRANSPARENT
    };
    let mut text_color = theme.semantic_color(SemanticColor::Foreground);
    let mut placeholder_color = theme.semantic_color(SemanticColor::MutedForeground);
    let mut icon_color = placeholder_color;

    if status == NativeSelectStatus::Opened {
        // `focus-visible:border-ring` (+ the ring halo approximation).
        border_color = ring_color(theme, color);
    }

    // The CSS cascade lets `aria-invalid` outrank `focus-visible`.
    if invalid {
        let destructive = theme.semantic_color(SemanticColor::Destructive);
        border_color = if theme.is_dark() {
            with_alpha(destructive, destructive.a * DARK_INVALID_BORDER_ALPHA)
        } else {
            destructive
        };
    }

    // `has-[select:disabled]:opacity-50` fades the whole wrapper.
    if disabled {
        background = with_alpha(background, background.a * NATIVE_SELECT_DISABLED_OPACITY);
        border_color = with_alpha(
            border_color,
            border_color.a * NATIVE_SELECT_DISABLED_OPACITY,
        );
        text_color = with_alpha(text_color, text_color.a * NATIVE_SELECT_DISABLED_OPACITY);
        placeholder_color = with_alpha(
            placeholder_color,
            placeholder_color.a * NATIVE_SELECT_DISABLED_OPACITY,
        );
        icon_color = with_alpha(icon_color, icon_color.a * NATIVE_SELECT_DISABLED_OPACITY);
    }

    NativeSelectStyle {
        background,
        border_color,
        border_width: 1.0,
        radius: field_radius_px(theme, size, radius),
        text_color,
        placeholder_color,
        icon_color,
    }
}

pub(super) fn resolve_menu_style(theme: &Theme) -> NativeSelectMenuStyle {
    let accent = theme.semantic_color(SemanticColor::Accent);

    NativeSelectMenuStyle {
        background: theme.semantic_color(SemanticColor::Popover),
        border_color: theme.semantic_color(SemanticColor::Border),
        radius: menu_radius_px(theme),
        shadow: Shadow {
            color: Color {
                a: MENU_SHADOW_ALPHA,
                ..Color::BLACK
            },
            offset: Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        },
        text_color: theme.semantic_color(SemanticColor::PopoverForeground),
        group_label_color: theme.semantic_color(SemanticColor::MutedForeground),
        disabled_text_color: {
            let muted = theme.semantic_color(SemanticColor::MutedForeground);
            with_alpha(muted, muted.a * DISABLED_ROW_ALPHA)
        },
        hovered_background: accent,
        hovered_text_color: theme.semantic_color(SemanticColor::AccentForeground),
        selected_background: with_alpha(accent, accent.a * SELECTED_ROW_ALPHA),
        row_radius: match recipe(theme).radius {
            ComponentRadius::None => 0.0,
            _ => theme.style.twill_radius_sm.px_value(),
        },
    }
}

fn ring_color(theme: &Theme, color: Option<AccentColor>) -> Color {
    match color {
        None => theme.semantic_color(SemanticColor::Ring),
        // Accent overlays keep the neutral `ring` token, so an explicit
        // per-select accent recolors the focus border with the accent primary.
        Some(accent) => theme.color_with_accent(accent, SemanticColor::Primary),
    }
}

fn field_radius_px(
    theme: &Theme,
    size: NativeSelectSize,
    radius: Option<NativeSelectRadius>,
) -> f32 {
    let pack = recipe(theme);

    match radius {
        Some(NativeSelectRadius::None) => 0.0,
        Some(NativeSelectRadius::Small) => theme.style.twill_radius_sm.px_value(),
        Some(NativeSelectRadius::Medium) => theme.style.twill_radius_md.px_value(),
        Some(NativeSelectRadius::Large) => theme.style.twill_radius_lg.px_value(),
        Some(NativeSelectRadius::Full) => 9999.0,
        None => {
            let intent = match size {
                NativeSelectSize::Sm => pack.radius_sm,
                NativeSelectSize::Default | NativeSelectSize::Lg => pack.radius,
            };

            component_radius_px(theme, intent)
        }
    }
}

/// The dropdown keeps a moderate corner treatment even for pill fields —
/// a `rounded-4xl` trigger still opens a `rounded-md`-ish popup, matching
/// how the OS popup ignores the field shape on the web.
fn menu_radius_px(theme: &Theme) -> f32 {
    match recipe(theme).radius {
        ComponentRadius::None => 0.0,
        ComponentRadius::Full => theme.style.twill_radius_md.px_value(),
        intent => component_radius_px(theme, intent),
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: alpha.clamp(0.0, 1.0),
        ..color
    }
}
