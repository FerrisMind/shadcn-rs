use crate::theme::Theme;
use crate::tokens::{ColorPalette, ControlSize, ControlVariant, ease_out_cubic, mix};
use egui::{
    Color32, CornerRadius, FontId, Painter, Pos2, Rect, Response, Sense, Stroke, StrokeKind,
    TextStyle, TextWrapMode, Ui, Vec2, WidgetText, pos2, vec2,
};
use log::trace;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,

    Solid,

    Classic,

    Soft,

    Surface,

    Destructive,

    Outline,

    Secondary,

    Ghost,

    Link,
}

impl From<ControlVariant> for ButtonVariant {
    fn from(variant: ControlVariant) -> Self {
        match variant {
            ControlVariant::Primary => ButtonVariant::Default,
            ControlVariant::Destructive => ButtonVariant::Destructive,
            ControlVariant::Outline => ButtonVariant::Outline,
            ControlVariant::Secondary => ButtonVariant::Secondary,
            ControlVariant::Ghost => ButtonVariant::Ghost,
            ControlVariant::Link => ButtonVariant::Link,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonRadius {
    None,

    Small,

    #[default]
    Medium,

    Large,

    Full,

    Custom(CornerRadius),
}

impl ButtonRadius {
    pub fn corner_radius(self) -> CornerRadius {
        match self {
            ButtonRadius::None => CornerRadius::same(0),
            ButtonRadius::Small => CornerRadius::same(4),
            ButtonRadius::Medium => CornerRadius::same(8),
            ButtonRadius::Large => CornerRadius::same(12),
            ButtonRadius::Full => CornerRadius::same(255),
            ButtonRadius::Custom(r) => r,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Sm,

    #[default]
    Default,

    Lg,

    Icon,

    IconSm,

    IconLg,
}

impl ButtonSize {
    pub fn height(self) -> f32 {
        match self {
            ButtonSize::Sm | ButtonSize::IconSm => 32.0,
            ButtonSize::Default | ButtonSize::Icon => 36.0,
            ButtonSize::Lg | ButtonSize::IconLg => 40.0,
        }
    }

    pub fn padding_x(self) -> f32 {
        match self {
            ButtonSize::Sm => 12.0,
            ButtonSize::Default => 16.0,
            ButtonSize::Lg => 24.0,
            ButtonSize::Icon | ButtonSize::IconSm | ButtonSize::IconLg => 0.0,
        }
    }

    pub fn padding_y(self) -> f32 {
        match self {
            ButtonSize::Sm => 6.0,
            ButtonSize::Default => 8.0,
            ButtonSize::Lg => 10.0,
            ButtonSize::Icon | ButtonSize::IconSm | ButtonSize::IconLg => 0.0,
        }
    }

    pub fn padding(self) -> Vec2 {
        vec2(self.padding_x(), self.padding_y())
    }

    pub fn rounding(self) -> CornerRadius {
        match self {
            ButtonSize::Sm | ButtonSize::IconSm => CornerRadius::same(6),
            ButtonSize::Default | ButtonSize::Icon => CornerRadius::same(8),
            ButtonSize::Lg | ButtonSize::IconLg => CornerRadius::same(10),
        }
    }

    pub fn font_size(self) -> f32 {
        match self {
            ButtonSize::Sm | ButtonSize::IconSm => 13.0,
            ButtonSize::Default | ButtonSize::Icon => 14.0,
            ButtonSize::Lg | ButtonSize::IconLg => 15.0,
        }
    }

    pub fn font(self) -> FontId {
        FontId::proportional(self.font_size())
    }

    pub fn is_icon(self) -> bool {
        matches!(
            self,
            ButtonSize::Icon | ButtonSize::IconSm | ButtonSize::IconLg
        )
    }

    pub fn icon_width(self) -> f32 {
        self.height()
    }

    pub fn gap(self) -> f32 {
        match self {
            ButtonSize::Sm | ButtonSize::IconSm => 6.0,
            ButtonSize::Default | ButtonSize::Icon => 8.0,
            ButtonSize::Lg | ButtonSize::IconLg => 12.0,
        }
    }

    pub fn icon_size(self) -> f32 {
        match self {
            ButtonSize::Sm | ButtonSize::IconSm => 14.0,
            ButtonSize::Default | ButtonSize::Icon => 16.0,
            ButtonSize::Lg | ButtonSize::IconLg => 18.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonScale {
    #[default]
    Size2,
    Size1,
    Size3,
    Size4,
}

impl From<ButtonScale> for ButtonSize {
    fn from(scale: ButtonScale) -> Self {
        match scale {
            ButtonScale::Size1 => ButtonSize::Sm,
            ButtonScale::Size2 => ButtonSize::Default,
            ButtonScale::Size3 | ButtonScale::Size4 => ButtonSize::Lg,
        }
    }
}

impl From<ControlSize> for ButtonSize {
    fn from(size: ControlSize) -> Self {
        match size {
            ControlSize::Sm => ButtonSize::Sm,
            ControlSize::Md => ButtonSize::Default,
            ControlSize::Lg => ButtonSize::Lg,
            ControlSize::IconSm => ButtonSize::IconSm,
            ControlSize::Icon => ButtonSize::Icon,
            ControlSize::IconLg => ButtonSize::IconLg,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ButtonStyle {
    pub bg: Color32,

    pub bg_hover: Color32,

    pub bg_active: Color32,

    pub text: Color32,

    pub text_hover: Color32,

    pub text_active: Color32,

    pub border: Color32,

    pub border_hover: Color32,

    pub focus_ring: Color32,

    pub disabled_opacity: f32,

    pub rounding: CornerRadius,
}

impl ButtonStyle {
    pub fn from_variant(palette: &ColorPalette, variant: ButtonVariant) -> Self {
        let focus_ring = Color32::from_rgba_unmultiplied(
            palette.ring.r(),
            palette.ring.g(),
            palette.ring.b(),
            128,
        );
        match variant {
            ButtonVariant::Default | ButtonVariant::Solid => Self {
                bg: palette.primary,
                bg_hover: mix(palette.primary, palette.background, 0.12),
                bg_active: mix(palette.primary, palette.background, 0.22),
                text: palette.primary_foreground,
                text_hover: palette.primary_foreground,
                text_active: palette.primary_foreground,
                border: Color32::TRANSPARENT,
                border_hover: Color32::TRANSPARENT,
                focus_ring,
                disabled_opacity: 0.5,
                rounding: CornerRadius::same(8),
            },
            ButtonVariant::Classic => Self {
                bg: palette.primary,
                bg_hover: mix(palette.primary, palette.background, 0.08),
                bg_active: mix(palette.primary, palette.background, 0.15),
                text: palette.primary_foreground,
                text_hover: palette.primary_foreground,
                text_active: palette.primary_foreground,
                border: mix(palette.primary, palette.background, 0.22),
                border_hover: mix(palette.primary, palette.background, 0.27),
                focus_ring,
                disabled_opacity: 0.5,
                rounding: CornerRadius::same(8),
            },
            ButtonVariant::Soft => {
                let soft_bg = Color32::from_rgba_unmultiplied(
                    palette.primary.r(),
                    palette.primary.g(),
                    palette.primary.b(),
                    30,
                );
                Self {
                    bg: soft_bg,
                    bg_hover: Color32::from_rgba_unmultiplied(
                        palette.primary.r(),
                        palette.primary.g(),
                        palette.primary.b(),
                        45,
                    ),
                    bg_active: Color32::from_rgba_unmultiplied(
                        palette.primary.r(),
                        palette.primary.g(),
                        palette.primary.b(),
                        60,
                    ),
                    text: palette.foreground,
                    text_hover: palette.foreground,
                    text_active: palette.foreground,
                    border: Color32::TRANSPARENT,
                    border_hover: Color32::TRANSPARENT,
                    focus_ring,
                    disabled_opacity: 0.5,
                    rounding: CornerRadius::same(8),
                }
            }
            ButtonVariant::Surface => {
                let surface_bg = Color32::from_rgba_unmultiplied(
                    palette.primary.r(),
                    palette.primary.g(),
                    palette.primary.b(),
                    20,
                );
                Self {
                    bg: surface_bg,
                    bg_hover: Color32::from_rgba_unmultiplied(
                        palette.primary.r(),
                        palette.primary.g(),
                        palette.primary.b(),
                        30,
                    ),
                    bg_active: Color32::from_rgba_unmultiplied(
                        palette.primary.r(),
                        palette.primary.g(),
                        palette.primary.b(),
                        45,
                    ),
                    text: palette.foreground,
                    text_hover: palette.foreground,
                    text_active: palette.foreground,
                    border: Color32::from_rgba_unmultiplied(
                        palette.primary.r(),
                        palette.primary.g(),
                        palette.primary.b(),
                        100,
                    ),
                    border_hover: Color32::from_rgba_unmultiplied(
                        palette.primary.r(),
                        palette.primary.g(),
                        palette.primary.b(),
                        130,
                    ),
                    focus_ring,
                    disabled_opacity: 0.5,
                    rounding: CornerRadius::same(8),
                }
            }
            ButtonVariant::Destructive => {
                let destructive_ring = Color32::from_rgba_unmultiplied(
                    palette.destructive.r(),
                    palette.destructive.g(),
                    palette.destructive.b(),
                    51,
                );
                Self {
                    bg: palette.destructive,
                    bg_hover: mix(palette.destructive, Color32::WHITE, 0.1),
                    bg_active: mix(palette.destructive, Color32::WHITE, 0.15),
                    text: Color32::WHITE,
                    text_hover: Color32::WHITE,
                    text_active: Color32::WHITE,
                    border: Color32::TRANSPARENT,
                    border_hover: Color32::TRANSPARENT,
                    focus_ring: destructive_ring,
                    disabled_opacity: 0.5,
                    rounding: CornerRadius::same(8),
                }
            }
            ButtonVariant::Outline => outline_variant_style(palette, palette.accent, palette.input),
            ButtonVariant::Secondary => Self {
                bg: palette.secondary,
                bg_hover: mix(palette.secondary, Color32::WHITE, 0.08),
                bg_active: mix(palette.secondary, Color32::WHITE, 0.12),
                text: palette.secondary_foreground,
                text_hover: palette.secondary_foreground,
                text_active: palette.secondary_foreground,
                border: Color32::TRANSPARENT,
                border_hover: Color32::TRANSPARENT,
                focus_ring,
                disabled_opacity: 0.5,
                rounding: CornerRadius::same(8),
            },
            ButtonVariant::Ghost => Self {
                bg: Color32::TRANSPARENT,
                bg_hover: palette.accent,
                bg_active: mix(palette.accent, Color32::WHITE, 0.1),
                text: palette.foreground,
                text_hover: palette.foreground,
                text_active: palette.foreground,
                border: Color32::TRANSPARENT,
                border_hover: Color32::TRANSPARENT,
                focus_ring,
                disabled_opacity: 0.5,
                rounding: CornerRadius::same(8),
            },
            ButtonVariant::Link => Self {
                bg: Color32::TRANSPARENT,
                bg_hover: Color32::TRANSPARENT,
                bg_active: Color32::TRANSPARENT,
                text: palette.primary,
                text_hover: palette.primary,
                text_active: palette.primary,
                border: Color32::TRANSPARENT,
                border_hover: Color32::TRANSPARENT,
                focus_ring,
                disabled_opacity: 0.5,
                rounding: CornerRadius::same(8),
            },
        }
    }

    pub fn from_variant_with_accent(
        palette: &ColorPalette,
        variant: ButtonVariant,
        accent: Color32,
    ) -> Self {
        let mut style = Self::from_variant(palette, variant);
        match variant {
            ButtonVariant::Default | ButtonVariant::Solid => {
                style.bg = accent;
                style.bg_hover = mix(accent, palette.background, 0.12);
                style.bg_active = mix(accent, palette.background, 0.22);
                style.text = compute_contrast_color(accent, palette);
                style.focus_ring = mix(accent, palette.background, 0.35);
            }
            ButtonVariant::Classic => {
                style.bg = accent;
                style.bg_hover = mix(accent, palette.background, 0.08);
                style.bg_active = mix(accent, palette.background, 0.15);
                style.text = compute_contrast_color(accent, palette);
                style.border = mix(accent, palette.background, 0.22);
                style.border_hover = mix(accent, palette.background, 0.27);
                style.focus_ring = mix(accent, palette.background, 0.4);
            }
            ButtonVariant::Soft => {
                style.bg = Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 30);
                style.bg_hover =
                    Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 45);
                style.bg_active =
                    Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 60);
                style.text = accent;
                style.focus_ring =
                    Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 100);
            }
            ButtonVariant::Surface => {
                style.bg = Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 20);
                style.bg_hover =
                    Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 30);
                style.bg_active =
                    Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 45);
                style.text = accent;
                style.border =
                    Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 100);
                style.border_hover =
                    Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 130);
                style.focus_ring =
                    Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 100);
            }
            ButtonVariant::Destructive => {}
            ButtonVariant::Outline => {
                style = outline_variant_style(palette, accent, accent);
            }
            ButtonVariant::Secondary => {}
            ButtonVariant::Ghost => {
                style.text = accent;
                style.text_hover = accent;
                style.text_active = accent;
            }
            ButtonVariant::Link => {
                style.text = accent;
                style.text_hover = accent;
                style.text_active = accent;
                style.focus_ring =
                    Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 128);
            }
        }
        style
    }

