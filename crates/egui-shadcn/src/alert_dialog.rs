//! Alert Dialog component - modal confirmation dialog.
//!
//! # Example
//! ```ignore
//! alert_dialog(ui, &theme, AlertDialogProps::new(&mut open, "Are you sure?", "This cannot be undone."));
//! ```

use crate::dialog::{DialogProps, DialogSize, dialog};
use crate::button::{Button, ButtonVariant};
use crate::theme::Theme;
use egui::{RichText, Ui};

// =============================================================================
// AlertDialogProps
// =============================================================================

/// Properties for the AlertDialog component.
pub struct AlertDialogProps<'a> {
    pub open: &'a mut bool,
    pub title: &'a str,
    pub description: &'a str,
    pub cancel_text: &'a str,
    pub action_text: &'a str,
    pub action_variant: ButtonVariant,
}

impl<'a> AlertDialogProps<'a> {
    pub fn new(open: &'a mut bool, title: &'a str, description: &'a str) -> Self {
        Self {
            open,
            title,
            description,
            cancel_text: "Cancel",
            action_text: "Continue",
            action_variant: ButtonVariant::Default,
        }
    }

    pub fn cancel_text(mut self, text: &'a str) -> Self {
        self.cancel_text = text;
        self
    }

    pub fn action_text(mut self, text: &'a str) -> Self {
        self.action_text = text;
        self
    }

    pub fn destructive(mut self) -> Self {
        self.action_variant = ButtonVariant::Destructive;
        self
    }

    pub fn action_variant(mut self, variant: ButtonVariant) -> Self {
        self.action_variant = variant;
        self
    }
}

/// Result of the alert dialog interaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertDialogResult {
    None,
    Cancelled,
    Confirmed,
}

// =============================================================================
// Main function
// =============================================================================

/// Render an alert dialog for confirmation.
pub fn alert_dialog(ui: &mut Ui, theme: &Theme, props: AlertDialogProps<'_>) -> AlertDialogResult {
    use std::cell::Cell;
    
    if !*props.open {
        return AlertDialogResult::None;
    }

    // Extract props to avoid borrow issues
    let title = props.title;
    let description = props.description;
    let cancel_text = props.cancel_text;
    let action_text = props.action_text;
    let action_variant = props.action_variant;
    
    // Use Cell to allow mutation inside closure
    let result = Cell::new(AlertDialogResult::None);
    let should_close = Cell::new(false);

    dialog(
        ui,
        theme,
        DialogProps::new("alert-dialog".into(), props.open)
            .with_dialog_size(DialogSize::Size1)
            .with_close_on_background(false),
        |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 8.0;

                // Title
                ui.label(
                    RichText::new(title)
                        .size(18.0)
                        .strong()
                        .color(theme.palette.foreground),
                );

                // Description
                ui.label(
                    RichText::new(description)
                        .size(14.0)
                        .color(theme.palette.muted_foreground),
                );

                ui.add_space(16.0);

                // Action buttons
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Action button
                        if Button::new(action_text)
                            .variant(action_variant)
                            .show(ui, theme)
                            .clicked()
                        {
                            result.set(AlertDialogResult::Confirmed);
                            should_close.set(true);
                        }

                        // Cancel button
                        if Button::new(cancel_text)
                            .variant(ButtonVariant::Outline)
                            .show(ui, theme)
                            .clicked()
                        {
                            result.set(AlertDialogResult::Cancelled);
                            should_close.set(true);
                        }
                    });
                });
            });
        },
    );

    if should_close.get() {
        *props.open = false;
    }

    result.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_dialog_props_builder() {
        let mut open = true;
        let props = AlertDialogProps::new(&mut open, "Delete?", "This is permanent")
            .cancel_text("No")
            .action_text("Yes, delete")
            .destructive();

        assert_eq!(props.title, "Delete?");
        assert_eq!(props.cancel_text, "No");
        assert_eq!(props.action_text, "Yes, delete");
        assert_eq!(props.action_variant, ButtonVariant::Destructive);
    }

    #[test]
    fn alert_dialog_result_variants() {
        assert_eq!(AlertDialogResult::None, AlertDialogResult::None);
        assert_eq!(AlertDialogResult::Cancelled, AlertDialogResult::Cancelled);
        assert_eq!(AlertDialogResult::Confirmed, AlertDialogResult::Confirmed);
    }
}
