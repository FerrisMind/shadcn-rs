//! Mapping of `.cn-select-*` style-pack rules to resolved iced visuals.

use crate::iced_compat::{Color, Shadow, Vector};

use shadcn_common::{SELECT_DISABLED_OPACITY, SelectRecipe};
use twill_core::prelude::theme::SemanticColor;

use super::types::{SelectRadius, SelectSize};
use crate::recipes::component_radius_px;
use crate::theme::Theme;

/// `dark:aria-invalid:border-destructive/50`.
const DARK_INVALID_BORDER_ALPHA: f32 = 0.5;

/// Interaction status the trigger style is resolved for.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SelectStatus {
    /// Resting trigger.
    #[default]
    Active,
    /// Cursor over the closed trigger.
    Hovered,
    /// Dropdown is open (implies the web focus treatment).
    Opened,
    /// Trigger is disabled.
    Disabled,
}

/// Resolved visuals of the closed select trigger.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectTriggerStyle {
    /// Trigger fill (`bg-input/N` of the pack).
    pub background: Color,
    /// Trigger border color.
    pub border_color: Color,
    /// Trigger border width in px.
    pub border_width: f32,
    /// Sera's underline-only border treatment.
    pub underline_only: bool,
    /// Trigger corner radius in px.
    pub radius: f32,
    /// Color of the selected-value text.
    pub text_color: Color,
    /// Color of the placeholder text.
    pub placeholder_color: Color,
    /// Color of the chevron icon (`text-muted-foreground`).
    pub icon_color: Color,
}

/// Resolved visuals of the open select content surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectContentStyle {
    /// Surface fill (`bg-popover`).
    pub background: Color,
    /// Content text color (`text-popover-foreground`).
    pub text_color: Color,
    /// Muted label / placeholder color.
    pub muted_color: Color,
    /// Hairline ring color (`ring-foreground/N`).
    pub border_color: Color,
    /// Hairline ring width (`ring-1`).
    pub border_width: f32,
    /// Surface corner radius in px.
    pub radius: f32,
    /// Surface drop shadow.
    pub shadow: Shadow,
    /// Highlighted / focused item fill (`bg-accent`).
    pub item_highlight_background: Color,
    /// Highlighted / focused item text (`text-accent-foreground`).
    pub item_highlight_text: Color,
    /// Selected-item check color (inherits accent foreground when highlighted,
    /// otherwise the content foreground).
    pub item_indicator_color: Color,
    /// Separator hairline (`bg-border`).
    pub separator_color: Color,
    /// Disabled item text alpha multiplier.
    pub item_disabled_opacity: f32,
    /// Item corner radius in px.
    pub item_radius: f32,
}

/// `.cn-select-*` numbers of the active pack.
pub(super) fn recipe(theme: &Theme) -> SelectRecipe {
    theme.style.select()
}

/// Trigger value text size for `size`.
pub(super) fn pack_text_size(theme: &Theme, size: SelectSize) -> f32 {
    let recipe = recipe(theme);

    match size {
        SelectSize::Sm => recipe.trigger_text_size_sm_px,
        SelectSize::Default => recipe.trigger_text_size_px,
    }
}

/// Chevron edge length for `size`.
pub(super) fn pack_icon_size(theme: &Theme, size: SelectSize) -> f32 {
    let recipe = recipe(theme);

    match size {
        SelectSize::Sm => recipe.icon_size_sm_px,
        SelectSize::Default => recipe.icon_size_px,
    }
}

impl SelectSize {
    /// Control height in px from the style pack size ladder.
    pub(super) fn control_height(self, theme: &Theme) -> f32 {
        match self {
            Self::Sm => theme.style.control_height_sm_px,
            Self::Default => theme.style.control_height_md_px,
        }
    }
}

pub(super) fn resolve_trigger_style(
    theme: &Theme,
    size: SelectSize,
    radius: Option<SelectRadius>,
    invalid: bool,
    disabled: bool,
    status: SelectStatus,
) -> SelectTriggerStyle {
    let pack = recipe(theme);
    let input = theme.semantic_color(SemanticColor::Input);

    let hovered = matches!(status, SelectStatus::Hovered | SelectStatus::Opened) && !disabled;
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

    if status == SelectStatus::Opened {
        border_color = theme.semantic_color(SemanticColor::Ring);
    }

    if invalid {
        let destructive = theme.semantic_color(SemanticColor::Destructive);
        border_color = if theme.is_dark() {
            with_alpha(destructive, destructive.a * DARK_INVALID_BORDER_ALPHA)
        } else {
            destructive
        };
    }

    if disabled {
        background = with_alpha(background, background.a * SELECT_DISABLED_OPACITY);
        border_color = with_alpha(border_color, border_color.a * SELECT_DISABLED_OPACITY);
        text_color = with_alpha(text_color, text_color.a * SELECT_DISABLED_OPACITY);
        placeholder_color = with_alpha(
            placeholder_color,
            placeholder_color.a * SELECT_DISABLED_OPACITY,
        );
        icon_color = with_alpha(icon_color, icon_color.a * SELECT_DISABLED_OPACITY);
    }

    SelectTriggerStyle {
        background,
        border_color,
        border_width: 1.0,
        underline_only: pack.underline_only,
        radius: trigger_radius_px(theme, size, radius),
        text_color,
        placeholder_color,
        icon_color,
    }
}

pub(super) fn resolve_content_style(theme: &Theme) -> SelectContentStyle {
    let pack = recipe(theme);
    let ring_alpha = if theme.is_dark() {
        pack.content_ring_alpha_dark
    } else {
        pack.content_ring_alpha
    };

    SelectContentStyle {
        background: theme.palette.popover,
        text_color: theme.palette.popover_foreground,
        muted_color: theme.semantic_color(SemanticColor::MutedForeground),
        border_color: theme.palette.foreground.scale_alpha(ring_alpha),
        border_width: 1.0,
        radius: component_radius_px(theme, pack.content_radius),
        shadow: Shadow {
            color: Color::BLACK.scale_alpha(pack.content_shadow.alpha),
            offset: Vector::new(0.0, pack.content_shadow.offset_y_px),
            blur_radius: pack.content_shadow.blur_px,
        },
        item_highlight_background: theme.semantic_color(SemanticColor::Accent),
        item_highlight_text: theme.semantic_color(SemanticColor::AccentForeground),
        item_indicator_color: theme.palette.popover_foreground,
        separator_color: theme.semantic_color(SemanticColor::Border),
        item_disabled_opacity: SELECT_DISABLED_OPACITY,
        item_radius: component_radius_px(theme, pack.item_radius),
    }
}

fn trigger_radius_px(theme: &Theme, size: SelectSize, radius: Option<SelectRadius>) -> f32 {
    let pack = recipe(theme);

    match radius {
        Some(SelectRadius::None) => 0.0,
        Some(SelectRadius::Small) => theme.style.twill_radius_sm.px_value(),
        Some(SelectRadius::Medium) => theme.style.twill_radius_md.px_value(),
        Some(SelectRadius::Large) => theme.style.twill_radius_lg.px_value(),
        Some(SelectRadius::Full) => 9999.0,
        None => {
            let intent = match size {
                SelectSize::Sm => pack.trigger_radius_sm,
                SelectSize::Default => pack.trigger_radius,
            };

            component_radius_px(theme, intent)
        }
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        a: alpha.clamp(0.0, 1.0),
        ..color
    }
}
