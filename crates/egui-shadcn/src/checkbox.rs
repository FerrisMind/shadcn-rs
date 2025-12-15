use crate::theme::{Theme, widget_visuals};
use crate::tokens::{
    ColorPalette, ControlSize, ControlVariant, ToggleState, ToggleTokens, VariantTokens,
    checkbox_metrics, checkbox_tokens_with_high_contrast, ease_out_cubic, mix,
};
use egui::style::Widgets;
use egui::{
    Color32, CornerRadius, CursorIcon, Id, Pos2, Response, Sense, Stroke, StrokeKind, TextStyle,
    Ui, Vec2, WidgetText,
};
use log::trace;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxState {
    Unchecked,
    Checked,
    Indeterminate,
}

impl CheckboxState {
    pub fn is_checked(self) -> bool {
        matches!(self, CheckboxState::Checked)
    }

    pub fn is_active(self) -> bool {
        matches!(self, CheckboxState::Checked | CheckboxState::Indeterminate)
    }

    pub fn is_indeterminate(self) -> bool {
        matches!(self, CheckboxState::Indeterminate)
    }

    pub fn toggle(&mut self, cycle: CheckboxCycle) {
        *self = match (cycle, *self) {
            (CheckboxCycle::Binary, CheckboxState::Unchecked) => CheckboxState::Checked,
            (CheckboxCycle::Binary, _) => CheckboxState::Unchecked,
            (CheckboxCycle::TriState, CheckboxState::Unchecked) => CheckboxState::Checked,
            (CheckboxCycle::TriState, CheckboxState::Checked) => CheckboxState::Indeterminate,
            (CheckboxCycle::TriState, CheckboxState::Indeterminate) => CheckboxState::Unchecked,
        };
    }
}

impl From<bool> for CheckboxState {
    fn from(value: bool) -> Self {
        if value {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        }
    }
}

