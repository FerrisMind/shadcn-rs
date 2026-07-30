//! iced widget, overlay layout, interaction, and drawing for Sonner.

use std::f32::consts::TAU;
use std::time::{Duration, Instant};

use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, layout, overlay, renderer};
use crate::iced_compat::widget::text::{LineHeight, Shaping, Wrapping};
use crate::iced_compat::{
    Border, Event, Length, Pixels, Point, Rectangle, Renderer, Size, Theme, Vector, alignment,
    mouse, touch, window,
};

use super::state::{self, ToastSnapshot};
use super::style::{self, ToastStyle};
use super::types::{ToastId, ToastPosition, ToastType};
use crate::fonts::iced_font;
use crate::recipes::iced_font_weight;
use crate::theme::Theme as ShadcnTheme;
use shadcn_common::FontWeight;

const FRAME_INTERVAL: Duration = Duration::from_millis(16);
const TOAST_PADDING: f32 = 16.0;
/// Collapsed-stack scale step added per back card (mirrors svelte-sonner's
/// `1.0 + index * 0.05`); back cards grow so they peek behind the front card.
const TOAST_STACK_SCALE: f32 = 0.05;
const TOAST_ICON_SIZE: f32 = 18.0;
const TOAST_ICON_GAP: f32 = 10.0;
/// Lucide glyphs are authored on a 24x24 grid; `TOAST_ICON_SIZE` is the
/// on-screen box, so each grid unit maps to `size / LUCIDE_GRID` world pixels.
/// Lines use `LUCIDE_STROKE` grid units (the Lucide default stroke width).
const LUCIDE_GRID: f32 = 24.0;
const LUCIDE_STROKE: f32 = 2.0;
const TITLE_SIZE: f32 = 13.0;
const TITLE_LINE_HEIGHT: f32 = 19.5;
const DESCRIPTION_SIZE: f32 = 13.0;
const DESCRIPTION_LINE_HEIGHT: f32 = 18.2;
const ACTION_SIZE: f32 = 12.0;
const ACTION_HEIGHT: f32 = 24.0;
const ACTION_GAP: f32 = 8.0;
const ACTION_RADIUS: f32 = 4.0;
const CLOSE_SIZE: f32 = 20.0;
const TOAST_MIN_HEIGHT: f32 = 53.5;

/// Internal widget produced by [`super::Toaster`].
pub(super) struct ToasterWidget<'a, Message> {
    pub(super) theme: &'a ShadcnTheme,
    pub(super) position: ToastPosition,
    pub(super) duration_ms: u64,
    pub(super) gap: f32,
    pub(super) offset: f32,
    pub(super) width: f32,
    pub(super) visible_toasts: usize,
    pub(super) rich_colors: bool,
    pub(super) invert: bool,
    pub(super) close_button: bool,
    pub(super) expand: bool,
    pub(super) pause_on_hover: bool,
    pub(super) pause_when_page_is_hidden: bool,
    pub(super) animated: bool,
    pub(super) style_override: Option<Box<dyn Fn(ToastStyle) -> ToastStyle + 'a>>,
    pub(super) marker: std::marker::PhantomData<fn() -> Message>,
}

impl<Message: 'static> Widget<Message, Theme, Renderer> for ToasterWidget<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        Vec::new()
    }

    fn diff(&self, _tree: &mut Tree) {}

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ToasterState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ToasterState::focused())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn size_hint(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let bounds = limits.resolve(Length::Fill, Length::Fill, Size::ZERO);
        layout::Node::new(bounds)
    }

    fn update(
        &mut self,
        _tree: &mut Tree,
        _event: &Event,
        _layout: layout::Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        if state::has_changed() {
            state::reset_changed();
            shell.invalidate_layout();
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: layout::Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        // The normal pass is intentionally empty. The full-window overlay
        // owns the toast stack so it stays above all application content.
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        _layout: layout::Layout<'b>,
        _renderer: &Renderer,
        viewport: &Rectangle,
        _translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        if !state::has_toasts() {
            return None;
        }

        let state = tree.state.downcast_mut::<ToasterState>();
        Some(overlay::Element::new(Box::new(ToasterOverlay {
            theme: self.theme,
            position: self.position,
            duration_ms: self.duration_ms,
            gap: self.gap,
            offset: self.offset,
            width: self.width,
            visible_toasts: self.visible_toasts,
            close_button: self.close_button,
            rich_colors: self.rich_colors,
            invert: self.invert,
            expand: self.expand,
            pause_on_hover: self.pause_on_hover,
            pause_when_page_is_hidden: self.pause_when_page_is_hidden,
            animated: self.animated,
            style_override: self.style_override.as_deref(),
            state,
            viewport: *viewport,
        })))
    }
}

/// Persistent widget-tree state for the overlay.
#[derive(Debug, Default)]
struct ToasterState {
    focused: bool,
    hovered: Option<ToastId>,
    pointer_over: bool,
    last_now: Option<Instant>,
    layouts: Vec<ToastLayout>,
}

impl ToasterState {
    fn focused() -> Self {
        Self {
            focused: true,
            ..Self::default()
        }
    }
}

/// The overlay that owns the viewport-sized hit area and toast stack.
struct ToasterOverlay<'a, 'b> {
    theme: &'a ShadcnTheme,
    position: ToastPosition,
    duration_ms: u64,
    gap: f32,
    offset: f32,
    width: f32,
    visible_toasts: usize,
    close_button: bool,
    rich_colors: bool,
    invert: bool,
    expand: bool,
    pause_on_hover: bool,
    pause_when_page_is_hidden: bool,
    animated: bool,
    style_override: Option<&'b (dyn Fn(ToastStyle) -> ToastStyle + 'a)>,
    state: &'b mut ToasterState,
    viewport: Rectangle,
}

