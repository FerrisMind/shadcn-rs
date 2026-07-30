//! Style resolution for sonner toast notifications.
//!
//! Maps toast types and theme mode to the visual properties (background,
//! border, text color) matching shadcn-svelte's sonner defaults.

use crate::iced_compat::Color;
use crate::theme::Theme;

use super::types::ToastType;

/// Resolved visual style for a toast notification.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ToastStyle {
    /// Toast background color.
    pub background: Color,
    /// Text color for the title and description.
    pub text: Color,
    /// Border color.
    pub border_color: Color,
    /// Border width in pixels.
    pub border_width: f32,
    /// Border radius in pixels.
    pub border_radius: f32,
    /// Shadow.
    pub shadow: crate::iced_compat::Shadow,
}

/// Default toast width in pixels (matches shadcn-svelte `--width: 356px`).
pub(crate) const TOAST_WIDTH: f32 = 356.0;

/// Default gap between toasts in pixels.
pub(crate) const TOAST_GAP: f32 = 14.0;

/// Default viewport offset in pixels.
pub(crate) const TOAST_OFFSET: f32 = 24.0;

/// Default toast padding in pixels.
pub(crate) const TOAST_PADDING: f32 = 16.0;

/// Default border radius in pixels.
pub(crate) const TOAST_RADIUS: f32 = 8.0;

/// Default toast font size in pixels.
pub(crate) const TOAST_FONT_SIZE: f32 = 13.0;

/// Default description font weight (400 = normal).
pub(crate) const DESCRIPTION_FONT_WEIGHT: u16 = 400;

/// Default title font weight (500 = medium).
pub(crate) const TITLE_FONT_WEIGHT: u16 = 500;

/// Default auto-dismiss duration in milliseconds.
pub(crate) const DEFAULT_DURATION_MS: u64 = 4000;

/// Maximum number of visible toasts.
pub(crate) const MAX_VISIBLE_TOASTS: usize = 3;

/// Scale factor for stacked toasts behind the front toast.
pub(crate) const STACK_SCALE_STEP: f32 = 0.05;

/// Duration of the enter/exit animation in milliseconds.
pub(crate) const ANIMATION_MS: u64 = 400;

/// Duration before unmounting after exit animation.
pub(crate) const UNMOUNT_DELAY_MS: u64 = 200;

/// Resolves the visual style for a toast based on its type and the current
/// theme.
pub(crate) fn resolve_toast_style(
    theme: &Theme,
    toast_type: ToastType,
    rich_colors: bool,
) -> ToastStyle {
    let is_dark = theme.is_dark();

    match toast_type {
        ToastType::Default => default_toast_style(theme, is_dark),
        ToastType::Success => {
            if rich_colors {
                rich_success_style(is_dark)
            } else {
                default_toast_style(theme, is_dark)
            }
        }
        ToastType::Info => {
            if rich_colors {
                rich_info_style(is_dark)
            } else {
                default_toast_style(theme, is_dark)
            }
        }
        ToastType::Warning => {
            if rich_colors {
                rich_warning_style(is_dark)
            } else {
                default_toast_style(theme, is_dark)
            }
        }
        ToastType::Error => {
            if rich_colors {
                rich_error_style(is_dark)
            } else {
                default_toast_style(theme, is_dark)
            }
        }
        ToastType::Loading => default_toast_style(theme, is_dark),
    }
}

/// Style for inverted toasts (dark in light mode, light in dark mode).
pub(crate) fn inverted_toast_style(is_dark: bool) -> ToastStyle {
    if is_dark {
        ToastStyle {
            background: Color::WHITE,
            text: Color::from_rgb(0.09, 0.09, 0.09),
            border_color: Color::from_rgba(0.87, 0.87, 0.87, 1.0),
            border_width: 1.0,
            border_radius: TOAST_RADIUS,
            shadow: default_shadow(),
        }
    } else {
        ToastStyle {
            background: Color::BLACK,
            text: Color::from_rgba(0.98, 0.98, 0.98, 1.0),
            border_color: Color::from_rgba(0.2, 0.2, 0.2, 1.0),
            border_width: 1.0,
            border_radius: TOAST_RADIUS,
            shadow: default_shadow(),
        }
    }
}

