//! Toaster widget and overlay rendering for sonner toasts.
//!
//! The `ToasterWidget` occupies zero layout space and provides an overlay
//! that renders all active toasts at the configured screen position. The
//! overlay handles layout (positioning, stacking), drawing (backgrounds,
//! borders, text, icons, buttons), event dispatch (clicks, dismiss), and
//! lifecycle (auto-close timers, enter/exit animations).

use crate::iced_compat::advanced::renderer::Renderer as _;
use crate::iced_compat::advanced::widget::{Tree, tree};
use crate::iced_compat::advanced::{Clipboard, Shell, Widget, layout, overlay, renderer};
use crate::iced_compat::{
    Border, Event, Length, Point, Rectangle, Renderer, Size, Theme, Vector, mouse, time, touch,
    window,
};

use super::state::{has_changed, reset_changed, with_toasts, with_toasts_mut};
use super::style::{self, MAX_VISIBLE_TOASTS, TOAST_PADDING, TOAST_WIDTH, UNMOUNT_DELAY_MS};
use super::types::{RawToast, ToastPosition, ToastType};
use crate::fonts::iced_font;
use crate::theme::Theme as ShadcnTheme;

/// Frame pacing for animations.
const FRAME_INTERVAL: time::Duration = time::Duration::from_millis(16);

/// Internal widget produced by the [`super::Toaster`] builder.
pub(super) struct ToasterWidget<'a, Message> {
    pub(super) theme: &'a ShadcnTheme,
    pub(super) position: ToastPosition,
    pub(super) duration_ms: u64,
    pub(super) gap: f32,
    pub(super) offset: f32,
    pub(super) max_visible: usize,
    pub(super) rich_colors: bool,
    pub(super) invert: bool,
    pub(super) close_button: bool,
    pub(super) expand: bool,
    pub(super) _marker: std::marker::PhantomData<Message>,
}

impl<Message> Widget<Message, Theme, Renderer> for ToasterWidget<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        Vec::new()
    }

    fn diff(&self, _tree: &mut Tree) {}

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ToasterState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ToasterState::new())
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
        // The toaster takes up the full viewport to provide the overlay
        // anchor, but renders nothing in the normal layout pass.
        let bounds = limits.resolve(Length::Fill, Length::Fill, Size::ZERO);
        layout::Node::new(bounds)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        _layout: layout::Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let _state = tree.state.downcast_mut::<ToasterState>();

        match event {
            Event::Window(window::Event::RedrawRequested(now)) => {
                // Check if global state changed.
                if has_changed() {
                    reset_changed();
                    shell.invalidate_layout();
                    shell.request_redraw();
                }

                // Drive auto-close timers.
                let now_ms = now.elapsed().as_millis() as u64;

                let mut changed = false;
                with_toasts_mut(|toasts| {
                    for toast in toasts.iter_mut() {
                        if toast.dismissed || toast.removing {
                            continue;
                        }
                        if let Some(duration) = toast.duration {
                            let elapsed = now_ms.saturating_sub(toast.created_at_ms);
                            if elapsed >= duration {
                                toast.dismissed = true;
                                changed = true;
                            }
                        }
                    }
                });

                // Remove toasts that have been dismissing long enough.
                let removing_changed = with_toasts_mut(|toasts| {
                    let before = toasts.len();
                    toasts.retain(|t| {
                        if t.removing {
                            now_ms.saturating_sub(t.created_at_ms) < UNMOUNT_DELAY_MS
                        } else {
                            true
                        }
                    });
                    toasts.len() != before
                })
                .unwrap_or(false);

                if changed || removing_changed {
                    shell.invalidate_layout();
                }

                // Schedule next frame if there are active toasts.
                let active =
                    with_toasts(|toasts| toasts.iter().any(|t| !t.dismissed)).unwrap_or(false);
                if active {
                    shell.request_redraw_at(*now + FRAME_INTERVAL);
                }
            }
            _ => {}
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
        // Nothing drawn in the normal pass; all rendering is in the overlay.
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        _layout: layout::Layout<'b>,
        _renderer: &Renderer,
        viewport: &Rectangle,
        _translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let _state = tree.state.downcast_mut::<ToasterState>();

        Some(overlay::Element::new(Box::new(ToasterOverlay {
            theme: self.theme,
            position: self.position,
            gap: self.gap,
            offset: self.offset,
            max_visible: self.max_visible,
            rich_colors: self.rich_colors,
            invert: self.invert,
            close_button: self.close_button,
            expand: self.expand,
            viewport: *viewport,
        })))
    }
}

