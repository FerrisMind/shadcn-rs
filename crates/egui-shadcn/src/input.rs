use crate::theme::Theme;
use crate::tokens::{
    ColorPalette, ControlSize, InputVariant as TokenInputVariant, input_tokens, mix,
};
use egui::{
    Color32, CornerRadius, FontId, Painter, Rect, Response, Sense, Stroke, StrokeKind, TextEdit,
    TextStyle, Ui, UiBuilder, Vec2, WidgetText, pos2, vec2,
};
use log::trace;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputVariant {
    Classic,

    #[default]
    Surface,

    Soft,
}

impl From<InputVariant> for TokenInputVariant {
    fn from(variant: InputVariant) -> Self {
        match variant {
            InputVariant::Surface => TokenInputVariant::Surface,
            InputVariant::Classic => TokenInputVariant::Classic,
            InputVariant::Soft => TokenInputVariant::Soft,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputSize {
    Size1,

    #[default]
    Size2,

    Size3,
}

impl InputSize {
    pub fn height(self) -> f32 {
        match self {
            InputSize::Size1 => 24.0,
            InputSize::Size2 => 32.0,
            InputSize::Size3 => 40.0,
        }
    }

    pub fn font_size(self) -> f32 {
        match self {
            InputSize::Size1 => 12.0,
            InputSize::Size2 => 14.0,
            InputSize::Size3 => 16.0,
        }
    }

    pub fn font(self) -> FontId {
        FontId::proportional(self.font_size())
    }

    pub fn padding(self) -> Vec2 {
        match self {
            InputSize::Size1 => vec2(6.0, 4.0),
            InputSize::Size2 => vec2(8.0, 6.0),
            InputSize::Size3 => vec2(12.0, 8.0),
        }
    }

    pub fn rounding(self) -> CornerRadius {
        match self {
            InputSize::Size1 => CornerRadius::same(4),
            InputSize::Size2 => CornerRadius::same(6),
            InputSize::Size3 => CornerRadius::same(8),
        }
    }

    pub fn slot_gap(self) -> f32 {
        match self {
            InputSize::Size1 => 4.0,
            InputSize::Size2 => 6.0,
            InputSize::Size3 => 8.0,
        }
    }

    pub fn slot_icon_size(self) -> f32 {
        match self {
            InputSize::Size1 => 12.0,
            InputSize::Size2 => 14.0,
            InputSize::Size3 => 16.0,
        }
    }
}

impl From<ControlSize> for InputSize {
    fn from(size: ControlSize) -> Self {
        match size {
            ControlSize::Sm | ControlSize::IconSm => InputSize::Size1,
            ControlSize::Md | ControlSize::Icon => InputSize::Size2,
            ControlSize::Lg | ControlSize::IconLg => InputSize::Size3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputRadius {
    None,

    Small,

    #[default]
    Medium,

    Large,

    Full,
}

impl InputRadius {
    pub fn corner_radius(self) -> CornerRadius {
        match self {
            InputRadius::None => CornerRadius::same(0),
            InputRadius::Small => CornerRadius::same(4),
            InputRadius::Medium => CornerRadius::same(6),
            InputRadius::Large => CornerRadius::same(8),
            InputRadius::Full => CornerRadius::same(255),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputType {
    #[default]
    Text,

    Password,

    Email,

    Number,

    Search,

    Tel,

    Url,
}

impl InputType {
    pub fn is_password(self) -> bool {
        matches!(self, InputType::Password)
    }
}

#[derive(Clone, Debug)]
pub struct InputStyle {
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

    pub slot_color: Color32,
}

impl InputStyle {
    pub fn from_palette(palette: &ColorPalette, variant: InputVariant) -> Self {
        match variant {
            InputVariant::Surface => Self {
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
                slot_color: palette.muted_foreground,
            },
            InputVariant::Classic => Self {
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
                slot_color: palette.muted_foreground,
            },
            InputVariant::Soft => Self {
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
                slot_color: palette.foreground,
            },
        }
    }

    pub fn from_palette_with_accent(
        palette: &ColorPalette,
        variant: InputVariant,
        accent: Color32,
    ) -> Self {
        let mut style = Self::from_palette(palette, variant);
        match variant {
            InputVariant::Soft => {
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
            InputVariant::Surface | InputVariant::Classic => {
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

impl Default for InputStyle {
    fn default() -> Self {
        Self::from_palette(&ColorPalette::default(), InputVariant::Surface)
    }
}

pub type SlotFn<'a> = &'a dyn Fn(&Painter, Rect, Color32);

type BoxedSlotFn<'a> = Option<Box<dyn Fn(&Painter, Rect, Color32) + 'a>>;

pub struct InputProps<'a, Id>
where
    Id: Hash + Debug,
{
    pub id_source: Id,

    pub value: &'a mut String,

    pub placeholder: &'a str,

    pub variant: InputVariant,

    pub size: InputSize,

    pub radius: InputRadius,

    pub input_type: InputType,

    pub enabled: bool,

    pub read_only: bool,

    pub is_invalid: bool,

    pub max_len: Option<usize>,

    pub width: Option<f32>,

    pub style: Option<InputStyle>,

    pub accent_color: Option<Color32>,

    pub high_contrast: bool,

    #[allow(clippy::type_complexity)]
    pub left_slot: BoxedSlotFn<'a>,

    #[allow(clippy::type_complexity)]
    pub right_slot: BoxedSlotFn<'a>,
}

impl<'a, Id: Hash + Debug> InputProps<'a, Id> {
    pub fn new(id_source: Id, value: &'a mut String) -> Self {
        Self {
            id_source,
            value,
            placeholder: "",
            variant: InputVariant::Surface,
            size: InputSize::Size2,
            radius: InputRadius::Medium,
            input_type: InputType::Text,
            enabled: true,
            read_only: false,
            is_invalid: false,
            max_len: None,
            width: None,
            style: None,
            accent_color: None,
            high_contrast: false,
            left_slot: None,
            right_slot: None,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    pub fn radius(mut self, radius: InputRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn input_type(mut self, input_type: InputType) -> Self {
        self.input_type = input_type;
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

    pub fn max_len(mut self, max_len: usize) -> Self {
        self.max_len = Some(max_len);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn style(mut self, style: InputStyle) -> Self {
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

    pub fn left_slot<F>(mut self, slot_fn: F) -> Self
    where
        F: Fn(&Painter, Rect, Color32) + 'a,
    {
        self.left_slot = Some(Box::new(slot_fn));
        self
    }

    pub fn right_slot<F>(mut self, slot_fn: F) -> Self
    where
        F: Fn(&Painter, Rect, Color32) + 'a,
    {
        self.right_slot = Some(Box::new(slot_fn));
        self
    }
}

pub struct Input<'a, Id>
where
    Id: Hash + Debug,
{
    pub id_source: Id,
    pub placeholder: &'a str,
    pub variant: InputVariant,
    pub size: InputSize,
    pub radius: InputRadius,
    pub input_type: InputType,
    pub enabled: bool,
    pub read_only: bool,
    pub is_invalid: bool,
    pub max_len: Option<usize>,
    pub width: Option<f32>,
    pub style: Option<InputStyle>,
    pub accent_color: Option<Color32>,
    pub high_contrast: bool,
    #[allow(clippy::type_complexity)]
    pub left_slot: BoxedSlotFn<'a>,
    #[allow(clippy::type_complexity)]
    pub right_slot: BoxedSlotFn<'a>,
}

impl<'a, Id: Hash + Debug> Input<'a, Id> {
    pub fn new(id_source: Id) -> Input<'static, Id> {
        Input {
            id_source,
            placeholder: "",
            variant: InputVariant::Surface,
            size: InputSize::Size2,
            radius: InputRadius::Medium,
            input_type: InputType::Text,
            enabled: true,
            read_only: false,
            is_invalid: false,
            max_len: None,
            width: None,
            style: None,
            accent_color: None,
            high_contrast: false,
            left_slot: None,
            right_slot: None,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    pub fn radius(mut self, radius: InputRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn input_type(mut self, input_type: InputType) -> Self {
        self.input_type = input_type;
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

    pub fn max_len(mut self, max_len: usize) -> Self {
        self.max_len = Some(max_len);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn style(mut self, style: InputStyle) -> Self {
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

    pub fn left_slot<F>(mut self, slot_fn: F) -> Self
    where
        F: Fn(&Painter, Rect, Color32) + 'a,
    {
        self.left_slot = Some(Box::new(slot_fn));
        self
    }

    pub fn right_slot<F>(mut self, slot_fn: F) -> Self
    where
        F: Fn(&Painter, Rect, Color32) + 'a,
    {
        self.right_slot = Some(Box::new(slot_fn));
        self
    }

    pub fn show(self, ui: &mut Ui, theme: &Theme, value: &mut String) -> Response {
        let props = InputProps {
            id_source: self.id_source,
            value,
            placeholder: self.placeholder,
            variant: self.variant,
            size: self.size,
            radius: self.radius,
            input_type: self.input_type,
            enabled: self.enabled,
            read_only: self.read_only,
            is_invalid: self.is_invalid,
            max_len: self.max_len,
            width: self.width,
            style: self.style,
            accent_color: self.accent_color,
            high_contrast: self.high_contrast,
            left_slot: self.left_slot,
            right_slot: self.right_slot,
        };
        text_input_with_props(ui, theme, props)
    }
}

#[derive(Clone, Debug)]
pub struct InputConfig {
    pub variant: TokenInputVariant,
    pub size: InputSize,
    pub is_invalid: bool,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            variant: TokenInputVariant::Surface,
            size: InputSize::Size2,
            is_invalid: false,
        }
    }
}

pub fn resolve_input_style(palette: &ColorPalette, config: &InputConfig) -> InputStyle {
    let variant = match config.variant {
        TokenInputVariant::Surface => InputVariant::Surface,
        TokenInputVariant::Classic => InputVariant::Classic,
        TokenInputVariant::Soft => InputVariant::Soft,
    };
    InputStyle::from_palette(palette, variant)
}

pub fn text_input_with_props<Id>(ui: &mut Ui, theme: &Theme, props: InputProps<'_, Id>) -> Response
where
    Id: Hash + Debug,
{
    trace!(
        "Rendering input variant={:?} size={:?} type={:?} invalid={} enabled={} read_only={}",
        props.variant,
        props.size,
        props.input_type,
        props.is_invalid,
        props.enabled,
        props.read_only
    );

    let apply_opacity = |color: Color32, opacity: f32| -> Color32 {
        Color32::from_rgba_unmultiplied(
            color.r(),
            color.g(),
            color.b(),
            (color.a() as f32 * opacity) as u8,
        )
    };

    let mut style = props.style.clone().unwrap_or_else(|| {
        if let Some(accent) = props.accent_color {
            InputStyle::from_palette_with_accent(&theme.palette, props.variant, accent)
        } else {
            InputStyle::from_palette(&theme.palette, props.variant)
        }
    });

    if props.high_contrast {
        style = style.with_high_contrast();
    }

    style.rounding = props.radius.corner_radius();

    let effectively_disabled = !props.enabled || props.read_only;

    let height = props.size.height();
    let width = props.width.unwrap_or(200.0);
    let padding = props.size.padding();
    let slot_gap = props.size.slot_gap();
    let slot_icon_size = props.size.slot_icon_size();

    let slot_width = |slot: &BoxedSlotFn<'_>| -> f32 {
        if slot.is_some() {
            slot_icon_size + slot_gap * 2.0
        } else {
            0.0
        }
    };

    let left_slot_width = slot_width(&props.left_slot);
    let right_slot_width = slot_width(&props.right_slot);

    let id = ui.make_persistent_id(&props.id_source);
    let desired_size = vec2(width, height);

    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    let edit_id = id.with("edit");
    let has_focus = response.has_focus() || ui.memory(|m| m.has_focus(edit_id));

    let bg_color = if effectively_disabled {
        apply_opacity(style.bg, style.disabled_opacity)
    } else {
        match (has_focus, response.hovered()) {
            (true, _) => style.bg_focus,
            (false, true) => style.bg_hover,
            (false, false) => style.bg,
        }
    };

    let border_color = match (
        props.is_invalid,
        has_focus,
        response.hovered() && !effectively_disabled,
    ) {
        (true, _, _) => style.invalid_border,
        (false, true, _) => style.border_focus,
        (false, false, true) => style.border_hover,
        _ => style.border,
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

    let slot_color = |color: Color32| -> Color32 {
        if effectively_disabled {
            apply_opacity(color, style.disabled_opacity)
        } else {
            color
        }
    };

    let paint_slot = |slot_fn: &BoxedSlotFn<'_>, align_left: bool| {
        if let Some(slot_fn) = slot_fn.as_ref() {
            let x = if align_left {
                rect.left() + slot_gap
            } else {
                rect.right() - slot_gap - slot_icon_size
            };
            let slot_rect = Rect::from_min_size(
                pos2(x, rect.top() + (height - slot_icon_size) / 2.0),
                vec2(slot_icon_size, slot_icon_size),
            );
            slot_fn(ui.painter(), slot_rect, slot_color(style.slot_color));
        }
    };

    paint_slot(&props.left_slot, true);
    paint_slot(&props.right_slot, false);

    let inner_rect = Rect::from_min_max(
        pos2(
            rect.left() + padding.x + left_slot_width,
            rect.top() + padding.y,
        ),
        pos2(
            rect.right() - padding.x - right_slot_width,
            rect.bottom() - padding.y,
        ),
    );

    let text_color = if effectively_disabled {
        apply_opacity(style.text_color, 0.6)
    } else {
        style.text_color
    };

    let placeholder_colored: WidgetText = props.placeholder.into();
    let placeholder_colored = placeholder_colored.color(style.placeholder_color);

    let token_variant = TokenInputVariant::from(props.variant);
    let tokens = input_tokens(&theme.palette, token_variant);

    let vertical_margin = (inner_rect.height() / 2.0) - (props.size.font_size() * 0.54);

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

        for visuals in [
            &mut inner_style.visuals.widgets.inactive,
            &mut inner_style.visuals.widgets.hovered,
            &mut inner_style.visuals.widgets.active,
        ] {
            visuals.bg_fill = Color32::TRANSPARENT;
            visuals.weak_bg_fill = Color32::TRANSPARENT;
            visuals.bg_stroke = Stroke::NONE;
        }

        inner_ui.set_style(inner_style);

        let mut edit = TextEdit::singleline(props.value)
            .id(edit_id)
            .hint_text(placeholder_colored)
            .text_color(text_color)
            .frame(false)
            .margin(vec2(0.0, vertical_margin))
            .desired_width(inner_rect.width());

        if props.input_type.is_password() {
            edit = edit.password(true);
        }

        if let Some(limit) = props.max_len {
            edit = edit.char_limit(limit);
        }

        if props.read_only {
            edit = edit.interactive(false);
        }

        inner_ui.add_enabled(props.enabled, edit)
    });

    if response.inner.clicked_elsewhere()
        && rect.contains(ui.ctx().pointer_hover_pos().unwrap_or_default())
    {
        ui.memory_mut(|m| m.request_focus(edit_id));
    }

    response.inner
}

pub fn text_input(ui: &mut Ui, theme: &Theme, value: &mut String) -> Response {
    text_input_with_config(ui, theme, value, "text_input", InputConfig::default())
}

pub fn text_input_with_config<Id: Hash + Debug>(
    ui: &mut Ui,
    theme: &Theme,
    value: &mut String,
    id_source: Id,
    config: InputConfig,
) -> Response {
    let variant = match config.variant {
        TokenInputVariant::Surface => InputVariant::Surface,
        TokenInputVariant::Classic => InputVariant::Classic,
        TokenInputVariant::Soft => InputVariant::Soft,
    };

    let props = InputProps::new(id_source, value)
        .variant(variant)
        .size(config.size)
        .invalid(config.is_invalid);

    text_input_with_props(ui, theme, props)
}
