//! Context Menu component - displays a menu on right-click
//!
//! Provides a context menu that appears on secondary (right) click.
//! Built on top of egui's response.context_menu() with shadcn styling.

use crate::separator::{separator, SeparatorOrientation, SeparatorProps};
use crate::theme::Theme;
use egui::{
    Color32, CornerRadius, CursorIcon, Frame, Id, Margin, Order, Response, RichText, Sense, Stroke,
    Ui, Vec2,
};

// ============================================================================
// Types
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ContextMenuItemVariant {
    #[default]
    Default,
    Destructive,
}

// ============================================================================
// Props
// ============================================================================

#[derive(Clone, Debug)]
pub struct ContextMenuItemProps<'a> {
    pub label: &'a str,
    pub shortcut: Option<&'a str>,
    pub variant: ContextMenuItemVariant,
    pub disabled: bool,
    pub inset: bool,
    pub icon: Option<&'a str>,
}

impl<'a> ContextMenuItemProps<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            shortcut: None,
            variant: ContextMenuItemVariant::Default,
            disabled: false,
            inset: false,
            icon: None,
        }
    }

    pub fn with_shortcut(mut self, shortcut: &'a str) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    pub fn with_variant(mut self, variant: ContextMenuItemVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    pub fn with_icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }
}

#[derive(Clone, Debug)]
pub struct ContextMenuCheckboxItemProps<'a> {
    pub label: &'a str,
    pub checked: bool,
    pub disabled: bool,
}

impl<'a> ContextMenuCheckboxItemProps<'a> {
    pub fn new(label: &'a str, checked: bool) -> Self {
        Self {
            label,
            checked,
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug)]
pub struct ContextMenuRadioItemProps<'a> {
    pub label: &'a str,
    pub value: &'a str,
    pub selected_value: &'a str,
    pub disabled: bool,
}

impl<'a> ContextMenuRadioItemProps<'a> {
    pub fn new(label: &'a str, value: &'a str, selected_value: &'a str) -> Self {
        Self {
            label,
            value,
            selected_value,
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug)]
pub struct ContextMenuLabelProps<'a> {
    pub label: &'a str,
    pub inset: bool,
}

impl<'a> ContextMenuLabelProps<'a> {
    pub fn new(label: &'a str) -> Self {
        Self { label, inset: false }
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }
}

// ============================================================================
// Tokens
// ============================================================================

#[derive(Clone, Copy, Debug)]
pub struct ContextMenuTokens {
    pub bg: Color32,
    pub border: Color32,
    pub text: Color32,
    pub text_muted: Color32,
    pub text_destructive: Color32,
    pub hover_bg: Color32,
    pub hover_bg_destructive: Color32,
    pub disabled_opacity: f32,
    pub rounding: CornerRadius,
    pub item_rounding: CornerRadius,
    pub padding: Margin,
    pub item_padding: Margin,
    pub min_width: f32,
}

pub fn context_menu_tokens(theme: &Theme) -> ContextMenuTokens {
    let palette = &theme.palette;
    ContextMenuTokens {
        bg: palette.popover,
        border: palette.border,
        text: palette.popover_foreground,
        text_muted: palette.muted_foreground,
        text_destructive: palette.destructive,
        hover_bg: palette.accent,
        hover_bg_destructive: palette.destructive.gamma_multiply(0.1),
        disabled_opacity: 0.5,
        rounding: CornerRadius::same(theme.radius.r2.round() as u8),
        item_rounding: CornerRadius::same(2),
        padding: Margin::same(4),
        item_padding: Margin::symmetric(8, 6),
        min_width: 128.0,
    }
}

// ============================================================================
// Menu Item
// ============================================================================