impl<'a, 'b> ToasterOverlay<'a, 'b> {
    fn recompute(&mut self, bounds: Rectangle, now: Instant) {
        let snapshots = state::snapshots();
        self.state.layouts = compute_layouts(
            bounds,
            &snapshots,
            LayoutConfig {
                position: self.position,
                gap: self.gap,
                offset: self.offset,
                width: self.width,
                visible_toasts: self.visible_toasts,
                close_button: self.close_button,
                expand: self.expand,
                animated: self.animated,
            },
            self.state.hovered,
            self.state.pointer_over,
            now,
        );
        self.viewport = bounds;
    }

    fn hovered_layout(&self, point: Point) -> Option<ToastId> {
        self.state
            .layouts
            .iter()
            .find(|layout| layout.visual_bounds.contains(point))
            .map(|layout| layout.id)
    }

    fn pointer_over_stack(&self, point: Point) -> bool {
        self.state.layouts.iter().any(|layout| {
            let mut bounds = layout.visual_bounds;
            for candidate in self
                .state
                .layouts
                .iter()
                .filter(|candidate| candidate.position == layout.position)
            {
                bounds = union_rect(bounds, candidate.visual_bounds);
            }
            bounds.contains(point)
        })
    }

    fn hit_layout(&self, point: Point) -> Option<(ToastLayout, HitTarget)> {
        for layout in &self.state.layouts {
            if let Some(close) = layout
                .close_bounds
                .map(|bounds| scale_rect(bounds, layout.scale, layout.base_bounds))
                && close.contains(point)
            {
                return Some((*layout, HitTarget::Close));
            }
            if let Some(action) = layout
                .action_bounds
                .map(|bounds| scale_rect(bounds, layout.scale, layout.base_bounds))
                && action.contains(point)
            {
                return Some((*layout, HitTarget::Action));
            }
            if let Some(cancel) = layout
                .cancel_bounds
                .map(|bounds| scale_rect(bounds, layout.scale, layout.base_bounds))
                && cancel.contains(point)
            {
                return Some((*layout, HitTarget::Cancel));
            }
            if layout.visual_bounds.contains(point) {
                return Some((*layout, HitTarget::Toast));
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy)]
enum HitTarget {
    Close,
    Action,
    Cancel,
    Toast,
}

impl<Message: 'static> overlay::Overlay<Message, Theme, Renderer> for ToasterOverlay<'_, '_> {
    fn layout(&mut self, _renderer: &Renderer, size: Size) -> layout::Node {
        let bounds = Rectangle::new(Point::ORIGIN, size);
        let now = self.state.last_now.unwrap_or_else(Instant::now);
        self.recompute(bounds, now);
        layout::Node::new(size)
    }

    fn update(
        &mut self,
        event: &Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let bounds = layout.bounds();

        match event {
            Event::Window(window::Event::Focused) => {
                self.state.focused = true;
                shell.request_redraw();
            }
            Event::Window(window::Event::Unfocused) => {
                self.state.focused = false;
                shell.request_redraw();
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                let lifecycle = state::update_lifecycle(
                    *now,
                    self.state.hovered,
                    self.state.pointer_over,
                    self.state.focused,
                    self.pause_on_hover,
                    self.pause_when_page_is_hidden,
                    self.duration_ms,
                    if self.animated {
                        state::DEFAULT_ANIMATION
                    } else {
                        Duration::ZERO
                    },
                );

                self.state.last_now = Some(*now);
                self.recompute(bounds, *now);

                for callback in lifecycle.auto_callbacks {
                    publish_callback::<Message>(Some(callback), shell);
                }

                // `on_dismiss` callbacks queued by [`state::dismiss_all_toasts`], which is
                // a free function without access to a `Shell`.
                for callback in state::take_pending_dismiss_callbacks() {
                    publish_callback::<Message>(Some(callback), shell);
                }

                if lifecycle.changed {
                    shell.invalidate_layout();
                }

                if state::has_toasts() {
                    shell.request_redraw_at(*now + FRAME_INTERVAL);
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(point) = cursor.position() {
                    let hovered = self.hovered_layout(point);
                    let pointer_over = self.pointer_over_stack(point);
                    if hovered != self.state.hovered || pointer_over != self.state.pointer_over {
                        self.state.hovered = hovered;
                        self.state.pointer_over = pointer_over;
                        self.recompute(bounds, self.state.last_now.unwrap_or_else(Instant::now));
                        shell.invalidate_layout();
                    }
                }
                shell.request_redraw();
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(point) = cursor.position()
                    && let Some((hit, target)) = self.hit_layout(point)
                {
                    match target {
                        HitTarget::Close if hit.close_bounds.is_some() => {
                            let callback = state::dismiss_toast(hit.id);
                            publish_callback::<Message>(callback, shell);
                        }
                        HitTarget::Action => {
                            let callback = state::take_action_callback(hit.id, false);
                            let dismiss_callback = state::dismiss_toast(hit.id);
                            publish_callback::<Message>(callback, shell);
                            publish_callback::<Message>(dismiss_callback, shell);
                        }
                        HitTarget::Cancel => {
                            let callback = state::take_action_callback(hit.id, true);
                            let dismiss_callback = state::dismiss_toast(hit.id);
                            publish_callback::<Message>(callback, shell);
                            publish_callback::<Message>(dismiss_callback, shell);
                        }
                        HitTarget::Toast => {}
                        HitTarget::Close => {}
                    }

                    shell.capture_event();
                }
            }
            Event::Mouse(_) | Event::Touch(_) if self.state.pointer_over => {
                shell.capture_event();
            }
            _ => {}
        }
    }

    fn operate(
        &mut self,
        _layout: layout::Layout<'_>,
        _renderer: &Renderer,
        _operation: &mut dyn crate::iced_compat::advanced::widget::Operation,
    ) {
    }

    fn mouse_interaction(
        &self,
        _layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        cursor
            .position()
            .map_or(mouse::Interaction::default(), |point| {
                if self.pointer_over_stack(point) {
                    mouse::Interaction::Pointer
                } else {
                    mouse::Interaction::default()
                }
            })
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let now = self.state.last_now.unwrap_or_else(Instant::now);
        let snapshots = state::snapshots();
        let point = cursor.position();
        let draw_context = ToastDrawContext {
            theme: self.theme,
            now,
            cursor: point,
            rich_colors: self.rich_colors,
            invert: self.invert,
            close_button: self.close_button,
            style_override: self.style_override,
            viewport: self.viewport,
        };

        // Older stacked cards are painted first, leaving the newest card on
        // top and keeping action buttons clickable in the front card.
        for layout in self.state.layouts.iter().rev() {
            if let Some(snapshot) = snapshots.iter().find(|snapshot| snapshot.id == layout.id) {
                draw_toast(renderer, snapshot, layout, draw_context);
            }
        }
    }

    fn overlay<'c>(
        &'c mut self,
        _layout: layout::Layout<'c>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
        None
    }
}

fn publish_callback<Message: 'static>(
    callback: Option<super::types::RawCallback>,
    shell: &mut Shell<'_, Message>,
) {
    let Some(callback) = callback else {
        return;
    };
    let Some(value) = callback() else {
        return;
    };
    if let Ok(message) = value.downcast::<Message>() {
        shell.publish(*message);
    }
}

