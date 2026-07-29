//! Theme-aware table measurements and semantic styles.

use crate::fonts::iced_font;
use crate::iced_compat::border::Border;
use crate::iced_compat::widget::container;
use crate::iced_compat::{Color, Font, Padding};
use crate::recipes::iced_font_weight;
use crate::theme::Theme;
use shadcn_common::{FontWeight, StyleId};

use super::SectionKind;

/// Resolved dimensions for one table style pack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct TableMetrics {
    /// The regular table text size in pixels.
    pub(super) text_size: f32,
    /// The regular table line height in pixels.
    pub(super) line_height: f32,
    /// The header row height in pixels.
    pub(super) header_height: f32,
    /// Horizontal header inset in pixels.
    pub(super) header_padding_x: f32,
    /// Body cell padding in pixels.
    pub(super) cell_padding: f32,
    /// Caption top margin in pixels.
    pub(super) caption_margin_top: f32,
    /// Header text size in pixels.
    pub(super) header_text_size: f32,
    /// Header line height in pixels.
    pub(super) header_line_height: f32,
    /// Whether header text is uppercase in this style pack.
    pub(super) header_uppercase: bool,
    /// Whether the header uses the muted foreground token.
    pub(super) header_is_muted: bool,
}

/// Resolves the table geometry transcribed from the shadcn style packs.
pub(super) fn metrics(theme: &Theme) -> TableMetrics {
    match theme.style_id() {
        StyleId::Lyra | StyleId::Mira => TableMetrics {
            text_size: 12.0,
            line_height: 16.0,
            header_height: 40.0,
            header_padding_x: 8.0,
            cell_padding: 8.0,
            caption_margin_top: 16.0,
            header_text_size: 12.0,
            header_line_height: 16.0,
            header_uppercase: false,
            header_is_muted: false,
        },
        StyleId::Maia | StyleId::Luma => TableMetrics {
            text_size: 14.0,
            line_height: 20.0,
            header_height: 48.0,
            header_padding_x: 12.0,
            cell_padding: 12.0,
            caption_margin_top: 16.0,
            header_text_size: 14.0,
            header_line_height: 20.0,
            header_uppercase: false,
            header_is_muted: false,
        },
        StyleId::Sera => TableMetrics {
            text_size: 14.0,
            line_height: 20.0,
            header_height: 48.0,
            header_padding_x: 12.0,
            cell_padding: 12.0,
            caption_margin_top: 16.0,
            header_text_size: 12.0,
            header_line_height: 16.0,
            header_uppercase: true,
            header_is_muted: true,
        },
        StyleId::Vega | StyleId::Nova | StyleId::Rhea => TableMetrics {
            text_size: 14.0,
            line_height: 20.0,
            header_height: 40.0,
            header_padding_x: 8.0,
            cell_padding: 8.0,
            caption_margin_top: 16.0,
            header_text_size: 14.0,
            header_line_height: 20.0,
            header_uppercase: false,
            header_is_muted: false,
        },
    }
}

/// Resolves the sans font used by table text.
pub(super) fn font(theme: &Theme, weight: FontWeight) -> Font {
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = iced_font_weight(weight);
    font
}

/// Returns the default cell padding for a header or body cell.
pub(super) fn cell_padding(theme: &Theme, is_header: bool) -> Padding {
    let metrics = metrics(theme);
    if is_header {
        Padding {
            top: 0.0,
            right: metrics.header_padding_x,
            bottom: 0.0,
            left: metrics.header_padding_x,
        }
    } else {
        Padding::from(metrics.cell_padding)
    }
}

/// Resolves a row's resting background and bottom border.
pub(super) fn row_style(
    theme: &Theme,
    section: SectionKind,
    selected: bool,
    has_bottom_border: bool,
) -> container::Style {
    let background = if selected {
        Some(theme.palette.muted)
    } else if section == SectionKind::Footer {
        Some(with_alpha(theme.palette.muted, 0.5))
    } else {
        None
    };

    container::Style {
        background: background.map(crate::iced_compat::Background::Color),
        text_color: Some(theme.palette.foreground),
        border: Border {
            width: if has_bottom_border { 1.0 } else { 0.0 },
            color: theme.palette.border,
            ..Border::default()
        },
        ..container::Style::default()
    }
}

/// Resolves the hover treatment for a row.
pub(super) fn hover_row_style(
    theme: &Theme,
    section: SectionKind,
    selected: bool,
    has_bottom_border: bool,
) -> container::Style {
    let mut style = row_style(theme, section, selected, has_bottom_border);
    if !selected {
        style.background = Some(crate::iced_compat::Background::Color(with_alpha(
            theme.palette.muted,
            0.5,
        )));
    }
    style
}

/// Adds an alpha multiplier without changing the semantic RGB value.
pub(super) fn with_alpha(mut color: Color, alpha: f32) -> Color {
    color.a *= alpha.clamp(0.0, 1.0);
    color
}

/// Normalizes a non-negative pixel value used by the public builder API.
pub(super) fn normalize_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

/// Normalizes a strictly positive pixel value used for text and row sizes.
pub(super) fn normalize_min_px(value: f32) -> f32 {
    if value.is_finite() {
        value.max(1.0)
    } else {
        1.0
    }
}