pub fn context_menu_item(ui: &mut Ui, theme: &Theme, props: ContextMenuItemProps<'_>) -> Response {
    let tokens = context_menu_tokens(theme);
    let inset_padding = if props.inset { 24.0 } else { 0.0 };

    let (text_color, hover_bg) = match props.variant {
        ContextMenuItemVariant::Default => (tokens.text, tokens.hover_bg),
        ContextMenuItemVariant::Destructive => (tokens.text_destructive, tokens.hover_bg_destructive),
    };

    let available_width = ui.available_width();
    let desired_size = Vec2::new(available_width, 28.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if ui.is_rect_visible(rect) {
        let visuals = if props.disabled {
            ui.visuals().widgets.inactive
        } else if response.hovered() {
            ui.visuals().widgets.hovered
        } else {
            ui.visuals().widgets.inactive
        };

        // Background on hover
        if response.hovered() && !props.disabled {
            ui.painter().rect_filled(rect, tokens.item_rounding, hover_bg);
        }

        let opacity = if props.disabled { tokens.disabled_opacity } else { 1.0 };

        // Icon area (if inset)
        let text_start_x = rect.left() + tokens.item_padding.left as f32 + inset_padding;

        // Icon
        if let Some(icon) = props.icon {
            let icon_rect = egui::Rect::from_min_size(
                egui::pos2(rect.left() + 8.0, rect.center().y - 8.0),
                Vec2::splat(16.0),
            );
            ui.painter().text(
                icon_rect.center(),
                egui::Align2::CENTER_CENTER,
                icon,
                egui::FontId::proportional(14.0),
                tokens.text_muted.gamma_multiply(opacity),
            );
        }

        // Label
        let label_pos = egui::pos2(text_start_x, rect.center().y);
        ui.painter().text(
            label_pos,
            egui::Align2::LEFT_CENTER,
            props.label,
            egui::FontId::proportional(14.0),
            text_color.gamma_multiply(opacity),
        );

        // Shortcut
        if let Some(shortcut) = props.shortcut {
            let shortcut_pos = egui::pos2(rect.right() - tokens.item_padding.right as f32, rect.center().y);
            ui.painter().text(
                shortcut_pos,
                egui::Align2::RIGHT_CENTER,
                shortcut,
                egui::FontId::proportional(12.0),
                tokens.text_muted.gamma_multiply(opacity),
            );
        }
    }

    if !props.disabled {
        response.on_hover_cursor(CursorIcon::PointingHand)
    } else {
        response
    }
}

// ============================================================================
// Checkbox Item
// ============================================================================

pub fn context_menu_checkbox_item(
    ui: &mut Ui,
    theme: &Theme,
    props: ContextMenuCheckboxItemProps<'_>,
) -> Response {
    let tokens = context_menu_tokens(theme);

    let available_width = ui.available_width();
    let desired_size = Vec2::new(available_width, 28.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if ui.is_rect_visible(rect) {
        if response.hovered() && !props.disabled {
            ui.painter().rect_filled(rect, tokens.item_rounding, tokens.hover_bg);
        }

        let opacity = if props.disabled { tokens.disabled_opacity } else { 1.0 };

        // Checkbox indicator area
        let check_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + 8.0, rect.center().y - 7.0),
            Vec2::splat(14.0),
        );

        if props.checked {
            // Draw checkmark
            ui.painter().text(
                check_rect.center(),
                egui::Align2::CENTER_CENTER,
                "✓",
                egui::FontId::proportional(12.0),
                tokens.text.gamma_multiply(opacity),
            );
        }

        // Label
        let label_pos = egui::pos2(rect.left() + 32.0, rect.center().y);
        ui.painter().text(
            label_pos,
            egui::Align2::LEFT_CENTER,
            props.label,
            egui::FontId::proportional(14.0),
            tokens.text.gamma_multiply(opacity),
        );
    }

    if !props.disabled {
        response.on_hover_cursor(CursorIcon::PointingHand)
    } else {
        response
    }
}

// ============================================================================
// Radio Item
// ============================================================================

pub fn context_menu_radio_item(
    ui: &mut Ui,
    theme: &Theme,
    props: ContextMenuRadioItemProps<'_>,
) -> Response {
    let tokens = context_menu_tokens(theme);
    let is_selected = props.value == props.selected_value;

    let available_width = ui.available_width();
    let desired_size = Vec2::new(available_width, 28.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    if ui.is_rect_visible(rect) {
        if response.hovered() && !props.disabled {
            ui.painter().rect_filled(rect, tokens.item_rounding, tokens.hover_bg);
        }

        let opacity = if props.disabled { tokens.disabled_opacity } else { 1.0 };

        // Radio indicator area
        let radio_center = egui::pos2(rect.left() + 15.0, rect.center().y);

        if is_selected {
            // Draw filled circle
            ui.painter().circle_filled(radio_center, 4.0, tokens.text.gamma_multiply(opacity));
        }

        // Label
        let label_pos = egui::pos2(rect.left() + 32.0, rect.center().y);
        ui.painter().text(
            label_pos,
            egui::Align2::LEFT_CENTER,
            props.label,
            egui::FontId::proportional(14.0),
            tokens.text.gamma_multiply(opacity),
        );
    }

    if !props.disabled {
        response.on_hover_cursor(CursorIcon::PointingHand)
    } else {
        response
    }
}

// ============================================================================
// Label
// ============================================================================

pub fn context_menu_label(ui: &mut Ui, theme: &Theme, props: ContextMenuLabelProps<'_>) -> Response {
    let tokens = context_menu_tokens(theme);
    let inset_padding = if props.inset { 24.0 } else { 0.0 };

    let available_width = ui.available_width();
    let desired_size = Vec2::new(available_width, 24.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

    if ui.is_rect_visible(rect) {
        let label_pos = egui::pos2(
            rect.left() + tokens.item_padding.left as f32 + inset_padding,
            rect.center().y,
        );
        ui.painter().text(
            label_pos,
            egui::Align2::LEFT_CENTER,
            props.label,
            egui::FontId::proportional(14.0),
            tokens.text,
        );
    }

    response
}

// ============================================================================
// Separator
// ============================================================================

pub fn context_menu_separator(ui: &mut Ui, theme: &Theme) -> Response {
    ui.add_space(4.0);
    let response = separator(
        ui,
        theme,
        SeparatorProps::default()
            .with_orientation(SeparatorOrientation::Horizontal)
            .with_gap(0.0),
    );
    ui.add_space(4.0);
    response
}

// ============================================================================
// Shortcut (helper for displaying keyboard shortcuts)
// ============================================================================

pub fn context_menu_shortcut(ui: &mut Ui, theme: &Theme, text: &str) -> Response {
    let tokens = context_menu_tokens(theme);
    let galley = ui.painter().layout_no_wrap(
        text.to_string(),
        egui::FontId::proportional(12.0),
        tokens.text_muted,
    );
    let desired_size = galley.size();
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());
    
    if ui.is_rect_visible(rect) {
        ui.painter().galley(rect.min, galley, tokens.text_muted);
    }
    
    response
}

// ============================================================================
// Sub Menu (trigger + content)
// ============================================================================

#[derive(Clone, Debug)]
pub struct ContextMenuSubProps<'a> {
    pub label: &'a str,
    pub inset: bool,
    pub disabled: bool,
}