#[derive(Debug, Clone, Copy)]
struct LayoutConfig {
    position: ToastPosition,
    gap: f32,
    offset: f32,
    width: f32,
    visible_toasts: usize,
    close_button: bool,
    expand: bool,
    animated: bool,
}

#[derive(Debug, Clone, Copy)]
struct ToastLayout {
    id: ToastId,
    position: ToastPosition,
    base_bounds: Rectangle,
    visual_bounds: Rectangle,
    content_visible: bool,
    close_bounds: Option<Rectangle>,
    action_bounds: Option<Rectangle>,
    cancel_bounds: Option<Rectangle>,
    scale: f32,
    alpha: f32,
}

#[derive(Clone, Copy)]
struct ToastDrawContext<'a, 'b> {
    theme: &'a ShadcnTheme,
    now: Instant,
    cursor: Option<Point>,
    rich_colors: bool,
    invert: bool,
    close_button: bool,
    style_override: Option<&'b (dyn Fn(ToastStyle) -> ToastStyle + 'a)>,
    viewport: Rectangle,
}

fn compute_layouts(
    viewport: Rectangle,
    snapshots: &[ToastSnapshot],
    config: LayoutConfig,
    hovered: Option<ToastId>,
    pointer_over: bool,
    now: Instant,
) -> Vec<ToastLayout> {
    let mut layouts = Vec::new();
    let available_width = (viewport.width - config.offset * 2.0).max(1.0);
    let toast_width = config.width.min(available_width);

    for position in positions() {
        let mut group: Vec<&ToastSnapshot> = snapshots
            .iter()
            .filter(|snapshot| {
                snapshot.position.unwrap_or(config.position) == position
                    && (snapshot.open || snapshot.dismissed_at.is_some())
            })
            .collect();

        if group.len() > config.visible_toasts {
            let important: Vec<&ToastSnapshot> = group
                .iter()
                .copied()
                .filter(|snapshot| snapshot.important)
                .collect();
            group.truncate(config.visible_toasts);
            for snapshot in important {
                if !group.iter().any(|entry| entry.id == snapshot.id) {
                    group.push(snapshot);
                }
            }
        }

        if group.is_empty() {
            continue;
        }

        let expanded = config.expand
            || pointer_over
            || hovered.is_some_and(|id| group.iter().any(|toast| toast.id == id));
        let heights: Vec<f32> = group
            .iter()
            .map(|toast| estimate_toast_height(toast, toast_width, expanded, config.close_button))
            .collect();
        let total_height =
            heights.iter().sum::<f32>() + config.gap * (heights.len().saturating_sub(1) as f32);
        let x = match position {
            ToastPosition::TopLeft | ToastPosition::BottomLeft => viewport.x + config.offset,
            ToastPosition::TopRight | ToastPosition::BottomRight => {
                viewport.x + viewport.width - config.offset - toast_width
            }
            ToastPosition::TopCenter | ToastPosition::BottomCenter => {
                viewport.x + (viewport.width - toast_width) / 2.0
            }
            // Future positions added to the shared, `non_exhaustive`
            // `ToastPosition` (in `shadcn_common::toast`) default to a
            // horizontally-centered stack.
            _ => viewport.x + (viewport.width - toast_width) / 2.0,
        };

        let first_y = if position.is_top() {
            viewport.y + config.offset
        } else {
            viewport.y + viewport.height - config.offset - total_height
        };

        for (index, toast) in group.into_iter().enumerate() {
            let height = heights[index];
            let (y, scale) = if expanded {
                let y = first_y + heights[..index].iter().sum::<f32>() + config.gap * index as f32;
                (y, 1.0)
            } else {
                let front_y = if position.is_top() {
                    viewport.y + config.offset
                } else {
                    viewport.y + viewport.height - config.offset - height
                };
                // Back cards scale up and peek toward the stack's interior by
                // the configured `gap` (matching svelte-sonner), instead of
                // shrinking and offsetting by a fixed 10px.
                let y = if position.is_top() {
                    front_y + index as f32 * config.gap
                } else {
                    front_y - index as f32 * config.gap
                };
                (y, (1.0 + index as f32 * TOAST_STACK_SCALE).min(1.10))
            };

            let base_bounds = Rectangle {
                x,
                y,
                width: toast_width,
                height,
            };
            let visual_bounds = scale_rect(base_bounds, scale, base_bounds);
            let content_visible = expanded || index == 0;
            let show_close =
                content_visible && toast.dismissible && (toast.close_button || config.close_button);
            let close_bounds = show_close.then_some(Rectangle {
                x: base_bounds.x + base_bounds.width - TOAST_PADDING - CLOSE_SIZE,
                y: base_bounds.y + 4.0,
                width: CLOSE_SIZE,
                height: CLOSE_SIZE,
            });
            let (action_bounds, cancel_bounds) = if content_visible {
                action_bounds(base_bounds, toast, show_close)
            } else {
                (None, None)
            };
            let alpha = if !config.animated {
                1.0
            } else if toast.open {
                eased_progress(
                    now.saturating_duration_since(toast.created_at),
                    state::DEFAULT_ANIMATION,
                )
            } else {
                toast
                    .dismissed_at
                    .map(|dismissed| {
                        1.0 - eased_progress(
                            now.saturating_duration_since(dismissed),
                            state::DEFAULT_ANIMATION,
                        )
                    })
                    .unwrap_or(0.0)
            };

            layouts.push(ToastLayout {
                id: toast.id,
                position,
                base_bounds,
                visual_bounds,
                content_visible,
                close_bounds,
                action_bounds,
                cancel_bounds,
                scale,
                alpha,
            });
        }
    }

    layouts
}

