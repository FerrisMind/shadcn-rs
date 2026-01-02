//! Context Menu component - displays a menu on right-click.
//!
//! Provides a context menu that appears on secondary (right) click.
//! Built on top of egui's response.context_menu() with shadcn styling.

use crate::theme::Theme;
use egui::{Response, Ui, Vec2};

pub use crate::menu_primitives::{
    MenuCheckboxItemProps as ContextMenuCheckboxItemProps, MenuItemProps as ContextMenuItemProps,
    MenuItemVariant as ContextMenuItemVariant, MenuLabelProps as ContextMenuLabelProps,
    MenuRadioGroupProps as ContextMenuRadioGroupProps,
    MenuRadioItemProps as ContextMenuRadioItemProps, MenuSubProps as ContextMenuSubProps,
    MenuTokens as ContextMenuTokens, menu_checkbox_item as context_menu_checkbox_item,
    menu_item as context_menu_item, menu_label as context_menu_label,
    menu_radio_group as context_menu_radio_group, menu_radio_item as context_menu_radio_item,
    menu_separator as context_menu_separator, menu_shortcut as context_menu_shortcut,
    menu_sub as context_menu_sub, menu_tokens as context_menu_tokens,
};

// ============================================================================
// Main Context Menu wrapper
// ============================================================================

/// Shows a context menu when the response is right-clicked.
/// This is a wrapper around egui's context_menu with shadcn styling.
///
/// # Example
/// ```ignore
/// let response = ui.add(egui::Label::new("Right-click me").sense(egui::Sense::click()));
/// context_menu(&response, theme, |ui| {
///     if context_menu_item(ui, theme, ContextMenuItemProps::new("Cut").with_shortcut("⌘X")).clicked() {
///         // handle cut
///         ui.close_menu();
///     }
///     context_menu_separator(ui, theme);
///     if context_menu_item(ui, theme, ContextMenuItemProps::new("Delete").with_variant(ContextMenuItemVariant::Destructive)).clicked() {
///         // handle delete
///         ui.close_menu();
///     }
/// });
/// ```
pub fn context_menu(response: &Response, theme: &Theme, add_contents: impl FnOnce(&mut Ui)) {
    let tokens = context_menu_tokens(theme);

    response.context_menu(|ui| {
        ui.visuals_mut().override_text_color = Some(tokens.text);
        ui.spacing_mut().item_spacing = Vec2::new(0.0, 2.0);
        ui.set_min_width(tokens.min_width);

        add_contents(ui);
    });
}
