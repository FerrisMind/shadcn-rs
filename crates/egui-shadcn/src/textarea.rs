use crate::theme::Theme;
use crate::tokens::{ColorPalette, ControlSize, InputVariant, input_tokens, mix};
use egui::{
    Color32, CornerRadius, FontId, Rect, Response, Sense, Stroke, StrokeKind, TextEdit, TextStyle,
    Ui, UiBuilder, Vec2, WidgetText, pos2, vec2,
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
            TextareaSize::Size1 => 64.0,
            TextareaSize::Size2 => 80.0,
            TextareaSize::Size3 => 96.0,
        }
    }

    pub fn font_size(self) -> f32 {
        match self {
            TextareaSize::Size1 => 12.0,
            TextareaSize::Size2 => 14.0,
            TextareaSize::Size3 => 16.0,
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
        match variant {
            TextareaVariant::Surface => Self {
                bg: Color32::TRANSPARENT,
                bg_hover: Color32::TRANSPARENT,
                bg_focus: Color32::TRANSPARENT,
                border: palette.input,
                border_hover: palette.input,
                border_focus: palette.ring,
                text_color: palette.foreground,
                placeholder_color: palette.muted_foreground,
                selection_bg: palette.primary,
                selection_fg: palette.primary_foreground,
                focus_ring: Color32::from_rgba_unmultiplied(
                    palette.ring.r(),
                    palette.ring.g(),
                    palette.ring.b(),
                    128,
                ),
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
                border: palette.input,
                border_hover: palette.input,
                border_focus: palette.ring,
                text_color: palette.foreground,
                placeholder_color: palette.muted_foreground,
                selection_bg: palette.primary,
                selection_fg: palette.primary_foreground,
                focus_ring: Color32::from_rgba_unmultiplied(
                    palette.ring.r(),
                    palette.ring.g(),
                    palette.ring.b(),
                    128,
                ),
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
                bg: Color32::from_rgba_unmultiplied(
                    palette.primary.r(),
                    palette.primary.g(),
                    palette.primary.b(),
                    30,
                ),
                bg_hover: Color32::from_rgba_unmultiplied(
                    palette.primary.r(),
                    palette.primary.g(),
                    palette.primary.b(),
                    40,
                ),
                bg_focus: Color32::from_rgba_unmultiplied(
                    palette.primary.r(),
                    palette.primary.g(),
                    palette.primary.b(),
                    50,
                ),
                border: Color32::TRANSPARENT,
                border_hover: Color32::TRANSPARENT,
                border_focus: Color32::TRANSPARENT,
                text_color: palette.foreground,
                placeholder_color: palette.muted_foreground,
                selection_bg: palette.primary,
                selection_fg: palette.primary_foreground,
                focus_ring: Color32::from_rgba_unmultiplied(
                    palette.ring.r(),
                    palette.ring.g(),
                    palette.ring.b(),
                    128,
                ),
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
        match variant {
            TextareaVariant::Soft => {
                style.bg = Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 30);
                style.bg_hover =
                    Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 40);
                style.bg_focus =
                    Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 50);
                style.selection_bg = accent;
                style.selection_fg = palette.primary_foreground;
                style.focus_ring =
                    Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 128);
            }
            TextareaVariant::Surface | TextareaVariant::Classic => {
                style.border_focus = accent;
                style.focus_ring =
                    Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 128);
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
    pub resizable: bool,
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
            resizable: true,
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
        self.resizable = resizable;
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
            resizable: true,
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
    pub resizable: bool,
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
        self.resizable = resizable;
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
            resizable: self.resizable,
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

    if props.high_contrast {
        style = style.with_high_contrast();
    }

    let rounding = props.radius.corner_radius();
    style.rounding = rounding;

    let effectively_disabled = !props.enabled || props.read_only;

    let min_height = if let Some(rows) = props.rows {
        let line_height = props.size.font_size() * 1.4;
        line_height * rows as f32 + props.size.padding().y * 2.0
    } else {
        props.size.min_height()
    };

    let width = props.width.unwrap_or(200.0);

    let id = ui.make_persistent_id(&props.id_source);
    let desired_size = vec2(width, min_height);

    let (rect, response) = if props.resizable {
        let resize_id = id.with("resize");
        let resize = egui::Resize::default()
            .id(resize_id)
            .default_size(desired_size)
            .min_size(vec2(64.0, min_height))
            .with_stroke(false);

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

    let bg_color = if effectively_disabled {
        Color32::from_rgba_unmultiplied(
            style.bg.r(),
            style.bg.g(),
            style.bg.b(),
            (style.bg.a() as f32 * style.disabled_opacity) as u8,
        )
    } else if has_focus {
        style.bg_focus
    } else if response.hovered() {
        style.bg_hover
    } else {
        style.bg
    };

    let border_color = if props.is_invalid {
        style.invalid_border
    } else if has_focus {
        style.border_focus
    } else if response.hovered() && !effectively_disabled {
        style.border_hover
    } else {
        style.border
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

        if has_focus && !effectively_disabled {
            let ring_color = if props.is_invalid {
                style.invalid_ring
            } else {
                style.focus_ring
            };
            painter.rect_stroke(
                rect,
                style.rounding,
                Stroke::new(style.focus_ring_width, ring_color),
                StrokeKind::Outside,
            );
        }
    }

    let inner_rect = rect.shrink2(props.size.padding());

    let text_color = if effectively_disabled {
        Color32::from_rgba_unmultiplied(
            style.text_color.r(),
            style.text_color.g(),
            style.text_color.b(),
            (style.text_color.a() as f32 * 0.6) as u8,
        )
    } else {
        style.text_color
    };

    let placeholder_colored: WidgetText = props.placeholder.into();
    let placeholder_colored = placeholder_colored.color(style.placeholder_color);

    let tokens = input_tokens(&theme.palette, InputVariant::Surface);

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

    if props.resizable {
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
