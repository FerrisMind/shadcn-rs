//! Theme-inherited row choices for [`super::TreeView`].
//!
//! This module intentionally contains no tree-specific recipe. Rows delegate
//! their surface treatment to the existing button component and only select
//! semantic colors from the supplied [`Theme`](crate::Theme).

use crate::components::button::ButtonVariant;
use crate::iced_compat::Color;
use crate::theme::Theme;

pub(super) fn row_variant(selected: bool) -> ButtonVariant {
    if selected {
        ButtonVariant::Secondary
    } else {
        ButtonVariant::Ghost
    }
}

pub(super) fn text_color(theme: &Theme, selected: bool, disabled: bool) -> Color {
    if disabled {
        theme.palette.muted_foreground
    } else if selected {
        theme.palette.secondary_foreground
    } else {
        theme.palette.foreground
    }
}

pub(super) fn icon_color(theme: &Theme, selected: bool, disabled: bool) -> Color {
    if disabled {
        theme.palette.muted_foreground
    } else if selected {
        theme.palette.secondary_foreground
    } else {
        theme.palette.muted_foreground
    }
}
