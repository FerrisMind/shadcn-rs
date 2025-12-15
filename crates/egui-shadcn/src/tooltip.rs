use crate::theme::Theme;
use crate::tokens::{ColorPalette, DEFAULT_MOTION, ease_out_cubic, mix};
use egui::epaint::Shadow;
use egui::{
    Color32, CornerRadius, Frame, Id, Order, Pos2, Rect, Response, Stroke, Ui, Vec2, WidgetText,
    vec2,
};
use log::trace;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TooltipPosition {
    Cursor,
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TooltipSide {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

impl TooltipSide {
    pub fn from_position(pos: TooltipPosition) -> Self {
        match pos {
            TooltipPosition::Top => TooltipSide::Top,
            TooltipPosition::Bottom => TooltipSide::Bottom,
            TooltipPosition::Left => TooltipSide::Left,
            TooltipPosition::Right => TooltipSide::Right,
            TooltipPosition::Cursor => TooltipSide::Top,
        }
    }

    pub fn offset_direction(&self) -> Vec2 {
        match self {
            TooltipSide::Top => vec2(0.0, -1.0),
            TooltipSide::Bottom => vec2(0.0, 1.0),
            TooltipSide::Left => vec2(-1.0, 0.0),
            TooltipSide::Right => vec2(1.0, 0.0),
        }
    }

    pub fn flip(&self) -> Self {
        match self {
            TooltipSide::Top => TooltipSide::Bottom,
            TooltipSide::Bottom => TooltipSide::Top,
            TooltipSide::Left => TooltipSide::Right,
            TooltipSide::Right => TooltipSide::Left,
        }
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self, TooltipSide::Top | TooltipSide::Bottom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TooltipAlign {
    #[default]
    Center,
    Start,
    End,
}

impl TooltipAlign {
    pub fn factor(&self) -> f32 {
        match self {
            TooltipAlign::Center => 0.0,
            TooltipAlign::Start => -1.0,
            TooltipAlign::End => 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TooltipSticky {
    #[default]
    Partial,
    Always,
}

impl From<bool> for TooltipSticky {
    fn from(value: bool) -> Self {
        if value {
            TooltipSticky::Always
        } else {
            TooltipSticky::Partial
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TooltipUpdatePositionStrategy {
    #[default]
    Optimized,
    Always,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TooltipCollisionPadding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl TooltipCollisionPadding {
    pub fn all(value: f32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
}

impl Default for TooltipCollisionPadding {
    fn default() -> Self {
        Self::all(10.0)
    }
}

impl From<f32> for TooltipCollisionPadding {
    fn from(value: f32) -> Self {
        TooltipCollisionPadding::all(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TooltipPortalContainer {
    Tooltip,
    Foreground,
    Middle,
    Background,
}

impl TooltipPortalContainer {
    fn order(self) -> Order {
        match self {
            TooltipPortalContainer::Tooltip => Order::Tooltip,
            TooltipPortalContainer::Foreground => Order::Foreground,
            TooltipPortalContainer::Middle => Order::Middle,
            TooltipPortalContainer::Background => Order::Background,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TooltipPreventable {
    default_prevented: bool,
}

impl TooltipPreventable {
    pub fn prevent_default(&mut self) {
        self.default_prevented = true;
    }

    pub fn default_prevented(&self) -> bool {
        self.default_prevented
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TooltipEscapeKeyDownEvent {
    pub key: egui::Key,
    pub preventable: TooltipPreventable,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TooltipPointerDownOutsideEvent {
    pub pointer_pos: Option<Pos2>,
    pub preventable: TooltipPreventable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TooltipAnimationState {
    Closed,

    DelayedOpen,

    InstantOpen,
}

#[derive(Clone, Debug, Default)]
pub struct TooltipOpenState {
    pub is_open: bool,

    pub animation_progress: f32,

    pub hover_start_time: Option<f64>,

    pub last_close_time: Option<f64>,
}

impl TooltipOpenState {
    pub fn is_visible(&self) -> bool {
        self.is_open || self.animation_progress > 0.0
    }

    pub fn is_animating(&self) -> bool {
        if self.is_open {
            self.animation_progress < 1.0
        } else {
            self.animation_progress > 0.0
        }
    }

    pub fn should_skip_delay(&self, current_time: f64, skip_delay_ms: u64) -> bool {
        if let Some(close_time) = self.last_close_time {
            let elapsed = current_time - close_time;
            let skip_delay_secs = skip_delay_ms as f64 / 1000.0;
            elapsed < skip_delay_secs
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TooltipState {
    pub open_state: TooltipOpenState,

    pub computed_side: Option<TooltipSide>,

    pub computed_align: Option<TooltipAlign>,
}

impl TooltipState {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug)]
pub struct TooltipStyle {
    pub bg: Color32,
    pub border: Color32,
    pub border_width: f32,
    pub text: Color32,
    pub rounding: CornerRadius,
    pub shadow: Shadow,

    pub arrow_fill: Color32,
}

impl TooltipStyle {
    pub fn from_palette(palette: &ColorPalette, high_contrast: bool) -> Self {
        let bg = if high_contrast {
            palette.foreground
        } else {
            mix(palette.foreground, palette.background, 0.1)
        };

        let border = if high_contrast {
            palette.foreground
        } else {
            mix(palette.border, palette.foreground, 0.2)
        };

        let text = palette.background;

        let rounding = CornerRadius::same(6);
        let shadow = Shadow::default();
        Self {
            bg,
            border,
            border_width: if high_contrast { 0.0 } else { 1.0 },
            text,
            rounding,
            shadow,
            arrow_fill: bg,
        }
    }
}

pub struct TooltipProps<'a> {
    pub text: WidgetText,

    pub delay_ms: u64,
    pub skip_delay_ms: u64,

    pub max_width: f32,

    pub position: TooltipPosition,

    pub side: TooltipSide,

    pub align: TooltipAlign,

    pub offset: Vec2,

    pub side_offset: f32,

    pub align_offset: f32,

    pub collision_padding: TooltipCollisionPadding,

    /// Mirrors Radix Popper.Content `collisionBoundary` (simplified as a single `Rect`).
    pub collision_boundary: Option<Rect>,

    /// Mirrors Radix Tooltip.Content `aria-label`.
    pub aria_label: Option<String>,

    pub high_contrast: bool,
    pub persistent_id: Option<Id>,
    pub style: Option<TooltipStyle>,
    pub show_when_disabled: bool,

    pub show_arrow: bool,

    pub arrow_width: f32,

    pub arrow_height: f32,

    pub arrow_padding: f32,

    /// Mirrors Radix Popper.Content `sticky`.
    pub sticky: TooltipSticky,

    /// Mirrors Radix Popper.Content `hideWhenDetached`.
    pub hide_when_detached: bool,

    /// Mirrors Radix Popper.Content `updatePositionStrategy`.
    pub update_position_strategy: TooltipUpdatePositionStrategy,

    /// Mirrors Radix Portal `container` (mapped to egui layer order).
    pub container: Option<TooltipPortalContainer>,

    pub force_mount: bool,

    pub disable_hoverable_content: bool,

    pub animation_duration_ms: u64,

    pub open: Option<bool>,

    pub default_open: bool,

    pub avoid_collisions: bool,

    /// Mirrors Radix Tooltip.Root `onOpenChange`.
    pub on_open_change: Option<&'a mut dyn FnMut(bool)>,

    /// Mirrors Radix Tooltip.Content `onEscapeKeyDown`.
    pub on_escape_key_down: Option<&'a mut dyn FnMut(&mut TooltipEscapeKeyDownEvent)>,

    /// Mirrors Radix Tooltip.Content `onPointerDownOutside`.
    pub on_pointer_down_outside: Option<&'a mut dyn FnMut(&mut TooltipPointerDownOutsideEvent)>,
}

impl std::fmt::Debug for TooltipProps<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TooltipProps")
            .field("text", &self.text.text())
            .field("delay_ms", &self.delay_ms)
            .field("skip_delay_ms", &self.skip_delay_ms)
            .field("max_width", &self.max_width)
            .field("position", &self.position)
            .field("side", &self.side)
            .field("align", &self.align)
            .field("offset", &self.offset)
            .field("side_offset", &self.side_offset)
            .field("align_offset", &self.align_offset)
            .field("collision_padding", &self.collision_padding)
            .field("collision_boundary", &self.collision_boundary)
            .field("aria_label", &self.aria_label)
            .field("high_contrast", &self.high_contrast)
            .field("persistent_id", &self.persistent_id)
            .field("style", &self.style.is_some())
            .field("show_when_disabled", &self.show_when_disabled)
            .field("show_arrow", &self.show_arrow)
            .field("arrow_width", &self.arrow_width)
            .field("arrow_height", &self.arrow_height)
            .field("arrow_padding", &self.arrow_padding)
            .field("sticky", &self.sticky)
            .field("hide_when_detached", &self.hide_when_detached)
            .field("update_position_strategy", &self.update_position_strategy)
            .field("container", &self.container)
            .field("force_mount", &self.force_mount)
            .field("disable_hoverable_content", &self.disable_hoverable_content)
            .field("animation_duration_ms", &self.animation_duration_ms)
            .field("open", &self.open)
            .field("default_open", &self.default_open)
            .field("avoid_collisions", &self.avoid_collisions)
            .field("on_open_change", &self.on_open_change.is_some())
            .field("on_escape_key_down", &self.on_escape_key_down.is_some())
            .field(
                "on_pointer_down_outside",
                &self.on_pointer_down_outside.is_some(),
            )
            .finish()
    }
}

impl<'a> TooltipProps<'a> {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            delay_ms: 700,
            skip_delay_ms: 300,
            max_width: 360.0,
            position: TooltipPosition::Top,
            side: TooltipSide::Top,
            align: TooltipAlign::Center,
            offset: vec2(0.0, 8.0),
            side_offset: 4.0,
            align_offset: 0.0,
            collision_padding: TooltipCollisionPadding::default(),
            collision_boundary: None,
            aria_label: None,
            high_contrast: false,
            persistent_id: None,
            style: None,
            show_when_disabled: false,
            show_arrow: false,
            arrow_width: 11.0,
            arrow_height: 5.0,
            arrow_padding: 0.0,
            sticky: TooltipSticky::default(),
            hide_when_detached: false,
            update_position_strategy: TooltipUpdatePositionStrategy::default(),
            container: None,
            force_mount: false,
            disable_hoverable_content: false,
            animation_duration_ms: DEFAULT_MOTION.base_ms as u64,
            open: None,
            default_open: false,
            avoid_collisions: true,
            on_open_change: None,
            on_escape_key_down: None,
            on_pointer_down_outside: None,
        }
    }

    pub fn delay_ms(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    pub fn delay_duration(self, delay_ms: u64) -> Self {
        self.delay_ms(delay_ms)
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self.side = TooltipSide::from_position(position);
        self
    }

    pub fn side(mut self, side: TooltipSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: TooltipAlign) -> Self {
        self.align = align;
        self
    }

    pub fn offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    pub fn side_offset(mut self, offset: f32) -> Self {
        self.side_offset = offset;
        self
    }

    pub fn align_offset(mut self, offset: f32) -> Self {
        self.align_offset = offset;
        self
    }

    pub fn collision_padding(mut self, padding: impl Into<TooltipCollisionPadding>) -> Self {
        self.collision_padding = padding.into();
        self
    }

    pub fn collision_boundary(mut self, boundary: Rect) -> Self {
        self.collision_boundary = Some(boundary);
        self
    }

    pub fn aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn persistent_id(mut self, id: Id) -> Self {
        self.persistent_id = Some(id);
        self
    }

    pub fn style(mut self, style: TooltipStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn show_when_disabled(mut self, show: bool) -> Self {
        self.show_when_disabled = show;
        self
    }

    pub fn show_arrow(mut self, show: bool) -> Self {
        self.show_arrow = show;
        self
    }

    pub fn arrow_size(mut self, width: f32, height: f32) -> Self {
        self.arrow_width = width;
        self.arrow_height = height;
        self
    }

    pub fn arrow_padding(mut self, padding: f32) -> Self {
        self.arrow_padding = padding;
        self
    }

    pub fn force_mount(mut self, force: bool) -> Self {
        self.force_mount = force;
        self
    }

    pub fn skip_delay_ms(mut self, skip_delay: u64) -> Self {
        self.skip_delay_ms = skip_delay;
        self
    }

    pub fn skip_delay_duration(self, skip_delay_ms: u64) -> Self {
        self.skip_delay_ms(skip_delay_ms)
    }

    pub fn disable_hoverable_content(mut self, disable: bool) -> Self {
        self.disable_hoverable_content = disable;
        self
    }

    pub fn animation_duration_ms(mut self, duration: u64) -> Self {
        self.animation_duration_ms = duration;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn on_open_change(mut self, on_open_change: &'a mut dyn FnMut(bool)) -> Self {
        self.on_open_change = Some(on_open_change);
        self
    }

    pub fn sticky(mut self, sticky: impl Into<TooltipSticky>) -> Self {
        self.sticky = sticky.into();
        self
    }

    pub fn sticky_enabled(mut self, enabled: bool) -> Self {
        self.sticky = if enabled {
            TooltipSticky::Always
        } else {
            TooltipSticky::Partial
        };
        self
    }

    pub fn hide_when_detached(mut self, hide_when_detached: bool) -> Self {
        self.hide_when_detached = hide_when_detached;
        self
    }

    pub fn update_position_strategy(
        mut self,
        update_position_strategy: TooltipUpdatePositionStrategy,
    ) -> Self {
        self.update_position_strategy = update_position_strategy;
        self
    }

    pub fn container(mut self, container: TooltipPortalContainer) -> Self {
        self.container = Some(container);
        self
    }

    pub fn on_escape_key_down(
        mut self,
        on_escape_key_down: &'a mut dyn FnMut(&mut TooltipEscapeKeyDownEvent),
    ) -> Self {
        self.on_escape_key_down = Some(on_escape_key_down);
        self
    }

    pub fn on_pointer_down_outside(
        mut self,
        on_pointer_down_outside: &'a mut dyn FnMut(&mut TooltipPointerDownOutsideEvent),
    ) -> Self {
        self.on_pointer_down_outside = Some(on_pointer_down_outside);
        self
    }

    pub fn avoid_collisions(mut self, avoid: bool) -> Self {
        self.avoid_collisions = avoid;
        self
    }
}

#[allow(clippy::too_many_arguments)]
fn calculate_tooltip_pos(
    anchor_rect: Rect,
    tooltip_size: Vec2,
    side: TooltipSide,
    align: TooltipAlign,
    side_offset: f32,
    align_offset: f32,
    collision_padding: TooltipCollisionPadding,
    collision_boundary: Rect,
    avoid_collisions: bool,
    arrow_height: f32,
    show_arrow: bool,
) -> (Pos2, TooltipSide) {
    let effective_side_offset = if show_arrow {
        side_offset + arrow_height
    } else {
        side_offset
    };

    let mut current_side = side;
    let mut pos = calculate_position_for_side(
        anchor_rect,
        tooltip_size,
        current_side,
        align,
        effective_side_offset,
        align_offset,
    );

    let viewport_rect = collision_boundary;
    let padded_viewport = Rect::from_min_max(
        Pos2::new(
            viewport_rect.left() + collision_padding.left,
            viewport_rect.top() + collision_padding.top,
        ),
        Pos2::new(
            viewport_rect.right() - collision_padding.right,
            viewport_rect.bottom() - collision_padding.bottom,
        ),
    );

    if avoid_collisions {
        let tooltip_rect = Rect::from_min_size(pos, tooltip_size);

        if !padded_viewport.contains_rect(tooltip_rect) {
            let flipped_side = current_side.flip();
            let flipped_pos = calculate_position_for_side(
                anchor_rect,
                tooltip_size,
                flipped_side,
                align,
                effective_side_offset,
                align_offset,
            );
            let flipped_rect = Rect::from_min_size(flipped_pos, tooltip_size);

            if padded_viewport.contains_rect(flipped_rect) {
                current_side = flipped_side;
                pos = flipped_pos;
            }
        }
    }

    let min_x = padded_viewport.left();
    let max_x = (padded_viewport.right() - tooltip_size.x).max(min_x);
    let min_y = padded_viewport.top();
    let max_y = (padded_viewport.bottom() - tooltip_size.y).max(min_y);

    pos.x = pos.x.clamp(min_x, max_x);
    pos.y = pos.y.clamp(min_y, max_y);

    (pos, current_side)
}

fn calculate_position_for_side(
    anchor_rect: Rect,
    tooltip_size: Vec2,
    side: TooltipSide,
    align: TooltipAlign,
    side_offset: f32,
    align_offset: f32,
) -> Pos2 {
    let anchor_center = anchor_rect.center();

    match side {
        TooltipSide::Top => {
            let x = calculate_aligned_pos(
                anchor_center.x,
                anchor_rect.width(),
                tooltip_size.x,
                align,
                align_offset,
            );
            let y = anchor_rect.top() - tooltip_size.y - side_offset;
            Pos2::new(x, y)
        }
        TooltipSide::Bottom => {
            let x = calculate_aligned_pos(
                anchor_center.x,
                anchor_rect.width(),
                tooltip_size.x,
                align,
                align_offset,
            );
            let y = anchor_rect.bottom() + side_offset;
            Pos2::new(x, y)
        }
        TooltipSide::Left => {
            let x = anchor_rect.left() - tooltip_size.x - side_offset;
            let y = calculate_aligned_pos(
                anchor_center.y,
                anchor_rect.height(),
                tooltip_size.y,
                align,
                align_offset,
            );
            Pos2::new(x, y)
        }
        TooltipSide::Right => {
            let x = anchor_rect.right() + side_offset;
            let y = calculate_aligned_pos(
                anchor_center.y,
                anchor_rect.height(),
                tooltip_size.y,
                align,
                align_offset,
            );
            Pos2::new(x, y)
        }
    }
}

fn calculate_aligned_pos(
    anchor_center: f32,
    anchor_size: f32,
    tooltip_size: f32,
    align: TooltipAlign,
    align_offset: f32,
) -> f32 {
    match align {
        TooltipAlign::Center => anchor_center - tooltip_size / 2.0 + align_offset,
        TooltipAlign::Start => anchor_center - anchor_size / 2.0 + align_offset,
        TooltipAlign::End => anchor_center + anchor_size / 2.0 - tooltip_size + align_offset,
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_arrow(
    painter: &egui::Painter,
    content_rect: Rect,
    side: TooltipSide,
    arrow_width: f32,
    arrow_height: f32,
    fill: Color32,
    anchor_rect: Rect,
    arrow_padding: f32,
) {
    let arrow_center = match side {
        TooltipSide::Top | TooltipSide::Bottom => {
            let min_x = content_rect.left() + arrow_padding + arrow_width / 2.0;
            let max_x = content_rect.right() - arrow_padding - arrow_width / 2.0;
            anchor_rect.center().x.clamp(min_x, max_x)
        }
        TooltipSide::Left | TooltipSide::Right => {
            let min_y = content_rect.top() + arrow_padding + arrow_width / 2.0;
            let max_y = content_rect.bottom() - arrow_padding - arrow_width / 2.0;
            anchor_rect.center().y.clamp(min_y, max_y)
        }
    };

    let points = match side {
        TooltipSide::Top => {
            let tip_y = content_rect.bottom() + arrow_height;
            vec![
                Pos2::new(arrow_center - arrow_width / 2.0, content_rect.bottom()),
                Pos2::new(arrow_center + arrow_width / 2.0, content_rect.bottom()),
                Pos2::new(arrow_center, tip_y),
            ]
        }
        TooltipSide::Bottom => {
            let tip_y = content_rect.top() - arrow_height;
            vec![
                Pos2::new(arrow_center - arrow_width / 2.0, content_rect.top()),
                Pos2::new(arrow_center + arrow_width / 2.0, content_rect.top()),
                Pos2::new(arrow_center, tip_y),
            ]
        }
        TooltipSide::Left => {
            let tip_x = content_rect.right() + arrow_height;
            vec![
                Pos2::new(content_rect.right(), arrow_center - arrow_width / 2.0),
                Pos2::new(content_rect.right(), arrow_center + arrow_width / 2.0),
                Pos2::new(tip_x, arrow_center),
            ]
        }
        TooltipSide::Right => {
            let tip_x = content_rect.left() - arrow_height;
            vec![
                Pos2::new(content_rect.left(), arrow_center - arrow_width / 2.0),
                Pos2::new(content_rect.left(), arrow_center + arrow_width / 2.0),
                Pos2::new(tip_x, arrow_center),
            ]
        }
    };

    let shape = egui::epaint::PathShape::convex_polygon(points, fill, Stroke::NONE);
    painter.add(shape);
}

fn get_global_last_close_time(ctx: &egui::Context) -> Option<f64> {
    ctx.data(|d| d.get_temp::<f64>(Id::new("__tooltip_global_last_close__")))
}

fn set_global_last_close_time(ctx: &egui::Context, time: f64) {
    ctx.data_mut(|d| d.insert_temp(Id::new("__tooltip_global_last_close__"), time));
}

pub fn tooltip(anchor: &Response, ui: &mut Ui, theme: &Theme, mut props: TooltipProps<'_>) -> bool {
    let ctx = ui.ctx();
    let now = ctx.input(|i| i.time);

    let anchor_hovered = anchor.hovered() || anchor.has_focus();
    let disabled = !anchor.enabled();

    if disabled && !props.show_when_disabled && !props.force_mount {
        return false;
    }

    let id = props
        .persistent_id
        .unwrap_or_else(|| anchor.id.with("tooltip"));

    let anchor_rect = anchor.rect;
    let collision_boundary = props
        .collision_boundary
        .unwrap_or_else(|| ctx.viewport_rect());
    if props.hide_when_detached && !collision_boundary.intersects(anchor_rect) && !props.force_mount
    {
        return false;
    }

    let delay_secs = props.delay_ms as f64 / 1000.0;
    let animation_duration = (props.animation_duration_ms as f32).max(1.0) / 1000.0;

    let global_last_close = get_global_last_close_time(ctx);
    let should_skip_delay = global_last_close.is_some_and(|close_time| {
        let elapsed = now - close_time;
        elapsed < (props.skip_delay_ms as f64 / 1000.0)
    });

    let tooltip_area_id = id.with("area");
    let tooltip_hovered = if !props.disable_hoverable_content {
        ctx.data(|d| d.get_temp::<bool>(tooltip_area_id))
            .unwrap_or(false)
    } else {
        false
    };

    let want_open = anchor_hovered || tooltip_hovered;

    let is_controlled = props.open.is_some();
    let controlled_open = props.open.unwrap_or(false);

    let init_key = id.with("default-open-initialized");
    let hover_start_key = id.with("hover-start");
    let internal_open_key = id.with("is-open");
    let last_request_key = id.with("last-open-request");
    let last_visible_key = id.with("last-visible-open");

    let (elapsed_hover, internal_open_before, internal_open_after, requested_open, applied_default) =
        ctx.data_mut(|d| {
            let internal_before = d.get_temp::<bool>(internal_open_key).unwrap_or(false);
            let mut internal_after = internal_before;
            let mut requested = false;
            let mut elapsed_hover = 0.0;
            let mut applied_default_open = false;

            let initialized = d.get_temp::<bool>(init_key).unwrap_or(false);
            if !initialized {
                d.insert_temp(init_key, true);
                if props.default_open {
                    applied_default_open = true;
                    requested = true;
                    if !is_controlled {
                        internal_after = true;
                        d.insert_temp(internal_open_key, true);
                    }
                }
            }

            if want_open && d.get_temp::<f64>(hover_start_key).is_none() {
                d.insert_temp(hover_start_key, now);
            }
            if want_open {
                let start = d.get_temp::<f64>(hover_start_key).unwrap_or(now);
                elapsed_hover = now - start;
            } else {
                d.remove::<f64>(hover_start_key);
                if !applied_default_open {
                    requested = false;
                }
            }

            if !applied_default_open {
                let effective_delay = if should_skip_delay { 0.0 } else { delay_secs };
                requested = want_open && elapsed_hover >= effective_delay;
            }

            if !is_controlled {
                if requested {
                    d.insert_temp(internal_open_key, true);
                    internal_after = true;
                } else {
                    d.remove::<bool>(internal_open_key);
                    internal_after = false;
                }
            }

            (
                elapsed_hover,
                internal_before,
                internal_after,
                requested,
                applied_default_open,
            )
        });

    let render_open = if is_controlled {
        controlled_open
    } else {
        internal_open_after
    };

    if is_controlled {
        let last_requested = ctx
            .data(|d| d.get_temp::<bool>(last_request_key))
            .unwrap_or(controlled_open);
        if requested_open != last_requested {
            ctx.data_mut(|d| d.insert_temp(last_request_key, requested_open));
            if requested_open != controlled_open
                && let Some(cb) = props.on_open_change.as_mut()
            {
                cb(requested_open);
            }
        }
    } else if internal_open_after != internal_open_before
        && let Some(cb) = props.on_open_change.as_mut()
    {
        cb(internal_open_after);
    }

    if internal_open_before && !internal_open_after && !is_controlled {
        set_global_last_close_time(ctx, now);
    }

    let last_visible = ctx
        .data(|d| d.get_temp::<bool>(last_visible_key))
        .unwrap_or(false);
    if last_visible && !render_open {
        set_global_last_close_time(ctx, now);
    }
    ctx.data_mut(|d| d.insert_temp(last_visible_key, render_open));

    let animation_progress = ctx.animate_bool_with_time_and_easing(
        id.with("animation"),
        render_open,
        animation_duration,
        ease_out_cubic,
    );

    if animation_progress <= 0.0 && !props.force_mount {
        if (want_open || applied_default) && elapsed_hover < delay_secs && !should_skip_delay {
            ctx.request_repaint_after(Duration::from_secs_f64(delay_secs - elapsed_hover));
        }
        return false;
    }

    let style = props
        .style
        .clone()
        .unwrap_or_else(|| TooltipStyle::from_palette(&theme.palette, props.high_contrast));

    let (measured_size, text_galley) = {
        let text_str = props.text.text().to_string();
        let available_width = props.max_width - 24.0;

        let galley = ctx.fonts_mut(|fonts| {
            fonts.layout(
                text_str,
                egui::FontId::default(),
                style.text,
                available_width,
            )
        });

        let text_size = galley.size();
        let size = Vec2::new(text_size.x + 24.0, text_size.y + 12.0);

        (size, galley)
    };

    let _ = text_galley;

    let (tooltip_pos, computed_side) = calculate_tooltip_pos(
        anchor_rect,
        measured_size,
        props.side,
        props.align,
        props.side_offset,
        props.align_offset,
        props.collision_padding,
        collision_boundary,
        props.avoid_collisions,
        props.arrow_height,
        props.show_arrow,
    );

    let slide_offset = match computed_side {
        TooltipSide::Top => vec2(0.0, 4.0),
        TooltipSide::Bottom => vec2(0.0, -4.0),
        TooltipSide::Left => vec2(4.0, 0.0),
        TooltipSide::Right => vec2(-4.0, 0.0),
    };

    let scale = 0.96 + 0.04 * animation_progress;
    let scaled_size = measured_size * scale;
    let scale_offset = (measured_size - scaled_size) * 0.5;

    let animated_offset = slide_offset * (1.0 - animation_progress);
    let final_pos = tooltip_pos + animated_offset + scale_offset;

    let opacity = animation_progress;

    trace!(
        "Showing tooltip at {:?}, side={:?}, progress={:.2}",
        final_pos, computed_side, animation_progress
    );

    let order = props
        .container
        .unwrap_or(TooltipPortalContainer::Tooltip)
        .order();

    let area_response = egui::Area::new(id)
        .order(order)
        .interactable(render_open)
        .fixed_pos(final_pos)
        .show(ctx, |tooltip_ui| {
            tooltip_ui.set_max_width(props.max_width);

            let mut visuals = tooltip_ui.visuals().clone();
            visuals.widgets.noninteractive.bg_fill = style.bg.gamma_multiply(opacity);
            tooltip_ui.ctx().set_visuals(visuals);

            let mut frame = Frame::popup(tooltip_ui.style());
            frame.fill = style.bg.gamma_multiply(opacity);
            frame.stroke = Stroke::new(style.border_width, style.border.gamma_multiply(opacity));
            frame.corner_radius = style.rounding;
            frame.shadow = Shadow {
                offset: style.shadow.offset,
                blur: style.shadow.blur,
                spread: style.shadow.spread,
                color: style.shadow.color.gamma_multiply(opacity),
            };
            frame.inner_margin = egui::Margin::symmetric(12, 6);

            let frame_response = frame.show(tooltip_ui, |content_ui| {
                content_ui.style_mut().visuals.override_text_color =
                    Some(style.text.gamma_multiply(opacity));

                content_ui.label(props.text.clone().color(style.text.gamma_multiply(opacity)));
            });

            if props.show_arrow {
                let painter = tooltip_ui.painter();
                draw_arrow(
                    painter,
                    frame_response.response.rect,
                    computed_side,
                    props.arrow_width,
                    props.arrow_height,
                    style.arrow_fill.gamma_multiply(opacity),
                    anchor_rect,
                    props.arrow_padding,
                );
            }
        });

    let mut should_close = false;

    if render_open && ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        let mut evt = TooltipEscapeKeyDownEvent {
            key: egui::Key::Escape,
            preventable: TooltipPreventable::default(),
        };
        if let Some(cb) = props.on_escape_key_down.as_mut() {
            cb(&mut evt);
        }
        if !evt.preventable.default_prevented() {
            should_close = true;
        }
    }

    let tooltip_rect = area_response.response.rect;
    let (any_click, interact_pos) =
        ctx.input(|i| (i.pointer.any_click(), i.pointer.interact_pos()));
    if render_open
        && any_click
        && interact_pos.is_some_and(|pos| !tooltip_rect.contains(pos) && !anchor_rect.contains(pos))
    {
        let mut evt = TooltipPointerDownOutsideEvent {
            pointer_pos: interact_pos,
            preventable: TooltipPreventable::default(),
        };
        if let Some(cb) = props.on_pointer_down_outside.as_mut() {
            cb(&mut evt);
        }
        if !evt.preventable.default_prevented() {
            should_close = true;
        }
    }

    if should_close {
        if is_controlled {
            if let Some(cb) = props.on_open_change.as_mut() {
                cb(false);
            }
        } else {
            let was_open = ctx
                .data(|d| d.get_temp::<bool>(internal_open_key))
                .unwrap_or(false);
            if was_open {
                ctx.data_mut(|d| d.remove::<bool>(internal_open_key));
                if let Some(cb) = props.on_open_change.as_mut() {
                    cb(false);
                }
                set_global_last_close_time(ctx, now);
            }
        }
    }

    if !props.disable_hoverable_content && render_open {
        let expanded_rect = tooltip_rect.expand(4.0);
        let mouse_pos = ctx.input(|i| i.pointer.hover_pos());
        let content_hovered = mouse_pos.is_some_and(|pos| expanded_rect.contains(pos));
        ctx.data_mut(|d| d.insert_temp(tooltip_area_id, content_hovered));
    }

    if props.update_position_strategy == TooltipUpdatePositionStrategy::Always
        || (opacity > 0.0 && opacity < 1.0)
    {
        ctx.request_repaint();
    }

    opacity > 0.0 || props.force_mount
}