    pub fn with_high_contrast(mut self, palette: &ColorPalette) -> Self {
        self.bg = mix(self.bg, palette.foreground, 0.15);
        self.bg_hover = mix(self.bg_hover, palette.foreground, 0.15);
        self.text = palette.foreground;
        self
    }
}

fn outline_variant_style(
    palette: &ColorPalette,
    _accent: Color32,
    _border_color: Color32,
) -> ButtonStyle {
    let focus_ring =
        Color32::from_rgba_unmultiplied(palette.ring.r(), palette.ring.g(), palette.ring.b(), 128);

    // Согласно Radix UI Themes (референс для shadcn/ui):
    // - Фон прозрачный
    // - Border: на основе accent цвета с alpha ~50% (accent-a8 в Radix)
    // - Текст: foreground цвет
    // - Hover: заметный полупрозрачный фон
    let bg_transparent = Color32::TRANSPARENT;
    let bg_hover_subtle = Color32::from_rgba_unmultiplied(
        palette.foreground.r(),
        palette.foreground.g(),
        palette.foreground.b(),
        55, // ~10% непрозрачности для заметного hover
    );
    let bg_active_subtle = Color32::from_rgba_unmultiplied(
        palette.foreground.r(),
        palette.foreground.g(),
        palette.foreground.b(),
        68, // ~15% непрозрачности для active
    );

    // Граница на основе foreground с хорошей видимостью (~50% альфа)
    let border_visible = Color32::from_rgba_unmultiplied(
        palette.foreground.r(),
        palette.foreground.g(),
        palette.foreground.b(),
        128, // 50% непрозрачности для хорошей видимости
    );

    ButtonStyle {
        bg: bg_transparent,
        bg_hover: bg_hover_subtle,
        bg_active: bg_active_subtle,
        text: palette.foreground,
        text_hover: palette.foreground,
        text_active: palette.foreground,
        border: border_visible,
        border_hover: border_visible,
        focus_ring,
        disabled_opacity: 0.5,
        rounding: CornerRadius::same(8),
    }
}

