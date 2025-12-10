use crate::theme::Theme;
use crate::tokens::{ColorPalette, ControlSize, ControlVariant, mix};
use egui::{
    Color32, CornerRadius, FontId, Painter, Pos2, Response, Sense, Stroke, StrokeKind, Ui, Vec2,
    WidgetText, pos2, vec2,
};
use log::trace;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,

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

    pub border: Color32,

    pub border_hover: Color32,

    pub focus_ring: Color32,

    pub disabled_opacity: f32,

    pub rounding: CornerRadius,
}

impl ButtonStyle {
    pub fn from_variant(palette: &ColorPalette, variant: ButtonVariant) -> Self {
        match variant {
            ButtonVariant::Default => Self {
                bg: palette.primary,

                bg_hover: Color32::from_rgb(207, 207, 207),
                bg_active: Color32::from_rgb(229, 229, 229),
                text: palette.primary_foreground,
                border: Color32::TRANSPARENT,
                border_hover: Color32::TRANSPARENT,
                focus_ring: mix(palette.primary, Color32::from_rgb(150, 150, 150), 0.3),
                disabled_opacity: 0.5,
                rounding: CornerRadius::same(8),
            },
            ButtonVariant::Destructive => Self {
                bg: palette.destructive,
                bg_hover: mix(palette.destructive, Color32::WHITE, 0.1),
                bg_active: mix(palette.destructive, Color32::WHITE, 0.15),
                text: Color32::WHITE,
                border: Color32::TRANSPARENT,
                border_hover: Color32::TRANSPARENT,
                focus_ring: mix(palette.destructive, Color32::WHITE, 0.3),
                disabled_opacity: 0.5,
                rounding: CornerRadius::same(8),
            },
            ButtonVariant::Outline => Self {
                bg: Color32::TRANSPARENT,
                bg_hover: palette.accent,
                bg_active: mix(palette.accent, Color32::WHITE, 0.1),
                text: palette.foreground,
                border: palette.border,
                border_hover: mix(palette.border, palette.foreground, 0.1),
                focus_ring: Color32::from_rgba_unmultiplied(
                    palette.border.r(),
                    palette.border.g(),
                    palette.border.b(),
                    128,
                ),
                disabled_opacity: 0.5,
                rounding: CornerRadius::same(8),
            },
            ButtonVariant::Secondary => Self {
                bg: palette.secondary,
                bg_hover: mix(palette.secondary, Color32::WHITE, 0.08),
                bg_active: mix(palette.secondary, Color32::WHITE, 0.12),
                text: palette.secondary_foreground,
                border: Color32::TRANSPARENT,
                border_hover: Color32::TRANSPARENT,
                focus_ring: Color32::from_rgba_unmultiplied(
                    palette.secondary.r(),
                    palette.secondary.g(),
                    palette.secondary.b(),
                    128,
                ),
                disabled_opacity: 0.5,
                rounding: CornerRadius::same(8),
            },
            ButtonVariant::Ghost => Self {
                bg: Color32::TRANSPARENT,
                bg_hover: palette.accent,
                bg_active: mix(palette.accent, Color32::WHITE, 0.1),
                text: palette.foreground,
                border: Color32::TRANSPARENT,
                border_hover: Color32::TRANSPARENT,
                focus_ring: Color32::from_rgba_unmultiplied(
                    palette.border.r(),
                    palette.border.g(),
                    palette.border.b(),
                    128,
                ),
                disabled_opacity: 0.5,
                rounding: CornerRadius::same(8),
            },
            ButtonVariant::Link => Self {
                bg: Color32::TRANSPARENT,
                bg_hover: Color32::TRANSPARENT,
                bg_active: Color32::TRANSPARENT,
                text: palette.primary,
                border: Color32::TRANSPARENT,
                border_hover: Color32::TRANSPARENT,
                focus_ring: Color32::from_rgba_unmultiplied(
                    palette.primary.r(),
                    palette.primary.g(),
                    palette.primary.b(),
                    128,
                ),
                disabled_opacity: 0.5,
                rounding: CornerRadius::same(8),
            },
        }
    }

    pub fn with_high_contrast(mut self) -> Self {
        self.bg = mix(self.bg, Color32::WHITE, 0.15);
        self.bg_hover = mix(self.bg_hover, Color32::WHITE, 0.15);
        self.text = Color32::WHITE;
        self
    }
}

#[derive(Clone)]
pub struct ButtonProps<'a> {
    pub label: WidgetText,

    pub variant: ButtonVariant,

    pub size: ButtonSize,

    pub enabled: bool,

    pub loading: bool,

    pub high_contrast: bool,

    pub style: Option<ButtonStyle>,

    #[allow(clippy::type_complexity)]
    pub icon: Option<&'a dyn Fn(&Painter, Pos2, f32, Color32)>,
}

impl<'a> std::fmt::Debug for ButtonProps<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ButtonProps")
            .field("label", &self.label)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("enabled", &self.enabled)
            .field("loading", &self.loading)
            .field("high_contrast", &self.high_contrast)
            .field("style", &self.style)
            .field("icon", &self.icon.as_ref().map(|_| "<fn>"))
            .finish()
    }
}