fn positions() -> [ToastPosition; 6] {
    [
        ToastPosition::TopLeft,
        ToastPosition::TopCenter,
        ToastPosition::TopRight,
        ToastPosition::BottomLeft,
        ToastPosition::BottomCenter,
        ToastPosition::BottomRight,
    ]
}

fn action_bounds(
    bounds: Rectangle,
    toast: &ToastSnapshot,
    close_button: bool,
) -> (Option<Rectangle>, Option<Rectangle>) {
    let mut action_width = toast
        .action_label
        .as_deref()
        .map(button_width)
        .unwrap_or(0.0);
    let mut cancel_width = toast
        .cancel_label
        .as_deref()
        .map(button_width)
        .unwrap_or(0.0);
    let has_action = toast.action_label.is_some();
    let has_cancel = toast.cancel_label.is_some();
    let available = (bounds.width - TOAST_PADDING * 2.0).max(0.0);
    let required_gap = if has_action && has_cancel {
        ACTION_GAP
    } else {
        0.0
    };
    let total = action_width + cancel_width + required_gap;
    if total > available && total > required_gap {
        let scale = ((available - required_gap).max(0.0) / (action_width + cancel_width)).min(1.0);
        action_width *= scale;
        cancel_width *= scale;
    }
    let y = bounds.y + (bounds.height - ACTION_HEIGHT) / 2.0;
    let close_reserve = if close_button {
        CLOSE_SIZE + ACTION_GAP
    } else {
        0.0
    };
    let mut right = bounds.x + bounds.width - TOAST_PADDING - close_reserve;

    let action = toast.action_label.as_ref().map(|_| {
        let x = right - action_width;
        right = x - ACTION_GAP;
        Rectangle {
            x,
            y,
            width: action_width,
            height: ACTION_HEIGHT,
        }
    });
    let cancel = toast.cancel_label.as_ref().map(|_| Rectangle {
        x: right - cancel_width,
        y,
        width: cancel_width,
        height: ACTION_HEIGHT,
    });

    (action, cancel)
}

fn estimate_toast_height(
    toast: &ToastSnapshot,
    width: f32,
    expanded: bool,
    close_button: bool,
) -> f32 {
    let icon_width = icon_for(toast.toast_type).map_or(0.0, |_| TOAST_ICON_SIZE + TOAST_ICON_GAP);
    let close_width = if toast.dismissible && (toast.close_button || close_button) {
        CLOSE_SIZE + TOAST_PADDING
    } else {
        0.0
    };
    let action_width = action_group_width(toast);
    let text_width = (width
        - TOAST_PADDING * 2.0
        - icon_width
        - close_width
        - action_width
        - if action_width > 0.0 { ACTION_GAP } else { 0.0 })
    .max(64.0);
    let title_lines = if toast.title.is_empty() {
        0
    } else if expanded {
        estimate_wrapped_lines(&toast.title, max_chars(text_width, TITLE_SIZE))
    } else {
        1
    };
    let description_lines = toast.description.as_deref().map_or(0, |description| {
        if expanded {
            estimate_wrapped_lines(description, max_chars(text_width, DESCRIPTION_SIZE))
        } else {
            1
        }
    });
    let text_height = TITLE_LINE_HEIGHT * title_lines as f32
        + if title_lines > 0 && description_lines > 0 {
            2.0
        } else {
            0.0
        }
        + DESCRIPTION_LINE_HEIGHT * description_lines as f32;
    let content_height = text_height.max(ACTION_HEIGHT);
    (TOAST_PADDING * 2.0 + content_height + 2.0).max(TOAST_MIN_HEIGHT)
}