fn compute_contrast_color(bg: Color32, palette: &ColorPalette) -> Color32 {
    let luminance = 0.299 * bg.r() as f32 + 0.587 * bg.g() as f32 + 0.114 * bg.b() as f32;
    if luminance > 128.0 {
        palette.background
    } else {
        palette.foreground
    }
}

fn apply_disabled_opacity(color: Color32, disabled_opacity: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(
        color.r(),
        color.g(),
        color.b(),
        (color.a() as f32 * disabled_opacity) as u8,
    )
}

fn resolve_style(theme: &Theme, props: &ButtonProps<'_>) -> ButtonStyle {
    let mut style = props.style.clone().unwrap_or_else(|| {
        if let Some(accent) = props.accent_color {
            ButtonStyle::from_variant_with_accent(&theme.palette, props.variant, accent)
        } else {
            ButtonStyle::from_variant(&theme.palette, props.variant)
        }
    });

    style.rounding = props.radius.corner_radius();

    if props.high_contrast {
        style = style.with_high_contrast(&theme.palette);
    }

    style
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonJustify {
    Start,

    #[default]
    Center,

    Between,
}

fn desired_button_size(ui: &Ui, props: &ButtonProps<'_>) -> Vec2 {
    let height = props.size.height();

    let width = if props.size.is_icon() {
        props.size.icon_width()
    } else {
        let text_galley = props.label.clone().into_galley(
            ui,
            Some(TextWrapMode::Extend),
            f32::INFINITY,
            TextStyle::Button,
        );
        let text_width = text_galley.size().x;

        let leading_width = if props.icon.is_some() || props.loading {
            props.size.icon_size() + props.size.gap()
        } else {
            0.0
        };

        let trailing_width = if props.trailing_icon.is_some() {
            props.size.gap() + props.size.icon_size()
        } else {
            0.0
        };

        text_width + leading_width + trailing_width + props.size.padding_x() * 2.0
    };

    let base_width = width.max(40.0);
    let target_width = props.min_width.unwrap_or(base_width);

    vec2(target_width.max(base_width), height)
}

fn background_color(
    style: &ButtonStyle,
    effectively_disabled: bool,
    hover_t: f32,
    active_t: f32,
) -> Color32 {
    if effectively_disabled {
        apply_disabled_opacity(style.bg, style.disabled_opacity)
    } else {
        let hover_bg = mix(style.bg, style.bg_hover, hover_t);
        mix(hover_bg, style.bg_active, active_t)
    }
}

fn text_color(
    style: &ButtonStyle,
    effectively_disabled: bool,
    hover_t: f32,
    active_t: f32,
) -> Color32 {
    if effectively_disabled {
        apply_disabled_opacity(style.text, style.disabled_opacity)
    } else {
        let hover_text = mix(style.text, style.text_hover, hover_t);
        mix(hover_text, style.text_active, active_t)
    }
}

fn border_color(style: &ButtonStyle, effectively_disabled: bool, hover_t: f32) -> Color32 {
    if effectively_disabled {
        apply_disabled_opacity(style.border, style.disabled_opacity)
    } else {
        mix(style.border, style.border_hover, hover_t)
    }
}

fn paint_background(
    painter: &Painter,
    rect: egui::Rect,
    style: &ButtonStyle,
    bg_color: Color32,
    border_color: Color32,
) {
    painter.rect_filled(rect, style.rounding, bg_color);

    if border_color != Color32::TRANSPARENT {
        painter.rect_stroke(
            rect,
            style.rounding,
            Stroke::new(1.0, border_color),
            StrokeKind::Inside,
        );
    }
}

fn paint_focus_ring(painter: &Painter, rect: egui::Rect, style: &ButtonStyle, has_focus: bool) {
    if has_focus {
        let ring_rect = rect.expand(2.0);
        painter.rect_stroke(
            ring_rect,
            style.rounding,
            Stroke::new(3.0, style.focus_ring),
            StrokeKind::Outside,
        );
    }
}

fn paint_icon_button(
    ui: &Ui,
    painter: &Painter,
    props: &ButtonProps<'_>,
    text_color: Color32,
    center: Pos2,
) {
    let icon_size = props.size.icon_size();
    if props.loading {
        let t = ui.ctx().input(|i| i.time) as f32;
        draw_spinner(painter, center, icon_size, text_color, t * 2.0);
        ui.ctx().request_repaint();
    } else if let Some(icon_fn) = props.icon {
        icon_fn(painter, center, icon_size, text_color);
    } else {
        let label_text = props.label.text().to_string();
        if !label_text.is_empty() {
            let text_galley = painter.layout_no_wrap(label_text, props.size.font(), text_color);
            let text_pos = pos2(
                center.x - text_galley.rect.width() / 2.0,
                center.y - text_galley.rect.height() / 2.0,
            );
            painter.galley(text_pos, text_galley, text_color);
        }
    }
}

fn paint_text_button(
    ui: &Ui,
    painter: &Painter,
    props: &ButtonProps<'_>,
    text_color: Color32,
    rect: Rect,
) {
    let icon_size = props.size.icon_size();
    let gap = props.size.gap();

    let text_galley = props.label.clone().into_galley(
        ui,
        Some(TextWrapMode::Extend),
        f32::INFINITY,
        TextStyle::Button,
    );
    let text_width = text_galley.size().x;

    let leading_width = if props.loading || props.icon.is_some() {
        icon_size + gap
    } else {
        0.0
    };

    let trailing_width = if props.trailing_icon.is_some() {
        gap + icon_size
    } else {
        0.0
    };

    let content_width = leading_width + text_width + trailing_width;
    let center = rect.center();
    let padding_x = props.size.padding_x();

    let start_x = match props.justify {
        ButtonJustify::Center => center.x - content_width / 2.0,
        ButtonJustify::Start | ButtonJustify::Between => rect.left() + padding_x,
    };

    let text_y = center.y - text_galley.size().y / 2.0;

    let trailing_anchor_x =
        if props.justify == ButtonJustify::Between && props.trailing_icon.is_some() {
            rect.right() - padding_x
        } else {
            start_x + content_width
        };

    if props.loading {
        let spinner_center = pos2(start_x + icon_size / 2.0, center.y);
        let t = ui.ctx().input(|i| i.time) as f32;
        draw_spinner(painter, spinner_center, icon_size, text_color, t * 2.0);
        ui.ctx().request_repaint();

        let text_pos = pos2(start_x + icon_size + gap, text_y);
        painter.galley(text_pos, text_galley, text_color);
    } else if let Some(icon_fn) = props.icon {
        let icon_center = pos2(start_x + icon_size / 2.0, center.y);
        icon_fn(painter, icon_center, icon_size, text_color);

        let text_pos = pos2(start_x + icon_size + gap, text_y);
        painter.galley(text_pos, text_galley, text_color);
    } else {
        let text_pos = pos2(start_x, text_y);
        painter.galley(text_pos, text_galley, text_color);
    }

    if let Some(trailing_icon) = props.trailing_icon {
        let icon_center = pos2(trailing_anchor_x - icon_size / 2.0, center.y);
        trailing_icon(painter, icon_center, icon_size, text_color);
    }
}

fn paint_link_underline(
    ui: &Ui,
    painter: &Painter,
    props: &ButtonProps<'_>,
    text_color: Color32,
    center: Pos2,
) {
    let text_galley = props.label.clone().into_galley(
        ui,
        Some(TextWrapMode::Extend),
        f32::INFINITY,
        TextStyle::Button,
    );
    let text_width = text_galley.size().x;
    let text_bottom = center.y + text_galley.size().y / 2.0 - 2.0;
    let underline_y = text_bottom + 2.0;

    painter.line_segment(
        [
            pos2(center.x - text_width / 2.0, underline_y),
            pos2(center.x + text_width / 2.0, underline_y),
        ],
        Stroke::new(1.0, text_color),
    );
}

#[derive(Clone)]
pub struct ButtonProps<'a> {
    pub label: WidgetText,

    pub variant: ButtonVariant,

    pub size: ButtonSize,

    pub scale: ButtonScale,

    pub radius: ButtonRadius,

    pub enabled: bool,

    pub loading: bool,

    pub high_contrast: bool,

    pub accent_color: Option<Color32>,

    pub style: Option<ButtonStyle>,

    #[allow(clippy::type_complexity)]
    pub icon: Option<&'a dyn Fn(&Painter, Pos2, f32, Color32)>,

    #[allow(clippy::type_complexity)]
    pub trailing_icon: Option<&'a dyn Fn(&Painter, Pos2, f32, Color32)>,

    pub justify: ButtonJustify,

    pub min_width: Option<f32>,
}

