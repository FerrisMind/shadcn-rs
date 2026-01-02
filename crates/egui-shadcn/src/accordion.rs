//! Accordion component - a group of collapsible sections.
//!
//! Based on Radix Accordion API with Single/Multiple modes.
//!
//! # Example
//! ```ignore
//! let mut value: Option<String> = Some("item-1".to_string());
//! accordion(ui, &theme, AccordionProps::new("my-accordion", &mut value), |ui, ctx| {
//!     accordion_item(ui, &theme, ctx, AccordionItemProps::new("item-1"),
//!         |ui, _| { ui.label("Section 1"); },
//!         |ui| { ui.label("Content 1"); },
//!     );
//! });
//! ```

use crate::collapsible::{CollapsibleProps, collapsible};
use crate::theme::Theme;
use egui::{Id, InnerResponse, Response, Ui};
use log::trace;
use std::hash::Hash;

// =============================================================================
// AccordionType
// =============================================================================

/// Type of accordion behavior.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AccordionType {
    /// Only one item can be open at a time.
    #[default]
    Single,
    /// Multiple items can be open simultaneously.
    Multiple,
}

// =============================================================================
// AccordionState
// =============================================================================

/// Internal state for managing open accordion items.
#[derive(Clone, Debug)]
pub enum AccordionState {
    Single(Option<String>),
    Multiple(Vec<String>),
}

impl AccordionState {
    pub fn single(value: Option<String>) -> Self {
        AccordionState::Single(value)
    }

    pub fn multiple(values: Vec<String>) -> Self {
        AccordionState::Multiple(values)
    }

    pub fn is_open(&self, value: &str) -> bool {
        match self {
            AccordionState::Single(current) => current.as_deref() == Some(value),
            AccordionState::Multiple(open_items) => open_items.iter().any(|v| v == value),
        }
    }

    /// Toggle an item. In single mode with collapsible=false, the open item cannot be closed.
    pub fn toggle(&mut self, value: &str, collapsible: bool) {
        match self {
            AccordionState::Single(current) => {
                if current.as_deref() == Some(value) {
                    // Item is already open
                    if collapsible {
                        *current = None;
                    }
                    // If not collapsible, keep it open
                } else {
                    *current = Some(value.to_string());
                }
            }
            AccordionState::Multiple(open_items) => {
                if let Some(pos) = open_items.iter().position(|v| v == value) {
                    open_items.remove(pos);
                } else {
                    open_items.push(value.to_string());
                }
            }
        }
    }
}

// =============================================================================
// AccordionProps
// =============================================================================

/// Properties for the Accordion root component.
#[derive(Debug)]
pub struct AccordionProps<'a, Id: Hash> {
    pub id_source: Id,
    pub accordion_type: AccordionType,
    pub value: &'a mut Option<String>,
    pub collapsible: bool,
    pub disabled: bool,
    pub default_value: Option<String>,
    pub animate: bool,
    pub animation_ms: Option<f32>,
}

impl<'a, Id: Hash> AccordionProps<'a, Id> {
    pub fn new(id_source: Id, value: &'a mut Option<String>) -> Self {
        Self {
            id_source,
            accordion_type: AccordionType::Single,
            value,
            collapsible: false,
            disabled: false,
            default_value: None,
            animate: true,
            animation_ms: None,
        }
    }

    pub fn accordion_type(mut self, accordion_type: AccordionType) -> Self {
        self.accordion_type = accordion_type;
        self
    }

    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    pub fn animation_ms(mut self, ms: f32) -> Self {
        self.animation_ms = Some(ms.max(0.0));
        self
    }
}

// =============================================================================
// AccordionItemProps
// =============================================================================

/// Properties for an individual accordion item.
#[derive(Clone, Debug)]
pub struct AccordionItemProps<'a> {
    pub value: &'a str,
    pub disabled: bool,
}

impl<'a> AccordionItemProps<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            value,
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

// =============================================================================
// AccordionContext
// =============================================================================

/// Context passed to accordion children for state management.
pub struct AccordionContext<'a> {
    id: Id,
    value: &'a mut Option<String>,
    accordion_type: AccordionType,
    collapsible: bool,
    disabled: bool,
    animate: bool,
    animation_ms: Option<f32>,
    #[allow(dead_code)] // Reserved for future styling enhancements
    theme: &'a Theme,
}

impl<'a> AccordionContext<'a> {
    /// Check if a specific item is currently open.
    pub fn is_open(&self, item_value: &str) -> bool {
        self.value.as_deref() == Some(item_value)
    }

    /// Toggle an item's open state.
    pub fn toggle(&mut self, ui: &Ui, item_value: &str) {
        if self.disabled {
            return;
        }

        let currently_open = self.is_open(item_value);

        match self.accordion_type {
            AccordionType::Single => {
                if currently_open {
                    if self.collapsible {
                        *self.value = None;
                    }
                } else {
                    *self.value = Some(item_value.to_string());
                }
            }
            AccordionType::Multiple => {
                // For multiple mode, we'd need Vec<String> instead of Option<String>
                // For now, treat as single with toggle behavior
                if currently_open {
                    *self.value = None;
                } else {
                    *self.value = Some(item_value.to_string());
                }
            }
        }

        ui.ctx().request_repaint();
    }
}

