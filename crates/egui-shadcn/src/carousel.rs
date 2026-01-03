use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::theme::Theme;
use egui::{Align, Color32, Id, Key, Layout, Response, Sense, Stroke, Ui, Vec2};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug)]
pub struct CarouselOptions {
    pub autoplay: bool,
    pub looped: bool,
    pub autoplay_delay_ms: f32,
}

impl Default for CarouselOptions {
    fn default() -> Self {
        Self {
            autoplay: false,
            looped: false,
            autoplay_delay_ms: 3000.0,
        }
    }
}

impl CarouselOptions {
    pub fn autoplay(mut self, autoplay: bool) -> Self {
        self.autoplay = autoplay;
        self
    }

    pub fn looped(mut self, looped: bool) -> Self {
        self.looped = looped;
        self
    }

    pub fn autoplay_delay_ms(mut self, delay_ms: f32) -> Self {
        self.autoplay_delay_ms = delay_ms;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CarouselProps {
    pub id_source: Id,
    pub orientation: CarouselOrientation,
    pub opts: CarouselOptions,
}

impl CarouselProps {
    pub fn new(id_source: Id) -> Self {
        Self {
            id_source,
            orientation: CarouselOrientation::Horizontal,
            opts: CarouselOptions::default(),
        }
    }

    pub fn orientation(mut self, orientation: CarouselOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn opts(mut self, opts: CarouselOptions) -> Self {
        self.opts = opts;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CarouselContentProps {
    pub size: Option<Vec2>,
    pub spacing: f32,
    pub item_basis: f32,
}

impl Default for CarouselContentProps {
    fn default() -> Self {
        Self {
            size: None,
            spacing: 16.0,
            item_basis: 1.0,
        }
    }
}

impl CarouselContentProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: Vec2) -> Self {
        self.size = Some(size);
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn item_basis(mut self, basis: f32) -> Self {
        self.item_basis = basis;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CarouselItemProps {
    pub index: usize,
    pub basis: Option<f32>,
}

impl CarouselItemProps {
    pub fn new(index: usize) -> Self {
        Self { index, basis: None }
    }

    pub fn basis(mut self, basis: f32) -> Self {
        self.basis = Some(basis);
        self
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct CarouselState {
    index: usize,
    item_count: usize,
    last_autoplay_time: f64,
    drag_total: Vec2,
    drag_active: bool,
}

pub struct CarouselContext {
    pub id_source: Id,
    pub orientation: CarouselOrientation,
    pub opts: CarouselOptions,
    pub current_index: usize,
    pub item_count: usize,
    viewport: egui::Rect,
    item_spacing: f32,
    item_basis: f32,
    animated_offset: f32,
    measured_count: usize,
    changed: bool,
    interacted: bool,
    drag_total: Vec2,
    drag_active: bool,
}

impl CarouselContext {
    pub fn can_scroll_prev(&self) -> bool {
        if self.item_count <= 1 {
            return false;
        }
        self.opts.looped || self.current_index > 0
    }

    pub fn can_scroll_next(&self) -> bool {
        if self.item_count <= 1 {
            return false;
        }
        self.opts.looped || self.current_index + 1 < self.item_count
    }

    pub fn scroll_prev(&mut self) {
        self.scroll_prev_internal(true);
    }

    pub fn scroll_next(&mut self) {
        self.scroll_next_internal(true);
    }

    fn scroll_prev_internal(&mut self, interacted: bool) {
        if self.item_count == 0 {
            return;
        }
        let next = if self.current_index == 0 {
            if self.opts.looped {
                self.item_count.saturating_sub(1)
            } else {
                return;
            }
        } else {
            self.current_index - 1
        };
        if next != self.current_index {
            self.current_index = next;
            self.changed = true;
            if interacted {
                self.interacted = true;
            }
        }
    }

    fn scroll_next_internal(&mut self, interacted: bool) {
        if self.item_count == 0 {
            return;
        }
        let last = self.item_count.saturating_sub(1);
        let next = if self.current_index >= last {
            if self.opts.looped {
                0
            } else {
                return;
            }
        } else {
            self.current_index + 1
        };
        if next != self.current_index {
            self.current_index = next;
            self.changed = true;
            if interacted {
                self.interacted = true;
            }
        }
    }
}

pub struct CarouselResponse<R> {
    pub inner: R,
    pub response: Response,
    pub index: usize,
    pub count: usize,
    pub can_scroll_prev: bool,
    pub can_scroll_next: bool,
}

pub fn carousel<R>(
    ui: &mut Ui,
    _theme: &Theme,
    props: CarouselProps,
    add_contents: impl FnOnce(&mut Ui, &mut CarouselContext) -> R,
) -> CarouselResponse<R> {
    let ctx = ui.ctx().clone();
    let mut state = ctx
        .data(|d| d.get_temp::<CarouselState>(props.id_source))
        .unwrap_or_default();

    let mut context = CarouselContext {
        id_source: props.id_source,
        orientation: props.orientation,
        opts: props.opts,
        current_index: state.index,
        item_count: state.item_count,
        viewport: egui::Rect::NOTHING,
        item_spacing: 16.0,
        item_basis: 1.0,
        animated_offset: 0.0,
        measured_count: 0,
        changed: false,
        interacted: false,
        drag_total: state.drag_total,
        drag_active: state.drag_active,
    };

    let inner = ui.push_id(props.id_source, |ui| add_contents(ui, &mut context));

    if context.measured_count > 0 {
        state.item_count = context.measured_count;
    }
    context.item_count = state.item_count;

    if state.item_count > 0 {
        context.current_index = context.current_index.min(state.item_count - 1);
    } else {
        context.current_index = 0;
    }

    state.index = context.current_index;

    let now = ui.input(|i| i.time);
    if context.interacted || context.changed {
        state.last_autoplay_time = now;
    }

    if props.opts.autoplay && state.item_count > 1 {
        if state.last_autoplay_time <= 0.0 {
            state.last_autoplay_time = now;
        }
        let delay = (props.opts.autoplay_delay_ms.max(0.0) / 1000.0) as f64;
        let elapsed = now - state.last_autoplay_time;
        if elapsed >= delay {
            context.scroll_next_internal(false);
            state.index = context.current_index;
            state.last_autoplay_time = now;
        } else {
            ctx.request_repaint_after(Duration::from_secs_f64(delay - elapsed));
        }
    }

    state.drag_total = context.drag_total;
    state.drag_active = context.drag_active;
    ctx.data_mut(|d| d.insert_temp(props.id_source, state));

    CarouselResponse {
        inner: inner.inner,
        response: inner.response,
        index: context.current_index,
        count: state.item_count,
        can_scroll_prev: context.can_scroll_prev(),
        can_scroll_next: context.can_scroll_next(),
    }
}

pub struct CarouselContentResponse<R> {
    pub inner: R,
    pub response: Response,
}

pub fn carousel_content<R>(
    ui: &mut Ui,
    theme: &Theme,
    context: &mut CarouselContext,
    props: CarouselContentProps,
    add_contents: impl FnOnce(&mut Ui, &mut CarouselContext) -> R,
) -> CarouselContentResponse<R> {
    let available = ui.available_size();
    let fallback = ui.spacing().interact_size;
    let size = props.size.unwrap_or_else(|| {
        let width = if available.x.is_finite() && available.x > 0.0 {
            available.x
        } else {
            fallback.x.max(1.0)
        };
        let height = if available.y.is_finite() && available.y > 0.0 {
            available.y
        } else {
            fallback.y.max(1.0)
        };
        Vec2::new(width, height)
    });

    let (rect, response) = ui.allocate_exact_size(size, Sense::drag());
    context.viewport = rect;
    context.item_spacing = props.spacing;
    context.item_basis = props.item_basis.max(0.1);
    context.measured_count = 0;

    let (main_size, spacing) = match context.orientation {
        CarouselOrientation::Horizontal => (rect.width(), context.item_spacing),
        CarouselOrientation::Vertical => (rect.height(), context.item_spacing),
    };
    let item_main = main_size * context.item_basis;
    let target_offset = (context.current_index as f32) * (item_main + spacing);
    let anim_ms = theme.motion.base_ms.max(1.0) / 1000.0;
    context.animated_offset =
        ui.ctx()
            .animate_value_with_time(context.id_source.with("offset"), target_offset, anim_ms);

    if response.clicked() {
        response.request_focus();
    }

    if response.drag_started() {
        context.drag_active = true;
        context.drag_total = Vec2::ZERO;
        context.interacted = true;
    }

    if response.dragged() {
        context.drag_total += response.drag_delta();
        context.interacted = true;
    }

    if response.has_focus() {
        let (prev_key, next_key) = match context.orientation {
            CarouselOrientation::Horizontal => (Key::ArrowLeft, Key::ArrowRight),
            CarouselOrientation::Vertical => (Key::ArrowUp, Key::ArrowDown),
        };
        if ui.input(|i| i.key_pressed(prev_key)) {
            context.scroll_prev();
        }
        if ui.input(|i| i.key_pressed(next_key)) {
            context.scroll_next();
        }
    }

    if response.drag_stopped() && context.drag_active {
        let drag = match context.orientation {
            CarouselOrientation::Horizontal => context.drag_total.x,
            CarouselOrientation::Vertical => context.drag_total.y,
        };
        context.drag_active = false;
        context.drag_total = Vec2::ZERO;

        let threshold = item_main * 0.25;
        if drag.abs() > threshold {
            if drag > 0.0 {
                context.scroll_prev();
            } else {
                context.scroll_next();
            }
        }
    }

    CarouselContentResponse {
        inner: {
            let mut content_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(rect)
                    .id_salt(context.id_source.with("content")),
            );
            content_ui.set_clip_rect(rect);
            add_contents(&mut content_ui, context)
        },
        response,
    }
}

pub struct CarouselItemResponse<R> {
    pub inner: R,
    pub response: Response,
}

pub fn carousel_item<R>(
    ui: &mut Ui,
    context: &mut CarouselContext,
    props: CarouselItemProps,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> CarouselItemResponse<R> {
    let basis = props.basis.unwrap_or(context.item_basis).max(0.1);
    let (viewport_main, viewport_cross) = match context.orientation {
        CarouselOrientation::Horizontal => (context.viewport.width(), context.viewport.height()),
        CarouselOrientation::Vertical => (context.viewport.height(), context.viewport.width()),
    };
    let item_main = viewport_main * basis;
    let item_cross = viewport_cross;
    let spacing = context.item_spacing;
    let offset = context.animated_offset;
    let start = context.viewport.min;

    let (min, size) = match context.orientation {
        CarouselOrientation::Horizontal => {
            let x = start.x + (props.index as f32) * (item_main + spacing) - offset;
            (egui::pos2(x, start.y), Vec2::new(item_main, item_cross))
        }
        CarouselOrientation::Vertical => {
            let y = start.y + (props.index as f32) * (item_main + spacing) - offset;
            (egui::pos2(start.x, y), Vec2::new(item_cross, item_main))
        }
    };

    let rect = egui::Rect::from_min_size(min, size);
    let response = ui.interact(rect, ui.next_auto_id(), Sense::hover());
    let inner = ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(Layout::top_down(Align::Min)),
        |item_ui| {
            item_ui.set_clip_rect(context.viewport);
            item_ui.set_min_size(size);
            add_contents(item_ui)
        },
    );

    context.measured_count = context.measured_count.max(props.index + 1);

    CarouselItemResponse {
        inner: inner.inner,
        response,
    }
}

pub fn carousel_previous(ui: &mut Ui, theme: &Theme, context: &mut CarouselContext) -> Response {
    let icon = match context.orientation {
        CarouselOrientation::Horizontal => icon_arrow_left,
        CarouselOrientation::Vertical => icon_arrow_up,
    };
    let enabled = context.can_scroll_prev();
    let response = Button::new("")
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::Icon)
        .enabled(enabled)
        .icon(&icon)
        .show(ui, theme);

    if response.clicked() && enabled {
        context.scroll_prev();
    }

    response
}

pub fn carousel_next(ui: &mut Ui, theme: &Theme, context: &mut CarouselContext) -> Response {
    let icon = match context.orientation {
        CarouselOrientation::Horizontal => icon_arrow_right,
        CarouselOrientation::Vertical => icon_arrow_down,
    };
    let enabled = context.can_scroll_next();
    let response = Button::new("")
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::Icon)
        .enabled(enabled)
        .icon(&icon)
        .show(ui, theme);

    if response.clicked() && enabled {
        context.scroll_next();
    }

    response
}

fn icon_arrow_left(painter: &egui::Painter, center: egui::Pos2, size: f32, color: Color32) {
    let stroke = Stroke::new((size * 0.12).clamp(1.5, 2.5), color);
    let x = center.x - size * 0.1;
    let y = center.y;
    let left = x - size * 0.3;
    painter.line_segment([egui::pos2(left, y), egui::pos2(x + size * 0.2, y)], stroke);
    painter.line_segment(
        [
            egui::pos2(left + size * 0.15, y - size * 0.2),
            egui::pos2(left, y),
        ],
        stroke,
    );
    painter.line_segment(
        [
            egui::pos2(left + size * 0.15, y + size * 0.2),
            egui::pos2(left, y),
        ],
        stroke,
    );
}

fn icon_arrow_right(painter: &egui::Painter, center: egui::Pos2, size: f32, color: Color32) {
    let stroke = Stroke::new((size * 0.12).clamp(1.5, 2.5), color);
    let x = center.x + size * 0.1;
    let y = center.y;
    let right = x + size * 0.3;
    painter.line_segment(
        [egui::pos2(x - size * 0.2, y), egui::pos2(right, y)],
        stroke,
    );
    painter.line_segment(
        [
            egui::pos2(right - size * 0.15, y - size * 0.2),
            egui::pos2(right, y),
        ],
        stroke,
    );
    painter.line_segment(
        [
            egui::pos2(right - size * 0.15, y + size * 0.2),
            egui::pos2(right, y),
        ],
        stroke,
    );
}

fn icon_arrow_up(painter: &egui::Painter, center: egui::Pos2, size: f32, color: Color32) {
    let stroke = Stroke::new((size * 0.12).clamp(1.5, 2.5), color);
    let x = center.x;
    let y = center.y - size * 0.1;
    let top = y - size * 0.3;
    painter.line_segment([egui::pos2(x, top), egui::pos2(x, y + size * 0.2)], stroke);
    painter.line_segment(
        [
            egui::pos2(x - size * 0.2, top + size * 0.15),
            egui::pos2(x, top),
        ],
        stroke,
    );
    painter.line_segment(
        [
            egui::pos2(x + size * 0.2, top + size * 0.15),
            egui::pos2(x, top),
        ],
        stroke,
    );
}

fn icon_arrow_down(painter: &egui::Painter, center: egui::Pos2, size: f32, color: Color32) {
    let stroke = Stroke::new((size * 0.12).clamp(1.5, 2.5), color);
    let x = center.x;
    let y = center.y + size * 0.1;
    let bottom = y + size * 0.3;
    painter.line_segment(
        [egui::pos2(x, y - size * 0.2), egui::pos2(x, bottom)],
        stroke,
    );
    painter.line_segment(
        [
            egui::pos2(x - size * 0.2, bottom - size * 0.15),
            egui::pos2(x, bottom),
        ],
        stroke,
    );
    painter.line_segment(
        [
            egui::pos2(x + size * 0.2, bottom - size * 0.15),
            egui::pos2(x, bottom),
        ],
        stroke,
    );
}