/// Default toast style (normal-bg / normal-text / normal-border).
fn default_toast_style(theme: &Theme, is_dark: bool) -> ToastStyle {
    let (bg, text, border) = if is_dark {
        (
            Color::BLACK,
            Color::from_rgba(0.98, 0.98, 0.98, 1.0), // gray1
            Color::from_rgba(0.2, 0.2, 0.2, 1.0),    // gray4
        )
    } else {
        (
            Color::WHITE,
            Color::from_rgba(0.09, 0.09, 0.09, 1.0), // gray12
            Color::from_rgba(0.93, 0.93, 0.93, 1.0), // gray4
        )
    };

    ToastStyle {
        background: bg,
        text,
        border_color: border,
        border_width: 1.0,
        border_radius: TOAST_RADIUS,
        shadow: default_shadow(),
    }
}

/// Rich success style.
fn rich_success_style(is_dark: bool) -> ToastStyle {
    let (bg, border, text) = if is_dark {
        (
            Color::from_rgba(0.06, 0.45, 0.19, 1.0), // hsl(150, 100%, 6%)
            Color::from_rgba(0.12, 0.52, 0.27, 1.0), // hsl(147, 100%, 12%)
            Color::from_rgba(0.65, 0.86, 0.55, 1.0), // hsl(150, 86%, 65%)
        )
    } else {
        (
            Color::from_rgba(0.96, 0.98, 0.96, 1.0), // hsl(143, 85%, 96%)
            Color::from_rgba(0.87, 0.95, 0.88, 1.0), // hsl(145, 92%, 87%)
            Color::from_rgba(0.13, 0.55, 0.13, 1.0), // hsl(140, 100%, 27%)
        )
    };

    ToastStyle {
        background: bg,
        text,
        border_color: border,
        border_width: 1.0,
        border_radius: TOAST_RADIUS,
        shadow: default_shadow(),
    }
}

/// Rich info style.
fn rich_info_style(is_dark: bool) -> ToastStyle {
    let (bg, border, text) = if is_dark {
        (
            Color::from_rgba(0.06, 0.22, 0.47, 1.0), // hsl(215, 100%, 6%)
            Color::from_rgba(0.17, 0.31, 0.52, 1.0), // hsl(223, 43%, 17%)
            Color::from_rgba(0.65, 0.77, 0.85, 1.0), // hsl(216, 87%, 65%)
        )
    } else {
        (
            Color::from_rgba(0.97, 0.98, 1.0, 1.0),  // hsl(208, 100%, 97%)
            Color::from_rgba(0.93, 0.95, 0.98, 1.0), // hsl(221, 91%, 93%)
            Color::from_rgba(0.18, 0.45, 0.82, 1.0), // hsl(210, 92%, 45%)
        )
    };

    ToastStyle {
        background: bg,
        text,
        border_color: border,
        border_width: 1.0,
        border_radius: TOAST_RADIUS,
        shadow: default_shadow(),
    }
}

/// Rich warning style.
fn rich_warning_style(is_dark: bool) -> ToastStyle {
    let (bg, border, text) = if is_dark {
        (
            Color::from_rgba(0.27, 0.27, 0.06, 1.0), // hsl(64, 100%, 6%)
            Color::from_rgba(0.33, 0.33, 0.09, 1.0), // hsl(60, 100%, 9%)
            Color::from_rgba(0.85, 0.77, 0.40, 1.0), // hsl(46, 87%, 65%)
        )
    } else {
        (
            Color::from_rgba(1.0, 0.99, 0.97, 1.0),  // hsl(49, 100%, 97%)
            Color::from_rgba(1.0, 0.95, 0.84, 1.0),  // hsl(49, 91%, 84%)
            Color::from_rgba(0.55, 0.30, 0.18, 1.0), // hsl(31, 92%, 45%)
        )
    };

    ToastStyle {
        background: bg,
        text,
        border_color: border,
        border_width: 1.0,
        border_radius: TOAST_RADIUS,
        shadow: default_shadow(),
    }
}

