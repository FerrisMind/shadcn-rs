//! Dialog overlay with Radix-like layout controls, alignments, and portal/container options.
//!
//! # Example
//! ```rust
//! use egui_shadcn::{dialog, DialogAlign, DialogProps, DialogSize, Theme};
//!
//! fn ui(ui: &mut egui::Ui, theme: &Theme, open: &mut bool) {
//!     dialog(
//!         ui,
//!         theme,
//!         DialogProps {
//!             open,
//!             title: Some("Confirm"),
//!             size: DialogSize::Size3,
//!             align: DialogAlign::Center,
//!             ..Default::default()
//!         },
//!         |ui| {
//!             ui.label("Are you sure?");
//!         },
//!     );
//! }
//! ```

use crate::scroll_area::{ScrollAreaProps, ScrollDirection, scroll_area};
use crate::theme::Theme;
use egui::{
    Align2, Color32, CornerRadius, FontId, Frame, Id, LayerId, Margin, Order, Rect, Sense, Stroke,
    StrokeKind, Ui, Vec2, pos2, vec2,
};
use log::trace;
use lucide_icons::Icon;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogAlign {
    Start,
    Center,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DialogSize {
    Size1,
    Size2,
    #[default]
    Size3,
    Size4,
}

impl DialogSize {
    pub fn padding(self) -> Vec2 {
        match self {
            DialogSize::Size1 => vec2(12.0, 10.0),
            DialogSize::Size2 => vec2(16.0, 12.0),
            DialogSize::Size3 => vec2(24.0, 24.0),
            DialogSize::Size4 => vec2(24.0, 20.0),
        }
    }

    pub fn rounding_with_scale(self, scale: &crate::tokens::RadiusScale) -> CornerRadius {
        let radius = match self {
            DialogSize::Size1 | DialogSize::Size2 => scale.r4,
            DialogSize::Size3 | DialogSize::Size4 => scale.r5,
        };
        let clamped = radius.round().clamp(0.0, u8::MAX as f32) as u8;
        CornerRadius::same(clamped)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DialogLayoutTokens {
    pub padding: Margin,
    pub rounding: CornerRadius,
}

pub fn dialog_layout_tokens(theme: &Theme, size: DialogSize) -> DialogLayoutTokens {
    let pad = size.padding();
    DialogLayoutTokens {
        padding: Margin::symmetric(pad.x as i8, pad.y as i8),
        rounding: size.rounding_with_scale(&theme.radius),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DialogTokens {
    pub background: Color32,
    pub border: Stroke,
    pub shadow: egui::epaint::Shadow,
    pub layout: DialogLayoutTokens,
}

pub fn dialog_tokens_with_options(
    theme: &Theme,
    size: DialogSize,
    high_contrast: bool,
) -> DialogTokens {
    let palette = &theme.palette;
    let background = palette.background;
    let border = Stroke::new(1.0, if high_contrast { palette.foreground } else { palette.border });
    let shadow_alpha = if high_contrast { 90 } else { 70 };
    let shadow = egui::epaint::Shadow {
        offset: [0, 4],
        blur: 12,
        spread: 0,
        color: Color32::from_rgba_unmultiplied(
            palette.foreground.r(),
            palette.foreground.g(),
            palette.foreground.b(),
            shadow_alpha,
        ),
    };
    let layout = dialog_layout_tokens(theme, size);
    DialogTokens {
        background,
        border,
        shadow,
        layout,
    }
}

#[derive(Debug)]
pub struct DialogProps<'a> {
    pub id_source: Id,
    pub open: &'a mut bool,
    pub title: Option<String>,
    pub description: Option<String>,

    pub size: Vec2,

    pub dialog_size: DialogSize,
    pub align: DialogAlign,
    pub as_child: bool,

    pub width: Option<f32>,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,

    pub height: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,

    pub scrollable: bool,
    pub close_on_background: bool,
    pub close_on_escape: bool,
    pub scrim_opacity: u8,
    pub scrim_color: Option<Color32>,
    pub offset: Vec2,
    pub animate: bool,
    pub high_contrast: bool,
    pub show_close_button: bool,
    pub close_button_text: Option<String>,
    pub tokens_override: Option<DialogTokens>,
}

impl<'a> DialogProps<'a> {
    pub fn new(id_source: Id, open: &'a mut bool) -> Self {
        Self {
            id_source,
            open,
            title: None,
            description: None,
            size: Vec2::new(480.0, 0.0),
            dialog_size: DialogSize::Size3,
            align: DialogAlign::Center,
            as_child: false,
            width: None,
            min_width: None,
            max_width: Some(600.0),
            height: None,
            min_height: None,
            max_height: None,
            scrollable: true,
            close_on_background: true,
            close_on_escape: true,
            scrim_opacity: 140,
            scrim_color: None,
            offset: Vec2::ZERO,
            animate: true,
            high_contrast: false,
            show_close_button: true,
            close_button_text: Some(Icon::X.unicode().to_string()),
            tokens_override: None,
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

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn with_dialog_size(mut self, size: DialogSize) -> Self {
        self.dialog_size = size;
        self
    }

    pub fn with_align(mut self, align: DialogAlign) -> Self {
        self.align = align;
        self
    }

    pub fn with_as_child(mut self, as_child: bool) -> Self {
        self.as_child = as_child;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    pub fn with_max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_min_height(mut self, height: f32) -> Self {
        self.min_height = Some(height);
        self
    }

    pub fn with_max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    pub fn scrollable(mut self, enabled: bool) -> Self {
        self.scrollable = enabled;
        self
    }

    pub fn with_close_on_background(mut self, close: bool) -> Self {
        self.close_on_background = close;
        self
    }

    pub fn with_close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    pub fn with_scrim_opacity(mut self, opacity: u8) -> Self {
        self.scrim_opacity = opacity;
        self
    }

    pub fn with_scrim_color(mut self, color: Color32) -> Self {
        self.scrim_color = Some(color);
        self
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_animation(mut self, enabled: bool) -> Self {
        self.animate = enabled;
        self
    }

    pub fn high_contrast(mut self, enabled: bool) -> Self {
        self.high_contrast = enabled;
        self
    }

    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }

    pub fn close_button_text(mut self, text: impl Into<String>) -> Self {
        self.close_button_text = Some(text.into());
        self
    }

    pub fn with_tokens_override(mut self, tokens: DialogTokens) -> Self {
        self.tokens_override = Some(tokens);
        self
    }
}

#[allow(clippy::too_many_arguments)]
pub fn compute_dialog_rect(
    screen: Rect,
    width: Option<f32>,
    min_width: Option<f32>,
    max_width: Option<f32>,
    height: Option<f32>,
    min_height: Option<f32>,
    max_height: Option<f32>,
    offset: Vec2,
    align: DialogAlign,
) -> Rect {
    let fallback_width = 480.0;
    let w = width.unwrap_or(fallback_width);
    let min_w = min_width.unwrap_or(0.0);
    let max_w = max_width.unwrap_or(600.0);
    let width_clamped = w.clamp(min_w, max_w.min(screen.width()));

    let available_h = screen.height().max(0.0);
    let max_h = max_height
        .unwrap_or(available_h - 96.0)
        .clamp(0.0, available_h);
    let h_raw = height.unwrap_or(0.0);
    let min_h = min_height.unwrap_or(0.0);
    let height_clamped = if h_raw <= 0.0 {
        max_h.max(min_h)
    } else {
        h_raw.clamp(min_h, max_h)
    };

    let base_origin = match align {
        DialogAlign::Center => screen.center(),
        DialogAlign::Start => pos2(screen.left(), screen.top()),
    };
    let align_offset = match align {
        DialogAlign::Center => vec2(-width_clamped * 0.5, -height_clamped * 0.5),
        DialogAlign::Start => vec2(32.0, 48.0),
    };
    let min = base_origin + align_offset + offset;
    Rect::from_min_size(min, Vec2::new(width_clamped, height_clamped))
}

pub fn dialog<R>(
    ui: &mut Ui,
    theme: &Theme,
    props: DialogProps<'_>,
    render_content: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let DialogProps {
        id_source,
        open,
        title,
        description,
        size,
        dialog_size,
        align,
        as_child: _as_child,
        width,
        min_width,
        max_width,
        height,
        min_height,
        max_height,
        scrollable,
        close_on_background,
        close_on_escape,
        scrim_opacity,
        scrim_color,
        offset,
        animate,
        high_contrast,
        show_close_button,
        close_button_text,
        tokens_override,
    } = props;

    let ctx = ui.ctx();
    let state_id = id_source.with("last-open");
    let last_open = ctx.data(|d| d.get_temp::<bool>(state_id)).unwrap_or(false);
    let was_open = *open || last_open;
    let opened_now = *open && !last_open;

    trace!("render dialog {:?}", id_source);
    let screen = ctx.available_rect();
    let overlay_id = LayerId::new(Order::Foreground, id_source.with("overlay"));
    let anim_key = id_source.with("open-anim");
    let anim_t = if animate {
        ctx.animate_bool(anim_key, *open)
    } else if *open {
        1.0
    } else {
        0.0
    };
    if !*open && anim_t <= 0.0 {
        ctx.data_mut(|d| d.insert_temp(state_id, *open));
        return None;
    }

    let overlay_painter = ctx.layer_painter(overlay_id);
    let scrim_alpha = (scrim_opacity as f32 * anim_t).round().clamp(0.0, 255.0) as u8;
    let base_scrim = scrim_color.unwrap_or(Color32::BLACK);
    overlay_painter.rect_filled(
        screen,
        CornerRadius::same(0),
        Color32::from_rgba_unmultiplied(
            base_scrim.r(),
            base_scrim.g(),
            base_scrim.b(),
            scrim_alpha,
        ),
    );
    egui::Area::new(id_source.with("scrim-area"))
        .order(Order::Foreground)
        .interactable(true)
        .movable(false)
        .fixed_pos(screen.min)
        .show(ctx, |scrim_ui| {
            scrim_ui.allocate_exact_size(screen.size(), Sense::click());
        });

    let slide = if align == DialogAlign::Center {
        Vec2::ZERO
    } else {
        Vec2::new(0.0, 8.0)
    };

    let resolved_width = width.or(Some(size.x));
    let resolved_height = height.or((size.y > 0.0).then_some(size.y));
    let dialog_rect = compute_dialog_rect(
        screen,
        resolved_width,
        min_width,
        max_width,
        resolved_height,
        min_height,
        max_height,
        offset,
        align,
    );
    let animated_pos = dialog_rect.min + slide * (1.0 - anim_t);

    let mut result = None;
    let content_id = id_source.with("content");
    let area = egui::Area::new(content_id)
        .order(Order::Tooltip)
        .interactable(true)
        .movable(false)
        .fixed_pos(animated_pos);

    area.show(ctx, |area_ui| {
        let tokens = tokens_override
            .unwrap_or_else(|| dialog_tokens_with_options(theme, dialog_size, high_contrast));
        let frame = Frame::popup(area_ui.style())
            .fill(tokens.background)
            .stroke(tokens.border)
            .shadow(tokens.shadow)
            .corner_radius(tokens.layout.rounding)
            .inner_margin(tokens.layout.padding);

        let frame_resp = frame.show(area_ui, |content_ui| {
            content_ui.set_min_width(dialog_rect.width());
            content_ui.set_max_width(dialog_rect.width());
            content_ui.set_max_height(dialog_rect.height());

            if title.is_some() {
                content_ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::Min),
                    |header_ui| {
                        if let Some(t) = &title {
                            header_ui.label(
                                egui::RichText::new(t)
                                    .strong()
                                    .size(18.0)
                                    .color(theme.palette.foreground),
                            );
                        }
                        if false {
                            header_ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Min),
                                |close_ui| {
                                    let close_text = close_button_text.as_deref().unwrap_or("×");
                                    let size = vec2(20.0, 20.0);
                                    let (rect, close_resp) =
                                        close_ui.allocate_exact_size(size, Sense::click());

                                    let icon_color = if close_resp.hovered() {
                                        theme.palette.foreground
                                    } else {
                                        theme.palette.muted_foreground
                                    };

                                    close_ui.painter().text(
                                        rect.center(),
                                        Align2::CENTER_CENTER,
                                        close_text,
                                        FontId::proportional(16.0),
                                        icon_color,
                                    );

                                    if close_resp.has_focus() {
                                        let ring_color = theme.palette.ring;
                                        let ring_rect = rect.expand(2.0);
                                        close_ui.painter().rect_stroke(
                                            ring_rect,
                                            CornerRadius::same(4),
                                            Stroke::new(2.0, ring_color),
                                            StrokeKind::Outside,
                                        );
                                    }

                                    if close_resp.clicked() {
                                        close_resp.request_focus();
                                        *open = false;
                                    }
                                },
                            );
                        }
                    },
                );
            }
            if let Some(desc) = &description {
                if title.is_some() {
                    content_ui.add_space(6.0);
                }
                content_ui.label(
                    egui::RichText::new(desc)
                        .size(14.0)
                        .color(theme.palette.muted_foreground),
                );
                content_ui.add_space(12.0);
            }

            if scrollable {
                let remaining_height = content_ui.available_height().max(0.0);
                let scroll_props = ScrollAreaProps::default()
                    .with_id(content_id.with("body-scroll"))
                    .with_direction(ScrollDirection::Vertical)
                    .with_max_size(Vec2::new(dialog_rect.width(), remaining_height))
                    .with_auto_shrink([false; 2]);
                result = Some(scroll_area(content_ui, theme, scroll_props, |scroll_ui| {
                    scroll_ui.set_max_width(dialog_rect.width());
                    scroll_ui.set_max_height(remaining_height);
                    render_content(scroll_ui)
                }));
            } else {
                result = Some(render_content(content_ui));
            }
        });

        if show_close_button {
            let close_text = close_button_text
                .clone()
                .unwrap_or_else(|| Icon::X.unicode().to_string());
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
                *open = false;
            }
        }

        let escape = close_on_escape && area_ui.input(|i| i.key_pressed(egui::Key::Escape));
        let any_click = area_ui.input(|i| i.pointer.any_click());
        let interact = area_ui.input(|i| i.pointer.interact_pos());
        let contains = interact
            .map(|pos| frame_resp.response.rect.contains(pos))
            .unwrap_or(false);
        let outside_click =
            !opened_now && was_open && close_on_background && any_click && !contains;

        if escape || outside_click {
            *open = false;
        }
    });

    ctx.data_mut(|d| d.insert_temp(state_id, *open));

    result
}
