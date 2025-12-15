use crate::theme::Theme;
use crate::tokens::mix;
use egui::{
    CornerRadius, Frame, Id, Margin, Order, Pos2, Rect, Response, Stroke, Ui, Vec2, pos2, vec2,
};
use log::trace;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopoverPlacement {
    Above,
    Below,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopoverAlign {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopoverSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl PopoverSide {
    pub fn to_placement(self) -> PopoverPlacement {
        match self {
            PopoverSide::Top => PopoverPlacement::Above,
            PopoverSide::Right => PopoverPlacement::Right,
            PopoverSide::Bottom => PopoverPlacement::Below,
            PopoverSide::Left => PopoverPlacement::Left,
        }
    }
}

impl From<PopoverPlacement> for PopoverSide {
    fn from(value: PopoverPlacement) -> Self {
        match value {
            PopoverPlacement::Above => PopoverSide::Top,
            PopoverPlacement::Below => PopoverSide::Bottom,
            PopoverPlacement::Left => PopoverSide::Left,
            PopoverPlacement::Right => PopoverSide::Right,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PopoverSticky {
    #[default]
    Partial,
    Always,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PopoverUpdatePositionStrategy {
    #[default]
    Optimized,
    Always,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PopoverCollisionPadding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl PopoverCollisionPadding {
    pub fn all(value: f32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
}

impl Default for PopoverCollisionPadding {
    fn default() -> Self {
        Self::all(0.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopoverPortalContainer {
    Tooltip,
    Foreground,
    Middle,
    Background,
}

impl PopoverPortalContainer {
    fn order(self) -> Order {
        match self {
            PopoverPortalContainer::Tooltip => Order::Tooltip,
            PopoverPortalContainer::Foreground => Order::Foreground,
            PopoverPortalContainer::Middle => Order::Middle,
            PopoverPortalContainer::Background => Order::Background,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PopoverPreventable {
    default_prevented: bool,
}

impl PopoverPreventable {
    pub fn prevent_default(&mut self) {
        self.default_prevented = true;
    }

    pub fn default_prevented(&self) -> bool {
        self.default_prevented
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PopoverEscapeKeyDownEvent {
    pub key: egui::Key,
    pub preventable: PopoverPreventable,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PopoverPointerDownOutsideEvent {
    pub pointer_pos: Option<Pos2>,
    pub preventable: PopoverPreventable,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PopoverFocusOutsideEvent {
    pub preventable: PopoverPreventable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopoverInteractOutsideKind {
    PointerDown,
    Focus,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PopoverInteractOutsideEvent {
    pub kind: PopoverInteractOutsideKind,
    pub pointer_pos: Option<Pos2>,
    pub preventable: PopoverPreventable,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PopoverAutoFocusEvent {
    pub preventable: PopoverPreventable,
}

pub struct PopoverProps<'a> {
    pub id_source: Id,
    pub open: &'a mut bool,
    pub default_open: bool,
    pub on_open_change: Option<&'a mut dyn FnMut(bool)>,
    pub modal: bool,
    pub side: Option<PopoverSide>,
    pub placement: PopoverPlacement,
    pub align: PopoverAlign,
    pub align_offset: f32,
    pub side_offset: f32,
    pub avoid_collisions: bool,
    pub collision_boundary: Option<Rect>,
    pub collision_padding: PopoverCollisionPadding,
    pub arrow_padding: f32,
    pub sticky: PopoverSticky,
    pub hide_when_detached: bool,
    pub update_position_strategy: PopoverUpdatePositionStrategy,
    pub force_mount: bool,
    pub container: Option<PopoverPortalContainer>,
    pub on_escape_key_down: Option<&'a mut dyn FnMut(&mut PopoverEscapeKeyDownEvent)>,
    pub on_pointer_down_outside: Option<&'a mut dyn FnMut(&mut PopoverPointerDownOutsideEvent)>,
    pub on_focus_outside: Option<&'a mut dyn FnMut(&mut PopoverFocusOutsideEvent)>,
    pub on_interact_outside: Option<&'a mut dyn FnMut(&mut PopoverInteractOutsideEvent)>,
    pub on_open_auto_focus: Option<&'a mut dyn FnMut(&mut PopoverAutoFocusEvent)>,
    pub on_close_auto_focus: Option<&'a mut dyn FnMut(&mut PopoverAutoFocusEvent)>,
    pub width: Option<f32>,
    pub max_height: Option<f32>,
    pub match_trigger_width: bool,
    pub constrain_to_screen: bool,
    pub animate: bool,
}

impl std::fmt::Debug for PopoverProps<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PopoverProps")
            .field("id_source", &self.id_source)
            .field("open", &self.open)
            .field("default_open", &self.default_open)
            .field("modal", &self.modal)
            .field("side", &self.side)
            .field("placement", &self.placement)
            .field("align", &self.align)
            .field("align_offset", &self.align_offset)
            .field("side_offset", &self.side_offset)
            .field("avoid_collisions", &self.avoid_collisions)
            .field("collision_boundary", &self.collision_boundary)
            .field("collision_padding", &self.collision_padding)
            .field("arrow_padding", &self.arrow_padding)
            .field("sticky", &self.sticky)
            .field("hide_when_detached", &self.hide_when_detached)
            .field("update_position_strategy", &self.update_position_strategy)
            .field("force_mount", &self.force_mount)
            .field("container", &self.container)
            .field("width", &self.width)
            .field("max_height", &self.max_height)
            .field("match_trigger_width", &self.match_trigger_width)
            .field("constrain_to_screen", &self.constrain_to_screen)
            .field("animate", &self.animate)
            .field("on_open_change", &self.on_open_change.is_some())
            .field("on_escape_key_down", &self.on_escape_key_down.is_some())
            .field(
                "on_pointer_down_outside",
                &self.on_pointer_down_outside.is_some(),
            )
            .field("on_focus_outside", &self.on_focus_outside.is_some())
            .field("on_interact_outside", &self.on_interact_outside.is_some())
            .field("on_open_auto_focus", &self.on_open_auto_focus.is_some())
            .field("on_close_auto_focus", &self.on_close_auto_focus.is_some())
            .finish()
    }
}

impl<'a> PopoverProps<'a> {
    pub fn new(id_source: Id, open: &'a mut bool) -> Self {
        Self {
            id_source,
            open,
            default_open: false,
            on_open_change: None,
            modal: false,
            side: None,
            placement: PopoverPlacement::Below,
            align: PopoverAlign::Center,
            align_offset: 0.0,
            side_offset: 4.0,
            avoid_collisions: true,
            collision_boundary: None,
            collision_padding: PopoverCollisionPadding::default(),
            arrow_padding: 0.0,
            sticky: PopoverSticky::default(),
            hide_when_detached: false,
            update_position_strategy: PopoverUpdatePositionStrategy::default(),
            force_mount: false,
            container: None,
            on_escape_key_down: None,
            on_pointer_down_outside: None,
            on_focus_outside: None,
            on_interact_outside: None,
            on_open_auto_focus: None,
            on_close_auto_focus: None,
            width: None,
            max_height: None,
            match_trigger_width: false,
            constrain_to_screen: true,
            animate: true,
        }
    }

    pub fn with_placement(mut self, placement: PopoverPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn with_align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }

    pub fn with_align_offset(mut self, offset: f32) -> Self {
        self.align_offset = offset;
        self
    }

    pub fn with_side_offset(mut self, offset: f32) -> Self {
        self.side_offset = offset;
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

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = Some(side);
        self.placement = side.to_placement();
        self
    }

    pub fn side_offset(mut self, side_offset: f32) -> Self {
        self.side_offset = side_offset;
        self
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }

    pub fn align_offset(mut self, align_offset: f32) -> Self {
        self.align_offset = align_offset;
        self
    }

    pub fn avoid_collisions(mut self, avoid_collisions: bool) -> Self {
        self.avoid_collisions = avoid_collisions;
        self.constrain_to_screen = avoid_collisions;
        self
    }

    pub fn collision_boundary(mut self, boundary: Rect) -> Self {
        self.collision_boundary = Some(boundary);
        self
    }

    pub fn collision_padding(mut self, padding: PopoverCollisionPadding) -> Self {
        self.collision_padding = padding;
        self
    }

    pub fn arrow_padding(mut self, arrow_padding: f32) -> Self {
        self.arrow_padding = arrow_padding;
        self
    }

    pub fn sticky(mut self, sticky: PopoverSticky) -> Self {
        self.sticky = sticky;
        self
    }

    pub fn hide_when_detached(mut self, hide_when_detached: bool) -> Self {
        self.hide_when_detached = hide_when_detached;
        self
    }

    pub fn update_position_strategy(
        mut self,
        update_position_strategy: PopoverUpdatePositionStrategy,
    ) -> Self {
        self.update_position_strategy = update_position_strategy;
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn container(mut self, container: PopoverPortalContainer) -> Self {
        self.container = Some(container);
        self
    }

    pub fn on_escape_key_down(
        mut self,
        on_escape_key_down: &'a mut dyn FnMut(&mut PopoverEscapeKeyDownEvent),
    ) -> Self {
        self.on_escape_key_down = Some(on_escape_key_down);
        self
    }

    pub fn on_pointer_down_outside(
        mut self,
        on_pointer_down_outside: &'a mut dyn FnMut(&mut PopoverPointerDownOutsideEvent),
    ) -> Self {
        self.on_pointer_down_outside = Some(on_pointer_down_outside);
        self
    }

    pub fn on_focus_outside(
        mut self,
        on_focus_outside: &'a mut dyn FnMut(&mut PopoverFocusOutsideEvent),
    ) -> Self {
        self.on_focus_outside = Some(on_focus_outside);
        self
    }

    pub fn on_interact_outside(
        mut self,
        on_interact_outside: &'a mut dyn FnMut(&mut PopoverInteractOutsideEvent),
    ) -> Self {
        self.on_interact_outside = Some(on_interact_outside);
        self
    }

    pub fn on_open_auto_focus(
        mut self,
        on_open_auto_focus: &'a mut dyn FnMut(&mut PopoverAutoFocusEvent),
    ) -> Self {
        self.on_open_auto_focus = Some(on_open_auto_focus);
        self
    }

    pub fn on_close_auto_focus(
        mut self,
        on_close_auto_focus: &'a mut dyn FnMut(&mut PopoverAutoFocusEvent),
    ) -> Self {
        self.on_close_auto_focus = Some(on_close_auto_focus);
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn match_trigger_width(mut self, match_width: bool) -> Self {
        self.match_trigger_width = match_width;
        self
    }

    pub fn constrain_to_screen(mut self, constrain: bool) -> Self {
        self.constrain_to_screen = constrain;
        self.avoid_collisions = constrain;
        self
    }

    pub fn with_animation(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }
}

pub fn popover<R>(
    ui: &mut Ui,
    theme: &Theme,
    mut props: PopoverProps<'_>,
    trigger: impl FnOnce(&mut Ui) -> Response,
    content: impl FnOnce(&mut Ui) -> R,
) -> (Response, Option<R>) {
    let trigger_response = trigger(ui);
    let ctx = ui.ctx();
    let state_id = props.id_source.with("last-open");
    let init_id = props.id_source.with("default-open-initialized");
    let initialized = ctx.data(|d| d.get_temp::<bool>(init_id)).unwrap_or(false);
    if !initialized {
        *props.open = props.default_open;
        ctx.data_mut(|d| d.insert_temp(init_id, true));
    }

    let last_open = ctx.data(|d| d.get_temp::<bool>(state_id)).unwrap_or(false);

    if trigger_response.clicked() {
        let next = !*props.open;
        *props.open = next;
        if let Some(cb) = props.on_open_change.as_mut() {
            cb(next);
        }
    }

    let opened_now = *props.open && !last_open;

    let mut inner: Option<R> = None;
    let anim_t = if props.animate {
        ui.ctx()
            .animate_bool(props.id_source.with("open-anim"), *props.open)
    } else if *props.open {
        1.0
    } else {
        0.0
    };

    let is_mounted = *props.open || anim_t > 0.0 || props.force_mount;
    let is_visible = *props.open || anim_t > 0.0;

    if is_mounted {
        trace!("render popover {:?}", props.id_source);
        let palette = &theme.palette;
        let bg = mix(palette.input, palette.background, 0.85).gamma_multiply(anim_t);
        let border = mix(palette.border, palette.foreground, 0.15).gamma_multiply(anim_t);
        let rounding = CornerRadius::same(theme.radius.r3.round() as u8);
        let width = match (props.match_trigger_width, props.width) {
            (true, _) => trigger_response.rect.width().max(180.0),
            (false, Some(w)) => w,
            (false, None) => trigger_response.rect.width().max(220.0),
        };
        let max_height = props.max_height.unwrap_or(320.0);

        let screen = ui.ctx().available_rect();
        let boundary = props.collision_boundary.unwrap_or(screen);
        if props.hide_when_detached && !boundary.intersects(trigger_response.rect) {
            ctx.data_mut(|d| d.insert_temp(state_id, *props.open));
            return (trigger_response, None);
        }

        let side = props
            .side
            .unwrap_or_else(|| PopoverSide::from(props.placement));
        let (position_rect, computed_side) = compute_popover_rect_with_collision(
            trigger_response.rect,
            boundary,
            side,
            props.align,
            props.side_offset,
            props.align_offset,
            width,
            max_height,
            props.avoid_collisions && props.constrain_to_screen,
            props.collision_padding,
            props.sticky,
        );

        if props.modal && is_visible {
            let scrim_id = props.id_source.with("scrim");
            let scrim_rect = ui.ctx().available_rect();
            let _ = egui::Area::new(scrim_id)
                .order(Order::Foreground)
                .interactable(true)
                .movable(false)
                .fixed_pos(scrim_rect.min)
                .show(ui.ctx(), |scrim_ui| {
                    let _resp = scrim_ui.interact(scrim_rect, scrim_id, egui::Sense::click());
                });
        }

        let slide_offset = match computed_side {
            PopoverSide::Bottom => vec2(0.0, -8.0),
            PopoverSide::Top => vec2(0.0, 8.0),
            PopoverSide::Left => vec2(8.0, 0.0),
            PopoverSide::Right => vec2(-8.0, 0.0),
        };
        let animated_origin = position_rect.min + slide_offset * (1.0 - anim_t);

        let mut popup_rect = position_rect;
        let content_id = props.id_source.with("content");
        let order = props
            .container
            .unwrap_or(PopoverPortalContainer::Tooltip)
            .order();
        egui::Area::new(content_id)
            .order(order)
            .interactable(is_visible)
            .movable(false)
            .fixed_pos(animated_origin)
            .show(ui.ctx(), |popup_ui| {
                popup_ui.set_min_width(width);
                popup_ui.set_max_height(position_rect.height());
                let frame = Frame::popup(popup_ui.style())
                    .fill(bg)
                    .stroke(Stroke::new(1.0, border))
                    .corner_radius(rounding)
                    .inner_margin(Margin::symmetric(12, 10));

                let frame_resp = frame.show(popup_ui, |content_ui| {
                    if is_visible {
                        inner = Some(content(content_ui));
                    } else if props.force_mount {
                        let _ = content(content_ui);
                    }
                });
                popup_rect = frame_resp.response.rect;
            });

        if opened_now && props.modal {
            let mut evt = PopoverAutoFocusEvent {
                preventable: PopoverPreventable::default(),
            };
            if let Some(cb) = props.on_open_auto_focus.as_mut() {
                cb(&mut evt);
            }
            if !evt.preventable.default_prevented() {
                ui.ctx().memory_mut(|m| m.request_focus(content_id));
            }
        }

        let escape = ui.input(|i| i.key_pressed(egui::Key::Escape));
        if escape && is_visible {
            let mut evt = PopoverEscapeKeyDownEvent {
                key: egui::Key::Escape,
                preventable: PopoverPreventable::default(),
            };
            if let Some(cb) = props.on_escape_key_down.as_mut() {
                cb(&mut evt);
            }

            if !evt.preventable.default_prevented() {
                *props.open = false;
                if let Some(cb) = props.on_open_change.as_mut() {
                    cb(false);
                }
                let mut close_evt = PopoverAutoFocusEvent {
                    preventable: PopoverPreventable::default(),
                };
                if let Some(cb) = props.on_close_auto_focus.as_mut() {
                    cb(&mut close_evt);
                }
                if !close_evt.preventable.default_prevented() {
                    trigger_response.request_focus();
                }
            }
        }

        let (pointer_any_click, pointer_pos) =
            ui.input(|i| (i.pointer.any_click(), i.pointer.interact_pos()));
        let outside_click = is_visible
            && !opened_now
            && pointer_any_click
            && pointer_pos
                .map(|pos| !popup_rect.contains(pos))
                .unwrap_or(false);
        if outside_click {
            let mut pointer_evt = PopoverPointerDownOutsideEvent {
                pointer_pos,
                preventable: PopoverPreventable::default(),
            };
            if let Some(cb) = props.on_pointer_down_outside.as_mut() {
                cb(&mut pointer_evt);
            }

            let mut interact_evt = PopoverInteractOutsideEvent {
                kind: PopoverInteractOutsideKind::PointerDown,
                pointer_pos,
                preventable: PopoverPreventable::default(),
            };
            if let Some(cb) = props.on_interact_outside.as_mut() {
                cb(&mut interact_evt);
            }

            let prevented = pointer_evt.preventable.default_prevented()
                || interact_evt.preventable.default_prevented();
            if !prevented {
                *props.open = false;
                if let Some(cb) = props.on_open_change.as_mut() {
                    cb(false);
                }

                let mut close_evt = PopoverAutoFocusEvent {
                    preventable: PopoverPreventable::default(),
                };
                if let Some(cb) = props.on_close_auto_focus.as_mut() {
                    cb(&mut close_evt);
                }
                if !close_evt.preventable.default_prevented() {
                    trigger_response.request_focus();
                }
            }
        }
    }

    ctx.data_mut(|d| d.insert_temp(state_id, *props.open));

    (trigger_response, inner)
}

#[allow(clippy::too_many_arguments)]
pub fn compute_popover_rect_with_collision(
    trigger_rect: Rect,
    boundary: Rect,
    side: PopoverSide,
    align: PopoverAlign,
    side_offset: f32,
    align_offset: f32,
    width: f32,
    max_height: f32,
    avoid_collisions: bool,
    collision_padding: PopoverCollisionPadding,
    _sticky: PopoverSticky,
) -> (Rect, PopoverSide) {
    let boundary = Rect::from_min_max(
        pos2(
            boundary.left() + collision_padding.left,
            boundary.top() + collision_padding.top,
        ),
        pos2(
            boundary.right() - collision_padding.right,
            boundary.bottom() - collision_padding.bottom,
        ),
    );

    let available_height = max_height.min(boundary.height());

    let compute_for_side = |side: PopoverSide| -> Rect {
        let (left, top) = match side {
            PopoverSide::Top => {
                let top = trigger_rect.top() - side_offset - available_height;
                let left = match align {
                    PopoverAlign::Start => trigger_rect.left(),
                    PopoverAlign::Center => trigger_rect.center().x - width * 0.5,
                    PopoverAlign::End => trigger_rect.right() - width,
                } + align_offset;
                (left, top)
            }
            PopoverSide::Bottom => {
                let top = trigger_rect.bottom() + side_offset;
                let left = match align {
                    PopoverAlign::Start => trigger_rect.left(),
                    PopoverAlign::Center => trigger_rect.center().x - width * 0.5,
                    PopoverAlign::End => trigger_rect.right() - width,
                } + align_offset;
                (left, top)
            }
            PopoverSide::Left => {
                let left = trigger_rect.left() - side_offset - width;
                let top = match align {
                    PopoverAlign::Start => trigger_rect.top(),
                    PopoverAlign::Center => trigger_rect.center().y - available_height * 0.5,
                    PopoverAlign::End => trigger_rect.bottom() - available_height,
                } + align_offset;
                (left, top)
            }
            PopoverSide::Right => {
                let left = trigger_rect.right() + side_offset;
                let top = match align {
                    PopoverAlign::Start => trigger_rect.top(),
                    PopoverAlign::Center => trigger_rect.center().y - available_height * 0.5,
                    PopoverAlign::End => trigger_rect.bottom() - available_height,
                } + align_offset;
                (left, top)
            }
        };

        Rect::from_min_size(pos2(left, top), Vec2::new(width, available_height))
    };

    let mut computed_side = side;
    let mut rect = compute_for_side(side);

    if avoid_collisions {
        match side {
            PopoverSide::Bottom if rect.bottom() > boundary.bottom() => {
                let flipped = compute_for_side(PopoverSide::Top);
                if flipped.top() >= boundary.top() {
                    rect = flipped;
                    computed_side = PopoverSide::Top;
                }
            }
            PopoverSide::Top if rect.top() < boundary.top() => {
                let flipped = compute_for_side(PopoverSide::Bottom);
                if flipped.bottom() <= boundary.bottom() {
                    rect = flipped;
                    computed_side = PopoverSide::Bottom;
                }
            }
            PopoverSide::Right if rect.right() > boundary.right() => {
                let flipped = compute_for_side(PopoverSide::Left);
                if flipped.left() >= boundary.left() {
                    rect = flipped;
                    computed_side = PopoverSide::Left;
                }
            }
            PopoverSide::Left if rect.left() < boundary.left() => {
                let flipped = compute_for_side(PopoverSide::Right);
                if flipped.right() <= boundary.right() {
                    rect = flipped;
                    computed_side = PopoverSide::Right;
                }
            }
            _ => {}
        }

        let mut translation = vec2(0.0, 0.0);
        if rect.left() < boundary.left() {
            translation.x = boundary.left() - rect.left();
        } else if rect.right() > boundary.right() {
            translation.x = boundary.right() - rect.right();
        }

        if rect.top() < boundary.top() {
            translation.y = boundary.top() - rect.top();
        } else if rect.bottom() > boundary.bottom() {
            translation.y = boundary.bottom() - rect.bottom();
        }

        rect = rect.translate(translation);
        rect.set_height(rect.height().min(boundary.height()));
    }

    (rect, computed_side)
}

#[allow(clippy::too_many_arguments)]
pub fn compute_popover_rect(
    trigger_rect: Rect,
    screen: Rect,
    placement: PopoverPlacement,
    align: PopoverAlign,
    side_offset: f32,
    align_offset: f32,
    width: f32,
    max_height: f32,
    constrain_to_screen: bool,
) -> Rect {
    compute_popover_rect_with_collision(
        trigger_rect,
        screen,
        PopoverSide::from(placement),
        align,
        side_offset,
        align_offset,
        width,
        max_height,
        constrain_to_screen,
        PopoverCollisionPadding::default(),
        PopoverSticky::default(),
    )
    .0
}
