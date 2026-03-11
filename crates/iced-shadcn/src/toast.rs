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
const TOAST_CLOSE_RESERVED_WIDTH: f32 = 28.0;

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
    pub expandable: bool,
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
            expandable: true,
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
            expandable: true,
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

    pub fn expandable(mut self, expandable: bool) -> Self {
        self.expandable = expandable;
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

        let now = iced::time::Instant::now();
        let toast_id = toast.id.clone();

        if let Ok(mut state) = self.state.lock() {
            if let Some(entry) = state
                .entries
                .iter_mut()
                .find(|entry| entry.toast.id == toast.id)
            {
                entry.toast = toast.clone();
                entry.created_at = now;
                entry.paused_at = None;
                entry.paused_total = iced::time::Duration::ZERO;
                if !entry.toast.expandable {
                    entry.expanded = false;
                }
                entry.open = true;
                entry.dismissed_at = None;
                return entry.toast.id.clone();
            }

            state.entries.insert(0, ToastEntry::new(toast, now));
        }

        toast_id
    }

    pub fn dismiss(&self, toast_id: &str) {
        let now = iced::time::Instant::now();
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
        let now = iced::time::Instant::now();
        if let Ok(mut state) = self.state.lock() {
            for entry in &mut state.entries {
                entry.open = false;
                entry.dismissed_at.get_or_insert(now);
            }
        }
    }

    pub fn toggle_expanded(&self, toast_id: &str) {
        if let Ok(mut state) = self.state.lock() {
            for entry in &mut state.entries {
                if entry.toast.id == toast_id {
                    if entry.toast.expandable {
                        entry.expanded = !entry.expanded;
                    }
                    break;
                }
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
    created_at: iced::time::Instant,
    paused_at: Option<iced::time::Instant>,
    paused_total: iced::time::Duration,
    expanded: bool,
    open: bool,
    dismissed_at: Option<iced::time::Instant>,
}

impl ToastEntry {
    fn new(toast: Toast, now: iced::time::Instant) -> Self {
        Self {
            toast,
            created_at: now,
            paused_at: None,
            paused_total: iced::time::Duration::ZERO,
            expanded: false,
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
    toggle_bounds: Rectangle,
    close_bounds: Rectangle,
    expandable: bool,
    expanded: bool,
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

#[derive(Debug)]
struct ToasterOverlayState {
    last_redraw: Option<iced::time::Instant>,
    layout: Vec<ToastLayout>,
    window_focused: bool,
}

impl Default for ToasterOverlayState {
    fn default() -> Self {
        Self {
            last_redraw: None,
            layout: Vec::new(),
            window_focused: true,
        }
    }
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

        match event {
            Event::Window(window::Event::Focused) => {
                state.window_focused = true;
            }
            Event::Window(window::Event::Unfocused) => {
                state.window_focused = false;
            }
            _ => {}
        }

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.last_redraw = Some(*now);

            if let Ok(mut toaster) = self.toaster.state.lock() {
                let preview = compute_layout(
                    layout.bounds(),
                    &toaster.entries,
                    toaster.position,
                    *now,
                    &self.theme,
                );

                let hovered_toast = cursor.position_in(*viewport).and_then(|pos| {
                    preview
                        .layout
                        .iter()
                        .find(|item| item.bounds.contains(pos))
                        .map(|item| small_to_string(item.id, item.id_len))
                });
                let visible_toasts = preview
                    .layout
                    .iter()
                    .map(|item| small_to_string(item.id, item.id_len))
                    .collect::<Vec<_>>();

                update_toasts(
                    &mut toaster,
                    *now,
                    &self.theme,
                    &visible_toasts,
                    hovered_toast.as_deref(),
                    state.window_focused,
                );

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
                    state.last_redraw = Some(iced::time::Instant::now());
                }

                if let Some(pos) = cursor.position_in(*viewport) {
                    for layout in &state.layout {
                        if layout.expandable && layout.toggle_bounds.contains(pos) {
                            self.toaster
                                .toggle_expanded(&small_to_string(layout.id, layout.id_len));
                            shell.capture_event();
                            break;
                        }
                        if layout.dismissible && layout.close_bounds.contains(pos) {
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
        let now = state.last_redraw.unwrap_or_else(iced::time::Instant::now);
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

fn update_toasts(
    state: &mut ToasterState,
    now: iced::time::Instant,
    theme: &Theme,
    visible_toasts: &[String],
    hovered_toast: Option<&str>,
    window_focused: bool,
) {
    let anim = iced::time::Duration::from_millis(theme.styles.toast.animation_ms);

    for entry in &mut state.entries {
        let is_visible = visible_toasts.iter().any(|id| id == &entry.toast.id);
        let is_hovered = hovered_toast.is_some_and(|id| id == entry.toast.id);
        let should_pause = is_hovered || !window_focused;

        if should_pause {
            if entry.paused_at.is_none() {
                entry.paused_at = Some(now);
            }
        } else if let Some(paused_at) = entry.paused_at.take() {
            entry.paused_total += now.saturating_duration_since(paused_at);
        }

        if entry.open
            && is_visible
            && window_focused
            && entry.paused_at.is_none()
            && let Some(duration_ms) = entry.toast.duration_ms
        {
            let duration = iced::time::Duration::from_millis(duration_ms);
            let elapsed = now
                .saturating_duration_since(entry.created_at)
                .saturating_sub(entry.paused_total);
            if elapsed >= duration {
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

fn estimate_max_chars_per_line(text_width: f32, font_size: f32) -> usize {
    let avg_char_width = (font_size * 0.55).max(1.0);
    (text_width / avg_char_width).floor().max(1.0) as usize
}

fn estimate_wrapped_lines(text: &str, max_chars_per_line: usize) -> usize {
    if text.is_empty() {
        return 0;
    }

    let max_chars = max_chars_per_line.max(1);
    let mut total_lines = 0usize;

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            total_lines += 1;
            continue;
        }

        let mut lines = 1usize;
        let mut current = 0usize;

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
                continue;
            }

            if current + 1 + word_len <= max_chars {
                current += 1 + word_len;
            } else {
                lines += 1;
                if word_len <= max_chars {
                    current = word_len;
                } else {
                    lines += (word_len - 1) / max_chars;
                    current = word_len % max_chars;
                    if current == 0 {
                        current = max_chars;
                    }
                }
            }
        }

        total_lines += lines;
    }

    total_lines.max(1)
}

fn estimate_toast_width(
    title: &str,
    description: &str,
    min_width: f32,
    max_width: f32,
    text_chrome_width: f32,
) -> f32 {
    let title_chars = title.chars().count().min(96);
    let desc_chars = description.chars().count().min(120);
    let max_chars = title_chars.max(desc_chars) as f32;
    let estimated_text_width = max_chars * 6.8;
    (text_chrome_width + estimated_text_width).clamp(min_width, max_width)
}

fn truncate_for_single_line(text: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let mut chars = text.chars().collect::<Vec<_>>();
    if chars.len() <= max_chars {
        return text.to_string();
    }

    let keep = max_chars.saturating_sub(1);
    chars.truncate(keep);
    let mut out = chars.into_iter().collect::<String>();
    out.push('…');
    out
}

fn compute_layout(
    viewport: Rectangle,
    entries: &[ToastEntry],
    position: ToastPosition,
    now: iced::time::Instant,
    theme: &Theme,
) -> LayoutResult {
    let toast_style = theme.styles.toast;
    let horizontal_margin =
        if viewport.width < toast_style.max_width + toast_style.horizontal_margin * 2.0 {
            toast_style.narrow_viewport_padding
        } else {
            toast_style.horizontal_margin
        };
    let max_width = toast_style
        .max_width
        .min((viewport.width - horizontal_margin * 2.0).max(180.0));
    let min_width = 220.0_f32.min(max_width);
    let base_width = toast_style.width.clamp(min_width, max_width);
    let max_stack_height = (viewport.height * toast_style.max_viewport_height_ratio).max(0.0);
    let mut used_stack_height = 0.0;

    const LEFT_PADDING: f32 = 12.0;
    const RIGHT_PADDING_BASE: f32 = 12.0;
    const ICON_WITH_GAP: f32 = 22.0;
    const TOP_PADDING: f32 = 12.0;
    const TITLE_LINE_HEIGHT: f32 = 20.0;
    const TITLE_DESC_GAP: f32 = 4.0;
    const DESCRIPTION_LINE_HEIGHT: f32 = 16.0;
    const BOTTOM_PADDING: f32 = 12.0;

    let mut y = if position.is_top() {
        viewport.y + toast_style.vertical_margin
    } else {
        viewport.y + viewport.height - toast_style.vertical_margin
    };

    let mut layout_out = Vec::with_capacity(entries.len());

    for (shown, entry) in entries.iter().enumerate() {
        if shown >= toast_style.max_visible {
            break;
        }

        let title = entry.toast.title.as_deref().unwrap_or("");
        let description = entry.toast.description.as_deref().unwrap_or("");
        let controls_reserved_width = RIGHT_PADDING_BASE
            + if entry.toast.dismissible {
                TOAST_CLOSE_RESERVED_WIDTH
            } else {
                0.0
            }
            + if entry.toast.expandable {
                TOAST_CLOSE_RESERVED_WIDTH
            } else {
                0.0
            };
        let text_chrome_width = LEFT_PADDING + controls_reserved_width + ICON_WITH_GAP;
        let width =
            estimate_toast_width(title, description, min_width, max_width, text_chrome_width)
                .max(base_width.min(max_width));
        let text_width = (width - text_chrome_width).max(1.0);
        let title_lines = if title.is_empty() {
            0
        } else if entry.expanded {
            let max_chars = estimate_max_chars_per_line(text_width, 14.0);
            estimate_wrapped_lines(title, max_chars)
        } else {
            1
        };
        let description_lines = if description.is_empty() {
            0
        } else if entry.expanded {
            let max_chars = estimate_max_chars_per_line(text_width, 12.0);
            estimate_wrapped_lines(description, max_chars)
        } else {
            1
        };

        let title_block = TITLE_LINE_HEIGHT * title_lines as f32;
        let description_block = DESCRIPTION_LINE_HEIGHT * description_lines as f32;
        let content_height = TOP_PADDING
            + title_block
            + if description_lines > 0 && title_lines > 0 {
                TITLE_DESC_GAP
            } else {
                0.0
            }
            + description_block
            + BOTTOM_PADDING;

        let height = toast_style.height.max(content_height);
        let next_stack_height = if shown == 0 {
            height
        } else {
            used_stack_height + toast_style.gap + height
        };
        if shown > 0 && next_stack_height > max_stack_height {
            break;
        }

        let x = if position.is_center() {
            viewport.x + (viewport.width - width).max(0.0) / 2.0
        } else if position.is_left() {
            viewport.x + horizontal_margin
        } else {
            viewport.x + viewport.width - horizontal_margin - width
        };

        let bounds = if position.is_top() {
            let bounds = Rectangle {
                x,
                y,
                width,
                height,
            };
            y += height + toast_style.gap;
            bounds
        } else {
            y -= height;
            let bounds = Rectangle {
                x,
                y,
                width,
                height,
            };
            y -= toast_style.gap;
            bounds
        };

        let (id, id_len) = id_to_small(&entry.toast.id);

        let mut control_right = bounds.x + bounds.width - toast_style.close_inset;
        let close_bounds = if entry.toast.dismissible {
            control_right -= toast_style.close_size;
            let rect = Rectangle {
                x: control_right,
                y: bounds.y + toast_style.close_inset,
                width: toast_style.close_size,
                height: toast_style.close_size,
            };
            control_right -= 10.0;
            rect
        } else {
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            }
        };
        let toggle_bounds = if entry.toast.expandable {
            control_right -= toast_style.close_size;
            Rectangle {
                x: control_right,
                y: bounds.y + toast_style.close_inset,
                width: toast_style.close_size,
                height: toast_style.close_size,
            }
        } else {
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            }
        };

        let anim = iced::time::Duration::from_millis(toast_style.animation_ms);
        let mut anim_t = 1.0;
        if entry.open {
            let elapsed = now.saturating_duration_since(entry.created_at);
            let raw_t = (elapsed.as_secs_f32() / anim.as_secs_f32()).clamp(0.0, 1.0);
            anim_t = ease_out_cubic(raw_t);
        } else if let Some(dismissed) = entry.dismissed_at {
            let elapsed = now.saturating_duration_since(dismissed);
            anim_t = 1.0 - (elapsed.as_secs_f32() / anim.as_secs_f32()).clamp(0.0, 1.0);
        }

        let slide = (1.0 - anim_t) * bounds.height;
        let bounds = Rectangle {
            y: bounds.y + slide,
            ..bounds
        };
        let close_bounds = Rectangle {
            y: close_bounds.y + slide,
            ..close_bounds
        };
        let toggle_bounds = Rectangle {
            y: toggle_bounds.y + slide,
            ..toggle_bounds
        };

        layout_out.push(ToastLayout {
            id,
            id_len,
            bounds,
            toggle_bounds,
            close_bounds,
            expandable: entry.toast.expandable,
            expanded: entry.expanded,
            dismissible: entry.toast.dismissible,
        });
        used_stack_height = next_stack_height;
    }

    LayoutResult { layout: layout_out }
}

fn ease_out_cubic(t: f32) -> f32 {
    let one_minus = 1.0 - t.clamp(0.0, 1.0);
    1.0 - one_minus * one_minus * one_minus
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
    now: iced::time::Instant,
) {
    for (entry, layout) in entries.iter().zip(layout.iter()) {
        draw_toast(renderer, theme, entry, layout, cursor, viewport, now);
    }
}

fn draw_toast(
    renderer: &mut iced::Renderer,
    theme: &Theme,
    entry: &ToastEntry,
    layout: &ToastLayout,
    cursor: mouse::Cursor,
    viewport: &Rectangle,
    now: iced::time::Instant,
) {
    let font = renderer.default_font();
    let icon_font = Font::with_name("lucide");

    let text_color = theme.palette.popover_foreground;
    let background = Background::Color(theme.palette.popover);
    let border_color = theme.palette.border;
    let radius = theme.radius.md.max(0.0);

    let bounds = layout.bounds;
    if !bounds.intersects(viewport) {
        return;
    }

    let anim = iced::time::Duration::from_millis(theme.styles.toast.animation_ms);
    let mut alpha = 1.0;
    if entry.open {
        let elapsed = now.saturating_duration_since(entry.created_at);
        let raw_t = (elapsed.as_secs_f32() / anim.as_secs_f32()).clamp(0.0, 1.0);
        alpha = ease_out_cubic(raw_t);
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
    let controls_reserved_width =
        12.0 + if entry.toast.dismissible {
            TOAST_CLOSE_RESERVED_WIDTH
        } else {
            0.0
        } + if entry.toast.expandable {
            TOAST_CLOSE_RESERVED_WIDTH
        } else {
            0.0
        };
    let text_width = (bounds.width - 12.0 - controls_reserved_width - 22.0).max(0.0);
    let title_text = if entry.expanded {
        title.to_string()
    } else {
        let title_max_chars = estimate_max_chars_per_line(text_width, 14.0);
        truncate_for_single_line(title, title_max_chars)
    };
    let title_wrapping = if entry.expanded {
        text::Wrapping::Word
    } else {
        text::Wrapping::None
    };
    let title_lines = if title.is_empty() {
        0
    } else if entry.expanded {
        let max_chars = estimate_max_chars_per_line(text_width, 14.0);
        estimate_wrapped_lines(title, max_chars)
    } else {
        1
    };
    let title_height = (20.0 * title_lines as f32).max(20.0);

    renderer.fill_text(
        text::Text {
            content: title_text,
            size: 14.0.into(),
            line_height: text::LineHeight::Absolute(20.0.into()),
            font,
            bounds: Size::new(text_width, title_height),
            align_x: text::Alignment::Left,
            align_y: iced::alignment::Vertical::Top,
            shaping: text::Shaping::Basic,
            wrapping: title_wrapping,
        },
        Point::new(text_x, bounds.y + 12.0),
        apply_opacity(text_color, alpha),
        *viewport,
    );

    if !description.is_empty() {
        let description_y = bounds.y + 12.0 + title_height + 4.0;
        let description_height = (bounds.height - (description_y - bounds.y) - 12.0).max(0.0);
        let description_text = if entry.expanded {
            description.to_string()
        } else {
            let desc_max_chars = estimate_max_chars_per_line(text_width, 12.0);
            truncate_for_single_line(description, desc_max_chars)
        };
        let description_wrapping = if entry.expanded {
            text::Wrapping::Word
        } else {
            text::Wrapping::None
        };
        renderer.fill_text(
            text::Text {
                content: description_text,
                size: 12.0.into(),
                line_height: text::LineHeight::Absolute(16.0.into()),
                font,
                bounds: Size::new(text_width, description_height),
                align_x: text::Alignment::Left,
                align_y: iced::alignment::Vertical::Top,
                shaping: text::Shaping::Basic,
                wrapping: description_wrapping,
            },
            Point::new(text_x, description_y),
            apply_opacity(apply_opacity(text_color, 0.8), alpha),
            *viewport,
        );
    }

    if layout.expandable {
        let toggle = layout.toggle_bounds;
        let toggle_hovered = cursor
            .position_in(*viewport)
            .is_some_and(|pos| toggle.contains(pos));
        let icon = if layout.expanded {
            LucideIcon::ChevronUp
        } else {
            LucideIcon::ChevronDown
        };
        let toggle_color = if toggle_hovered {
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
                bounds: Size::new(toggle.width, toggle.height),
                align_x: text::Alignment::Left,
                align_y: iced::alignment::Vertical::Top,
                shaping: text::Shaping::Basic,
                wrapping: text::Wrapping::default(),
            },
            Point::new(
                toggle.x + theme.styles.toast.close_glyph_nudge_x,
                toggle.y + theme.styles.toast.close_glyph_nudge_y,
            ),
            toggle_color,
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