impl<'a> ButtonProps<'a> {
    pub fn new(label: impl Into<WidgetText>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            enabled: true,
            loading: false,
            high_contrast: false,
            style: None,
            icon: None,
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

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn icon(mut self, icon: &'a dyn Fn(&Painter, Pos2, f32, Color32)) -> Self {
        self.icon = Some(icon);
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

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.props.style = Some(style);
        self
    }

    pub fn icon(mut self, icon: &'a dyn Fn(&Painter, Pos2, f32, Color32)) -> Self {
        self.props.icon = Some(icon);
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

    let mut style = props
        .style
        .clone()
        .unwrap_or_else(|| ButtonStyle::from_variant(&theme.palette, props.variant));

    style.rounding = props.size.rounding();

    if props.high_contrast {
        style = style.with_high_contrast();
    }

    let effectively_disabled = !props.enabled || props.loading;

    let height = props.size.height();

    let width = if props.size.is_icon() {
        props.size.icon_width()
    } else {
        let label_text = props.label.text().to_string();
        let approx_char_width = props.size.font_size() * 0.6;
        let text_width = approx_char_width * label_text.chars().count() as f32;

        let icon_width = if props.icon.is_some() || props.loading {
            props.size.icon_size() + props.size.gap()
        } else {
            0.0
        };

        text_width + icon_width + props.size.padding_x() * 2.0
    };

    let desired_size = vec2(width.max(40.0), height);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    let painter = ui.painter();

    let is_hovered = response.hovered() && !effectively_disabled;
    let is_pressed = response.is_pointer_button_down_on() && !effectively_disabled;
    let has_focus = response.has_focus() && !effectively_disabled;

    let bg_color = if effectively_disabled {
        Color32::from_rgba_unmultiplied(
            style.bg.r(),
            style.bg.g(),
            style.bg.b(),
            (style.bg.a() as f32 * style.disabled_opacity) as u8,
        )
    } else if is_pressed {
        style.bg_active
    } else if is_hovered {
        style.bg_hover
    } else {
        style.bg
    };

    let text_color = if effectively_disabled {
        Color32::from_rgba_unmultiplied(
            style.text.r(),
            style.text.g(),
            style.text.b(),
            (style.text.a() as f32 * style.disabled_opacity) as u8,
        )
    } else {
        style.text
    };

    let border_color = if is_hovered && !effectively_disabled {
        style.border_hover
    } else {
        style.border
    };

    painter.rect_filled(rect, style.rounding, bg_color);

    if border_color != Color32::TRANSPARENT {
        painter.rect_stroke(
            rect,
            style.rounding,
            Stroke::new(1.0, border_color),
            StrokeKind::Inside,
        );
    }

    if has_focus {
        let ring_rect = rect.expand(2.0);
        painter.rect_stroke(
            ring_rect,
            style.rounding,
            Stroke::new(3.0, style.focus_ring),
            StrokeKind::Outside,
        );
    }

    let center = rect.center();

    if props.size.is_icon() {
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
    } else {
        let icon_size = props.size.icon_size();
        let gap = props.size.gap();

        let label_text = props.label.text().to_string();
        let text_galley = painter.layout_no_wrap(label_text, props.size.font(), text_color);
        let text_width = text_galley.rect.width();

        if props.loading {
            let total_width = icon_size + gap + text_width;
            let start_x = center.x - total_width / 2.0;

            let spinner_center = pos2(start_x + icon_size / 2.0, center.y);
            let t = ui.ctx().input(|i| i.time) as f32;
            draw_spinner(painter, spinner_center, icon_size, text_color, t * 2.0);
            ui.ctx().request_repaint();

            let text_pos = pos2(
                start_x + icon_size + gap,
                center.y - text_galley.rect.height() / 2.0,
            );
            painter.galley(text_pos, text_galley, text_color);
        } else if let Some(icon_fn) = props.icon {
            let total_width = icon_size + gap + text_width;
            let start_x = center.x - total_width / 2.0;

            let icon_center = pos2(start_x + icon_size / 2.0, center.y);
            icon_fn(painter, icon_center, icon_size, text_color);

            let text_pos = pos2(
                start_x + icon_size + gap,
                center.y - text_galley.rect.height() / 2.0,
            );
            painter.galley(text_pos, text_galley, text_color);
        } else {
            let text_pos = pos2(
                center.x - text_width / 2.0,
                center.y - text_galley.rect.height() / 2.0,
            );
            painter.galley(text_pos, text_galley, text_color);
        }
    }

    if props.variant == ButtonVariant::Link && is_hovered {
        let label_text = props.label.text().to_string();
        let text_galley = painter.layout_no_wrap(label_text, props.size.font(), text_color);
        let text_width = text_galley.rect.width();
        let text_bottom = center.y + text_galley.rect.height() / 2.0 - 2.0;
        let underline_y = text_bottom + 2.0;

        painter.line_segment(
            [
                pos2(center.x - text_width / 2.0, underline_y),
                pos2(center.x + text_width / 2.0, underline_y),
            ],
            Stroke::new(1.0, text_color),
        );
    }

    response
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