fn draw_toast(
    renderer: &mut Renderer,
    toast: &ToastSnapshot,
    layout: &ToastLayout,
    context: ToastDrawContext<'_, '_>,
) {
    let theme = context.theme;
    let cursor = context.cursor;
    let now = context.now;
    let rich_colors = context.rich_colors;
    let invert = context.invert;
    let close_button = context.close_button;
    let style_override = context.style_override;
    let viewport = context.viewport;
    let content_visible = layout.content_visible;

    let Some(clipped) = layout.visual_bounds.intersection(&viewport) else {
        return;
    };
    if layout.alpha <= 0.0 {
        return;
    }

    let mut style = style::resolve_toast_style(
        theme,
        toast.toast_type,
        rich_colors || toast.rich_colors,
        invert || toast.invert,
    );
    if let Some(style_override) = style_override {
        style = style_override(style);
    }

    let origin = layout.base_bounds.center();
    let transform = crate::iced_compat::Transformation::translate(origin.x, origin.y)
        * crate::iced_compat::Transformation::scale(layout.scale)
        * crate::iced_compat::Transformation::translate(-origin.x, -origin.y);
    renderer.with_layer(clipped, |renderer| {
        renderer.with_transformation(transform, |renderer| {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.base_bounds,
                    border: Border {
                        color: style.border().scale_alpha(layout.alpha),
                        width: style.border_width(),
                        radius: style.radius().into(),
                    },
                    shadow: scale_shadow(style.shadow(), layout.alpha),
                    ..renderer::Quad::default()
                },
                style.background().scale_alpha(layout.alpha),
            );

            if !content_visible {
                return;
            }

            let has_icon = icon_for(toast.toast_type).is_some();
            let icon_x = layout.base_bounds.x + TOAST_PADDING;
            let text_x = if has_icon {
                icon_x + TOAST_ICON_SIZE + TOAST_ICON_GAP
            } else {
                icon_x
            };
            let close_reserve = if toast.dismissible && (close_button || toast.close_button) {
                CLOSE_SIZE + TOAST_PADDING / 2.0
            } else {
                0.0
            };
            let action_reserve = action_group_width(toast);
            let text_width = (layout.base_bounds.width
                - (text_x - layout.base_bounds.x)
                - TOAST_PADDING
                - close_reserve
                - action_reserve
                - if action_reserve > 0.0 {
                    ACTION_GAP
                } else {
                    0.0
                })
            .max(48.0);

            if has_icon {
                draw_lucide_icon(
                    renderer,
                    toast.toast_type,
                    Point::new(icon_x, layout.base_bounds.y + TOAST_PADDING),
                    TOAST_ICON_SIZE,
                    style.icon(),
                    layout.alpha,
                    now,
                    toast.created_at,
                );
            }

            let title_y = layout.base_bounds.y + TOAST_PADDING;
            let title_text = if layout.scale < 1.0 {
                truncate_single_line(&toast.title, max_chars(text_width, TITLE_SIZE))
            } else {
                toast.title.clone()
            };
            let title_height = TITLE_LINE_HEIGHT
                * (if layout.scale >= 1.0 {
                    estimate_wrapped_lines(&title_text, max_chars(text_width, TITLE_SIZE))
                } else {
                    1
                } as f32);
            draw_text(
                renderer,
                &title_text,
                Point::new(text_x, title_y),
                Size::new(text_width, title_height),
                TextAppearance {
                    size: TITLE_SIZE,
                    line_height: TITLE_LINE_HEIGHT,
                    color: style.text().scale_alpha(layout.alpha),
                    font: toast_font(theme, FontWeight::Medium),
                    wrapping: if layout.scale >= 1.0 {
                        Wrapping::Word
                    } else {
                        Wrapping::None
                    },
                    align_x: alignment::Horizontal::Left,
                    align_y: alignment::Vertical::Top,
                },
            );

            if let Some(description) = toast.description.as_deref() {
                let description_y = title_y + title_height + 2.0;
                let description_text = if layout.scale < 1.0 {
                    truncate_single_line(description, max_chars(text_width, DESCRIPTION_SIZE))
                } else {
                    description.to_owned()
                };
                let description_height = (layout.base_bounds.height
                    - (description_y - layout.base_bounds.y)
                    - TOAST_PADDING)
                    .max(DESCRIPTION_LINE_HEIGHT);
                draw_text(
                    renderer,
                    &description_text,
                    Point::new(text_x, description_y),
                    Size::new(text_width, description_height),
                    TextAppearance {
                        size: DESCRIPTION_SIZE,
                        line_height: DESCRIPTION_LINE_HEIGHT,
                        color: style.description().scale_alpha(layout.alpha),
                        font: iced_font(theme.font_pack().sans),
                        wrapping: if layout.scale >= 1.0 {
                            Wrapping::Word
                        } else {
                            Wrapping::None
                        },
                        align_x: alignment::Horizontal::Left,
                        align_y: alignment::Vertical::Top,
                    },
                );
            }

            if let Some(action) = layout.action_bounds {
                draw_action(
                    renderer,
                    toast.action_label.as_deref().unwrap_or_default(),
                    action,
                    ActionAppearance {
                        background: style.action_background(),
                        text: style.action_text(),
                        alpha: layout.alpha,
                        border: theme.palette.border.scale_alpha(layout.alpha * 0.5),
                        radius: ACTION_RADIUS,
                        font: toast_font(theme, FontWeight::Medium),
                        hovered: cursor.map(|point| {
                            scale_rect(action, layout.scale, layout.base_bounds).contains(point)
                        }),
                    },
                );
            }
            if let Some(cancel) = layout.cancel_bounds {
                draw_action(
                    renderer,
                    toast.cancel_label.as_deref().unwrap_or_default(),
                    cancel,
                    ActionAppearance {
                        background: style.cancel_background(),
                        text: style.cancel_text(),
                        alpha: layout.alpha,
                        border: theme.palette.border.scale_alpha(layout.alpha * 0.5),
                        radius: ACTION_RADIUS,
                        font: toast_font(theme, FontWeight::Medium),
                        hovered: cursor.map(|point| {
                            scale_rect(cancel, layout.scale, layout.base_bounds).contains(point)
                        }),
                    },
                );
            }

            if let Some(close) = layout.close_bounds {
                let hovered = cursor.is_some_and(|point| {
                    scale_rect(close, layout.scale, layout.base_bounds).contains(point)
                });
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: close,
                        border: Border {
                            color: style.border().scale_alpha(layout.alpha),
                            width: 1.0,
                            radius: (close.width / 2.0).into(),
                        },
                        ..renderer::Quad::default()
                    },
                    style
                        .background()
                        .scale_alpha(layout.alpha * if hovered { 0.92 } else { 1.0 }),
                );
                draw_text(
                    renderer,
                    "×",
                    Point::new(close.x, close.y),
                    Size::new(close.width, close.height),
                    TextAppearance {
                        size: 14.0,
                        line_height: 17.5,
                        color: style
                            .text()
                            .scale_alpha(layout.alpha * if hovered { 1.0 } else { 0.72 }),
                        font: iced_font(theme.font_pack().sans),
                        wrapping: Wrapping::None,
                        align_x: alignment::Horizontal::Center,
                        align_y: alignment::Vertical::Center,
                    },
                );
            }
        });
    });
}

