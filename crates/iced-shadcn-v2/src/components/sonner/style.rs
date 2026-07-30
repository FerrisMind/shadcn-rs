//! Theme-aware style resolution for Sonner toasts.

use crate::iced_compat::{Color, Shadow, Vector};
use crate::theme::Theme;
use shadcn_common::StyleId;

use super::types::ToastType;

/// Resolved colors and geometry for one toast surface.
///
/// The type is intentionally configured through `with_*` methods so style
/// overrides remain forward-compatible if more visual tokens are added.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToastStyle {
    background: Color,
    text: Color,
    description: Color,
    border: Color,
    icon: Color,
    action_background: Color,
    action_text: Color,
    cancel_background: Color,
    cancel_text: Color,
    border_width: f32,
    radius: f32,
    shadow: Shadow,
}

impl ToastStyle {
    /// Returns the toast surface color.
    pub const fn background(self) -> Color {
        self.background
    }

    /// Returns the title color.
    pub const fn text(self) -> Color {
        self.text
    }

    /// Returns the description color — a muted foreground, matching
    /// svelte-sonner's `color: #3f3f3f` (light) / `hsl(0,0%,91%)` (dark),
    /// captured on shadcn-svelte.com as `rgb(63, 63, 63)`.
    pub const fn description(self) -> Color {
        self.description
    }

    /// Returns the outline color.
    pub const fn border(self) -> Color {
        self.border
    }

    /// Returns the semantic icon color.
    pub const fn icon(self) -> Color {
        self.icon
    }

    /// Returns the primary action background color.
    pub const fn action_background(self) -> Color {
        self.action_background
    }

    /// Returns the primary action text color.
    pub const fn action_text(self) -> Color {
        self.action_text
    }

    /// Returns the cancel action background color.
    pub const fn cancel_background(self) -> Color {
        self.cancel_background
    }

    /// Returns the cancel action text color.
    pub const fn cancel_text(self) -> Color {
        self.cancel_text
    }

    /// Returns the outline width in pixels.
    pub const fn border_width(self) -> f32 {
        self.border_width
    }

    /// Returns the surface corner radius in pixels.
    pub const fn radius(self) -> f32 {
        self.radius
    }

    /// Returns the surface shadow.
    pub const fn shadow(self) -> Shadow {
        self.shadow
    }

    /// Replaces the surface color.
    #[must_use = "style methods return the modified style"]
    pub const fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }

    /// Replaces the text color.
    #[must_use = "style methods return the modified style"]
    pub const fn with_text(mut self, text: Color) -> Self {
        self.text = text;
        self
    }

    /// Replaces the description color.
    #[must_use = "style methods return the modified style"]
    pub const fn with_description(mut self, description: Color) -> Self {
        self.description = description;
        self
    }

    /// Replaces the outline color.
    #[must_use = "style methods return the modified style"]
    pub const fn with_border(mut self, border: Color) -> Self {
        self.border = border;
        self
    }

    /// Replaces the icon color.
    #[must_use = "style methods return the modified style"]
    pub const fn with_icon(mut self, icon: Color) -> Self {
        self.icon = icon;
        self
    }

    /// Replaces the primary action colors.
    #[must_use = "style methods return the modified style"]
    pub const fn with_action(mut self, background: Color, text: Color) -> Self {
        self.action_background = background;
        self.action_text = text;
        self
    }

    /// Replaces the cancel action colors.
    #[must_use = "style methods return the modified style"]
    pub const fn with_cancel(mut self, background: Color, text: Color) -> Self {
        self.cancel_background = background;
        self.cancel_text = text;
        self
    }

    /// Replaces the outline width, clamped to a non-negative value.
    #[must_use = "style methods return the modified style"]
    pub fn with_border_width(mut self, border_width: f32) -> Self {
        self.border_width = finite_non_negative(border_width);
        self
    }

    /// Replaces the corner radius, clamped to a non-negative value.
    #[must_use = "style methods return the modified style"]
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = finite_non_negative(radius);
        self
    }

    /// Replaces the surface shadow.
    #[must_use = "style methods return the modified style"]
    pub const fn with_shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = shadow;
        self
    }
}

pub(super) fn resolve_toast_style(
    theme: &Theme,
    toast_type: ToastType,
    rich_colors: bool,
    invert: bool,
) -> ToastStyle {
    let palette = &theme.palette;
    let base_background = if invert {
        palette.foreground
    } else {
        palette.popover
    };
    let base_text = if invert {
        palette.background
    } else {
        palette.popover_foreground
    };
    let base_border = if invert {
        palette.background.scale_alpha(0.32)
    } else {
        palette.border
    };

    let semantic = match toast_type {
        ToastType::Success => palette.chart_2,
        ToastType::Info => palette.chart_1,
        ToastType::Warning => palette.chart_4,
        ToastType::Error => palette.destructive,
        ToastType::Default | ToastType::Loading => palette.muted_foreground,
        // Future kinds added to the shared `non_exhaustive` `ToastType` fall
        // back to the neutral index color used by the default/loading kinds.
        _ => palette.muted_foreground,
    };

    // `richColors` recolours only the success/info/warning/error kinds — the
    // default and loading kinds stay on the neutral popover palette even in
    // rich mode, matching svelte-sonner's
    // `[data-rich-colors='true'] [data-type='success'|'info'|'warning'|'error']`
    // selector (verified against shadcn-svelte.com via Playwright).
    let colored = rich_colors && !matches!(toast_type, ToastType::Default | ToastType::Loading);
    let (background, border, icon) = if colored {
        (
            mix(
                base_background,
                semantic,
                if theme.is_dark() { 0.22 } else { 0.10 },
            ),
            mix(base_border, semantic, 0.42),
            semantic,
        )
    } else {
        // Without rich colors the icon inherits the toast's foreground color
        // (one neutral glyph), exactly like svelte-sonner's non-rich styling.
        (base_background, base_border, base_text)
    };

    ToastStyle {
        background,
        text: base_text,
        description: palette.muted_foreground,
        border,
        icon,
        action_background: base_text,
        action_text: base_background,
        cancel_background: base_border.scale_alpha(0.36),
        cancel_text: base_text,
        border_width: 1.0,
        radius: toast_radius(theme),
        shadow: Shadow {
            color: Color::BLACK.scale_alpha(0.10),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        },
    }
}

/// Resolves the `.cn-toast` radius from the active shadcn style pack.
///
/// The Svelte style sheets use `rounded-2xl` for the regular packs,
/// `rounded-md` for Mira, and `rounded-none` for Lyra/Sera. The radius scale
/// already includes custom picker overrides, so using it here keeps Sonner in
/// sync with the rest of the theme instead of freezing the surface at 8 px.
fn toast_radius(theme: &Theme) -> f32 {
    match theme.style_id() {
        StyleId::Mira => theme.radius_scale().md_px.max(0.0),
        StyleId::Lyra | StyleId::Sera => 0.0,
        StyleId::Vega | StyleId::Nova | StyleId::Maia | StyleId::Luma | StyleId::Rhea => {
            theme.radius_scale().xxl_px.max(0.0)
        }
    }
}

fn finite_non_negative(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

fn mix(start: Color, end: Color, amount: f32) -> Color {
    let amount = amount.clamp(0.0, 1.0);
    Color {
        r: start.r + (end.r - start.r) * amount,
        g: start.g + (end.g - start.g) * amount,
        b: start.b + (end.b - start.b) * amount,
        a: start.a + (end.a - start.a) * amount,
    }
}
