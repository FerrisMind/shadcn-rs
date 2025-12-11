//! Тема egui-shadcn: связывает токены цвета, радиусов и анимаций с визуалами egui.
use crate::tokens::{
    ColorPalette, ControlSize, ControlVariant, DEFAULT_FOCUS, DEFAULT_MOTION, DEFAULT_RADIUS,
    FocusTokens, InputTokens, InputVariant, MotionTokens, RadiusScale, StateColors, VariantTokens,
    input_tokens, variant_tokens,
};
use egui::style::{WidgetVisuals, Widgets};
use egui::{Color32, CornerRadius, FontId, Stroke, Ui, Vec2};
use log::{info, trace};

#[derive(Clone, Debug)]
pub struct ControlVisuals {
    pub widgets: Widgets,
    pub padding: Vec2,
    pub text_style: FontId,
    pub expansion: f32,
}

#[derive(Clone, Debug)]
pub struct InputVisuals {
    pub widgets: Widgets,
    pub padding: Vec2,
    pub text_style: FontId,
    pub focus_stroke: Stroke,
    pub invalid_stroke: Stroke,
    pub selection_bg: Color32,
    pub selection_fg: Color32,
    pub placeholder: Color32,
    pub text_color: Color32,
    pub rounding: CornerRadius,
    pub expansion: f32,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub palette: ColorPalette,
    pub motion: MotionTokens,
    pub radius: RadiusScale,
    pub focus: FocusTokens,
}

impl Theme {
    pub fn new(palette: ColorPalette) -> Self {
        info!("Initializing egui-shadcn theme");
        Self {
            palette,
            motion: DEFAULT_MOTION,
            radius: DEFAULT_RADIUS,
            focus: DEFAULT_FOCUS,
        }
    }

    pub fn with_tokens(
        palette: ColorPalette,
        motion: MotionTokens,
        radius: RadiusScale,
        focus: FocusTokens,
    ) -> Self {
        info!("Initializing egui-shadcn theme with custom tokens");
        Self {
            palette,
            motion,
            radius,
            focus,
        }
    }

    pub fn control(&self, variant: ControlVariant, size: ControlSize) -> ControlVisuals {
        trace!("Building style for variant {:?} size {:?}", variant, size);
        let tokens = variant_tokens(&self.palette, variant);
        let rounding = size.rounding_with_scale(&self.radius);
        let expansion = size.expansion();
        ControlVisuals {
            widgets: widgets_from_variant(&tokens, rounding, expansion),
            padding: size.padding(),
            text_style: size.font(),
            expansion,
        }
    }

    pub fn input(&self, size: ControlSize) -> InputVisuals {
        self.input_variant(size, InputVariant::Surface)
    }

    pub fn input_variant(&self, size: ControlSize, variant: InputVariant) -> InputVisuals {
        trace!(
            "Building style for input size {:?} variant {:?}",
            size, variant
        );
        let tokens = input_tokens(&self.palette, variant);
        let rounding = size.rounding_with_scale(&self.radius);
        let expansion = size.expansion();
        InputVisuals {
            widgets: widgets_from_input(&tokens, rounding, expansion),
            padding: size.padding(),
            text_style: size.font(),
            focus_stroke: tokens.focused.border,
            invalid_stroke: tokens.invalid.border,
            selection_bg: tokens.selection_bg,
            selection_fg: tokens.selection_fg,
            placeholder: tokens.placeholder,
            text_color: tokens.idle.fg_stroke.color,
            rounding,
            expansion,
        }
    }

    pub fn scoped<R>(&self, ui: &mut Ui, widgets: Widgets, inner: impl FnOnce(&mut Ui) -> R) -> R {
        ui.scope(|scope_ui| {
            let mut style = scope_ui.style().as_ref().clone();
            style.visuals.widgets = widgets;
            scope_ui.set_style(style);
            inner(scope_ui)
        })
        .inner
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new(ColorPalette::default())
    }
}

pub(crate) fn widget_visuals(
    state: &StateColors,
    corner_radius: CornerRadius,
    expansion: f32,
) -> WidgetVisuals {
    WidgetVisuals {
        bg_fill: state.bg_fill,
        weak_bg_fill: state.bg_fill,
        bg_stroke: state.border,
        fg_stroke: state.fg_stroke,
        corner_radius,
        expansion,
    }
}

pub(crate) fn widgets_from_variant(
    tokens: &VariantTokens,
    corner_radius: CornerRadius,
    expansion: f32,
) -> Widgets {
    Widgets {
        noninteractive: widget_visuals(&tokens.disabled, corner_radius, expansion),
        inactive: widget_visuals(&tokens.idle, corner_radius, expansion),
        hovered: widget_visuals(&tokens.hovered, corner_radius, expansion),
        active: widget_visuals(&tokens.active, corner_radius, expansion),
        open: widget_visuals(&tokens.hovered, corner_radius, expansion),
    }
}

pub(crate) fn widgets_from_input(
    tokens: &InputTokens,
    corner_radius: CornerRadius,
    expansion: f32,
) -> Widgets {
    Widgets {
        noninteractive: widget_visuals(&tokens.disabled, corner_radius, expansion),
        inactive: widget_visuals(&tokens.idle, corner_radius, expansion),
        hovered: widget_visuals(&tokens.hovered, corner_radius, expansion),
        active: widget_visuals(&tokens.focused, corner_radius, expansion),
        open: widget_visuals(&tokens.hovered, corner_radius, expansion),
    }
}