#[derive(Clone, Copy)]
struct ActionAppearance {
    background: crate::iced_compat::Color,
    text: crate::iced_compat::Color,
    alpha: f32,
    border: crate::iced_compat::Color,
    radius: f32,
    font: crate::iced_compat::Font,
    hovered: Option<bool>,
}

fn draw_action(
    renderer: &mut Renderer,
    label: &str,
    bounds: Rectangle,
    appearance: ActionAppearance,
) {
    let background = if appearance.hovered == Some(true) {
        appearance.background.scale_alpha(appearance.alpha * 0.88)
    } else {
        appearance.background.scale_alpha(appearance.alpha)
    };
    renderer.fill_quad(
        renderer::Quad {
            bounds,
            border: Border {
                color: appearance.border,
                width: 1.0,
                radius: appearance.radius.into(),
            },
            ..renderer::Quad::default()
        },
        background,
    );
    draw_text(
        renderer,
        label,
        Point::new(bounds.x + 8.0, bounds.y),
        Size::new(bounds.width - 16.0, bounds.height),
        TextAppearance {
            size: ACTION_SIZE,
            line_height: ACTION_SIZE * 1.25,
            color: appearance.text.scale_alpha(appearance.alpha),
            font: appearance.font,
            wrapping: Wrapping::None,
            align_x: alignment::Horizontal::Left,
            align_y: alignment::Vertical::Center,
        },
    );
}

#[derive(Clone, Copy)]
struct TextAppearance {
    size: f32,
    line_height: f32,
    color: crate::iced_compat::Color,
    font: crate::iced_compat::Font,
    wrapping: Wrapping,
    align_x: alignment::Horizontal,
    align_y: alignment::Vertical,
}

fn draw_text(
    renderer: &mut Renderer,
    content: &str,
    position: Point,
    bounds: Size,
    appearance: TextAppearance,
) {
    use iced_core::text::Renderer as TextRenderer;

    let anchor = Point::new(
        match appearance.align_x {
            alignment::Horizontal::Left => position.x,
            alignment::Horizontal::Center => position.x + bounds.width / 2.0,
            alignment::Horizontal::Right => position.x + bounds.width,
        },
        match appearance.align_y {
            alignment::Vertical::Top => position.y,
            alignment::Vertical::Center => position.y + bounds.height / 2.0,
            alignment::Vertical::Bottom => position.y + bounds.height,
        },
    );

    renderer.fill_text(
        iced_core::Text {
            content: content.to_owned(),
            bounds,
            size: Pixels(appearance.size),
            line_height: LineHeight::Absolute(Pixels(appearance.line_height)),
            font: appearance.font,
            align_x: appearance.align_x.into(),
            align_y: appearance.align_y,
            shaping: Shaping::Advanced,
            wrapping: appearance.wrapping,
        },
        anchor,
        appearance.color,
        Rectangle {
            x: position.x,
            y: position.y,
            width: bounds.width,
            height: bounds.height,
        },
    );
}

/// Returns whether a toast of `toast_type` shows a leading icon. The glyph
/// strings are kept only to express presence — the icons are drawn vectorially
/// by [`draw_lucide_icon`], never as glyphs.
fn icon_for(toast_type: ToastType) -> Option<&'static str> {
    match toast_type {
        ToastType::Default => None,
        ToastType::Success => Some("check"),
        ToastType::Error => Some("x"),
        ToastType::Warning => Some("alert"),
        ToastType::Info => Some("info"),
        ToastType::Loading => Some("loader"),
        _ => None,
    }
}

/// Draws a Lucide outline icon for `toast_type` rooted at `origin` inside a
/// `size` x `size` box, tinted with `color` modulated by `alpha`.
///
/// `iced` 0.14's [`crate::iced_compat::Transformation`] only exposes
/// translate/scale (no rotation) and the overlay renderer cannot stroke
/// arbitrary paths, so strokes are rasterised from axis-aligned primitives:
/// rings are a single bordered transparent quad, while straight segments and
/// arcs are "stamp-stroked" as overlapping filled circles. The loader's spin is
/// baked into its arc angle math (`phase` advances the gap's angle), which is
/// visually equivalent to a `Transformation::rotate` (unavailable here).
// NOTE: kept at eight parameters on purpose — each one is an orthogonal
// coordinate of the icon (renderer, type, origin, size, color, alpha, and
// the two `Instant`s driving the loader spin). Bundling them would invent a
// throwaway struct for a single call site, so the clippy lint is suppressed
// here, mirroring `update_lifecycle` in `state.rs`.
#[allow(clippy::too_many_arguments)]
fn draw_lucide_icon(
    renderer: &mut Renderer,
    toast_type: ToastType,
    origin: Point,
    size: f32,
    color: crate::iced_compat::Color,
    alpha: f32,
    now: Instant,
    created_at: Instant,
) {
    if size <= 0.0 || alpha <= 0.0 {
        return;
    }
    let tint = color.scale_alpha(alpha);
    let s = size / LUCIDE_GRID;
    let stroke = LUCIDE_STROKE * s;
    let center = Point::new(origin.x + 12.0 * s, origin.y + 12.0 * s);
    let pt = |gx: f32, gy: f32| Point::new(origin.x + gx * s, origin.y + gy * s);

    match toast_type {
        ToastType::Success => {
            // lucide circle-check: ring r=9 plus a two-segment check.
            draw_ring(renderer, center, 9.0 * s + stroke / 2.0, stroke, tint);
            draw_line_segment(renderer, pt(9.0, 12.0), pt(11.0, 14.0), stroke, tint);
            draw_line_segment(renderer, pt(11.0, 14.0), pt(15.0, 10.0), stroke, tint);
        }
        ToastType::Error => {
            // lucide circle-x: ring r=9 plus an "x".
            draw_ring(renderer, center, 9.0 * s + stroke / 2.0, stroke, tint);
            draw_line_segment(renderer, pt(9.0, 9.0), pt(15.0, 15.0), stroke, tint);
            draw_line_segment(renderer, pt(15.0, 9.0), pt(9.0, 15.0), stroke, tint);
        }
        ToastType::Info => {
            // lucide info: ring r=9, a stem and a small top dot.
            draw_ring(renderer, center, 9.0 * s + stroke / 2.0, stroke, tint);
            draw_line_segment(renderer, pt(12.0, 16.0), pt(12.0, 12.0), stroke, tint);
            draw_dot(renderer, pt(12.0, 8.0), stroke, tint);
        }
        ToastType::Warning => {
            // lucide triangle-alert: outlined triangle, a stem and a dot.
            draw_line_segment(renderer, pt(12.0, 3.5), pt(3.5, 19.0), stroke, tint);
            draw_line_segment(renderer, pt(3.5, 19.0), pt(20.5, 19.0), stroke, tint);
            draw_line_segment(renderer, pt(20.5, 19.0), pt(12.0, 3.5), stroke, tint);
            draw_line_segment(renderer, pt(12.0, 9.0), pt(12.0, 13.0), stroke, tint);
            draw_dot(renderer, pt(12.0, 16.5), stroke, tint);
        }
        ToastType::Loading => {
            // lucide loader-2: a near-full ring with a ~16deg gap that spins.
            let phase = (now.saturating_duration_since(created_at).as_secs_f32() / 1.2)
                .rem_euclid(1.0)
                * TAU;
            let gap = (16.0_f32).to_radians();
            draw_arc(
                renderer,
                center,
                9.0 * s,
                stroke,
                tint,
                phase + gap,
                TAU - gap,
            );
        }
        ToastType::Default => {}
        _ => {}
    }
}

