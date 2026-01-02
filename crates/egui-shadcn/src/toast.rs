//! Toast/Sonner component - transient notifications with variants and positions.
//!
//! Make sure the Lucide font is loaded if you want the icon glyphs to render properly.

use crate::spinner::{SpinnerProps, SpinnerSize, SpinnerVariant, spinner};
use crate::theme::Theme;
use crate::tokens::ease_out_cubic;
use egui::{
    Area, Color32, Context, CornerRadius, Direction, Frame, Id, Layout, Order, RichText, Stroke,
    Ui, Vec2, pos2, vec2,
};
use lucide_icons::Icon;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

const DEFAULT_TOAST_DURATION_MS: u64 = 5000;
const DEFAULT_TOAST_HEIGHT: f32 = 64.0;
const DEFAULT_TOAST_WIDTH: f32 = 360.0;
const TOASTER_STATE_KEY: &str = "egui_shadcn_toaster_state";
static TOAST_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

fn next_toast_id() -> String {
    let id = TOAST_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("toast-{id}")
}

// =============================================================================
// ToastVariant
// =============================================================================

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

// =============================================================================
// ToastPosition
// =============================================================================

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

// =============================================================================
// Toast
// =============================================================================

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

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_variant(mut self, variant: ToastVariant) -> Self {
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

// =============================================================================
// ToastPromise
// =============================================================================

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

// =============================================================================
// Toaster
// =============================================================================

#[derive(Clone, Debug)]
pub struct Toaster {
    ctx: Context,
    state_id: Id,
}

impl Toaster {
    pub fn get_or_init(ctx: &Context) -> Self {
        let state_id = Id::new(TOASTER_STATE_KEY);
        ctx.data_mut(|data| {
            if data.get_temp::<ToasterState>(state_id).is_none() {
                data.insert_temp(state_id, ToasterState::default());
            }
        });
        Self {
            ctx: ctx.clone(),
            state_id,
        }
    }

    pub fn set_position(&self, position: ToastPosition) {
        self.with_state(|state| {
            state.position = position;
        });
    }

    pub fn show(&self, mut toast: Toast) -> String {
        if toast.id.is_empty() {
            toast.id = next_toast_id();
        }

        let now = self.ctx.input(|i| i.time);
        let mut result_id = toast.id.clone();
        self.with_state(|state| {
            if let Some(entry) = state
                .toasts
                .iter_mut()
                .find(|entry| entry.toast.id == toast.id)
            {
                entry.toast = toast;
                entry.created_at = now;
                entry.open = true;
                entry.last_size.y = entry.last_size.y.max(DEFAULT_TOAST_HEIGHT);
                result_id = entry.toast.id.clone();
                return;
            }

            let entry = ToastEntry::new(toast, now);
            result_id = entry.toast.id.clone();
            state.toasts.insert(0, entry);
        });

        result_id
    }

    pub fn dismiss(&self, toast_id: &str) {
        self.with_state(|state| {
            for entry in state.toasts.iter_mut() {
                if entry.toast.id == toast_id {
                    entry.open = false;
                }
            }
        });
    }

    pub fn dismiss_all(&self) {
        self.with_state(|state| {
            for entry in state.toasts.iter_mut() {
                entry.open = false;
            }
        });
    }

    pub fn promise(&self, mut toast: Toast) -> ToastPromise {
        toast.variant = ToastVariant::Loading;
        toast.duration_ms = None;
        let id = self.show(toast);
        ToastPromise { id }
    }

    pub fn render(&self, ui: &mut Ui, theme: &Theme) {
        let ctx = ui.ctx();
        let now = ctx.input(|i| i.time);
        let tokens = toast_tokens(theme);
        let anim_duration = theme.motion.base_ms / 1000.0;
        let mut next_repaint: Option<Duration> = None;

        let mut state = ctx.data(|data| {
            data.get_temp::<ToasterState>(self.state_id)
                .unwrap_or_default()
        });

        if state.toasts.is_empty() {
            return;
        }

        let screen = ctx.available_rect();
        let available_width = (screen.width() - tokens.margin * 2.0).max(0.0);
        let width = if available_width < tokens.min_width {
            available_width
        } else {
            available_width.min(tokens.max_width)
        };

        let mut y_offset = 0.0;
        for entry in state.toasts.iter_mut() {
            if entry.open
                && let Some(duration_ms) = entry.toast.duration_ms
                && duration_ms > 0
            {
                let elapsed_ms = ((now - entry.created_at).max(0.0) * 1000.0) as u64;
                if elapsed_ms >= duration_ms {
                    entry.open = false;
                } else {
                    let remaining = duration_ms.saturating_sub(elapsed_ms);
                    next_repaint = Some(match next_repaint {
                        Some(current) => current.min(Duration::from_millis(remaining)),
                        None => Duration::from_millis(remaining),
                    });
                }
            }

            let anim_t = ctx.animate_bool_with_time_and_easing(
                entry.anim_id(),
                entry.open,
                anim_duration,
                ease_out_cubic,
            );
            if anim_t <= 0.0 && !entry.open {
                continue;
            }

            let entry_height = entry.last_size.y.max(tokens.default_height).max(1.0);
            let x = if state.position.is_center() {
                screen.center().x - width * 0.5
            } else if state.position.is_left() {
                screen.left() + tokens.margin
            } else {
                screen.right() - width - tokens.margin
            };
            let y = if state.position.is_top() {
                screen.top() + tokens.margin + y_offset
            } else {
                screen.bottom() - tokens.margin - y_offset - entry_height
            };

            let slide_dir = if state.position.is_top() {
                vec2(0.0, -1.0)
            } else {
                vec2(0.0, 1.0)
            };
            let slide_offset = slide_dir * tokens.slide_distance * (1.0 - anim_t);
            let pos = pos2(x, y) + slide_offset;

            let area = Area::new(entry.area_id())
                .order(Order::Foreground)
                .fixed_pos(pos)
                .movable(false)
                .interactable(true);

            let response = area.show(ctx, |toast_ui| {
                toast_ui.set_min_width(width);
                toast_ui.set_max_width(width);
                let frame = Frame::popup(toast_ui.style())
                    .fill(fade_color(tokens.bg, anim_t))
                    .stroke(Stroke::new(1.0, fade_color(tokens.border, anim_t)))
                    .corner_radius(tokens.rounding)
                    .inner_margin(tokens.padding);
                frame
                    .show(toast_ui, |content_ui| {
                        render_toast_content(content_ui, theme, &entry.toast, &tokens, anim_t)
                    })
                    .inner
            });

            entry.last_size = response.response.rect.size();
            entry.last_size.x = width.max(1.0);
            if response.inner {
                entry.open = false;
            }

            y_offset += entry.last_size.y + tokens.stack_gap;
        }

        state.toasts.retain(|entry| {
            let t = ctx.animate_bool_with_time_and_easing(
                entry.anim_id(),
                entry.open,
                anim_duration,
                ease_out_cubic,
            );
            entry.open || t > 0.0
        });

        ctx.data_mut(|data| data.insert_temp(self.state_id, state));

        if let Some(delay) = next_repaint {
            ctx.request_repaint_after(delay);
        }
    }

    fn with_state<R>(&self, f: impl FnOnce(&mut ToasterState) -> R) -> R {
        self.ctx.data_mut(|data| {
            let mut state = data
                .get_temp::<ToasterState>(self.state_id)
                .unwrap_or_default();
            let result = f(&mut state);
            data.insert_temp(self.state_id, state);
            result
        })
    }
}

// =============================================================================
// Internal state
// =============================================================================

#[derive(Clone, Debug)]
struct ToastEntry {
    toast: Toast,
    created_at: f64,
    open: bool,
    last_size: Vec2,
}

impl ToastEntry {
    fn new(toast: Toast, now: f64) -> Self {
        Self {
            toast,
            created_at: now,
            open: true,
            last_size: Vec2::new(DEFAULT_TOAST_WIDTH, DEFAULT_TOAST_HEIGHT),
        }
    }

    fn area_id(&self) -> Id {
        Id::new("egui_shadcn_toast_area").with(self.toast.id.clone())
    }

    fn anim_id(&self) -> Id {
        Id::new("egui_shadcn_toast_anim").with(self.toast.id.clone())
    }
}

#[derive(Clone, Debug)]
struct ToasterState {
    toasts: Vec<ToastEntry>,
    position: ToastPosition,
}

impl Default for ToasterState {
    fn default() -> Self {
        Self {
            toasts: Vec::new(),
            position: ToastPosition::BottomRight,
        }
    }
}

// =============================================================================
// Styling helpers
// =============================================================================

struct ToastTokens {
    bg: Color32,
    border: Color32,
    text: Color32,
    text_muted: Color32,
    padding: egui::Margin,
    rounding: CornerRadius,
    icon_size: f32,
    close_size: f32,
    gap: f32,
    stack_gap: f32,
    margin: f32,
    min_width: f32,
    max_width: f32,
    default_height: f32,
    slide_distance: f32,
}

fn toast_tokens(theme: &Theme) -> ToastTokens {
    ToastTokens {
        bg: theme.palette.popover,
        border: theme.palette.border,
        text: theme.palette.popover_foreground,
        text_muted: theme.palette.muted_foreground,
        padding: egui::Margin::symmetric(12, 10),
        rounding: CornerRadius::same(theme.radius.r3.round() as u8),
        icon_size: 16.0,
        close_size: 14.0,
        gap: 12.0,
        stack_gap: 8.0,
        margin: 16.0,
        min_width: 240.0,
        max_width: 420.0,
        default_height: DEFAULT_TOAST_HEIGHT,
        slide_distance: 12.0,
    }
}

fn render_toast_content(
    ui: &mut Ui,
    theme: &Theme,
    toast: &Toast,
    tokens: &ToastTokens,
    alpha: f32,
) -> bool {
    let mut close_clicked = false;
    let text_color = fade_color(tokens.text, alpha);
    let muted_color = fade_color(tokens.text_muted, alpha);
    let (icon, icon_color) = toast_icon(theme, toast.variant);
    let icon_color = fade_color(icon_color, alpha);

    let icon_side = Vec2::splat(tokens.icon_size + 4.0);

    ui.spacing_mut().item_spacing.x = tokens.gap;
    ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
        if toast.variant == ToastVariant::Loading {
            let spinner_props = SpinnerProps::default()
                .with_size(SpinnerSize::Size2)
                .with_variant(SpinnerVariant::LucideLoaderCircle)
                .with_color(icon_color)
                .with_opacity(1.0);
            ui.allocate_ui_with_layout(
                icon_side,
                Layout::centered_and_justified(Direction::TopDown),
                |icon_ui| {
                    spinner(icon_ui, theme, spinner_props);
                },
            );
        } else {
            ui.add_sized(
                icon_side,
                egui::Label::new(
                    RichText::new(icon.unicode())
                        .size(tokens.icon_size)
                        .color(icon_color),
                ),
            );
        }

        ui.vertical(|col| {
            col.spacing_mut().item_spacing.y = 4.0;
            if let Some(title) = &toast.title {
                col.label(RichText::new(title).size(14.0).strong().color(text_color));
            }
            if let Some(description) = &toast.description {
                col.label(RichText::new(description).size(12.0).color(muted_color));
            }
        });

        if toast.dismissible {
            let close = egui::Button::new(
                RichText::new(Icon::X.unicode())
                    .size(tokens.close_size)
                    .color(muted_color),
            )
            .frame(false);
            if ui.add(close).clicked() {
                close_clicked = true;
            }
        }
    });

    close_clicked
}

fn toast_icon(theme: &Theme, variant: ToastVariant) -> (Icon, Color32) {
    match variant {
        ToastVariant::Default => (Icon::Bell, theme.palette.foreground),
        ToastVariant::Success => (Icon::CircleCheck, Color32::from_rgb(34, 197, 94)),
        ToastVariant::Error => (Icon::OctagonX, theme.palette.destructive),
        ToastVariant::Warning => (Icon::TriangleAlert, Color32::from_rgb(245, 158, 11)),
        ToastVariant::Info => (Icon::Info, Color32::from_rgb(59, 130, 246)),
        ToastVariant::Loading => (Icon::Loader, theme.palette.muted_foreground),
    }
}

fn fade_color(color: Color32, t: f32) -> Color32 {
    let alpha = (color.a() as f32 * t.clamp(0.0, 1.0)).round() as u8;
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}