/// Rich error style.
fn rich_error_style(is_dark: bool) -> ToastStyle {
    let (bg, border, text) = if is_dark {
        (
            Color::from_rgba(0.35, 0.08, 0.10, 1.0), // hsl(358, 76%, 10%)
            Color::from_rgba(0.45, 0.13, 0.16, 1.0), // hsl(357, 89%, 16%)
            Color::from_rgba(1.0, 0.44, 0.48, 1.0),  // hsl(358, 100%, 81%)
        )
    } else {
        (
            Color::from_rgba(1.0, 0.97, 0.97, 1.0), // hsl(359, 100%, 97%)
            Color::from_rgba(1.0, 0.94, 0.94, 1.0), // hsl(359, 100%, 94%)
            Color::from_rgba(0.88, 0.0, 0.0, 1.0),  // hsl(360, 100%, 45%)
        )
    };

    ToastStyle {
        background: bg,
        text,
        border_color: border,
        border_width: 1.0,
        border_radius: TOAST_RADIUS,
        shadow: default_shadow(),
    }
}

/// Default toast shadow.
fn default_shadow() -> crate::iced_compat::Shadow {
    crate::iced_compat::Shadow {
        color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
        offset: crate::iced_compat::Vector::new(0.0, 4.0),
        blur_radius: 12.0,
    }
}

/// Close button style for a toast.
pub(crate) fn close_button_style(is_dark: bool) -> ToastStyle {
    ToastStyle {
        background: if is_dark {
            Color::from_rgba(0.15, 0.15, 0.15, 1.0) // gray2 dark
        } else {
            Color::from_rgba(0.97, 0.97, 0.97, 1.0) // gray2 light
        },
        text: if is_dark {
            Color::from_rgba(0.09, 0.09, 0.09, 1.0) // gray12 dark
        } else {
            Color::from_rgba(0.09, 0.09, 0.09, 1.0) // gray12 light
        },
        border_color: if is_dark {
            Color::from_rgba(0.25, 0.25, 0.25, 1.0) // gray5 dark
        } else {
            Color::from_rgba(0.93, 0.93, 0.93, 1.0) // gray4 light
        },
        border_width: 1.0,
        border_radius: 9999.0, // fully rounded
        shadow: crate::iced_compat::Shadow::default(),
    }
}

/// Action button style.
pub(crate) fn action_button_style(is_dark: bool) -> ToastStyle {
    let (bg, text) = if is_dark {
        (Color::BLACK, Color::WHITE) // normal-bg, normal-text inverted
    } else {
        (Color::from_rgb(0.09, 0.09, 0.09), Color::WHITE)
    };

    ToastStyle {
        background: bg,
        text,
        border_color: Color::TRANSPARENT,
        border_width: 0.0,
        border_radius: 4.0,
        shadow: crate::iced_compat::Shadow::default(),
    }
}

/// Cancel button style.
pub(crate) fn cancel_button_style(is_dark: bool) -> ToastStyle {
    let (bg, text) = if is_dark {
        (
            Color::from_rgba(1.0, 1.0, 1.0, 0.3), // rgba white
            Color::from_rgba(0.98, 0.98, 0.98, 1.0),
        )
    } else {
        (
            Color::from_rgba(0.0, 0.0, 0.0, 0.08), // rgba black
            Color::from_rgba(0.09, 0.09, 0.09, 1.0),
        )
    };

    ToastStyle {
        background: bg,
        text,
        border_color: Color::TRANSPARENT,
        border_width: 0.0,
        border_radius: 4.0,
        shadow: crate::iced_compat::Shadow::default(),
    }
}