/// Draws an outline circle (a Lucide "ring") centred at `center` with outer
/// radius `r_outer` and `thickness`-wide stroke, tinted `color`.
///
/// `iced` strokes a quad border along a path inset by `thickness / 2` with
/// radius reduced by the same amount, so the visible band spans
/// `r_outer - thickness .. r_outer`. Passing `9*s + stroke/2` therefore places
/// the ring's centre-line on the Lucide path radius `9`.
fn draw_ring(
    renderer: &mut Renderer,
    center: Point,
    r_outer: f32,
    thickness: f32,
    color: crate::iced_compat::Color,
) {
    if r_outer <= 0.0 || thickness <= 0.0 {
        return;
    }
    let side = r_outer * 2.0;
    renderer.fill_quad(
        renderer::Quad {
            bounds: Rectangle {
                x: center.x - r_outer,
                y: center.y - r_outer,
                width: side,
                height: side,
            },
            border: Border {
                color,
                width: thickness,
                radius: (side / 2.0).into(),
            },
            ..renderer::Quad::default()
        },
        crate::iced_compat::Color::TRANSPARENT,
    );
}

/// Draws a filled circle of `diameter` centred at `center`, tinted `color`.
fn draw_dot(
    renderer: &mut Renderer,
    center: Point,
    diameter: f32,
    color: crate::iced_compat::Color,
) {
    if diameter <= 0.0 {
        return;
    }
    let radius = diameter / 2.0;
    renderer.fill_quad(
        renderer::Quad {
            bounds: Rectangle {
                x: center.x - radius,
                y: center.y - radius,
                width: diameter,
                height: diameter,
            },
            border: Border {
                radius: radius.into(),
                ..Border::default()
            },
            ..renderer::Quad::default()
        },
        color,
    );
}

/// Stamp-strokes a thick rounded-cap line segment from `a` to `b` in world
/// coordinates, tinted `color`. Implemented as a chain of overlapping filled
/// circles because `iced` 0.14 cannot rotate quads or stroke segments.
fn draw_line_segment(
    renderer: &mut Renderer,
    a: Point,
    b: Point,
    width: f32,
    color: crate::iced_compat::Color,
) {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let length = (dx * dx + dy * dy).sqrt();
    if width <= 0.0 {
        return;
    }
    if length <= 0.0 {
        draw_dot(renderer, a, width, color);
        return;
    }
    let step = (width * 0.5).max(0.5);
    let count = ((length / step).ceil().clamp(2.0, 512.0)) as usize;
    for i in 0..=count {
        let t = i as f32 / count as f32;
        draw_dot(
            renderer,
            Point::new(a.x + dx * t, a.y + dy * t),
            width,
            color,
        );
    }
}

/// Stamp-strokes a thick arc of `radius` around `center` between angles
/// `start .. start + sweep`, tinted `color`. The angular step keeps
/// neighbouring stamps overlapping for a smooth curve; the loader rotates by
/// advancing `start` with time instead of applying a transformation.
fn draw_arc(
    renderer: &mut Renderer,
    center: Point,
    radius: f32,
    width: f32,
    color: crate::iced_compat::Color,
    start: f32,
    sweep: f32,
) {
    if radius <= 0.0 || width <= 0.0 || sweep <= 0.0 {
        return;
    }
    let step = (width * 0.5 / radius).max(0.01);
    let count = ((sweep / step).ceil().clamp(2.0, 512.0)) as usize;
    for i in 0..=count {
        let theta = start + sweep * (i as f32 / count as f32);
        draw_dot(
            renderer,
            Point::new(
                center.x + radius * theta.cos(),
                center.y + radius * theta.sin(),
            ),
            width,
            color,
        );
    }
}

fn scale_shadow(shadow: crate::iced_compat::Shadow, alpha: f32) -> crate::iced_compat::Shadow {
    crate::iced_compat::Shadow {
        color: shadow.color.scale_alpha(alpha),
        ..shadow
    }
}

fn scale_rect(rectangle: Rectangle, scale: f32, origin: Rectangle) -> Rectangle {
    let center = origin.center();
    let width = rectangle.width * scale;
    let height = rectangle.height * scale;
    Rectangle {
        x: center.x + (rectangle.x - center.x) * scale,
        y: center.y + (rectangle.y - center.y) * scale,
        width,
        height,
    }
}

