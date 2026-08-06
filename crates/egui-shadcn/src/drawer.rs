//! Drawer component - slide-in panel, typically from the bottom.
//!
//! Drawer is similar to Sheet but defaults to Bottom side and is typically
//! used for mobile-style bottom sheets.
//!
//! # Example
//! ```ignore
//! drawer(ui, &theme, DrawerProps::new(true).title("Actions"), |ui| { /* content */ });
//! ```

use crate::theme::Theme;
use egui::{
    Color32, CornerRadius, Frame, Id, LayerId, Order, Rect, Response, Sense, Ui, Vec2, pos2, vec2,
};

/// Side from which the drawer slides in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerSide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

/// Properties for the Drawer component.
#[derive(Debug)]
pub struct DrawerProps<'a> {
    pub id_source: Id,
    pub open: &'a mut bool,
    pub side: DrawerSide,
}

impl<'a> DrawerProps<'a> {
    pub fn new(id_source: Id, open: &'a mut bool) -> Self {
        Self {
            id_source,
            open,
            side: DrawerSide::Bottom,
        }
    }

    pub fn side(mut self, side: DrawerSide) -> Self {
        self.side = side;
        self
    }
}

/// Render a drawer panel.
pub fn drawer<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: DrawerProps<'_>,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let ctx = ui.ctx();
    let anim_t = ctx.animate_bool(props.id_source.with("drawer-anim"), *props.open);

    if !*props.open && anim_t <= 0.0 {
        return None;
    }

    let screen = ctx.available_rect();

    // Scrim overlay
    let overlay_id = LayerId::new(Order::Foreground, props.id_source.with("drawer-overlay"));
    let overlay_painter = ctx.layer_painter(overlay_id);
    let scrim_alpha = (160.0 * anim_t).round().clamp(0.0, 255.0) as u8;
    overlay_painter.rect_filled(
        screen,
        CornerRadius::same(0),
        Color32::from_rgba_unmultiplied(0, 0, 0, scrim_alpha),
    );

    // Click outside to close
    egui::Area::new(props.id_source.with("drawer-scrim"))
        .order(Order::Foreground)
        .interactable(true)
        .movable(false)
        .fixed_pos(screen.min)
        .show(ctx, |scrim_ui| {
            scrim_ui.allocate_exact_size(screen.size(), Sense::click());
        });

    let (panel_size, base_pos, offset) = drawer_layout(screen, props.side);
    let animated_pos = base_pos + offset * (1.0 - anim_t);

    let mut result = None;
    egui::Area::new(props.id_source.with("drawer-content"))
        .order(Order::Tooltip)
        .interactable(true)
        .movable(false)
        .fixed_pos(animated_pos)
        .show(ctx, |area_ui| {
            let rounding = match props.side {
                DrawerSide::Bottom => CornerRadius {
                    nw: 12,
                    ne: 12,
                    sw: 0,
                    se: 0,
                },
                DrawerSide::Top => CornerRadius {
                    nw: 0,
                    ne: 0,
                    sw: 12,
                    se: 12,
                },
                DrawerSide::Left => CornerRadius {
                    nw: 0,
                    ne: 12,
                    sw: 0,
                    se: 12,
                },
                DrawerSide::Right => CornerRadius {
                    nw: 12,
                    ne: 0,
                    sw: 12,
                    se: 0,
                },
            };

            let frame = Frame::popup(area_ui.style())
                .fill(theme.palette.background)
                .stroke(egui::Stroke::new(1.0_f32, theme.palette.border))
                .corner_radius(rounding);

            frame.show(area_ui, |content_ui| {
                content_ui.set_min_width(panel_size.x);
                content_ui.set_max_width(panel_size.x);
                content_ui.set_min_height(panel_size.y);
                content_ui.set_max_height(panel_size.y);
                result = Some(add_contents(content_ui));
            });

            let escape = area_ui.input(|i| i.key_pressed(egui::Key::Escape));
            if escape {
                *props.open = false;
            }
        });

    result
}

fn drawer_layout(screen: Rect, side: DrawerSide) -> (Vec2, egui::Pos2, Vec2) {
    match side {
        DrawerSide::Bottom => {
            let height = (screen.height() * 0.4).clamp(200.0, 320.0);
            let size = Vec2::new(screen.width(), height);
            let base = pos2(screen.left(), screen.bottom() - size.y);
            let offset = vec2(0.0, size.y);
            (size, base, offset)
        }
        DrawerSide::Top => {
            let height = (screen.height() * 0.4).clamp(200.0, 320.0);
            let size = Vec2::new(screen.width(), height);
            let base = pos2(screen.left(), screen.top());
            let offset = vec2(0.0, -size.y);
            (size, base, offset)
        }
        DrawerSide::Left => {
            let width = (screen.width() * 0.75).clamp(240.0, 320.0);
            let size = Vec2::new(width, screen.height());
            let base = pos2(screen.left(), screen.top());
            let offset = vec2(-size.x, 0.0);
            (size, base, offset)
        }
        DrawerSide::Right => {
            let width = (screen.width() * 0.75).clamp(240.0, 320.0);
            let size = Vec2::new(width, screen.height());
            let base = pos2(screen.right() - size.x, screen.top());
            let offset = vec2(size.x, 0.0);
            (size, base, offset)
        }
    }
}

/// Render drawer title text.
pub fn drawer_title(ui: &mut Ui, theme: &Theme, title: impl Into<egui::WidgetText>) -> Response {
    let text: egui::WidgetText = title.into();
    let base = match text {
        egui::WidgetText::RichText(t) => (*t).clone(),
        _ => egui::RichText::new(text.text().to_string()),
    };
    ui.label(base.size(16.0).strong().color(theme.palette.foreground))
}

/// Render drawer description text.
pub fn drawer_description(
    ui: &mut Ui,
    theme: &Theme,
    desc: impl Into<egui::WidgetText>,
) -> Response {
    let text: egui::WidgetText = desc.into();
    let base = match text {
        egui::WidgetText::RichText(t) => (*t).clone(),
        _ => egui::RichText::new(text.text().to_string()),
    };
    ui.label(base.size(12.0).color(theme.palette.muted_foreground))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawer_side_default() {
        assert_eq!(DrawerSide::default(), DrawerSide::Bottom);
    }
}
