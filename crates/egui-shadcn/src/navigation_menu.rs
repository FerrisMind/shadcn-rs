//! Navigation Menu component - top navigation with dropdown content.

use crate::icons::icon_chevrons_up_down;
use crate::popover::{
    PopoverAlign, PopoverCollisionPadding, PopoverSide, PopoverSticky,
    compute_popover_rect_with_collision,
};
use crate::theme::Theme;
use crate::{Button, ButtonSize, ButtonVariant};
use egui::{
    Color32, CornerRadius, Frame, Id, Margin, Order, Rect, Response, Sense, Stroke, Ui, UiBuilder,
    vec2,
};

const DEFAULT_CONTENT_WIDTH: f32 = 320.0;
const DEFAULT_CONTENT_MAX_HEIGHT: f32 = 360.0;

#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuProps {
    pub id_source: Id,
    pub item_gap: f32,
    pub indicator_width: f32,
    pub indicator_height: f32,
    pub indicator_offset: f32,
}

impl NavigationMenuProps {
    pub fn new(id_source: Id) -> Self {
        Self {
            id_source,
            item_gap: 6.0,
            indicator_width: 24.0,
            indicator_height: 2.0,
            indicator_offset: 2.0,
        }
    }

    pub fn item_gap(mut self, item_gap: f32) -> Self {
        self.item_gap = item_gap;
        self
    }

    pub fn indicator_width(mut self, indicator_width: f32) -> Self {
        self.indicator_width = indicator_width;
        self
    }

    pub fn indicator_height(mut self, indicator_height: f32) -> Self {
        self.indicator_height = indicator_height;
        self
    }