fn union_rect(first: Rectangle, second: Rectangle) -> Rectangle {
    let left = first.x.min(second.x);
    let top = first.y.min(second.y);
    let right = (first.x + first.width).max(second.x + second.width);
    let bottom = (first.y + first.height).max(second.y + second.height);
    Rectangle {
        x: left,
        y: top,
        width: right - left,
        height: bottom - top,
    }
}

fn button_width(label: &str) -> f32 {
    (label.chars().count() as f32 * 7.0 + 16.0).clamp(48.0, 144.0)
}

fn toast_font(theme: &ShadcnTheme, weight: FontWeight) -> crate::iced_compat::Font {
    let mut font = iced_font(theme.font_pack().sans);
    font.weight = iced_font_weight(weight);
    font
}

fn action_group_width(toast: &ToastSnapshot) -> f32 {
    let action = toast
        .action_label
        .as_deref()
        .map(button_width)
        .unwrap_or(0.0);
    let cancel = toast
        .cancel_label
        .as_deref()
        .map(button_width)
        .unwrap_or(0.0);
    action
        + cancel
        + if action > 0.0 && cancel > 0.0 {
            ACTION_GAP
        } else {
            0.0
        }
}

fn max_chars(width: f32, font_size: f32) -> usize {
    (width / (font_size * 0.56).max(1.0)).floor().max(1.0) as usize
}

fn estimate_wrapped_lines(text: &str, max_chars: usize) -> usize {
    if text.is_empty() {
        return 0;
    }

    let max_chars = max_chars.max(1);
    let mut total = 0;
    for line in text.lines() {
        let mut current = 0;
        let mut lines = 1;
        for word in line.split_whitespace() {
            let word_len = word.chars().count();
            if current == 0 {
                if word_len <= max_chars {
                    current = word_len;
                } else {
                    lines += (word_len - 1) / max_chars;
                    current = word_len % max_chars;
                    if current == 0 {
                        current = max_chars;
                    }
                }
            } else if current + word_len < max_chars {
                current += word_len + 1;
            } else {
                lines += 1;
                current = word_len.min(max_chars);
            }
        }
        total += lines;
    }
    total.max(1)
}

fn truncate_single_line(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let first: String = chars
        .by_ref()
        .take(max_chars.max(1).saturating_sub(1))
        .collect();
    if chars.next().is_some() {
        format!("{first}…")
    } else {
        first
    }
}

fn eased_progress(elapsed: Duration, duration: Duration) -> f32 {
    if duration.is_zero() {
        return 1.0;
    }
    let raw = (elapsed.as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0);
    let inverse = 1.0 - raw;
    1.0 - inverse * inverse * inverse
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(id: u64, title: &str) -> ToastSnapshot {
        ToastSnapshot {
            id: ToastId::from(id),
            title: title.to_owned(),
            toast_type: ToastType::Default,
            description: None,
            dismissible: true,
            close_button: true,
            rich_colors: false,
            invert: false,
            position: None,
            important: false,
            action_label: Some("Undo".to_owned()),
            cancel_label: None,
            created_at: Instant::now(),
            dismissed_at: None,
            open: true,
        }
    }

    #[test]
    fn compact_stack_hides_back_card_content_and_hit_regions() {
        let snapshots = [snapshot(1, "Newest"), snapshot(2, "Older")];
        let config = LayoutConfig {
            position: ToastPosition::BottomRight,
            gap: 14.0,
            offset: 24.0,
            width: 356.0,
            visible_toasts: 3,
            close_button: true,
            expand: false,
            animated: false,
        };

        let layouts = compute_layouts(
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: 1_024.0,
                height: 768.0,
            },
            &snapshots,
            config,
            None,
            false,
            Instant::now(),
        );

        assert_eq!(layouts.len(), 2);
        assert!(layouts[0].content_visible);
        assert!(layouts[0].close_bounds.is_some());
        assert!(layouts[0].action_bounds.is_some());
        assert!(!layouts[1].content_visible);
        assert!(layouts[1].close_bounds.is_none());
        assert!(layouts[1].action_bounds.is_none());
        assert!(layouts[1].cancel_bounds.is_none());
    }

    #[test]
    fn hovering_a_compact_stack_expands_every_card() {
        let snapshots = [snapshot(1, "Newest"), snapshot(2, "Older")];
        let config = LayoutConfig {
            position: ToastPosition::BottomRight,
            gap: 14.0,
            offset: 24.0,
            width: 356.0,
            visible_toasts: 3,
            close_button: true,
            expand: false,
            animated: false,
        };

        let layouts = compute_layouts(
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: 1_024.0,
                height: 768.0,
            },
            &snapshots,
            config,
            Some(ToastId::from(2)),
            false,
            Instant::now(),
        );

        assert!(layouts.iter().all(|layout| layout.content_visible));
        assert!(layouts.iter().all(|layout| layout.action_bounds.is_some()));
    }

    #[test]
    fn actions_share_the_toast_row_and_do_not_add_a_bottom_band() {
        let mut toast = snapshot(1, "Event has been created");
        toast.description = Some("Cancel this".to_owned());
        toast.cancel_label = Some("Cancel".to_owned());

        let height = estimate_toast_height(&toast, 356.0, true, false);
        assert!((height - 73.7).abs() < 0.01);

        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 356.0,
            height,
        };
        let (action, cancel) = action_bounds(bounds, &toast, false);
        let action = action.expect("action bounds");
        let cancel = cancel.expect("cancel bounds");

        assert!((action.y - (height - ACTION_HEIGHT) / 2.0).abs() < 0.01);
        assert!((cancel.y - action.y).abs() < 0.01);
        assert!(cancel.x + cancel.width <= action.x - ACTION_GAP + 0.01);
        assert!(action.x + action.width <= bounds.x + bounds.width - TOAST_PADDING + 0.01);
    }
}
