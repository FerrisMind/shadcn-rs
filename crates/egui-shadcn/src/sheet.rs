use crate::theme::Theme;
use egui::{
    Align, Align2, Color32, CornerRadius, FontId, Frame, Id, LayerId, Margin, Order, Rect,
    Response, Sense, Stroke, StrokeKind, Ui, Vec2, pos2, vec2,
};
use lucide_icons::Icon;

const DEFAULT_MAX_WIDTH: f32 = 384.0;
const DEFAULT_MAX_HEIGHT: f32 = 320.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

#[derive(Debug)]
pub struct SheetProps<'a> {
    pub id_source: Id,
    pub open: &'a mut bool,
    pub side: SheetSide,
}

impl<'a> SheetProps<'a> {
    pub fn new(id_source: Id, open: &'a mut bool) -> Self {
        Self {
            id_source,
            open,
            side: SheetSide::Right,
        }
    }

    pub fn side(mut self, side: SheetSide) -> Self {
        self.side = side;
        self
    }
}

pub struct SheetContext<'a> {
    pub id_source: Id,
    pub open: &'a mut bool,
    pub side: SheetSide,
}

pub fn sheet<R>(
    ui: &mut Ui,
    props: SheetProps<'_>,
    add_contents: impl FnOnce(&mut Ui, &mut SheetContext) -> R,
) -> R {
    let mut context = SheetContext {
        id_source: props.id_source,
        open: props.open,
        side: props.side,
    };
    add_contents(ui, &mut context)
}

pub fn sheet_trigger(
    ui: &mut Ui,
    context: &mut SheetContext,
    add_trigger: impl FnOnce(&mut Ui) -> Response,
) -> Response {
    let response = ui
        .push_id(context.id_source.with("trigger"), |ui| add_trigger(ui))
        .inner;
    if response.clicked() {
        *context.open = !*context.open;
        ui.ctx().request_repaint();
    }
    response
}

pub fn sheet_content<R>(
    ui: &mut Ui,
    theme: &Theme,
    context: &mut SheetContext,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let ctx = ui.ctx();
    let state_id = context.id_source.with("last-open");
    let last_open = ctx.data(|d| d.get_temp::<bool>(state_id)).unwrap_or(false);
    let was_open = *context.open || last_open;
    let opened_now = *context.open && !last_open;

    let anim_t = ctx.animate_bool(context.id_source.with("open-anim"), *context.open);
    if !*context.open && anim_t <= 0.0 {
        ctx.data_mut(|d| d.insert_temp(state_id, *context.open));
        return None;
    }

    let screen = ctx.available_rect();
    let (panel_size, base_pos, offset) = sheet_layout(screen, context.side);
    let animated_pos = base_pos + offset * (1.0 - anim_t);

    let overlay_id = LayerId::new(Order::Foreground, context.id_source.with("overlay"));
    let overlay_painter = ctx.layer_painter(overlay_id);
    let scrim_alpha = (160.0 * anim_t).round().clamp(0.0, 255.0) as u8;
    overlay_painter.rect_filled(
        screen,
        CornerRadius::same(0),
        Color32::from_rgba_unmultiplied(0, 0, 0, scrim_alpha),
    );

    egui::Area::new(context.id_source.with("scrim"))
        .order(Order::Foreground)
        .interactable(true)
        .movable(false)
        .fixed_pos(screen.min)
        .show(ctx, |scrim_ui| {
            scrim_ui.allocate_exact_size(screen.size(), Sense::click());
        });

    let mut result = None;
    let content_id = context.id_source.with("content");
    egui::Area::new(content_id)
        .order(Order::Tooltip)
        .interactable(true)
        .movable(false)
        .fixed_pos(animated_pos)
        .show(ctx, |area_ui| {
            let border = Stroke::new(1.0, theme.palette.border);
            let shadow = egui::epaint::Shadow {
                offset: [0, 4],
                blur: 12,
                spread: 0,
                color: Color32::from_rgba_unmultiplied(
                    theme.palette.foreground.r(),
                    theme.palette.foreground.g(),
                    theme.palette.foreground.b(),
                    70,
                ),
            };
            let frame = Frame::popup(area_ui.style())
                .fill(theme.palette.background)
                .stroke(border)
                .shadow(shadow)
                .corner_radius(CornerRadius::same(0));

            let frame_resp = frame.show(area_ui, |content_ui| {
                content_ui.set_min_width(panel_size.x);
                content_ui.set_max_width(panel_size.x);
                content_ui.set_min_height(panel_size.y);
                content_ui.set_max_height(panel_size.y);
                content_ui.spacing_mut().item_spacing.y = 16.0;
                content_ui.set_min_size(panel_size);
                result = Some(add_contents(content_ui));
            });

            let close_text = Icon::X.unicode().to_string();
            let close_size = vec2(24.0, 24.0);
            let offset = 16.0;
            let close_rect = Rect::from_min_size(
                pos2(
                    frame_resp.response.rect.right() - offset - close_size.x,
                    frame_resp.response.rect.top() + offset,
                ),
                close_size,
            );
            let close_id = content_id.with("close-button");
            let close_resp = area_ui.interact(close_rect, close_id, Sense::click());

            let icon_color = if close_resp.hovered() {
                theme.palette.foreground
            } else {
                theme.palette.muted_foreground
            };

            area_ui.painter().text(
                close_rect.center(),
                Align2::CENTER_CENTER,
                close_text,
                FontId::proportional(16.0),
                icon_color,
            );

            if close_resp.has_focus() {
                let ring_color = theme.palette.ring;
                area_ui.painter().rect_stroke(
                    close_rect,
                    CornerRadius::same(2),
                    Stroke::new(2.0, ring_color),
                    StrokeKind::Outside,
                );
            }

            if close_resp.clicked() {
                close_resp.request_focus();
                *context.open = false;
            }

            let escape = area_ui.input(|i| i.key_pressed(egui::Key::Escape));
            let any_click = area_ui.input(|i| i.pointer.any_click());
            let interact = area_ui.input(|i| i.pointer.interact_pos());
            let contains = interact
                .map(|pos| frame_resp.response.rect.contains(pos))
                .unwrap_or(false);
            let outside_click = !opened_now && was_open && any_click && !contains;

            if escape || outside_click {
                *context.open = false;
            }
        });

    ctx.data_mut(|d| d.insert_temp(state_id, *context.open));

    result
}

