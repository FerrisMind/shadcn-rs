use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use iced::advanced::Renderer as _;
use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::text;
use iced::advanced::text::Renderer as _;
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::border::Border;
use iced::mouse;
use iced::touch;
use iced::window;
use iced::{
    Background, Color, Element, Event, Font, Length, Point, Rectangle, Shadow, Size, Vector,
};
use lucide_icons::Icon as LucideIcon;

use crate::theme::Theme;

const DEFAULT_TOAST_DURATION_MS: u64 = 5000;

static TOAST_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

fn next_toast_id() -> String {
    let id = TOAST_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("toast-{id}")
}

fn apply_opacity(mut color: Color, opacity: f32) -> Color {
    color.a *= opacity;
    color
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Error,
    Warning,
    Info,
    Loading,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,

    #[default]
    BottomRight,
}

impl ToastPosition {
    fn is_top(self) -> bool {
        matches!(
            self,
            ToastPosition::TopLeft | ToastPosition::TopCenter | ToastPosition::TopRight
        )
    }

    fn is_center(self) -> bool {
        matches!(self, ToastPosition::TopCenter | ToastPosition::BottomCenter)
    }

    fn is_left(self) -> bool {
        matches!(self, ToastPosition::TopLeft | ToastPosition::BottomLeft)
    }
}

#[derive(Clone, Debug)]
pub struct Toast {
    pub id: String,
    pub variant: ToastVariant,
    pub title: Option<String>,
    pub description: Option<String>,
    pub duration_ms: Option<u64>,
    pub dismissible: bool,
}

impl Toast {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: next_toast_id(),
            variant: ToastVariant::Default,
            title: Some(title.into()),
            description: None,
            duration_ms: Some(DEFAULT_TOAST_DURATION_MS),
            dismissible: true,
        }
    }

    pub fn with_id(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            variant: ToastVariant::Default,
            title: None,
            description: None,
            duration_ms: Some(DEFAULT_TOAST_DURATION_MS),
            dismissible: true,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn duration_ms(mut self, duration_ms: u64) -> Self {
        self.duration_ms = if duration_ms == 0 {
            None
        } else {
            Some(duration_ms)
        };
        self
    }

    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }
}

#[derive(Clone, Debug)]
pub struct ToastPromise {
    id: String,
}

impl ToastPromise {
    pub fn success(self, toaster: &Toaster, mut toast: Toast) -> String {
        toast.id = self.id.clone();
        toast.variant = ToastVariant::Success;
        toaster.show(toast)
    }

    pub fn error(self, toaster: &Toaster, mut toast: Toast) -> String {
        toast.id = self.id.clone();
        toast.variant = ToastVariant::Error;
        toaster.show(toast)
    }
}

#[derive(Clone, Debug)]
pub struct Toaster {
    state: Arc<Mutex<ToasterState>>,
}

impl Default for Toaster {
    fn default() -> Self {
        Self::new()
    }
}

impl Toaster {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ToasterState::default())),
        }
    }

    pub fn set_position(&self, position: ToastPosition) {
        if let Ok(mut state) = self.state.lock() {
            state.position = position;
        }
    }

    pub fn show(&self, mut toast: Toast) -> String {
        if toast.id.is_empty() {
            toast.id = next_toast_id();
        }

        let now = std::time::Instant::now();
        let toast_id = toast.id.clone();

        if let Ok(mut state) = self.state.lock() {
            if let Some(entry) = state
                .entries
                .iter_mut()
                .find(|entry| entry.toast.id == toast.id)
            {
                entry.toast = toast.clone();
                entry.created_at = now;
                entry.open = true;
                entry.dismissed_at = None;
                return entry.toast.id.clone();
            }

            state.entries.insert(0, ToastEntry::new(toast, now));
        }

        toast_id
    }

    pub fn dismiss(&self, toast_id: &str) {
        let now = std::time::Instant::now();
        if let Ok(mut state) = self.state.lock() {
            for entry in &mut state.entries {
                if entry.toast.id == toast_id {
                    entry.open = false;
                    entry.dismissed_at.get_or_insert(now);
                }
            }
        }
    }

    pub fn dismiss_all(&self) {
        let now = std::time::Instant::now();
        if let Ok(mut state) = self.state.lock() {
            for entry in &mut state.entries {
                entry.open = false;
                entry.dismissed_at.get_or_insert(now);
            }
        }
    }

    pub fn promise(&self, mut toast: Toast) -> ToastPromise {
        toast.variant = ToastVariant::Loading;
        toast.duration_ms = None;
        let id = self.show(toast);
        ToastPromise { id }
    }

    pub fn overlay<'a, Message: 'a>(
        &self,
        base: impl Into<Element<'a, Message>>,
        theme: &Theme,
    ) -> Element<'a, Message> {
        let base = base.into();
        let overlay: Element<'a, Message> = ToasterOverlay::new(self.clone(), theme.clone()).into();
        iced::widget::stack![base, overlay].into()
    }
}

