//! Resizable panel component - panels with draggable resize handles.
//!
//! Based on react-resizable-panels API.
//!
//! # Example
//! ```ignore
//! let mut sizes = vec![50.0, 50.0];
//! resizable_panel_group(ui, &theme, ResizablePanelGroupProps::new("my-group"), &mut sizes, |ui, ctx| {
//!     resizable_panel(ui, ctx, ResizablePanelProps::new(50.0), 0, |ui| { ui.label("Left"); });
//!     resizable_handle(ui, &theme, ctx, ResizableHandleProps::new(), 0);
//!     resizable_panel(ui, ctx, ResizablePanelProps::new(50.0), 1, |ui| { ui.label("Right"); });
//! });
//! ```

use crate::theme::Theme;
use egui::{CursorIcon, Id, Response, Sense, Ui, Vec2};
use std::hash::Hash;

// =============================================================================
// ResizableDirection
// =============================================================================

/// Direction of the resizable panel group.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResizableDirection {
    #[default]
    Horizontal,
    Vertical,
}

// =============================================================================
// ResizablePanelGroupProps
// =============================================================================

/// Properties for the resizable panel group container.
#[derive(Clone, Debug)]
pub struct ResizablePanelGroupProps<Id: Hash> {
    pub id_source: Id,
    pub direction: ResizableDirection,
    pub auto_save_id: Option<String>,
}

impl<IdType: Hash> ResizablePanelGroupProps<IdType> {
    pub fn new(id_source: IdType) -> Self {
        Self {
            id_source,
            direction: ResizableDirection::Horizontal,
            auto_save_id: None,
        }
    }

    pub fn direction(mut self, direction: ResizableDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn auto_save_id(mut self, id: impl Into<String>) -> Self {
        self.auto_save_id = Some(id.into());
        self
    }
}

// =============================================================================
// ResizablePanelProps
// =============================================================================

/// Properties for an individual resizable panel.
#[derive(Clone, Debug)]
pub struct ResizablePanelProps {
    pub default_size: f32,
    pub min_size: Option<f32>,
    pub max_size: Option<f32>,
    pub collapsible: bool,
}

impl ResizablePanelProps {
    pub fn new(default_size: f32) -> Self {
        Self {
            default_size: default_size.clamp(0.0, 100.0),
            min_size: None,
            max_size: None,
            collapsible: false,
        }
    }

    pub fn min_size(mut self, min: f32) -> Self {
        self.min_size = Some(min.clamp(0.0, 100.0));
        self
    }

    pub fn max_size(mut self, max: f32) -> Self {
        self.max_size = Some(max.clamp(0.0, 100.0));
        self
    }

    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Clamp a size value to the min/max constraints.
    pub fn clamp_size(&self, size: f32) -> f32 {
        let min = self.min_size.unwrap_or(0.0);
        let max = self.max_size.unwrap_or(100.0);
        size.clamp(min, max)
    }
}

// =============================================================================
// ResizableHandleProps
// =============================================================================

/// Properties for the resize handle between panels.
#[derive(Clone, Debug, Default)]
pub struct ResizableHandleProps {
    pub with_handle: bool,
    pub disabled: bool,
}

impl ResizableHandleProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_handle(mut self, with_handle: bool) -> Self {
        self.with_handle = with_handle;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

// =============================================================================
// ResizableContext
// =============================================================================

/// Context passed to resizable panel children.
pub struct ResizableContext<'a> {
    id: Id,
    direction: ResizableDirection,
    sizes: &'a mut Vec<f32>,
    total_size: f32,
    #[allow(dead_code)]
    theme: &'a Theme,
}

impl<'a> ResizableContext<'a> {
    /// Get the current size percentage for a panel.
    pub fn get_size(&self, index: usize) -> f32 {
        self.sizes.get(index).copied().unwrap_or(0.0)
    }

    /// Get the pixel size for a panel based on total available space.
    pub fn get_pixel_size(&self, index: usize) -> f32 {
        let percent = self.get_size(index);
        self.total_size * percent / 100.0
    }

    /// Resize panels around a handle.
    pub fn resize(&mut self, handle_index: usize, delta_percent: f32) {
        if handle_index >= self.sizes.len().saturating_sub(1) {
            return;
        }

        let left_idx = handle_index;
        let right_idx = handle_index + 1;

        let left_size = self.sizes[left_idx];
        let right_size = self.sizes[right_idx];

        // Calculate new sizes
        let new_left = (left_size + delta_percent).clamp(5.0, 95.0);

        // Ensure they still sum to the original total
        let total = left_size + right_size;
        let adjusted_left = new_left.min(total - 5.0);
        let adjusted_right = total - adjusted_left;

        self.sizes[left_idx] = adjusted_left;
        self.sizes[right_idx] = adjusted_right;
    }