impl<'a> ContextMenuSubProps<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            inset: false,
            disabled: false,
        }
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub fn context_menu_sub<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: ContextMenuSubProps<'_>,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let tokens = context_menu_tokens(theme);
    let inset_padding = if props.inset { 24.0 } else { 0.0 };

    let available_width = ui.available_width();
    let desired_size = Vec2::new(available_width, 28.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

    let is_open = response.hovered() && !props.disabled;

    if ui.is_rect_visible(rect) {
        if is_open {
            ui.painter().rect_filled(rect, tokens.item_rounding, tokens.hover_bg);
        }

        let opacity = if props.disabled { tokens.disabled_opacity } else { 1.0 };

        // Label
        let label_pos = egui::pos2(
            rect.left() + tokens.item_padding.left as f32 + inset_padding,
            rect.center().y,
        );
        ui.painter().text(
            label_pos,
            egui::Align2::LEFT_CENTER,
            props.label,
            egui::FontId::proportional(14.0),
            tokens.text.gamma_multiply(opacity),
        );

        // Chevron right indicator
        let chevron_pos = egui::pos2(rect.right() - 16.0, rect.center().y);
        ui.painter().text(
            chevron_pos,
            egui::Align2::CENTER_CENTER,
            "›",
            egui::FontId::proportional(16.0),
            tokens.text_muted.gamma_multiply(opacity),
        );
    }

    // Show submenu content when hovered
    if is_open {
        let submenu_id = ui.id().with("submenu").with(props.label);
        let submenu_pos = egui::pos2(rect.right() + 2.0, rect.top());

        let area_response = egui::Area::new(submenu_id)
            .order(Order::Foreground)
            .fixed_pos(submenu_pos)
            .show(ui.ctx(), |ui| {
                Frame::popup(ui.style())
                    .fill(tokens.bg)
                    .stroke(Stroke::new(1.0, tokens.border))
                    .corner_radius(tokens.rounding)
                    .inner_margin(tokens.padding)
                    .show(ui, |ui| {
                        ui.set_min_width(160.0);
                        ui.visuals_mut().override_text_color = Some(tokens.text);
                        add_contents(ui)
                    })
            });

        Some(area_response.inner.inner)
    } else {
        None
    }
}

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
pub fn context_menu<R>(
    response: &Response,
    theme: &Theme,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let tokens = context_menu_tokens(theme);

    response.context_menu(|ui| {
        // Apply custom styling
        ui.visuals_mut().override_text_color = Some(tokens.text);
        ui.spacing_mut().item_spacing = Vec2::new(0.0, 2.0);
        ui.set_min_width(tokens.min_width);
        
        add_contents(ui)
    }).map(|inner| inner.inner)
}

// ============================================================================
// Radio Group helper
// ============================================================================

#[derive(Clone, Debug)]
pub struct ContextMenuRadioGroupProps<'a> {
    pub value: &'a str,
}

impl<'a> ContextMenuRadioGroupProps<'a> {
    pub fn new(value: &'a str) -> Self {
        Self { value }
    }
}

/// Helper to create a radio group within a context menu.
/// Returns the selected value if changed.
pub fn context_menu_radio_group<'a, R>(
    ui: &mut Ui,
    theme: &Theme,
    value: &'a str,
    add_contents: impl FnOnce(&mut Ui, &'a str) -> R,
) -> R {
    add_contents(ui, value)
}