    pub fn indicator_offset(mut self, indicator_offset: f32) -> Self {
        self.indicator_offset = indicator_offset;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuContext {
    pub id_source: Id,
    pub item_gap: f32,
    pub indicator_width: f32,
    pub indicator_height: f32,
    pub indicator_offset: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuItemContext {
    pub menu_id: Id,
    pub item_id: Id,
}

#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuContentProps {
    pub width: Option<f32>,
    pub max_height: Option<f32>,
    pub side: PopoverSide,
    pub align: PopoverAlign,
    pub side_offset: f32,
    pub align_offset: f32,
    pub padding: Margin,
}

impl Default for NavigationMenuContentProps {
    fn default() -> Self {
        Self::new()
    }
}

impl NavigationMenuContentProps {
    pub fn new() -> Self {
        Self {
            width: None,
            max_height: None,
            side: PopoverSide::Bottom,
            align: PopoverAlign::Start,
            side_offset: 6.0,
            align_offset: 0.0,
            padding: Margin::same(12),
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
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

    pub fn padding(mut self, padding: Margin) -> Self {
        self.padding = padding;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuLinkProps {
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub padding: Margin,
    pub rounding: CornerRadius,
    pub active: bool,
    pub disabled: bool,
}

impl Default for NavigationMenuLinkProps {
    fn default() -> Self {
        Self::new()
    }
}

impl NavigationMenuLinkProps {
    pub fn new() -> Self {
        Self {
            min_width: None,
            min_height: None,
            padding: Margin::symmetric(12, 8),
            rounding: CornerRadius::same(6),
            active: false,
            disabled: false,
        }
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    pub fn min_height(mut self, height: f32) -> Self {
        self.min_height = Some(height);
        self
    }

    pub fn padding(mut self, padding: Margin) -> Self {
        self.padding = padding;
        self
    }

    pub fn rounding(mut self, rounding: CornerRadius) -> Self {
        self.rounding = rounding;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NavigationMenuLinkState {
    pub hovered: bool,
    pub active: bool,
    pub disabled: bool,
}

pub struct NavigationMenuLinkResponse<R> {
    pub response: Response,
    pub inner: R,
}

#[derive(Clone, Debug, Default)]
struct NavigationMenuState {
    open_item: Option<Id>,
    indicator_rect: Option<Rect>,
    menu_hovered: bool,
    content_hovered: bool,
    hover_bounds: Option<Rect>,
}

fn nav_state_id(id_source: Id) -> Id {
    id_source.with("navigation-menu-state")
}

fn trigger_rect_id(item_id: Id) -> Id {
    item_id.with("navigation-menu-trigger-rect")
}

pub fn navigation_menu<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: NavigationMenuProps,
    add_contents: impl FnOnce(&mut Ui, &NavigationMenuContext) -> R,
) -> R {
    let ctx = NavigationMenuContext {
        id_source: props.id_source,
        item_gap: props.item_gap,
        indicator_width: props.indicator_width,
        indicator_height: props.indicator_height,
        indicator_offset: props.indicator_offset,
    };

    let state_id = nav_state_id(ctx.id_source);
    ui.ctx().data_mut(|d| {
        let mut state = d
            .get_temp::<NavigationMenuState>(state_id)
            .unwrap_or_default();
        state.menu_hovered = false;
        state.content_hovered = false;
        state.indicator_rect = None;
        state.hover_bounds = None;
        d.insert_temp(state_id, state);
    });

    let inner = add_contents(ui, &ctx);

    let mut state = ui
        .ctx()
        .data(|d| d.get_temp::<NavigationMenuState>(state_id))
        .unwrap_or_default();

    let pointer_pos = ui.ctx().input(|i| i.pointer.hover_pos());
    let pointer_in_bounds = pointer_pos.is_some_and(|pos| {
        state
            .hover_bounds
            .is_some_and(|bounds| bounds.contains(pos))
    });
    if !state.menu_hovered && !state.content_hovered && !pointer_in_bounds {
        state.open_item = None;
    }

    if let Some(rect) = state.indicator_rect {
        let indicator_rect = Rect::from_center_size(
            egui::pos2(
                rect.center().x,
                rect.bottom() + ctx.indicator_offset + ctx.indicator_height / 2.0,
            ),
            vec2(ctx.indicator_width, ctx.indicator_height),
        );
        ui.painter().rect_filled(
            indicator_rect,
            CornerRadius::same(ctx.indicator_height.round() as u8),
            theme.palette.border,
        );
    }

    ui.ctx().data_mut(|d| d.insert_temp(state_id, state));

    inner
}

pub fn navigation_menu_list<R>(
    ui: &mut Ui,
    ctx: &NavigationMenuContext,
    add_contents: impl FnOnce(&mut Ui, &NavigationMenuContext) -> R,
) -> R {
    ui.horizontal(|list_ui| {
        list_ui.spacing_mut().item_spacing = vec2(ctx.item_gap, 0.0);
        add_contents(list_ui, ctx)
    })
    .inner
}

pub fn navigation_menu_item<R, IdSource: std::hash::Hash>(
    ui: &mut Ui,
    ctx: &NavigationMenuContext,
    id_source: IdSource,
    add_contents: impl FnOnce(&mut Ui, &NavigationMenuItemContext) -> R,
) -> R {
    let item_id = ctx.id_source.with(id_source);
    let item_ctx = NavigationMenuItemContext {
        menu_id: ctx.id_source,
        item_id,
    };
    ui.push_id(item_id, |item_ui| add_contents(item_ui, &item_ctx))
        .inner
}

pub fn navigation_menu_trigger(
    ui: &mut Ui,
    theme: &Theme,
    ctx: &NavigationMenuContext,
    item_ctx: &NavigationMenuItemContext,
    label: impl Into<egui::WidgetText>,
) -> Response {
    let response = Button::new(label)
        .variant(ButtonVariant::Ghost)
        .size(ButtonSize::Sm)
        .trailing_icon(&icon_chevrons_up_down)
        .show(ui, theme);

    let state_id = nav_state_id(ctx.id_source);
    let hovered = response.hovered() || response.has_focus();
    let clicked = response.clicked();
    ui.ctx().data_mut(|d| {
        d.insert_temp(trigger_rect_id(item_ctx.item_id), response.rect);
        let mut state = d
            .get_temp::<NavigationMenuState>(state_id)
            .unwrap_or_default();
        if hovered || clicked {
            state.open_item = Some(item_ctx.item_id);
            state.menu_hovered = true;
        }
        if state.open_item == Some(item_ctx.item_id) {
            state.indicator_rect = Some(response.rect);
        }
        d.insert_temp(state_id, state);
    });

    response
}

pub fn navigation_menu_content<R>(
    ui: &mut Ui,
    theme: &Theme,
    ctx: &NavigationMenuContext,
    item_ctx: &NavigationMenuItemContext,
    props: NavigationMenuContentProps,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let state_id = nav_state_id(ctx.id_source);
    let state = ui
        .ctx()
        .data(|d| d.get_temp::<NavigationMenuState>(state_id))
        .unwrap_or_default();
    if state.open_item != Some(item_ctx.item_id) {
        return None;
    }

    let trigger_rect = ui
        .ctx()
        .data(|d| d.get_temp::<Rect>(trigger_rect_id(item_ctx.item_id)))?;

    let palette = &theme.palette;
    let width = props.width.unwrap_or(DEFAULT_CONTENT_WIDTH);
    let max_height = props.max_height.unwrap_or(DEFAULT_CONTENT_MAX_HEIGHT);
    let boundary = ui.ctx().available_rect();

    let (position_rect, _computed_side) = compute_popover_rect_with_collision(
        trigger_rect,
        boundary,
        props.side,
        props.align,
        props.side_offset,
        props.align_offset,
        width,
        max_height,
        true,
        PopoverCollisionPadding::default(),
        PopoverSticky::default(),
    );

    let mut popup_rect = position_rect;
    let mut inner: Option<R> = None;
    let content_id = item_ctx.item_id.with("content");

    egui::Area::new(content_id)
        .order(Order::Tooltip)
        .interactable(true)
        .movable(false)
        .fixed_pos(position_rect.min)
        .show(ui.ctx(), |popup_ui| {
            popup_ui.visuals_mut().override_text_color = Some(palette.popover_foreground);
            popup_ui.set_min_width(width);
            popup_ui.set_max_height(position_rect.height());

            let frame = Frame::popup(popup_ui.style())
                .fill(palette.popover)
                .stroke(Stroke::new(1.0, palette.border))
                .corner_radius(CornerRadius::same(theme.radius.r3.round() as u8))
                .inner_margin(props.padding);

            let frame_resp = frame.show(popup_ui, |content_ui| {
                inner = Some(add_contents(content_ui));
            });
            popup_rect = frame_resp.response.rect;
        });

    let expanded_rect = popup_rect.expand(4.0);
    let content_hovered = ui
        .ctx()
        .input(|i| i.pointer.hover_pos())
        .is_some_and(|pos| expanded_rect.contains(pos));

    ui.ctx().data_mut(|d| {
        let mut state = d
            .get_temp::<NavigationMenuState>(state_id)
            .unwrap_or_default();
        state.content_hovered = content_hovered;
        state.hover_bounds = Some(trigger_rect.union(popup_rect).expand(6.0));
        if content_hovered {
            state.menu_hovered = true;
        }
        d.insert_temp(state_id, state);
    });

    inner
}

pub fn navigation_menu_link<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: NavigationMenuLinkProps,
    add_contents: impl FnOnce(&mut Ui, NavigationMenuLinkState) -> R,
) -> NavigationMenuLinkResponse<R> {
    let palette = &theme.palette;
    let desired_width = props.min_width.unwrap_or(ui.spacing().interact_size.x);
    let desired_height = props.min_height.unwrap_or(ui.spacing().interact_size.y);
    let sense = if props.disabled {
        Sense::hover()
    } else {
        Sense::click()
    };
    let (rect, response) = ui.allocate_exact_size(vec2(desired_width, desired_height), sense);
    let hovered = response.hovered() || response.has_focus();

    let fill = if hovered && !props.disabled {
        palette.accent
    } else if props.active {
        palette.muted
    } else {
        Color32::TRANSPARENT
    };

    if fill != Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, props.rounding, fill);
    }

    let inner_rect = Rect::from_min_max(
        rect.min + vec2(props.padding.left as f32, props.padding.top as f32),
        rect.max - vec2(props.padding.right as f32, props.padding.bottom as f32),
    );
    let state = NavigationMenuLinkState {
        hovered,
        active: props.active,
        disabled: props.disabled,
    };
    let inner = ui.scope_builder(UiBuilder::new().max_rect(inner_rect), |content_ui| {
        content_ui.spacing_mut().item_spacing = vec2(0.0, 4.0);
        add_contents(content_ui, state)
    });

    let mut final_response = response;
    if !props.disabled {
        final_response = final_response.on_hover_cursor(egui::CursorIcon::PointingHand);
    }

    NavigationMenuLinkResponse {
        response: final_response,
        inner: inner.inner,
    }
}