impl From<CheckboxState> for bool {
    fn from(value: CheckboxState) -> Self {
        value.is_checked()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckboxCycle {
    Binary,
    TriState,
}

#[derive(Clone, Copy, Debug)]
pub struct CheckboxOptions {
    pub variant: ControlVariant,
    pub size: ControlSize,
    pub enabled: bool,
    pub invalid: bool,
    pub cycle: CheckboxCycle,
    pub animate: bool,
    pub high_contrast: bool,
    pub color: Option<Color32>,
}

impl Default for CheckboxOptions {
    fn default() -> Self {
        Self {
            variant: ControlVariant::Secondary,
            size: ControlSize::Md,
            enabled: true,
            invalid: false,
            cycle: CheckboxCycle::Binary,
            animate: true,
            high_contrast: false,
            color: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CheckboxVariant {
    #[default]
    Surface,
    Classic,
    Soft,
}

impl CheckboxVariant {
    fn to_control(self) -> ControlVariant {
        match self {
            CheckboxVariant::Surface => ControlVariant::Secondary,
            CheckboxVariant::Classic => ControlVariant::Outline,
            CheckboxVariant::Soft => ControlVariant::Primary,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CheckboxSize {
    Size1,
    #[default]
    Size2,
    Size3,
}

impl From<CheckboxSize> for ControlSize {
    fn from(size: CheckboxSize) -> Self {
        match size {
            CheckboxSize::Size1 => ControlSize::Sm,
            CheckboxSize::Size2 => ControlSize::Md,
            CheckboxSize::Size3 => ControlSize::Lg,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CheckboxProps {
    pub variant: CheckboxVariant,
    pub size: CheckboxSize,
    pub color: Option<Color32>,
    pub high_contrast: bool,
    pub enabled: bool,
    pub invalid: bool,
    pub cycle: CheckboxCycle,
    pub animate: bool,
}

impl Default for CheckboxProps {
    fn default() -> Self {
        Self {
            variant: CheckboxVariant::Surface,
            size: CheckboxSize::Size2,
            color: None,
            high_contrast: false,
            enabled: true,
            invalid: false,
            cycle: CheckboxCycle::Binary,
            animate: true,
        }
    }
}

impl CheckboxProps {
    pub fn with_variant(mut self, variant: CheckboxVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_size(mut self, size: CheckboxSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn with_cycle(mut self, cycle: CheckboxCycle) -> Self {
        self.cycle = cycle;
        self
    }

    pub fn with_animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    fn to_options(&self) -> CheckboxOptions {
        CheckboxOptions {
            variant: self.variant.to_control(),
            size: self.size.into(),
            enabled: self.enabled,
            invalid: self.invalid,
            cycle: self.cycle,
            animate: self.animate,
            high_contrast: self.high_contrast,
            color: self.color,
        }
    }
}

pub fn checkbox(
    ui: &mut Ui,
    theme: &Theme,
    checked: &mut bool,
    label: impl Into<WidgetText>,
    variant: ControlVariant,
    size: ControlSize,
    enabled: bool,
) -> Response {
    trace!(
        "Rendering checkbox variant={:?} size={:?} enabled={}",
        variant, size, enabled
    );
    let mut state = CheckboxState::from(*checked);
    let response = checkbox_state(
        ui,
        theme,
        &mut state,
        label,
        CheckboxOptions {
            variant,
            size,
            enabled,
            ..CheckboxOptions::default()
        },
    );
    *checked = bool::from(state);
    response
}

pub fn checkbox_with_props(
    ui: &mut Ui,
    theme: &Theme,
    state: &mut CheckboxState,
    label: impl Into<WidgetText>,
    props: CheckboxProps,
) -> Response {
    checkbox_state(ui, theme, state, label, props.to_options())
}

pub fn checkbox_state(
    ui: &mut Ui,
    theme: &Theme,
    state: &mut CheckboxState,
    label: impl Into<WidgetText>,
    options: CheckboxOptions,
) -> Response {
    let label_text: WidgetText = label.into();
    trace!(
        "Rendering checkbox state={:?} variant={:?} size={:?} enabled={} invalid={} animate={}",
        state, options.variant, options.size, options.enabled, options.invalid, options.animate
    );

    let CheckboxOptions {
        variant,
        size,
        enabled,
        invalid,
        cycle,
        animate,
        high_contrast,
        color,
    } = options;
    let visuals = theme.control(variant, size);
    let metrics = checkbox_metrics(size);
    let toggle_tokens = checkbox_tokens_with_options(&theme.palette, variant, high_contrast, color);
    let rounding = CornerRadius::same((metrics.track_size.x * 0.25).round() as u8);
    let icon_spacing = visuals.padding.x * 0.35;
    let focus_ring = Stroke::new(
        3.0,
        mix(
            toggle_tokens.on.idle.bg_fill,
            theme.palette.foreground,
            if high_contrast { 0.35 } else { 0.15 },
        ),
    );

    let invalid_ring = Stroke::new(3.0, scale_alpha(theme.palette.destructive, 0.40));
    let expansion = size.expansion();
    let widgets = Widgets {
        noninteractive: widget_visuals(&toggle_tokens.disabled, rounding, expansion),
        inactive: widget_visuals(&toggle_tokens.off.idle, rounding, expansion),
        hovered: widget_visuals(&toggle_tokens.off.hovered, rounding, expansion),
        active: widget_visuals(&toggle_tokens.on.active, rounding, expansion),
        open: widget_visuals(&toggle_tokens.off.hovered, rounding, expansion),
    };

    theme.scoped(ui, widgets, |scoped_ui| {
        let mut style = scoped_ui.style().as_ref().clone();
        {
            let spacing = &mut style.spacing;
            spacing.icon_width = metrics.track_size.x;
            spacing.icon_width_inner = metrics.thumb_size.x;
            spacing.icon_spacing = icon_spacing;
            spacing.item_spacing.x = icon_spacing;
            spacing.item_spacing.y = visuals.padding.y * 0.25;
        }
        style
            .text_styles
            .insert(TextStyle::Body, visuals.text_style.clone());
        scoped_ui.set_style(style);

        scoped_ui
            .horizontal(|row| {
                let sense = if enabled {
                    Sense::click()
                } else {
                    Sense::hover()
                };
                let (icon_rect, icon_response) = row.allocate_exact_size(metrics.track_size, sense);
                let label_response =
                    row.add_enabled(enabled, egui::Label::new(label_text.clone()).wrap());

                let clicked = enabled && (icon_response.clicked() || label_response.clicked());
                if clicked {
                    state.toggle(cycle);
                }

                let anim_id: Id = icon_response.id.with("checkbox");
                let anim_duration = theme.motion.base_ms / 1000.0;
                let animate_value = |id: Id, active: bool| -> f32 {
                    if animate {
                        row.ctx().animate_bool_with_time_and_easing(
                            id,
                            active,
                            anim_duration,
                            ease_out_cubic,
                        )
                    } else if active {
                        1.0
                    } else {
                        0.0
                    }
                };
                let on_t = animate_value(anim_id, state.is_active());
                let indeterminate_t =
                    animate_value(anim_id.with("indeterminate"), state.is_indeterminate());

                let pointer_down = icon_response.is_pointer_button_down_on();
                let hovered_icon = icon_response.hovered();
                let has_focus = icon_response.has_focus();

                let select_states = |pointer_down: bool, hovered: bool| {
                    if !enabled {
                        (toggle_tokens.disabled, toggle_tokens.disabled)
                    } else if pointer_down {
                        (toggle_tokens.off.active, toggle_tokens.on.active)
                    } else if hovered {
                        (toggle_tokens.off.hovered, toggle_tokens.on.hovered)
                    } else {
                        (toggle_tokens.off.idle, toggle_tokens.on.idle)
                    }
                };
                let (off_state, on_state) = select_states(pointer_down, hovered_icon);
                let track_state = lerp_state(off_state, on_state, on_t);

                let max_ring_width = focus_ring.width.max(invalid_ring.width);
                let painter = row.painter_at(icon_rect.expand(max_ring_width + 2.0));
                let track_rect =
                    egui::Rect::from_center_size(icon_rect.center(), metrics.track_size);

                painter.rect_filled(track_rect, rounding, track_state.bg_fill);
                if track_state.border != Stroke::NONE {
                    painter.rect_stroke(
                        track_rect,
                        rounding,
                        track_state.border,
                        StrokeKind::Outside,
                    );
                }

                paint_indicator(
                    &painter,
                    track_rect,
                    track_state.fg_stroke.color,
                    on_t,
                    indeterminate_t,
                    metrics.thumb_size,
                );

                if invalid && enabled {
                    painter.rect_stroke(track_rect, rounding, invalid_ring, StrokeKind::Outside);
                } else if has_focus && enabled {
                    painter.rect_stroke(track_rect, rounding, focus_ring, StrokeKind::Outside);
                }

                let mut response = icon_response | label_response;
                if clicked {
                    response.mark_changed();
                }
                if enabled {
                    response = response.on_hover_cursor(CursorIcon::PointingHand);
                }
                response
            })
            .inner
    })
}

fn lerp_state(
    off: crate::tokens::StateColors,
    on: crate::tokens::StateColors,
    t: f32,
) -> crate::tokens::StateColors {
    crate::tokens::StateColors {
        bg_fill: mix(off.bg_fill, on.bg_fill, t),
        fg_stroke: lerp_stroke(off.fg_stroke, on.fg_stroke, t),
        border: lerp_stroke(off.border, on.border, t),
    }
}

fn lerp_stroke(a: Stroke, b: Stroke, t: f32) -> Stroke {
    Stroke {
        width: a.width + (b.width - a.width) * t,
        color: mix(a.color, b.color, t),
    }
}

fn paint_indicator(
    painter: &egui::Painter,
    track_rect: egui::Rect,
    color: Color32,
    on_t: f32,
    indeterminate_t: f32,
    thumb_size: Vec2,
) {
    let check_weight = thumb_size.x.mul_add(0.22, 1.2).clamp(1.5, 2.4);
    let check_alpha = on_t * (1.0 - indeterminate_t);
    if check_alpha > 0.0 {
        let points = [
            Pos2::new(
                track_rect.left() + track_rect.width() * 0.26,
                track_rect.center().y,
            ),
            Pos2::new(
                track_rect.left() + track_rect.width() * 0.45,
                track_rect.bottom() - track_rect.height() * 0.28,
            ),
            Pos2::new(
                track_rect.right() - track_rect.width() * 0.2,
                track_rect.top() + track_rect.height() * 0.3,
            ),
        ];
        painter.add(egui::Shape::line(
            vec![points[0], points[1], points[2]],
            Stroke::new(check_weight, scale_alpha(color, check_alpha)),
        ));
    }

    let dash_alpha = on_t * indeterminate_t;
    if dash_alpha > 0.0 {
        let dash_height = check_weight;
        let dash_rect = egui::Rect::from_center_size(
            track_rect.center(),
            Vec2::new(track_rect.width() * 0.52, dash_height),
        );
        painter.rect_filled(
            dash_rect,
            CornerRadius::same((dash_height * 0.5) as u8),
            scale_alpha(color, dash_alpha),
        );
    }
}

fn scale_alpha(color: Color32, factor: f32) -> Color32 {
    let clamped = factor.clamp(0.0, 1.0);
    let [r, g, b, a] = color.to_array();
    let alpha = ((a as f32) * clamped).round() as u8;
    Color32::from_rgba_unmultiplied(r, g, b, alpha)
}

fn checkbox_tokens_with_options(
    palette: &ColorPalette,
    variant: ControlVariant,
    high_contrast: bool,
    color_override: Option<Color32>,
) -> ToggleTokens {
    let mut tokens = checkbox_tokens_with_high_contrast(palette, variant, false);

    if let Some(accent) = color_override {
        let accent_tokens = variant_tokens_from_accent(palette, accent);
        tokens.on = ToggleState {
            idle: accent_tokens.idle,
            hovered: accent_tokens.hovered,
            active: accent_tokens.active,
        };
        tokens.thumb_on = accent_tokens.idle.fg_stroke.color;
    }

    if high_contrast {
        tokens = apply_high_contrast(tokens, palette);
    }

    tokens
}

fn variant_tokens_from_accent(palette: &ColorPalette, accent: Color32) -> VariantTokens {
    let fg = if is_light(accent) {
        palette.background
    } else {
        Color32::WHITE
    };
    VariantTokens {
        idle: crate::tokens::StateColors::new(accent, fg, palette.border),
        hovered: crate::tokens::StateColors::new(
            mix(accent, Color32::WHITE, 0.06),
            fg,
            mix(palette.border, Color32::WHITE, 0.08),
        ),
        active: crate::tokens::StateColors::new(
            mix(accent, Color32::WHITE, 0.1),
            fg,
            mix(palette.border, Color32::WHITE, 0.12),
        ),
        disabled: crate::tokens::StateColors::new(
            mix(accent, palette.background, 0.4),
            mix(fg, palette.background, 0.4),
            mix(palette.border, palette.background, 0.4),
        ),
    }
}

fn apply_high_contrast(mut tokens: ToggleTokens, palette: &ColorPalette) -> ToggleTokens {
    tokens.on.idle.bg_fill = mix(tokens.on.idle.bg_fill, Color32::WHITE, 0.2);
    tokens.on.hovered.bg_fill = mix(tokens.on.hovered.bg_fill, Color32::WHITE, 0.25);
    tokens.on.active.bg_fill = mix(tokens.on.active.bg_fill, Color32::WHITE, 0.3);

    tokens.off.idle.bg_fill = mix(tokens.off.idle.bg_fill, Color32::WHITE, 0.25);
    tokens.off.hovered.bg_fill = mix(tokens.off.hovered.bg_fill, Color32::WHITE, 0.3);
    tokens.off.active.bg_fill = mix(tokens.off.active.bg_fill, Color32::WHITE, 0.35);

    tokens.disabled.bg_fill = mix(tokens.disabled.bg_fill, palette.background, 0.35);
    tokens.thumb_on = mix(tokens.thumb_on, palette.background, 0.12);
    tokens.thumb_off = mix(tokens.thumb_off, palette.background, 0.12);

    tokens
}

fn is_light(color: Color32) -> bool {
    let luminance =
        (color.r() as f32 * 0.299 + color.g() as f32 * 0.587 + color.b() as f32 * 0.114) / 255.0;
    luminance > 0.55
}