impl<'a> std::fmt::Debug for ButtonProps<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ButtonProps")
            .field("label", &self.label)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("radius", &self.radius)
            .field("enabled", &self.enabled)
            .field("loading", &self.loading)
            .field("high_contrast", &self.high_contrast)
            .field("accent_color", &self.accent_color)
            .field("style", &self.style)
            .field("icon", &self.icon.as_ref().map(|_| "<fn>"))
            .field(
                "trailing_icon",
                &self.trailing_icon.as_ref().map(|_| "<fn>"),
            )
            .field("justify", &self.justify)
            .finish()
    }
}

impl<'a> ButtonProps<'a> {
    pub fn new(label: impl Into<WidgetText>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            scale: ButtonScale::Size2,
            radius: ButtonRadius::default(),
            enabled: true,
            loading: false,
            high_contrast: false,
            accent_color: None,
            style: None,
            icon: None,
            trailing_icon: None,
            justify: ButtonJustify::default(),
            min_width: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn scale(mut self, scale: ButtonScale) -> Self {
        self.scale = scale;
        self.size = ButtonSize::from(scale);
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn accent_color(mut self, color: Color32) -> Self {
        self.accent_color = Some(color);
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.accent_color = Some(color);
        self
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn icon(mut self, icon: &'a dyn Fn(&Painter, Pos2, f32, Color32)) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn trailing_icon(mut self, icon: &'a dyn Fn(&Painter, Pos2, f32, Color32)) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    pub fn justify(mut self, justify: ButtonJustify) -> Self {
        self.justify = justify;
        self
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    pub fn show(self, ui: &mut Ui, theme: &Theme) -> Response {
        button_with_props(ui, theme, self)
    }
}

#[derive(Clone)]
pub struct Button<'a> {
    props: ButtonProps<'a>,
}

impl<'a> Button<'a> {
    pub fn new(label: impl Into<WidgetText>) -> Self {
        Self {
            props: ButtonProps::new(label),
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.props.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.props.size = size;
        self
    }

    pub fn scale(mut self, scale: ButtonScale) -> Self {
        self.props.scale = scale;
        self.props.size = ButtonSize::from(scale);
        self
    }

    pub fn radius(mut self, radius: ButtonRadius) -> Self {
        self.props.radius = radius;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.props.enabled = enabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.props.loading = loading;
        self
    }

    pub fn high_contrast(mut self, high_contrast: bool) -> Self {
        self.props.high_contrast = high_contrast;
        self
    }

    pub fn accent_color(mut self, color: Color32) -> Self {
        self.props.accent_color = Some(color);
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.props.accent_color = Some(color);
        self
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.props.style = Some(style);
        self
    }

    pub fn icon(mut self, icon: &'a dyn Fn(&Painter, Pos2, f32, Color32)) -> Self {
        self.props.icon = Some(icon);
        self
    }

    pub fn trailing_icon(mut self, icon: &'a dyn Fn(&Painter, Pos2, f32, Color32)) -> Self {
        self.props.trailing_icon = Some(icon);
        self
    }

    pub fn justify(mut self, justify: ButtonJustify) -> Self {
        self.props.justify = justify;
        self
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.props.min_width = Some(width);
        self
    }

    pub fn show(self, ui: &mut Ui, theme: &Theme) -> Response {
        button_with_props(ui, theme, self.props)
    }
}

fn draw_spinner(painter: &Painter, center: Pos2, size: f32, color: Color32, t: f32) {
    let segments = 12;
    let angle_offset = t * std::f32::consts::TAU;

    for i in 0..segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU + angle_offset;
        let opacity = ((segments - i) as f32 / segments as f32 * 255.0) as u8;
        let seg_color = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), opacity);

        let inner_r = size * 0.35;
        let outer_r = size * 0.5;

        let inner = pos2(
            center.x + angle.cos() * inner_r,
            center.y + angle.sin() * inner_r,
        );
        let outer = pos2(
            center.x + angle.cos() * outer_r,
            center.y + angle.sin() * outer_r,
        );

        painter.line_segment([inner, outer], Stroke::new(2.0, seg_color));
    }
}

fn button_with_props(ui: &mut Ui, theme: &Theme, props: ButtonProps<'_>) -> Response {
    trace!(
        "Rendering button variant={:?} size={:?} enabled={} loading={}",
        props.variant, props.size, props.enabled, props.loading
    );

    ui.scope(|ui| {
        let mut scoped_style = ui.style().as_ref().clone();
        scoped_style
            .text_styles
            .insert(TextStyle::Button, props.size.font());
        ui.set_style(scoped_style);

        let style = resolve_style(theme, &props);
        let effectively_disabled = !props.enabled || props.loading;

        let desired_size = desired_button_size(ui, &props);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        let painter = ui.painter();

        let is_hovered = response.hovered() && !effectively_disabled;
        let is_pressed = response.is_pointer_button_down_on() && !effectively_disabled;
        let has_focus = response.has_focus() && !effectively_disabled;

        let anim_duration = theme.motion.base_ms / 1000.0;
        let active_t = ui.ctx().animate_bool_with_time_and_easing(
            response.id.with("active"),
            is_pressed,
            anim_duration,
            ease_out_cubic,
        );
        let hover_t = ui.ctx().animate_bool_with_time_and_easing(
            response.id.with("hover"),
            is_hovered,
            anim_duration,
            ease_out_cubic,
        );

        let bg_color = background_color(&style, effectively_disabled, hover_t, active_t);
        let text_color = text_color(&style, effectively_disabled, hover_t, active_t);
        let border_color = border_color(&style, effectively_disabled, hover_t);

        paint_background(painter, rect, &style, bg_color, border_color);
        paint_focus_ring(painter, rect, &style, has_focus);

        if props.size.is_icon() {
            paint_icon_button(ui, painter, &props, text_color, rect.center());
        } else {
            paint_text_button(ui, painter, &props, text_color, rect);
        }

        if props.variant == ButtonVariant::Link && is_hovered {
            paint_link_underline(ui, painter, &props, text_color, rect.center());
        }

        response
    })
    .inner
}

pub fn button(
    ui: &mut Ui,
    theme: &Theme,
    label: impl Into<WidgetText>,
    variant: ControlVariant,
    size: ControlSize,
    enabled: bool,
) -> Response {
    Button::new(label)
        .variant(ButtonVariant::from(variant))
        .size(ButtonSize::from(size))
        .enabled(enabled)
        .show(ui, theme)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_from_scale_matches_expected() {
        assert_eq!(ButtonSize::from(ButtonScale::Size1), ButtonSize::Sm);
        assert_eq!(ButtonSize::from(ButtonScale::Size2), ButtonSize::Default);
        assert_eq!(ButtonSize::from(ButtonScale::Size3), ButtonSize::Lg);
        assert_eq!(ButtonSize::from(ButtonScale::Size4), ButtonSize::Lg);
    }

    #[test]
    fn builder_color_alias_sets_accent() {
        let btn = Button::new("Test").color(Color32::RED);
        assert_eq!(btn.props.accent_color, Some(Color32::RED));
    }

    #[test]
    fn builder_scale_sets_size() {
        let btn = Button::new("Test").scale(ButtonScale::Size1);
        assert_eq!(btn.props.scale, ButtonScale::Size1);
        assert_eq!(btn.props.size, ButtonSize::Sm);
    }
}