    pub fn direction(&self) -> ResizableDirection {
        self.direction
    }
}

// =============================================================================
// Main functions
// =============================================================================

/// Render a resizable panel group container.
pub fn resizable_panel_group<'a, IdType: Hash, R>(
    ui: &mut Ui,
    theme: &'a Theme,
    props: ResizablePanelGroupProps<IdType>,
    sizes: &'a mut Vec<f32>,
    add_contents: impl FnOnce(&mut Ui, &mut ResizableContext<'a>) -> R,
) -> R {
    let id = ui.id().with(&props.id_source);

    // Calculate total available size based on direction
    let available = ui.available_size();
    let total_size = match props.direction {
        ResizableDirection::Horizontal => available.x,
        ResizableDirection::Vertical => available.y,
    };

    let mut ctx = ResizableContext {
        id,
        direction: props.direction,
        sizes,
        total_size,
        theme,
    };

    // Layout based on direction
    match props.direction {
        ResizableDirection::Horizontal => {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                add_contents(ui, &mut ctx)
            })
            .inner
        }
        ResizableDirection::Vertical => {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                add_contents(ui, &mut ctx)
            })
            .inner
        }
    }
}

/// Render an individual resizable panel.
pub fn resizable_panel<R>(
    ui: &mut Ui,
    ctx: &ResizableContext<'_>,
    props: ResizablePanelProps,
    index: usize,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> R {
    let size_percent = ctx.get_size(index);
    let clamped_percent = props.clamp_size(size_percent);

    let size = match ctx.direction {
        ResizableDirection::Horizontal => {
            let width = ctx.total_size * clamped_percent / 100.0;
            Vec2::new(width.max(1.0), ui.available_height())
        }
        ResizableDirection::Vertical => {
            let height = ctx.total_size * clamped_percent / 100.0;
            Vec2::new(ui.available_width(), height.max(1.0))
        }
    };

    let panel_id = ctx.id.with(("panel", index));

    ui.push_id(panel_id, |ui| {
        ui.set_min_size(size);
        ui.set_max_size(size);

        ui.scope(|ui| add_contents(ui)).inner
    })
    .inner
}

/// Render a resize handle between panels.
pub fn resizable_handle(
    ui: &mut Ui,
    theme: &Theme,
    ctx: &mut ResizableContext<'_>,
    props: ResizableHandleProps,
    handle_index: usize,
) -> Response {
    let _handle_id = ctx.id.with(("handle", handle_index));

    let (handle_size, cursor) = match ctx.direction {
        ResizableDirection::Horizontal => (
            Vec2::new(4.0, ui.available_height()),
            CursorIcon::ResizeHorizontal,
        ),
        ResizableDirection::Vertical => (
            Vec2::new(ui.available_width(), 4.0),
            CursorIcon::ResizeVertical,
        ),
    };

    let (rect, response) = ui.allocate_exact_size(handle_size, Sense::drag());

    // Visual styling
    let is_hovered = response.hovered();
    let is_dragging = response.dragged();

    let bg_color = if is_dragging {
        theme.palette.primary
    } else if is_hovered {
        theme.palette.muted_foreground
    } else {
        theme.palette.border
    };

    ui.painter().rect_filled(rect, 0.0, bg_color);

    // Handle grip icon
    if props.with_handle {
        let grip_size = Vec2::splat(8.0);
        let grip_rect = egui::Rect::from_center_size(rect.center(), grip_size);

        ui.painter()
            .rect_filled(grip_rect, 2.0, theme.palette.border);

        // Draw grip lines
        let line_color = theme.palette.muted_foreground;
        let center = rect.center();

        match ctx.direction {
            ResizableDirection::Horizontal => {
                for offset in [-2.0, 0.0, 2.0] {
                    let y = center.y + offset;
                    ui.painter().line_segment(
                        [egui::pos2(center.x - 2.0, y), egui::pos2(center.x + 2.0, y)],
                        egui::Stroke::new(1.0, line_color),
                    );
                }
            }
            ResizableDirection::Vertical => {
                for offset in [-2.0, 0.0, 2.0] {
                    let x = center.x + offset;
                    ui.painter().line_segment(
                        [egui::pos2(x, center.y - 2.0), egui::pos2(x, center.y + 2.0)],
                        egui::Stroke::new(1.0, line_color),
                    );
                }
            }
        }
    }

    // Cursor
    if is_hovered || is_dragging {
        ui.ctx().set_cursor_icon(cursor);
    }

    // Handle dragging
    if response.dragged() && !props.disabled {
        let delta = response.drag_delta();
        let delta_px = match ctx.direction {
            ResizableDirection::Horizontal => delta.x,
            ResizableDirection::Vertical => delta.y,
        };

        // Convert pixel delta to percentage
        let delta_percent = if ctx.total_size > 0.0 {
            delta_px / ctx.total_size * 100.0
        } else {
            0.0
        };

        ctx.resize(handle_index, delta_percent);
        ui.ctx().request_repaint();
    }

    response
}
