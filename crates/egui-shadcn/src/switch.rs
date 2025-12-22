use crate::label::Label;
use crate::theme::Theme;
use crate::tokens::{
    ControlSize, ControlVariant, SwitchSize, SwitchTokenOptions, SwitchVariant, mix,
    switch_metrics, switch_tokens_with_options,
};
use egui::{
    Color32, CornerRadius, CursorIcon, Id, Response, Sense, Stroke, StrokeKind, Ui, Vec2,
    WidgetText,
};
use log::trace;
use std::fmt::{self, Debug};

pub struct OnCheckedChange<'a>(pub Box<dyn FnMut(bool) + 'a>);

impl<'a> Debug for OnCheckedChange<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OnCheckedChange").finish()
    }
}

#[derive(Debug)]
pub struct SwitchProps<'a> {
    pub id_source: Id,
    pub checked: &'a mut bool,
    pub label: WidgetText,
    pub default_checked: Option<bool>,
    pub on_checked_change: Option<OnCheckedChange<'a>>,
    pub disabled: bool,
    pub required: bool,
    pub name: Option<String>,
    pub value: Option<String>,
    pub as_child: bool,
    pub thumb_as_child: bool,
    pub size: SwitchSize,
    pub style: SwitchVariant,
    pub high_contrast: bool,
    pub animate: bool,
    pub accent: Option<Color32>,
    pub corner_radius: Option<CornerRadius>,
    pub thumb_color: Option<Color32>,
}

impl<'a> SwitchProps<'a> {
    pub fn new(id_source: Id, checked: &'a mut bool, label: impl Into<WidgetText>) -> Self {
        Self {
            id_source,
            checked,
            label: label.into(),
            default_checked: None,
            on_checked_change: None,
            disabled: false,
            required: false,
            name: None,
            value: Some("on".to_string()),
            as_child: false,
            thumb_as_child: false,
            size: SwitchSize::Two,
            style: SwitchVariant::Surface,
            high_contrast: false,
            animate: true,
            accent: None,
            corner_radius: None,
            thumb_color: None,
        }
    }

    pub fn with_default_checked(mut self, default: bool) -> Self {
        self.default_checked = Some(default);
        self
    }