pub fn sheet_header<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    Frame::default()
        .inner_margin(Margin::same(16))
        .show(ui, |content_ui| {
            content_ui.spacing_mut().item_spacing.y = 6.0;
            add_contents(content_ui)
        })
        .inner
}

pub fn sheet_footer<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    Frame::default()
        .inner_margin(Margin::same(16))
        .show(ui, |content_ui| {
            content_ui.spacing_mut().item_spacing.y = 8.0;
            content_ui
                .with_layout(egui::Layout::top_down(Align::Min), add_contents)
                .inner
        })
        .inner
}

pub fn sheet_title(ui: &mut Ui, theme: &Theme, text: impl Into<egui::WidgetText>) -> Response {
    let widget_text: egui::WidgetText = text.into();
    let base = match widget_text {
        egui::WidgetText::RichText(text) => (*text).clone(),
        _ => egui::RichText::new(widget_text.text().to_string()),
    };
    ui.label(base.size(16.0).strong().color(theme.palette.foreground))
}

pub fn sheet_description(
    ui: &mut Ui,
    theme: &Theme,
    text: impl Into<egui::WidgetText>,
) -> Response {
    let widget_text: egui::WidgetText = text.into();
    let base = match widget_text {
        egui::WidgetText::RichText(text) => (*text).clone(),
        _ => egui::RichText::new(widget_text.text().to_string()),
    };
    ui.label(base.size(12.0).color(theme.palette.muted_foreground))
}

fn sheet_layout(screen: Rect, side: SheetSide) -> (Vec2, egui::Pos2, Vec2) {
    let max_width = DEFAULT_MAX_WIDTH.min(screen.width());
    let max_height = DEFAULT_MAX_HEIGHT.min(screen.height());

    match side {
        SheetSide::Left => {
            let width = (screen.width() * 0.75).min(max_width).max(240.0);
            let size = Vec2::new(width, screen.height());
            let base = pos2(screen.left(), screen.top());
            let offset = vec2(-size.x, 0.0);
            (size, base, offset)
        }
        SheetSide::Right => {
            let width = (screen.width() * 0.75).min(max_width).max(240.0);
            let size = Vec2::new(width, screen.height());
            let base = pos2(screen.right() - size.x, screen.top());
            let offset = vec2(size.x, 0.0);
            (size, base, offset)
        }
        SheetSide::Top => {
            let height = (screen.height() * 0.4).min(max_height).max(200.0);
            let size = Vec2::new(screen.width(), height);
            let base = pos2(screen.left(), screen.top());
            let offset = vec2(0.0, -size.y);
            (size, base, offset)
        }
        SheetSide::Bottom => {
            let height = (screen.height() * 0.4).min(max_height).max(200.0);
            let size = Vec2::new(screen.width(), height);
            let base = pos2(screen.left(), screen.bottom() - size.y);
            let offset = vec2(0.0, size.y);
            (size, base, offset)
        }
    }
}
