use crate::theme::Theme;
use crate::tokens::mix;
use egui::{CornerRadius, Frame, Id, Margin, Order, Rect, Response, Stroke, Ui, Vec2, pos2, vec2};
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

#[derive(Debug)]
pub struct PopoverProps<'a> {
    pub id_source: Id,
    pub open: &'a mut bool,
    pub placement: PopoverPlacement,
    pub align: PopoverAlign,
    pub align_offset: f32,
    pub side_offset: f32,
    pub width: Option<f32>,
    pub max_height: Option<f32>,
    pub match_trigger_width: bool,
    pub constrain_to_screen: bool,
    pub animate: bool,
}

impl<'a> PopoverProps<'a> {
    pub fn new(id_source: Id, open: &'a mut bool) -> Self {
        Self {
            id_source,
            open,
            placement: PopoverPlacement::Below,
            align: PopoverAlign::Center,
            align_offset: 0.0,
            side_offset: 4.0,
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
    props: PopoverProps<'_>,
    trigger: impl FnOnce(&mut Ui) -> Response,
    content: impl FnOnce(&mut Ui) -> R,
) -> (Response, Option<R>) {
    let trigger_response = trigger(ui);
    if trigger_response.clicked() {
        *props.open = !*props.open;
    }

    let ctx = ui.ctx();
    let state_id = props.id_source.with("last-open");
    let last_open = ctx.data(|d| d.get_temp::<bool>(state_id)).unwrap_or(false);
    let opened_now = *props.open && !last_open;

    let mut inner: Option<R> = None;
    if *props.open {
        trace!("render popover {:?}", props.id_source);
        let palette = &theme.palette;
        let bg = mix(palette.input, palette.background, 0.85);
        let border = mix(palette.border, palette.foreground, 0.15);
        let rounding = CornerRadius::same(theme.radius.r3.round() as u8);
        let width = match (props.match_trigger_width, props.width) {
            (true, _) => trigger_response.rect.width().max(180.0),
            (false, Some(w)) => w,
            (false, None) => trigger_response.rect.width().max(220.0),
        };
        let max_height = props.max_height.unwrap_or(320.0);
        let screen = ui.ctx().available_rect();

        let position_rect = compute_popover_rect(
            trigger_response.rect,
            screen,
            props.placement,
            props.align,
            props.side_offset,
            props.align_offset,
            width,
            max_height,
            props.constrain_to_screen,
        );
        let anim_t = if props.animate {
            ui.ctx()
                .animate_bool(props.id_source.with("open-anim"), *props.open)
        } else {
            1.0
        };

        let slide_offset = match props.placement {
            PopoverPlacement::Below => vec2(0.0, -8.0),
            PopoverPlacement::Above => vec2(0.0, 8.0),
            PopoverPlacement::Left => vec2(8.0, 0.0),
            PopoverPlacement::Right => vec2(-8.0, 0.0),
        };
        let animated_origin = position_rect.min + slide_offset * (1.0 - anim_t);

        let mut popup_rect = position_rect;
        egui::Area::new(props.id_source.with("content"))
            .order(Order::Tooltip)
            .interactable(true)
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
                    inner = Some(content(content_ui));
                });
                popup_rect = frame_resp.response.rect;
            });

        let escape = ui.input(|i| i.key_pressed(egui::Key::Escape));
        let (any_click, interact_pos) =
            ui.input(|i| (i.pointer.any_click(), i.pointer.interact_pos()));
        let outside_click = !opened_now
            && any_click
            && interact_pos
                .map(|pos| !popup_rect.contains(pos))
                .unwrap_or(false);
        if escape || outside_click {
            *props.open = false;
        }
    }

    ctx.data_mut(|d| d.insert_temp(state_id, *props.open));

    (trigger_response, inner)
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
    let available_height = max_height.min(screen.height());
    let (left, top) = match placement {
        PopoverPlacement::Above => {
            let top = trigger_rect.top() - side_offset - available_height;
            let left = match align {
                PopoverAlign::Start => trigger_rect.left(),
                PopoverAlign::Center => trigger_rect.center().x - width * 0.5,
                PopoverAlign::End => trigger_rect.right() - width,
            } + align_offset;
            (left, top)
        }
        PopoverPlacement::Below => {
            let top = trigger_rect.bottom() + side_offset;
            let left = match align {
                PopoverAlign::Start => trigger_rect.left(),
                PopoverAlign::Center => trigger_rect.center().x - width * 0.5,
                PopoverAlign::End => trigger_rect.right() - width,
            } + align_offset;
            (left, top)
        }
        PopoverPlacement::Left => {
            let left = trigger_rect.left() - side_offset - width;
            let top = match align {
                PopoverAlign::Start => trigger_rect.top(),
                PopoverAlign::Center => trigger_rect.center().y - available_height * 0.5,
                PopoverAlign::End => trigger_rect.bottom() - available_height,
            } + align_offset;
            (left, top)
        }
        PopoverPlacement::Right => {
            let left = trigger_rect.right() + side_offset;
            let top = match align {
                PopoverAlign::Start => trigger_rect.top(),
                PopoverAlign::Center => trigger_rect.center().y - available_height * 0.5,
                PopoverAlign::End => trigger_rect.bottom() - available_height,
            } + align_offset;
            (left, top)
        }
    };

    let mut rect = Rect::from_min_size(pos2(left, top), Vec2::new(width, available_height));
    if constrain_to_screen {
        let mut translation = vec2(0.0, 0.0);
        if rect.left() < screen.left() {
            translation.x = screen.left() - rect.left();
        } else if rect.right() > screen.right() {
            translation.x = screen.right() - rect.right();
        }

        if rect.top() < screen.top() {
            translation.y = screen.top() - rect.top();
        } else if rect.bottom() > screen.bottom() {
            translation.y = screen.bottom() - rect.bottom();
        }

        rect = rect.translate(translation);
        let height = rect.height().min(screen.height());
        rect.set_height(height);
    }

    rect
}
