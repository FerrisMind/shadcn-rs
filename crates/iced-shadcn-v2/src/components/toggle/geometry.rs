//! Toggle dimensions and padding resolution.

use crate::iced_compat::Length;
use shadcn_common::{ControlSize, ToggleSizeRecipe};

use super::types::ToggleSize;
use crate::theme::Theme;

impl ToggleSize {
    pub(super) fn control_size(self) -> ControlSize {
        match self {
            Self::Sm => ControlSize::Sm,
            Self::Default => ControlSize::Md,
            Self::Lg => ControlSize::Lg,
        }
    }

    /// Size recipe from [`shadcn_common`] for the active style pack.
    pub(super) fn recipe(self, theme: &Theme) -> ToggleSizeRecipe {
        theme.style.toggle_size(self.control_size())
    }

    /// Control height in px from the style pack size ladder.
    pub(super) fn control_height(self, theme: &Theme) -> f32 {
        self.recipe(theme).height_px
    }

    pub(super) fn label_text_size(self, theme: &Theme) -> f32 {
        self.recipe(theme).text_size_px
    }

    /// Default horizontal padding (`px-*`); icon-only toggles use none.
    ///
    /// A side that carries an icon slot switches to the pack's tighter
    /// `has-data-[icon=inline-start|end]` padding, like the web component.
    pub(super) fn default_padding(
        self,
        theme: &Theme,
        icon_only: bool,
        icon_start: bool,
        icon_end: bool,
    ) -> crate::iced_compat::Padding {
        if icon_only {
            return crate::iced_compat::Padding::ZERO;
        }

        let recipe = self.recipe(theme);
        crate::iced_compat::Padding {
            top: 0.0,
            right: if icon_end {
                recipe.pad_x_icon_px
            } else {
                recipe.pad_x_px
            },
            bottom: 0.0,
            left: if icon_start {
                recipe.pad_x_icon_px
            } else {
                recipe.pad_x_px
            },
        }
    }
}

/// Icon-only toggles collapse to the square `min-w-*` footprint of the pack.
pub(super) fn resolve_toggle_width(
    width: Length,
    height: Length,
    full_width: bool,
    icon_only: bool,
    default_height: f32,
) -> Length {
    if full_width {
        Length::Fill
    } else if icon_only {
        match height {
            Length::Fixed(height) => Length::Fixed(height),
            _ => Length::Fixed(default_height),
        }
    } else {
        width
    }
}
