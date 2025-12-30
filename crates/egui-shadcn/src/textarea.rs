use crate::theme::Theme;
use crate::tokens::{
    ColorPalette, ControlSize, InputVariant as TokenInputVariant, ease_out_cubic, input_tokens, mix,
};
use egui::{
    Color32, CornerRadius, FontId, Rect, Response, Sense, Stroke, StrokeKind, TextEdit, TextStyle,
    Ui, UiBuilder, Vec2, Vec2b, WidgetText, pos2, vec2,
};
use log::trace;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextareaVariant {
    Classic,

    #[default]
    Surface,

    Soft,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextareaSize {
    Size1,

    #[default]
    Size2,

    Size3,
}

impl TextareaSize {
    pub fn min_height(self) -> f32 {
        match self {
            TextareaSize::Size1 => 48.0,
            TextareaSize::Size2 => 64.0,
            TextareaSize::Size3 => 80.0,
        }
    }

    pub fn font_size(self) -> f32 {
        match self {
            TextareaSize::Size1 => 12.0,
            TextareaSize::Size2 => 14.0,
            TextareaSize::Size3 => 16.0,
        }
    }

    pub fn line_height(self) -> f32 {
        match self {
            TextareaSize::Size1 => 16.0,
            TextareaSize::Size2 => 20.0,
            TextareaSize::Size3 => 24.0,
        }
    }

    pub fn font(self) -> FontId {
        FontId::proportional(self.font_size())
    }

    pub fn padding(self) -> Vec2 {
        match self {
            TextareaSize::Size1 => vec2(6.0, 4.0),
            TextareaSize::Size2 => vec2(8.0, 6.0),
            TextareaSize::Size3 => vec2(12.0, 8.0),
        }
    }

    pub fn rounding(self) -> CornerRadius {
        match self {
            TextareaSize::Size1 => CornerRadius::same(4),
            TextareaSize::Size2 => CornerRadius::same(6),
            TextareaSize::Size3 => CornerRadius::same(8),
        }
    }
}

impl From<ControlSize> for TextareaSize {
    fn from(size: ControlSize) -> Self {
        match size {
            ControlSize::Sm | ControlSize::IconSm => TextareaSize::Size1,
            ControlSize::Md | ControlSize::Icon => TextareaSize::Size2,
            ControlSize::Lg | ControlSize::IconLg => TextareaSize::Size3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextareaRadius {
    None,

    Small,

    #[default]
    Medium,

    Large,

    Full,
}

impl TextareaRadius {
    pub fn corner_radius(self) -> CornerRadius {
        match self {
            TextareaRadius::None => CornerRadius::same(0),
            TextareaRadius::Small => CornerRadius::same(4),
            TextareaRadius::Medium => CornerRadius::same(6),
            TextareaRadius::Large => CornerRadius::same(8),
            TextareaRadius::Full => CornerRadius::same(255),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextareaResize {
    None,
    Vertical,
    Horizontal,

    #[default]
    Both,
}

impl TextareaResize {
    pub fn resizable_axes(self) -> Vec2b {
        match self {
            TextareaResize::None => Vec2b::FALSE,
            TextareaResize::Vertical => Vec2b::new(false, true),
            TextareaResize::Horizontal => Vec2b::new(true, false),
            TextareaResize::Both => Vec2b::TRUE,
        }
    }
}

impl From<TextareaVariant> for TokenInputVariant {
    fn from(variant: TextareaVariant) -> Self {
        match variant {
            TextareaVariant::Surface => TokenInputVariant::Surface,
            TextareaVariant::Classic => TokenInputVariant::Classic,
            TextareaVariant::Soft => TokenInputVariant::Soft,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextareaStyle {
    pub bg: Color32,

    pub bg_hover: Color32,

    pub bg_focus: Color32,

    pub border: Color32,

    pub border_hover: Color32,

    pub border_focus: Color32,

    pub text_color: Color32,

    pub placeholder_color: Color32,

    pub selection_bg: Color32,

    pub selection_fg: Color32,

    pub focus_ring: Color32,

    pub focus_ring_width: f32,

    pub invalid_border: Color32,

    pub invalid_ring: Color32,

    pub disabled_opacity: f32,

    pub rounding: CornerRadius,
}

impl TextareaStyle {
    pub fn from_palette(palette: &ColorPalette, variant: TextareaVariant) -> Self {
        let token_variant = TokenInputVariant::from(variant);
        let tokens = input_tokens(palette, token_variant);
        let focus_ring = Color32::from_rgba_unmultiplied(
            palette.ring.r(),
            palette.ring.g(),
            palette.ring.b(),
            128,
        );
        let disabled_border = mix(palette.border, palette.muted_foreground, 0.5);
        let visible_border = mix(disabled_border, palette.foreground, 0.25);
        let soft_bg = mix(palette.accent, palette.background, 0.4);
        let soft_bg_hover = mix(soft_bg, Color32::WHITE, 0.08);
        let soft_bg_focus = mix(soft_bg, focus_ring, 0.2);
        match variant {
            TextareaVariant::Surface => Self {
                bg: Color32::TRANSPARENT,
                bg_hover: Color32::TRANSPARENT,
                bg_focus: Color32::TRANSPARENT,
                border: visible_border,
                border_hover: visible_border,
                border_focus: palette.ring,
                text_color: tokens.idle.fg_stroke.color,
                placeholder_color: tokens.placeholder,
                selection_bg: tokens.selection_bg,
                selection_fg: tokens.selection_fg,
                focus_ring,
                focus_ring_width: 3.0,
                invalid_border: palette.destructive,
                invalid_ring: Color32::from_rgba_unmultiplied(
                    palette.destructive.r(),
                    palette.destructive.g(),
                    palette.destructive.b(),
                    51,
                ),
                disabled_opacity: 0.5,
                rounding: CornerRadius::same(6),
            },
            TextareaVariant::Classic => Self {
                bg: palette.background,
                bg_hover: palette.background,
                bg_focus: palette.background,
                border: visible_border,
                border_hover: visible_border,
                border_focus: palette.ring,
                text_color: tokens.idle.fg_stroke.color,
                placeholder_color: tokens.placeholder,
                selection_bg: tokens.selection_bg,
                selection_fg: tokens.selection_fg,
                focus_ring,
                focus_ring_width: 3.0,
                invalid_border: palette.destructive,
                invalid_ring: Color32::from_rgba_unmultiplied(
                    palette.destructive.r(),
                    palette.destructive.g(),
                    palette.destructive.b(),
                    51,
                ),
                disabled_opacity: 0.5,
                rounding: CornerRadius::same(6),
            },
            TextareaVariant::Soft => Self {
                bg: soft_bg,
                bg_hover: soft_bg_hover,
                bg_focus: soft_bg_focus,
                border: Color32::TRANSPARENT,
                border_hover: Color32::TRANSPARENT,
                border_focus: Color32::TRANSPARENT,
                text_color: tokens.idle.fg_stroke.color,
                placeholder_color: tokens.placeholder,
                selection_bg: tokens.selection_bg,
                selection_fg: tokens.selection_fg,
                focus_ring,
                focus_ring_width: 3.0,
                invalid_border: palette.destructive,
                invalid_ring: Color32::from_rgba_unmultiplied(
                    palette.destructive.r(),
                    palette.destructive.g(),
                    palette.destructive.b(),
                    51,
                ),
                disabled_opacity: 0.5,
                rounding: CornerRadius::same(6),
            },
        }
    }

    pub fn from_palette_with_accent(
        palette: &ColorPalette,
        variant: TextareaVariant,
        accent: Color32,
    ) -> Self {
        let mut style = Self::from_palette(palette, variant);
        let accent_ring = Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 128);
        match variant {
            TextareaVariant::Soft => {
                style.bg = mix(accent, palette.background, 0.4);
                style.bg_hover = mix(style.bg, Color32::WHITE, 0.08);
                style.bg_focus = mix(style.bg, accent_ring, 0.2);
                style.selection_bg = mix(accent, Color32::WHITE, 0.12);
                style.selection_fg = palette.primary_foreground;
                style.focus_ring = accent_ring;
            }
            TextareaVariant::Surface | TextareaVariant::Classic => {
                style.border_focus = accent;
                style.focus_ring = accent_ring;
                style.selection_bg = accent;
                style.selection_fg = palette.primary_foreground;
            }
        }
        style
    }

    pub fn with_high_contrast(mut self) -> Self {
        self.text_color = Color32::WHITE;
        self.bg = mix(self.bg, Color32::WHITE, 0.1);
        self.bg_hover = mix(self.bg_hover, Color32::WHITE, 0.1);
        self
    }
}

impl Default for TextareaStyle {
    fn default() -> Self {
        Self::from_palette(&ColorPalette::default(), TextareaVariant::Surface)
    }
}

#[derive(Debug)]
pub struct TextareaProps<'a, Id>
where
    Id: Hash + Debug,
{
    pub id_source: Id,

    pub value: &'a mut String,

    pub placeholder: &'a str,

    pub variant: TextareaVariant,

    pub size: TextareaSize,

    pub radius: TextareaRadius,

    pub resize: TextareaResize,

    pub enabled: bool,

    pub read_only: bool,

    pub is_invalid: bool,

    pub show_counter: bool,

    pub max_len: Option<usize>,

    pub rows: Option<usize>,

    pub width: Option<f32>,

    pub style: Option<TextareaStyle>,

    pub accent_color: Option<Color32>,

    pub high_contrast: bool,
}

impl<'a, Id: Hash + Debug> TextareaProps<'a, Id> {
    pub fn new(id_source: Id, value: &'a mut String) -> Self {
        Self {
            id_source,
            value,
            placeholder: "",
            variant: TextareaVariant::Surface,
            size: TextareaSize::Size2,
            radius: TextareaRadius::Medium,
            resize: TextareaResize::Both,
            enabled: true,
            read_only: false,
            is_invalid: false,
            show_counter: false,
            max_len: None,
            rows: None,
            width: None,
            style: None,
            accent_color: None,
            high_contrast: false,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn variant(mut self, variant: TextareaVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: TextareaSize) -> Self {
        self.size = size;
        self
    }

    pub fn radius(mut self, radius: TextareaRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn resize(mut self, resize: TextareaResize) -> Self {
        self.resize = resize;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn invalid(mut self, is_invalid: bool) -> Self {
        self.is_invalid = is_invalid;
        self
    }

    pub fn show_counter(mut self, show_counter: bool) -> Self {
        self.show_counter = show_counter;
        self
    }

    pub fn max_len(mut self, max_len: usize) -> Self {
        self.max_len = Some(max_len);
        self
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = Some(rows);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn style(mut self, style: TextareaStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn accent_color(mut self, color: Color32) -> Self {
        self.accent_color = Some(color);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resize = if resizable {
            TextareaResize::Both
        } else {
            TextareaResize::None
        };
        self
    }
}

impl<'a, Id: Hash + Debug> TextareaBuilder<'a, Id> {
    pub fn new(id_source: Id) -> TextareaBuilder<'static, Id> {
        TextareaBuilder {
            id_source,
            placeholder: "",
            variant: TextareaVariant::Surface,
            size: TextareaSize::Size2,
            radius: TextareaRadius::Medium,
            resize: TextareaResize::Both,
            enabled: true,
            read_only: false,
            is_invalid: false,
            show_counter: false,
            max_len: None,
            rows: None,
            width: None,
            style: None,
            accent_color: None,
            high_contrast: false,
        }
    }
}

pub struct TextareaBuilder<'a, Id>
where
    Id: Hash + Debug,
{
    pub id_source: Id,
    pub placeholder: &'a str,
    pub variant: TextareaVariant,
    pub size: TextareaSize,
    pub radius: TextareaRadius,
    pub resize: TextareaResize,
    pub enabled: bool,
    pub read_only: bool,
    pub is_invalid: bool,
    pub show_counter: bool,
    pub max_len: Option<usize>,
    pub rows: Option<usize>,
    pub width: Option<f32>,
    pub style: Option<TextareaStyle>,
    pub accent_color: Option<Color32>,
    pub high_contrast: bool,
}

impl<'a, Id: Hash + Debug> TextareaBuilder<'a, Id> {
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn variant(mut self, variant: TextareaVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: TextareaSize) -> Self {
        self.size = size;
        self
    }

    pub fn radius(mut self, radius: TextareaRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn resize(mut self, resize: TextareaResize) -> Self {
        self.resize = resize;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn invalid(mut self, is_invalid: bool) -> Self {
        self.is_invalid = is_invalid;
        self
    }

    pub fn show_counter(mut self, show_counter: bool) -> Self {
        self.show_counter = show_counter;
        self
    }

    pub fn max_len(mut self, max_len: usize) -> Self {
        self.max_len = Some(max_len);
        self
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = Some(rows);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn style(mut self, style: TextareaStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn accent_color(mut self, color: Color32) -> Self {
        self.accent_color = Some(color);
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resize = if resizable {
            TextareaResize::Both
        } else {
            TextareaResize::None
        };
        self
    }

    pub fn show(self, ui: &mut Ui, theme: &Theme, value: &mut String) -> Response {
        let props = TextareaProps {
            id_source: self.id_source,
            value,
            placeholder: self.placeholder,
            variant: self.variant,
            size: self.size,
            radius: self.radius,
            resize: self.resize,
            enabled: self.enabled,
            read_only: self.read_only,
            is_invalid: self.is_invalid,
            show_counter: self.show_counter,
            max_len: self.max_len,
            rows: self.rows,
            width: self.width,
            style: self.style,
            accent_color: self.accent_color,
            high_contrast: self.high_contrast,
        };
        textarea_with_props(ui, theme, props)
    }
}

pub fn textarea_with_props<Id>(ui: &mut Ui, theme: &Theme, props: TextareaProps<'_, Id>) -> Response
where
    Id: Hash + Debug,
{
    trace!(
        "Rendering textarea variant={:?} size={:?} invalid={} enabled={} read_only={}",
        props.variant, props.size, props.is_invalid, props.enabled, props.read_only
    );

    let mut style = props.style.clone().unwrap_or_else(|| {
        if let Some(accent) = props.accent_color {
            TextareaStyle::from_palette_with_accent(&theme.palette, props.variant, accent)
        } else {
            TextareaStyle::from_palette(&theme.palette, props.variant)
        }
    });

    let apply_opacity = |color: Color32, opacity: f32| -> Color32 {
        Color32::from_rgba_unmultiplied(
            color.r(),
            color.g(),
            color.b(),
            (color.a() as f32 * opacity) as u8,
        )
    };

    if props.high_contrast {
        style = style.with_high_contrast();
    }

    let rounding = props.radius.corner_radius();
    style.rounding = rounding;

    let effectively_disabled = !props.enabled || props.read_only;

    let min_height = if let Some(rows) = props.rows {
        let line_height = props.size.line_height();
        line_height * rows as f32 + props.size.padding().y * 2.0
    } else {
        props.size.min_height()
    };

    let width = props.width.unwrap_or(200.0);

    let id = ui.make_persistent_id(&props.id_source);
    let desired_size = vec2(width, min_height);

    let resize_axes = props.resize.resizable_axes();
    let (rect, response) = if resize_axes.any() {
        let resize_id = id.with("resize");
        let mut resize = egui::Resize::default()
            .id(resize_id)
            .default_size(desired_size)
            .min_size(vec2(64.0, min_height))
            .with_stroke(false);

        if !resize_axes.x {
            resize = resize.min_width(desired_size.x).max_width(desired_size.x);
        }

        if !resize_axes.y {
            resize = resize.min_height(min_height).max_height(min_height);
        }

        resize = resize.resizable(resize_axes);

        let mut final_rect = Rect::NOTHING;
        let mut final_response = None;

        let mut style = ui.style().as_ref().clone();

        style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(0.0, Color32::TRANSPARENT);
        style.visuals.widgets.inactive.fg_stroke = Stroke::new(0.0, Color32::TRANSPARENT);
        style.visuals.widgets.hovered.fg_stroke = Stroke::new(0.0, Color32::TRANSPARENT);
        style.visuals.widgets.active.fg_stroke = Stroke::new(0.0, Color32::TRANSPARENT);
        ui.set_style(style);

        resize.show(ui, |ui| {
            let size = ui.available_size();
            let (rect, response) = ui.allocate_exact_size(size, Sense::click());
            final_rect = rect;
            final_response = Some(response);
        });

        if let (Some(r), Some(resp)) = (
            (final_rect != Rect::NOTHING).then_some(final_rect),
            final_response,
        ) {
            (r, resp)
        } else {
            ui.allocate_exact_size(desired_size, Sense::click())
        }
    } else {
        ui.allocate_exact_size(desired_size, Sense::click())
    };

    let has_focus = response.has_focus() || ui.memory(|m| m.has_focus(id.with("edit")));

    let anim_duration = theme.motion.base_ms / 1000.0;
    let hover_t = ui.ctx().animate_bool_with_time_and_easing(
        id.with("hover"),
        response.hovered() && !effectively_disabled,
        anim_duration,
        ease_out_cubic,
    );
    let focus_t = ui.ctx().animate_bool_with_time_and_easing(
        id.with("focus"),
        has_focus && !effectively_disabled,
        anim_duration,
        ease_out_cubic,
    );

    let bg_color = if effectively_disabled {
        apply_opacity(style.bg, style.disabled_opacity)
    } else {
        let hover_bg = mix(style.bg, style.bg_hover, hover_t);
        mix(hover_bg, style.bg_focus, focus_t)
    };

    let border_color = if props.is_invalid {
        style.invalid_border
    } else if effectively_disabled {
        style.border
    } else {
        let hover_border = mix(style.border, style.border_hover, hover_t);
        mix(hover_border, style.border_focus, focus_t)
    };

    {
        let painter = ui.painter();
        painter.rect_filled(rect, style.rounding, bg_color);

        if border_color != Color32::TRANSPARENT {
            painter.rect_stroke(
                rect,
                style.rounding,
                Stroke::new(1.0, border_color),
                StrokeKind::Inside,
            );
        }

        if focus_t > 0.0 && !effectively_disabled {
            let ring_color = if props.is_invalid {
                style.invalid_ring
            } else {
                style.focus_ring
            };
            let ring_width = style.focus_ring_width * focus_t;
            if ring_width > 0.0 {
                painter.rect_stroke(
                    rect,
                    style.rounding,
                    Stroke::new(ring_width, apply_opacity(ring_color, focus_t)),
                    StrokeKind::Outside,
                );
            }
        }
    }

    let inner_rect = rect.shrink2(props.size.padding());

    let text_color = if effectively_disabled {
        apply_opacity(style.text_color, 0.6)
    } else {
        style.text_color
    };

    let placeholder_colored: WidgetText = props.placeholder.into();
    let placeholder_colored = placeholder_colored.color(style.placeholder_color);

    let token_variant = TokenInputVariant::from(props.variant);
    let tokens = input_tokens(&theme.palette, token_variant);

    let response = ui.scope_builder(UiBuilder::new().max_rect(inner_rect), |inner_ui| {
        inner_ui.set_clip_rect(inner_rect);
        let mut inner_style = inner_ui.style().as_ref().clone();
        inner_style
            .text_styles
            .insert(TextStyle::Body, props.size.font());
        inner_style.visuals.selection.bg_fill = style.selection_bg;
        inner_style.visuals.selection.stroke = Stroke::new(1.0, style.selection_fg);
        inner_style.visuals.override_text_color = Some(text_color);
        inner_style.visuals.extreme_bg_color = tokens.idle.bg_fill;

        inner_style.visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
        inner_style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        inner_style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
        inner_style.visuals.widgets.hovered.bg_fill = Color32::TRANSPARENT;
        inner_style.visuals.widgets.hovered.weak_bg_fill = Color32::TRANSPARENT;
        inner_style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
        inner_style.visuals.widgets.active.bg_fill = Color32::TRANSPARENT;
        inner_style.visuals.widgets.active.weak_bg_fill = Color32::TRANSPARENT;
        inner_style.visuals.widgets.active.bg_stroke = Stroke::NONE;

        inner_ui.set_style(inner_style);

        let mut edit = TextEdit::multiline(props.value)
            .id(id.with("edit"))
            .hint_text(placeholder_colored)
            .text_color(text_color)
            .frame(false)
            .margin(vec2(0.0, 0.0))
            .desired_width(inner_rect.width())
            .desired_rows(props.rows.unwrap_or(3));

        if let Some(limit) = props.max_len {
            edit = edit.char_limit(limit);
        }

        if props.read_only {
            edit = edit.interactive(false);
        }

        inner_ui.add_enabled(props.enabled, edit)
    });

    if props.show_counter {
        let count = props.value.chars().count();
        let counter_text = if let Some(limit) = props.max_len {
            format!("{}/{}", count, limit)
        } else {
            format!("{}", count)
        };

        let painter = ui.painter();
        let counter_galley = painter.layout_no_wrap(
            counter_text,
            FontId::proportional(props.size.font_size() * 0.85),
            style.placeholder_color,
        );

        let counter_pos = pos2(
            rect.right() - counter_galley.rect.width() - 8.0,
            rect.bottom() - counter_galley.rect.height() - 4.0,
        );

        painter.galley(counter_pos, counter_galley, style.placeholder_color);
    }

    if resize_axes.any() {
        let painter = ui.painter();
        let grip_color = style.placeholder_color;
        let grip_padding = 3.0;
        let line_spacing = 4.0;

        let corner = pos2(rect.right() - grip_padding, rect.bottom() - grip_padding);

        painter.line_segment(
            [
                pos2(corner.x - 3.0, corner.y),
                pos2(corner.x, corner.y - 3.0),
            ],
            Stroke::new(1.0, grip_color),
        );

        painter.line_segment(
            [
                pos2(corner.x - 3.0 - line_spacing, corner.y),
                pos2(corner.x, corner.y - 3.0 - line_spacing),
            ],
            Stroke::new(1.0, grip_color),
        );
    }

    response.inner
}
