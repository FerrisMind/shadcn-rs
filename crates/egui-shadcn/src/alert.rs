//! Alert component - displays an important message with icon.
//!
//! # Example
//! ```ignore
//! alert(ui, &theme, AlertProps::new(AlertVariant::Info, "Important message"));
//! ```

use crate::theme::Theme;
use egui::{Color32, RichText, Ui, Vec2};

// =============================================================================
// AlertVariant
// =============================================================================

/// Visual style variants for the alert.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
    Warning,
    Success,
    Info,
}

// =============================================================================
// AlertProps
// =============================================================================

/// Properties for the Alert component.
#[derive(Clone, Debug)]
pub struct AlertProps<'a> {
    pub variant: AlertVariant,
    pub title: Option<&'a str>,
    pub description: &'a str,
}

impl<'a> AlertProps<'a> {
    pub fn new(description: &'a str) -> Self {
        Self {
            variant: AlertVariant::Default,
            title: None,
            description,
        }
    }

    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }
}

// =============================================================================
// Main function
// =============================================================================

/// Render an alert message.
pub fn alert(ui: &mut Ui, theme: &Theme, props: AlertProps<'_>) {
    let (bg_color, border_color, icon_color, icon) = match props.variant {
        AlertVariant::Default => (
            theme.palette.muted.gamma_multiply(0.3),
            theme.palette.border,
            theme.palette.foreground,
            "ℹ",
        ),
        AlertVariant::Destructive => (
            Color32::from_rgb(254, 202, 202).gamma_multiply(0.3),
            Color32::from_rgb(239, 68, 68),
            Color32::from_rgb(239, 68, 68),
            "⚠",
        ),
        AlertVariant::Warning => (
            Color32::from_rgb(254, 243, 199).gamma_multiply(0.5),
            Color32::from_rgb(245, 158, 11),
            Color32::from_rgb(245, 158, 11),
            "⚠",
        ),
        AlertVariant::Success => (
            Color32::from_rgb(187, 247, 208).gamma_multiply(0.3),
            Color32::from_rgb(34, 197, 94),
            Color32::from_rgb(34, 197, 94),
            "✓",
        ),
        AlertVariant::Info => (
            Color32::from_rgb(191, 219, 254).gamma_multiply(0.3),
            Color32::from_rgb(59, 130, 246),
            Color32::from_rgb(59, 130, 246),
            "ℹ",
        ),
    };

    let rounding = theme.radius.r2;
    let padding = Vec2::splat(16.0);

    egui::Frame::NONE
        .fill(bg_color)
        .stroke(egui::Stroke::new(1.0, border_color))
        .corner_radius(rounding)
        .inner_margin(padding)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 12.0;

                // Icon
                ui.label(RichText::new(icon).size(18.0).color(icon_color));

                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 4.0;

                    // Title (optional)
                    if let Some(title) = props.title {
                        ui.label(
                            RichText::new(title)
                                .size(14.0)
                                .strong()
                                .color(theme.palette.foreground),
                        );
                    }

                    // Description
                    ui.label(
                        RichText::new(props.description)
                            .size(14.0)
                            .color(theme.palette.muted_foreground),
                    );
                });
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_variant_default() {
        assert_eq!(AlertVariant::default(), AlertVariant::Default);
    }

    #[test]
    fn alert_props_builder() {
        let props = AlertProps::new("Test message")
            .variant(AlertVariant::Success)
            .title("Success!");

        assert_eq!(props.variant, AlertVariant::Success);
        assert_eq!(props.title, Some("Success!"));
        assert_eq!(props.description, "Test message");
    }
}