/// Widget tree state for the Toaster.
pub(super) struct ToasterState {
    /// Expanded state (hover expands the stack).
    pub expanded: bool,
}

impl ToasterState {
    fn new() -> Self {
        Self { expanded: false }
    }
}

/// Overlay that renders all active toasts at the configured screen position.
struct ToasterOverlay<'a> {
    theme: &'a ShadcnTheme,
    position: ToastPosition,
    gap: f32,
    offset: f32,
    max_visible: usize,
    rich_colors: bool,
    invert: bool,
    close_button: bool,
    expand: bool,
    viewport: Rectangle,
}

impl<Message> overlay::Overlay<Message, Theme, Renderer> for ToasterOverlay<'_> {
    fn layout(&mut self, _renderer: &Renderer, bounds: Size) -> layout::Node {
        layout::Node::new(bounds)
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
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if !cursor.is_over(bounds) {
                    return;
                }

                let cursor_pos = cursor.position().unwrap_or(Point::ORIGIN);
                let position = self.position;
                let gap = self.gap;
                let offset = self.offset;
                let max_visible = self.max_visible;

                with_toasts_mut(|toasts| {
                    let visible: Vec<_> = toasts
                        .iter()
                        .filter(|t| !t.dismissed)
                        .take(max_visible)
                        .collect();

                    for toast in visible.iter() {
                        let toast_bounds =
                            compute_toast_bounds(bounds, toast, &visible, position, gap, offset);

                        if !toast_bounds.contains(cursor_pos) {
                            continue;
                        }

                        // Check close button (top-right corner).
                        if toast.close_button || toast.dismissible {
                            let close_btn = Rectangle {
                                x: toast_bounds.x + toast_bounds.width - 28.0,
                                y: toast_bounds.y + 2.0,
                                width: 20.0,
                                height: 20.0,
                            };
                            if close_btn.contains(cursor_pos) {
                                let id = toast.id;
                                if let Some(t) = toasts.iter_mut().find(|t| t.id == id) {
                                    t.dismissed = true;
                                }
                                shell.capture_event();
                                return;
                            }
                        }
                    }
                });
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
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse::Interaction::default()
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: layout::Layout<'_>,
        _cursor: mouse::Cursor,
    ) {
        let bounds = layout.bounds();
        let is_dark = self.theme.is_dark();

        with_toasts(|toasts| {
            let visible: Vec<&RawToast> = toasts
                .iter()
                .filter(|t| !t.dismissed)
                .take(self.max_visible)
                .collect();

            for toast in visible.iter() {
                let toast_bounds = compute_toast_bounds(
                    bounds,
                    toast,
                    &visible,
                    self.position,
                    self.gap,
                    self.offset,
                );

                let style = if toast.invert || self.invert {
                    style::inverted_toast_style(is_dark)
                } else {
                    style::resolve_toast_style(self.theme, toast.toast_type, toast.rich_colors)
                };

                // Draw toast background with border and shadow.
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: toast_bounds,
                        border: Border {
                            color: style.border_color,
                            width: style.border_width,
                            radius: style.border_radius.into(),
                        },
                        shadow: style.shadow,
                        ..renderer::Quad::default()
                    },
                    style.background,
                );

                let content_x = toast_bounds.x + TOAST_PADDING;
                let content_y = toast_bounds.y + TOAST_PADDING;
                let _content_width = toast_bounds.width - TOAST_PADDING * 2.0;
                let mut text_x = content_x;

                // Draw icon based on toast type.
                let icon_size = 16.0;
                let icon_color = style.text;
                let icon_char: Option<&str> = match toast.toast_type {
                    ToastType::Success => Some("\u{2713}"),
                    ToastType::Error => Some("\u{2717}"),
                    ToastType::Warning => Some("\u{26A0}"),
                    ToastType::Info => Some("\u{2139}"),
                    ToastType::Loading => Some("\u{25CB}"),
                    ToastType::Default => None,
                };

                if let Some(icon) = icon_char {
                    draw_text(
                        renderer,
                        icon,
                        text_x,
                        content_y,
                        icon_size,
                        icon_color,
                        iced_font(self.theme.font_pack().sans),
                    );
                    text_x += icon_size + 6.0;
                }

                // Draw title text.
                draw_text(
                    renderer,
                    &toast.title,
                    text_x,
                    content_y,
                    style::TOAST_FONT_SIZE,
                    style.text,
                    iced_font(self.theme.font_pack().sans),
                );

                // Draw description if present.
                if let Some(ref desc) = toast.description {
                    let desc_color = if toast.rich_colors || self.rich_colors {
                        style.text
                    } else if is_dark {
                        crate::iced_compat::Color::from_rgba(0.91, 0.91, 0.91, 1.0)
                    } else {
                        crate::iced_compat::Color::from_rgba(0.25, 0.25, 0.25, 1.0)
                    };

                    draw_text(
                        renderer,
                        desc,
                        text_x,
                        content_y + 22.0,
                        style::TOAST_FONT_SIZE,
                        desc_color,
                        iced_font(self.theme.font_pack().sans),
                    );
                }

                // Draw close button if enabled.
                if toast.close_button || (toast.dismissible && self.close_button) {
                    let close_bounds = Rectangle {
                        x: toast_bounds.x + toast_bounds.width - 28.0,
                        y: toast_bounds.y + 2.0,
                        width: 20.0,
                        height: 20.0,
                    };

                    let close_style = style::close_button_style(is_dark);

                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: close_bounds,
                            border: Border {
                                color: close_style.border_color,
                                width: close_style.border_width,
                                radius: close_style.border_radius.into(),
                            },
                            ..renderer::Quad::default()
                        },
                        close_style.background,
                    );

                    draw_text(
                        renderer,
                        "\u{2715}",
                        close_bounds.x + 5.0,
                        close_bounds.y + 3.0,
                        10.0,
                        close_style.text,
                        iced_font(self.theme.font_pack().sans),
                    );
                }

                // Draw action button if present.
                if let Some(ref action_label) = toast.action_label {
                    let btn_style = style::action_button_style(is_dark);
                    let btn_width = (action_label.len() as f32 * 7.0 + 16.0).max(40.0);
                    let btn_height = 24.0;
                    let btn_bounds = Rectangle {
                        x: toast_bounds.x + toast_bounds.width - TOAST_PADDING - btn_width,
                        y: toast_bounds.y + toast_bounds.height - TOAST_PADDING - btn_height,
                        width: btn_width,
                        height: btn_height,
                    };

                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: btn_bounds,
                            border: Border {
                                color: btn_style.border_color,
                                width: btn_style.border_width,
                                radius: btn_style.border_radius.into(),
                            },
                            ..renderer::Quad::default()
                        },
                        btn_style.background,
                    );

                    draw_text(
                        renderer,
                        action_label,
                        btn_bounds.x + 8.0,
                        btn_bounds.y + 4.0,
                        12.0,
                        btn_style.text,
                        iced_font(self.theme.font_pack().sans),
                    );
                }

                // Draw cancel button if present.
                if let Some(ref cancel_label) = toast.cancel_label {
                    let btn_style = style::cancel_button_style(is_dark);
                    let btn_width = (cancel_label.len() as f32 * 7.0 + 16.0).max(40.0);
                    let btn_height = 24.0;
                    let action_width = toast
                        .action_label
                        .as_ref()
                        .map(|a| (a.len() as f32 * 7.0 + 16.0).max(40.0) + 8.0)
                        .unwrap_or(0.0);

                    let btn_bounds = Rectangle {
                        x: toast_bounds.x + toast_bounds.width
                            - TOAST_PADDING
                            - action_width
                            - btn_width,
                        y: toast_bounds.y + toast_bounds.height - TOAST_PADDING - btn_height,
                        width: btn_width,
                        height: btn_height,
                    };

                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: btn_bounds,
                            border: Border {
                                color: btn_style.border_color,
                                width: btn_style.border_width,
                                radius: btn_style.border_radius.into(),
                            },
                            ..renderer::Quad::default()
                        },
                        btn_style.background,
                    );

                    draw_text(
                        renderer,
                        cancel_label,
                        btn_bounds.x + 8.0,
                        btn_bounds.y + 4.0,
                        12.0,
                        btn_style.text,
                        iced_font(self.theme.font_pack().sans),
                    );
                }
            }
        });
    }
}