#[derive(Clone, Debug)]
struct ToastEntry {
    toast: Toast,
    created_at: std::time::Instant,
    open: bool,
    dismissed_at: Option<std::time::Instant>,
}

impl ToastEntry {
    fn new(toast: Toast, now: std::time::Instant) -> Self {
        Self {
            toast,
            created_at: now,
            open: true,
            dismissed_at: None,
        }
    }
}

#[derive(Debug)]
struct ToasterState {
    entries: Vec<ToastEntry>,
    position: ToastPosition,
}

impl Default for ToasterState {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            position: ToastPosition::BottomRight,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ToastLayout {
    id: [u8; 24],
    id_len: usize,
    bounds: Rectangle,
    close_bounds: Rectangle,
    dismissible: bool,
}

fn id_to_small(id: &str) -> ([u8; 24], usize) {
    let mut buf = [0u8; 24];
    let bytes = id.as_bytes();
    let len = bytes.len().min(buf.len());
    buf[..len].copy_from_slice(&bytes[..len]);
    (buf, len)
}

fn small_to_string(buf: [u8; 24], len: usize) -> String {
    String::from_utf8_lossy(&buf[..len]).to_string()
}

#[derive(Debug, Default)]
struct ToasterOverlayState {
    last_redraw: Option<std::time::Instant>,
    layout: Vec<ToastLayout>,
}

struct ToasterOverlay {
    toaster: Toaster,
    theme: Theme,
}

impl ToasterOverlay {
    fn new(toaster: Toaster, theme: Theme) -> Self {
        Self { toaster, theme }
    }
}

impl<Message> Widget<Message, iced::Theme, iced::Renderer> for ToasterOverlay {
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<ToasterOverlayState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(ToasterOverlayState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, Length::Fill)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<ToasterOverlayState>();

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.last_redraw = Some(*now);

            if let Ok(mut toaster) = self.toaster.state.lock() {
                update_toasts(&mut toaster, *now, &self.theme);
                state.layout = compute_layout(
                    layout.bounds(),
                    &toaster.entries,
                    toaster.position,
                    *now,
                    &self.theme,
                )
                .layout;
            }
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if state.last_redraw.is_none() {
                    state.last_redraw = Some(std::time::Instant::now());
                }

                if let Some(pos) = cursor.position_in(*viewport) {
                    for layout in &state.layout {
                        if layout.dismissible
                            && (layout.close_bounds.contains(pos) || layout.bounds.contains(pos))
                        {
                            self.toaster
                                .dismiss(&small_to_string(layout.id, layout.id_len));
                            shell.capture_event();
                            break;
                        }
                    }
                }
            }
            _ => {}
        }

        if let Ok(toaster) = self.toaster.state.lock()
            && !toaster.entries.is_empty()
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        _layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<ToasterOverlayState>();
        if let Some(pos) = cursor.position_in(*viewport)
            && state
                .layout
                .iter()
                .any(|layout| layout.bounds.contains(pos))
        {
            return mouse::Interaction::Pointer;
        }

        mouse::Interaction::default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        _theme: &iced::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if !bounds.intersects(viewport) {
            return;
        }

        let state = tree.state.downcast_ref::<ToasterOverlayState>();
        let now = state.last_redraw.unwrap_or_else(std::time::Instant::now);
        let (entries, position) = match self.toaster.state.lock() {
            Ok(toaster) => (toaster.entries.clone(), toaster.position),
            Err(_) => return,
        };

        let layout = if state.layout.is_empty() && !entries.is_empty() {
            compute_layout(bounds, &entries, position, now, &self.theme).layout
        } else {
            state.layout.clone()
        };

        draw_toasts(
            renderer,
            &self.theme,
            &entries,
            &layout,
            cursor,
            viewport,
            now,
        );
    }
}

impl<'a, Message: 'a> From<ToasterOverlay> for Element<'a, Message> {
    fn from(widget: ToasterOverlay) -> Element<'a, Message> {
        Element::new(widget)
    }
}

fn update_toasts(state: &mut ToasterState, now: std::time::Instant, theme: &Theme) {
    let anim = std::time::Duration::from_millis(theme.styles.toast.animation_ms);

    for entry in &mut state.entries {
        if entry.open
            && let Some(duration_ms) = entry.toast.duration_ms
        {
            let duration = std::time::Duration::from_millis(duration_ms);
            if now.saturating_duration_since(entry.created_at) >= duration {
                entry.open = false;
                entry.dismissed_at.get_or_insert(now);
            }
        }
    }

    state.entries.retain(|entry| {
        if entry.open {
            return true;
        }
        match entry.dismissed_at {
            Some(dismissed) => now.saturating_duration_since(dismissed) <= anim,
            None => true,
        }
    });
}