// =============================================================================
// AccordionItemContext
// =============================================================================

/// Context for a specific accordion item.
pub struct AccordionItemContext<'a> {
    pub value: &'a str,
    pub is_open: bool,
    pub disabled: bool,
}

// =============================================================================
// Main accordion function
// =============================================================================

/// Render an accordion container.
pub fn accordion<'a, IdType: Hash, R>(
    ui: &mut Ui,
    theme: &'a Theme,
    props: AccordionProps<'a, IdType>,
    add_contents: impl FnOnce(&mut Ui, &mut AccordionContext<'a>) -> R,
) -> R {
    let id = ui.id().with(&props.id_source);

    // Apply default value on first render
    let init_id = id.with("default-initialized");
    let initialized = ui
        .ctx()
        .data(|d| d.get_temp::<bool>(init_id))
        .unwrap_or(false);
    if !initialized {
        if let Some(default) = &props.default_value
            && props.value.is_none()
        {
            *props.value = Some(default.clone());
        }
        ui.ctx().data_mut(|d| d.insert_temp(init_id, true));
    }

    let mut ctx = AccordionContext {
        id,
        value: props.value,
        accordion_type: props.accordion_type,
        collapsible: props.collapsible,
        disabled: props.disabled,
        animate: props.animate,
        animation_ms: props.animation_ms,
        theme,
    };

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        if props.disabled {
            ui.disable();
        }
        add_contents(ui, &mut ctx)
    })
    .inner
}

/// Render an accordion item with trigger and content.
pub fn accordion_item<'a, R>(
    ui: &mut Ui,
    theme: &'a Theme,
    acc_ctx: &mut AccordionContext<'a>,
    props: AccordionItemProps<'_>,
    add_trigger: impl FnOnce(&mut Ui, &AccordionItemContext<'_>) -> Response,
    add_content: impl FnOnce(&mut Ui) -> R,
) -> Option<InnerResponse<R>> {
    let item_id = acc_ctx.id.with(props.value);
    let is_open = acc_ctx.is_open(props.value);
    let is_disabled = props.disabled || acc_ctx.disabled;

    trace!(
        "accordion_item value={} is_open={} disabled={}",
        props.value, is_open, is_disabled
    );

    let item_ctx = AccordionItemContext {
        value: props.value,
        is_open,
        disabled: is_disabled,
    };

    // Draw border at bottom
    let border_color = theme.palette.border;

    ui.scope(|ui| {
        if is_disabled {
            ui.disable();
        }

        // Trigger area
        let trigger_response = ui
            .horizontal(|ui| {
                ui.set_width(ui.available_width());
                add_trigger(ui, &item_ctx)
            })
            .inner;

        // Handle click on trigger
        if trigger_response.clicked() && !is_disabled {
            let value_copy = props.value.to_string();
            acc_ctx.toggle(ui, &value_copy);
        }

        // Content with animation
        let mut item_open = is_open;
        let content_result = collapsible(
            ui,
            theme,
            CollapsibleProps::new(item_id.with("collapsible"), &mut item_open)
                .with_animation(acc_ctx.animate)
                .animation_ms(acc_ctx.animation_ms.unwrap_or(200.0)),
            |ui, coll_ctx| {
                coll_ctx.content(ui, |ui| {
                    ui.add_space(4.0);
                    let result = add_content(ui);
                    ui.add_space(16.0);
                    result
                })
            },
        );

        // Draw bottom border
        let rect = ui.max_rect();
        let border_y = rect.max.y;
        ui.painter().hline(
            rect.x_range(),
            border_y,
            egui::Stroke::new(1.0, border_color),
        );

        content_result
    })
    .inner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accordion_state_single_toggle() {
        let mut state = AccordionState::single(None);
        state.toggle("item-1", false);
        assert!(state.is_open("item-1"));

        // Without collapsible, same item stays open
        state.toggle("item-1", false);
        assert!(state.is_open("item-1"));

        // Different item closes the first
        state.toggle("item-2", false);
        assert!(!state.is_open("item-1"));
        assert!(state.is_open("item-2"));
    }

    #[test]
    fn accordion_state_single_collapsible() {
        let mut state = AccordionState::single(Some("item-1".to_string()));

        // With collapsible, can close the open item
        state.toggle("item-1", true);
        assert!(!state.is_open("item-1"));
    }

    #[test]
    fn accordion_state_multiple() {
        let mut state = AccordionState::multiple(vec![]);

        state.toggle("item-1", true);
        state.toggle("item-2", true);
        assert!(state.is_open("item-1"));
        assert!(state.is_open("item-2"));

        state.toggle("item-1", true);
        assert!(!state.is_open("item-1"));
        assert!(state.is_open("item-2"));
    }
}
