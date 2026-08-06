//! Badge component - status labels and tags.
//!
//! # Example
//! ```ignore
//! badge(ui, &theme, BadgeProps::new("New").variant(BadgeVariant::Solid));
//! ```

use crate::theme::Theme;
use egui::{Color32, CursorIcon, Response, RichText, Sense, Ui, Vec2};
use lucide_icons::Icon;

// =============================================================================
// BadgeSize / BadgeVariant
// =============================================================================

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeSize {
    #[default]
    Size1,
    Size2,
    Size3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Destructive,
}

// =============================================================================
// BadgeProps
// =============================================================================

#[derive(Clone, Debug)]
pub struct BadgeProps<'a> {
    pub label: &'a str,
    pub icon: Option<Icon>,
    pub size: BadgeSize,
    pub variant: BadgeVariant,
    pub color: Option<Color32>,
    pub high_contrast: bool,
    /// When set, badge renders as a clickable link. Clicking opens the URL.
    pub href: Option<&'a str>,
}

impl<'a> BadgeProps<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            icon: None,
            size: BadgeSize::Size1,
            variant: BadgeVariant::Default,
            color: None,
            high_contrast: false,
            href: None,
        }
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn size(mut self, size: BadgeSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    /// When set, badge renders as a clickable link element.
    pub fn href(mut self, href: &'a str) -> Self {
        self.href = Some(href);
        self
    }
}

// =============================================================================
// Main function
// =============================================================================

/// Render a badge. Returns a [`Response`] that can be used to detect clicks.
///
/// When `href` is set, the badge renders as a clickable link:
/// - pointer cursor on hover
/// - slightly brighter background on hover
/// - clicking opens the URL via [`ui.ctx().open_url`]
pub fn badge(ui: &mut Ui, theme: &Theme, props: BadgeProps<'_>) -> Response {
    let is_link = props.href.is_some();

    let (bg_color, text_color, border_color) = match props.variant {
        BadgeVariant::Default => {
            let color = props.color.unwrap_or(theme.palette.primary);
            (color, theme.palette.primary_foreground, color)
        }
        BadgeVariant::Secondary => {
            let color = props.color.unwrap_or(theme.palette.secondary);
            (color, theme.palette.secondary_foreground, color)
        }
        BadgeVariant::Destructive => {
            let color = props.color.unwrap_or(theme.palette.destructive);
            (color, theme.palette.destructive_foreground, color)
        }
        BadgeVariant::Outline => (
            Color32::TRANSPARENT,
            theme.palette.foreground,
            theme.palette.border,
        ),
    };

    let (font_size, padding) = match props.size {
        BadgeSize::Size1 => (11.0, Vec2::new(6.0, 2.0)),
        BadgeSize::Size2 => (12.0, Vec2::new(8.0, 3.0)),
        BadgeSize::Size3 => (13.0, Vec2::new(10.0, 4.0)),
    };

    let rounding = theme.radius.r6; // Use r6 as pill shape

    let sense = if is_link {
        Sense::click()
    } else {
        Sense::hover()
    };

    let inner = egui::Frame::NONE
        .fill(bg_color)
        .stroke(egui::Stroke::new(1.0_f32, border_color))
        .corner_radius(rounding)
        .inner_margin(padding)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                if let Some(icon) = props.icon {
                    ui.label(
                        RichText::new(icon.unicode().to_string())
                            .size(font_size)
                            .color(text_color),
                    );
                }
                ui.add(
                    egui::Label::new(
                        RichText::new(props.label)
                            .size(font_size)
                            .color(text_color)
                            .strong(),
                    )
                    .sense(sense),
                );
                if is_link {
                    ui.label(
                        RichText::new(Icon::ExternalLink.unicode().to_string())
                            .size(font_size - 2.0)
                            .color(text_color.gamma_multiply(0.8)),
                    );
                }
            })
        });

    let response = inner.response;

    if is_link {
        if response.hovered() {
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }
        if response.clicked()
            && let Some(url) = props.href
        {
            ui.ctx().open_url(egui::OpenUrl::new_tab(url));
        }
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn badge_size_default() {
        assert_eq!(BadgeSize::default(), BadgeSize::Size1);
    }

    #[test]
    fn badge_variant_default() {
        assert_eq!(BadgeVariant::default(), BadgeVariant::Default);
    }

    #[test]
    fn badge_props_builder() {
        let props = BadgeProps::new("Test")
            .size(BadgeSize::Size2)
            .variant(BadgeVariant::Default)
            .high_contrast(true);

        assert_eq!(props.size, BadgeSize::Size2);
        assert_eq!(props.variant, BadgeVariant::Default);
        assert!(props.high_contrast);
    }
}