struct LayoutResult {
    layout: Vec<ToastLayout>,
}

fn compute_layout(
    viewport: Rectangle,
    entries: &[ToastEntry],
    position: ToastPosition,
    now: std::time::Instant,
    theme: &Theme,
) -> LayoutResult {
    let toast_style = theme.styles.toast;
    let mut y = if position.is_top() {
        viewport.y + toast_style.margin
    } else {
        viewport.y + viewport.height - toast_style.margin
    };

    let x = if position.is_center() {
        viewport.x + (viewport.width - toast_style.width).max(0.0) / 2.0
    } else if position.is_left() {
        viewport.x + toast_style.margin
    } else {
        viewport.x + viewport.width - toast_style.margin - toast_style.width
    };

    let mut layout_out = Vec::with_capacity(entries.len());

    for entry in entries {
        let height = toast_style.height;
        let bounds = if position.is_top() {
            let bounds = Rectangle {
                x,
                y,
                width: toast_style.width,
                height,
            };
            y += height + toast_style.gap;
            bounds
        } else {
            y -= height;
            let bounds = Rectangle {
                x,
                y,
                width: toast_style.width,
                height,
            };
            y -= toast_style.gap;
            bounds
        };

        let (id, id_len) = id_to_small(&entry.toast.id);

        let close_bounds = Rectangle {
            x: bounds.x + bounds.width - toast_style.close_size - toast_style.close_inset,
            y: bounds.y + toast_style.close_inset,
            width: toast_style.close_size,
            height: toast_style.close_size,
        };

        let anim = std::time::Duration::from_millis(toast_style.animation_ms);
        let mut anim_t = 1.0;
        if entry.open {
            let elapsed = now.saturating_duration_since(entry.created_at);
            anim_t = (elapsed.as_secs_f32() / anim.as_secs_f32()).clamp(0.0, 1.0);
        } else if let Some(dismissed) = entry.dismissed_at {
            let elapsed = now.saturating_duration_since(dismissed);
            anim_t = 1.0 - (elapsed.as_secs_f32() / anim.as_secs_f32()).clamp(0.0, 1.0);
        }

        let slide = (1.0 - anim_t) * theme.spacing.md;
        let bounds = Rectangle {
            y: bounds.y + slide,
            ..bounds
        };
        let close_bounds = Rectangle {
            y: close_bounds.y + slide,
            ..close_bounds
        };

        layout_out.push(ToastLayout {
            id,
            id_len,
            bounds,
            close_bounds,
            dismissible: entry.toast.dismissible,
        });
    }

    LayoutResult { layout: layout_out }
}

fn variant_icon(variant: ToastVariant) -> Option<LucideIcon> {
    match variant {
        ToastVariant::Default => Some(LucideIcon::Bell),
        ToastVariant::Success => Some(LucideIcon::CircleCheck),
        ToastVariant::Error => Some(LucideIcon::OctagonX),
        ToastVariant::Warning => Some(LucideIcon::TriangleAlert),
        ToastVariant::Info => Some(LucideIcon::Info),
        ToastVariant::Loading => Some(LucideIcon::Loader),
    }
}

fn variant_color(variant: ToastVariant, theme: &Theme) -> Color {
    match variant {
        ToastVariant::Default => theme.palette.muted_foreground,
        ToastVariant::Success => theme.palette.chart_2,
        ToastVariant::Error => theme.palette.destructive,
        ToastVariant::Warning => theme.palette.chart_4,
        ToastVariant::Info => theme.palette.chart_1,
        ToastVariant::Loading => theme.palette.muted_foreground,
    }
}