/// Draws text directly using the renderer without requiring a `Catalog`.
fn draw_text(
    renderer: &mut Renderer,
    content: &str,
    x: f32,
    y: f32,
    size: f32,
    color: crate::iced_compat::Color,
    font: crate::iced_compat::Font,
) {
    use iced_core::text::Renderer as TextRenderer;

    let text = iced_core::Text {
        content: content.to_string(),
        bounds: crate::iced_compat::Size::new(f32::INFINITY, f32::INFINITY),
        size: crate::iced_compat::Pixels(size),
        line_height: iced_core::widget::text::LineHeight::Absolute(crate::iced_compat::Pixels(
            size * 1.2,
        )),
        font,
        align_x: crate::iced_compat::alignment::Horizontal::Left.into(),
        align_y: crate::iced_compat::alignment::Vertical::Top.into(),
        shaping: iced_core::widget::text::Shaping::Advanced,
        wrapping: iced_core::widget::text::Wrapping::None,
    };

    let position = Point::new(x, y);
    let bounds = Rectangle {
        x,
        y,
        width: f32::INFINITY,
        height: f32::INFINITY,
    };

    renderer.fill_text(text, position, color, bounds);
}

/// Computes the bounding rectangle for a toast.
fn compute_toast_bounds(
    viewport: Rectangle,
    toast: &RawToast,
    all_visible: &[&RawToast],
    position: ToastPosition,
    gap: f32,
    offset: f32,
) -> Rectangle {
    let index = all_visible
        .iter()
        .position(|t| t.id == toast.id)
        .unwrap_or(0);
    let count = all_visible.len();

    let toast_height = estimate_toast_height(toast);
    let total_height: f32 = all_visible
        .iter()
        .take(count.min(MAX_VISIBLE_TOASTS))
        .map(|t| estimate_toast_height(t))
        .sum::<f32>()
        + gap * (count.min(MAX_VISIBLE_TOASTS) as f32 - 1.0);

    // Calculate vertical position.
    let y = match position {
        ToastPosition::TopLeft | ToastPosition::TopCenter | ToastPosition::TopRight => {
            viewport.y + offset + (index as f32 * (toast_height + gap))
        }
        ToastPosition::BottomLeft | ToastPosition::BottomCenter | ToastPosition::BottomRight => {
            viewport.y + viewport.height - offset - total_height
                + (index as f32 * (toast_height + gap))
        }
    };

    // Calculate horizontal position.
    let x = match position {
        ToastPosition::TopLeft | ToastPosition::BottomLeft => viewport.x + offset,
        ToastPosition::TopRight | ToastPosition::BottomRight => {
            viewport.x + viewport.width - offset - TOAST_WIDTH
        }
        ToastPosition::TopCenter | ToastPosition::BottomCenter => {
            viewport.x + (viewport.width - TOAST_WIDTH) / 2.0
        }
    };

    Rectangle {
        x,
        y,
        width: TOAST_WIDTH,
        height: toast_height,
    }
}

/// Estimates the height of a toast based on its content.
fn estimate_toast_height(toast: &RawToast) -> f32 {
    let mut height = TOAST_PADDING * 2.0;
    height += 20.0; // title line height

    if toast.description.is_some() {
        height += 22.0; // description + gap
    }

    if toast.action_label.is_some() || toast.cancel_label.is_some() {
        height += 32.0; // button row
    }

    height
}