    pub fn with_on_checked_change(mut self, callback: impl FnMut(bool) + 'a) -> Self {
        self.on_checked_change = Some(OnCheckedChange(Box::new(callback)));
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn as_child(mut self, as_child: bool) -> Self {
        self.as_child = as_child;
        self
    }

    pub fn thumb_as_child(mut self, as_child: bool) -> Self {
        self.thumb_as_child = as_child;
        self
    }

    pub fn with_size(mut self, size: SwitchSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_style(mut self, style: SwitchVariant) -> Self {
        self.style = style;
        self
    }

    pub fn with_high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn with_animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    pub fn with_accent(mut self, accent: Color32) -> Self {
        self.accent = Some(accent);
        self
    }

    pub fn with_accent_opt(mut self, accent: Option<Color32>) -> Self {
        self.accent = accent;
        self
    }

    pub fn with_corner_radius(mut self, radius: CornerRadius) -> Self {
        self.corner_radius = Some(radius);
        self
    }

    pub fn with_corner_radius_opt(mut self, radius: Option<CornerRadius>) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn with_thumb_color(mut self, color: Color32) -> Self {
        self.thumb_color = Some(color);
        self
    }

    pub fn with_thumb_color_opt(mut self, color: Option<Color32>) -> Self {
        self.thumb_color = color;
        self
    }

    pub fn with_default_checked_opt(mut self, default: Option<bool>) -> Self {
        self.default_checked = default;
        self
    }

    pub fn with_on_checked_change_opt(mut self, callback: Option<OnCheckedChange<'a>>) -> Self {
        self.on_checked_change = callback;
        self
    }

    pub fn with_name_opt(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }

    pub fn with_value_opt(mut self, value: Option<String>) -> Self {
        self.value = value;
        self
    }
}

#[derive(Debug)]
pub struct SwitchOptions<'a> {
    pub size: SwitchSize,
    pub style: SwitchVariant,
    pub enabled: bool,
    pub high_contrast: bool,
    pub animate: bool,
    pub accent: Option<Color32>,
    pub corner_radius: Option<CornerRadius>,
    pub thumb_color: Option<Color32>,

    pub as_child: bool,
    pub required: bool,
    pub name: Option<String>,
    pub value: Option<String>,
    pub on_checked_change: Option<OnCheckedChange<'a>>,
    pub default_checked: Option<bool>,
    pub thumb_as_child: bool,
}

impl<'a> Default for SwitchOptions<'a> {
    fn default() -> Self {
        Self {
            size: SwitchSize::Two,
            style: SwitchVariant::Surface,
            enabled: true,
            high_contrast: false,
            animate: true,
            accent: None,
            corner_radius: None,
            thumb_color: None,
            as_child: false,
            required: false,
            name: None,
            value: Some("on".to_string()),
            on_checked_change: None,
            default_checked: None,
            thumb_as_child: false,
        }
    }
}

pub fn switch(
    ui: &mut Ui,
    theme: &Theme,
    on: &mut bool,
    label: impl Into<WidgetText>,
    variant: ControlVariant,
    size: ControlSize,
    enabled: bool,
) -> Response {
    let accent = accent_from_control_variant(&theme.palette, variant);
    switch_with_options(
        ui,
        theme,
        on,
        label,
        SwitchOptions {
            size: SwitchSize::from(size),
            enabled,
            accent: Some(accent),
            ..SwitchOptions::default()
        },
    )
}

pub fn switch_with_props<'a>(ui: &mut Ui, theme: &Theme, mut props: SwitchProps<'a>) -> Response {
    apply_default_checked(ui, &mut props);

    let label_text: WidgetText = props.label.clone();
    let label_size: ControlSize = props.size.into();
    trace!(
        "Rendering switch style={:?} size={:?} enabled={} high_contrast={} animate={}",
        props.style, props.size, props.disabled, props.high_contrast, props.animate
    );

    let metrics = switch_metrics(props.size);
    let rounding = props
        .corner_radius
        .unwrap_or_else(|| CornerRadius::same((metrics.track_size.y * 0.5).round() as u8));
    let focus_size: ControlSize = props.size.into();
    let _visuals = theme.input(focus_size);
    let tokens = switch_tokens_with_options(
        &theme.palette,
        SwitchTokenOptions {
            variant: props.style,
            high_contrast: props.high_contrast,
            accent: props.accent.unwrap_or(theme.palette.primary),
            thumb_color: props.thumb_color,
        },
    );
    let desired_size =
        Vec2::new(metrics.track_size.x, metrics.track_size.y).max(ui.spacing().interact_size);
    let (hit_rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
    let track_rect = egui::Rect::from_center_size(hit_rect.center(), metrics.track_size);

    let anim_id: Id = response.id;
    let t = if props.animate {
        ui.ctx().animate_bool(anim_id, *props.checked)
    } else if *props.checked {
        1.0
    } else {
        0.0
    };

    let mut off_state = tokens.toggle.off.idle;
    let mut on_state = tokens.toggle.on.idle;
    if props.disabled {
        off_state = tokens.toggle.disabled;
        on_state = tokens.toggle.disabled;
    } else if response.is_pointer_button_down_on() {
        off_state = tokens.toggle.off.active;
        on_state = tokens.toggle.on.active;
    } else if response.hovered() {
        off_state = tokens.toggle.off.hovered;
        on_state = tokens.toggle.on.hovered;
    }

    if response.clicked() && !props.disabled {
        *props.checked = !*props.checked;
        if let Some(cb) = props.on_checked_change.as_mut() {
            (cb.0)(*props.checked);
        }
        response.mark_changed();
    }

    let track_state = lerp_state(&off_state, &on_state, t);
    let painter = ui.painter();
    painter.rect_filled(track_rect, rounding, track_state.bg_fill);
    if track_state.border != Stroke::NONE {
        painter.rect_stroke(
            track_rect,
            rounding,
            track_state.border,
            StrokeKind::Outside,
        );
    }

    let padding = (metrics.track_size.y - metrics.thumb_size.y) * 0.5;
    let off_x = track_rect.left() + padding;
    let on_x = track_rect.right() - padding - metrics.thumb_size.x;
    let thumb_x = off_x + (on_x - off_x) * t;
    let thumb_rect = egui::Rect::from_min_size(
        egui::pos2(thumb_x, track_rect.top() + padding),
        metrics.thumb_size,
    );
    let thumb_rounding = props
        .corner_radius
        .unwrap_or_else(|| CornerRadius::same((metrics.thumb_size.y * 0.5) as u8));
    let thumb_color = if props.disabled {
        tokens.toggle.thumb_disabled
    } else {
        mix(tokens.toggle.thumb_off, tokens.toggle.thumb_on, t)
    };
    painter.rect_filled(thumb_rect, thumb_rounding, thumb_color);

    if response.has_focus() && !props.disabled {
        painter.rect_stroke(track_rect, rounding, tokens.focus_ring, StrokeKind::Outside);
    }

    let label_resp = {
        let mut builder = Label::new(label_text.clone()).size(label_size);
        if props.disabled {
            builder = builder.disabled(true);
        }
        builder.show(ui, theme)
    };
    if label_resp.clicked() && !props.disabled {
        *props.checked = !*props.checked;
        if let Some(cb) = props.on_checked_change.as_mut() {
            (cb.0)(*props.checked);
        }
        response.mark_changed();
    }

    let mut combined = response | label_resp;
    if !props.disabled {
        combined = combined.on_hover_cursor(CursorIcon::PointingHand);
    }
    combined
}

fn apply_default_checked(ui: &Ui, props: &mut SwitchProps<'_>) {
    if let Some(default) = props.default_checked {
        let default_id = props.id_source.with("default_checked_applied");
        let already = ui
            .ctx()
            .memory_mut(|mem| mem.data.get_persisted::<bool>(default_id).unwrap_or(false));
        if !already {
            *props.checked = default;
            if let Some(cb) = props.on_checked_change.as_mut() {
                (cb.0)(*props.checked);
            }
            ui.ctx()
                .memory_mut(|mem| mem.data.insert_persisted(default_id, true));
        }
    }
}

fn widget_text_key(text: &WidgetText) -> String {
    match text {
        WidgetText::RichText(rt) => rt.text().to_string(),
        WidgetText::Text(t) => t.to_string(),
        WidgetText::Galley(g) => format!("galley:{}", g.rows.len()),
        WidgetText::LayoutJob(_) => "layout_job".to_string(),
    }
}

pub fn switch_with_options<'a>(
    ui: &mut Ui,
    theme: &Theme,
    on: &mut bool,
    label: impl Into<WidgetText>,
    options: SwitchOptions<'a>,
) -> Response {
    let label_widget: WidgetText = label.into();
    let id = ui.make_persistent_id(("switch", widget_text_key(&label_widget)));
    let props = SwitchProps::new(id, on, label_widget)
        .with_size(options.size)
        .with_style(options.style)
        .with_high_contrast(options.high_contrast)
        .with_animate(options.animate)
        .with_accent_opt(options.accent)
        .with_corner_radius_opt(options.corner_radius)
        .with_thumb_color_opt(options.thumb_color)
        .with_disabled(!options.enabled)
        .with_required(options.required)
        .with_name_opt(options.name)
        .with_value_opt(options.value)
        .as_child(options.as_child)
        .thumb_as_child(options.thumb_as_child)
        .with_default_checked_opt(options.default_checked)
        .with_on_checked_change_opt(options.on_checked_change);
    switch_with_props(ui, theme, props)
}

fn lerp_stroke(a: Stroke, b: Stroke, t: f32) -> Stroke {
    Stroke {
        width: a.width + (b.width - a.width) * t,
        color: mix(a.color, b.color, t),
    }
}

fn lerp_state(
    off: &crate::tokens::StateColors,
    on: &crate::tokens::StateColors,
    t: f32,
) -> crate::tokens::StateColors {
    crate::tokens::StateColors {
        bg_fill: mix(off.bg_fill, on.bg_fill, t),
        fg_stroke: lerp_stroke(off.fg_stroke, on.fg_stroke, t),
        border: lerp_stroke(off.border, on.border, t),
    }
}

fn accent_from_control_variant(
    palette: &crate::tokens::ColorPalette,
    variant: ControlVariant,
) -> Color32 {
    match variant {
        ControlVariant::Primary | ControlVariant::Link => palette.primary,
        ControlVariant::Destructive => palette.destructive,
        ControlVariant::Secondary => crate::tokens::mix(palette.border, palette.foreground, 0.5),
        ControlVariant::Ghost => palette.muted_foreground,
        ControlVariant::Outline => {
            crate::tokens::mix(palette.border, palette.muted_foreground, 0.4)
        }
    }
}