fn draw_toasts(
    renderer: &mut iced::Renderer,
    theme: &Theme,
    entries: &[ToastEntry],
    layout: &[ToastLayout],
    cursor: mouse::Cursor,
    viewport: &Rectangle,
    now: std::time::Instant,
) {
    let font = renderer.default_font();
    let icon_font = Font::with_name("lucide");

    let text_color = theme.palette.popover_foreground;
    let background = Background::Color(theme.palette.popover);
    let border_color = theme.palette.border;
    let radius = theme.radius.md.max(0.0);

    for (entry, layout) in entries.iter().zip(layout.iter()) {
        let bounds = layout.bounds;
        if !bounds.intersects(viewport) {
            continue;
        }

        let anim = std::time::Duration::from_millis(theme.styles.toast.animation_ms);
        let mut alpha = 1.0;
        if entry.open {
            let elapsed = now.saturating_duration_since(entry.created_at);
            alpha = (elapsed.as_secs_f32() / anim.as_secs_f32()).clamp(0.0, 1.0);
        } else if let Some(dismissed) = entry.dismissed_at {
            let elapsed = now.saturating_duration_since(dismissed);
            alpha = 1.0 - (elapsed.as_secs_f32() / anim.as_secs_f32()).clamp(0.0, 1.0);
        }

        let style = renderer::Quad {
            bounds,
            border: Border {
                color: apply_opacity(border_color, alpha),
                width: theme.styles.menu.border_width,
                radius: radius.into(),
            },
            shadow: Shadow {
                color: apply_opacity(
                    theme.palette.foreground,
                    theme.styles.toast.shadow.opacity * alpha,
                ),
                offset: Vector::new(0.0, theme.styles.toast.shadow.offset_y),
                blur_radius: theme.styles.toast.shadow.blur_radius,
            },
            ..renderer::Quad::default()
        };

        renderer.fill_quad(
            style,
            Background::Color(apply_opacity(
                match background {
                    Background::Color(c) => c,
                    _ => theme.palette.popover,
                },
                alpha,
            )),
        );

        let icon_color = variant_color(entry.toast.variant, theme);
        if let Some(icon) = variant_icon(entry.toast.variant) {
            renderer.fill_text(
                text::Text {
                    content: char::from(icon).to_string(),
                    size: 18.0.into(),
                    line_height: text::LineHeight::Absolute(18.0.into()),
                    font: icon_font,
                    bounds: Size::new(bounds.width, bounds.height),
                    align_x: text::Alignment::Left,
                    align_y: iced::alignment::Vertical::Top,
                    shaping: text::Shaping::Basic,
                    wrapping: text::Wrapping::default(),
                },
                Point::new(bounds.x + 12.0, bounds.y + 12.0),
                apply_opacity(icon_color, alpha),
                *viewport,
            );
        }

        let title = entry.toast.title.as_deref().unwrap_or("");
        let description = entry.toast.description.as_deref().unwrap_or("");

        let text_x = bounds.x + 12.0 + 22.0;
        let text_width = (bounds.width - 12.0 - 12.0 - 22.0).max(0.0);

        renderer.fill_text(
            text::Text {
                content: title.to_string(),
                size: 14.0.into(),
                line_height: text::LineHeight::Absolute(20.0.into()),
                font,
                bounds: Size::new(text_width, 20.0),
                align_x: text::Alignment::Left,
                align_y: iced::alignment::Vertical::Top,
                shaping: text::Shaping::Basic,
                wrapping: text::Wrapping::default(),
            },
            Point::new(text_x, bounds.y + 12.0),
            apply_opacity(text_color, alpha),
            *viewport,
        );

        if !description.is_empty() {
            renderer.fill_text(
                text::Text {
                    content: description.to_string(),
                    size: 12.0.into(),
                    line_height: text::LineHeight::Absolute(16.0.into()),
                    font,
                    bounds: Size::new(text_width, 32.0),
                    align_x: text::Alignment::Left,
                    align_y: iced::alignment::Vertical::Top,
                    shaping: text::Shaping::Basic,
                    wrapping: text::Wrapping::Word,
                },
                Point::new(text_x, bounds.y + 34.0),
                apply_opacity(apply_opacity(text_color, 0.8), alpha),
                *viewport,
            );
        }

        if entry.toast.dismissible {
            let close = layout.close_bounds;
            let is_hovered = cursor
                .position_in(*viewport)
                .is_some_and(|pos| close.contains(pos));

            let icon = LucideIcon::X;
            let close_color = if is_hovered {
                apply_opacity(text_color, 0.95 * alpha)
            } else {
                apply_opacity(text_color, 0.75 * alpha)
            };

            renderer.fill_text(
                text::Text {
                    content: char::from(icon).to_string(),
                    size: theme.styles.toast.close_size.into(),
                    line_height: text::LineHeight::Absolute(theme.styles.toast.close_size.into()),
                    font: icon_font,
                    bounds: Size::new(close.width, close.height),
                    align_x: text::Alignment::Left,
                    align_y: iced::alignment::Vertical::Top,
                    shaping: text::Shaping::Basic,
                    wrapping: text::Wrapping::default(),
                },
                Point::new(
                    close.x + theme.styles.toast.close_glyph_nudge_x,
                    close.y + theme.styles.toast.close_glyph_nudge_y,
                ),
                close_color,
                *viewport,
            );
        }
    }
}
