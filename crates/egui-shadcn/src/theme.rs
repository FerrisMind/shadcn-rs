//! Тема и хелперы применения токенов в egui.

use crate::tokens::{
    input_tokens, variant_tokens, ColorPalette, ControlSize, ControlVariant, InputTokens,
    StateColors, VariantTokens,
};
use egui::style::{WidgetVisuals, Widgets};
use egui::{FontId, Rounding, Stroke, Ui, Vec2};
use log::{info, trace};

/// Описывает визуальный набор для управляющего элемента.
#[derive(Clone, Debug)]
pub struct ControlVisuals {
    pub widgets: Widgets,
    pub padding: Vec2,
    pub text_style: FontId,
    pub expansion: f32,
}

/// Описывает визуальный набор для текстовых полей.
#[derive(Clone, Debug)]
pub struct InputVisuals {
    pub widgets: Widgets,
    pub padding: Vec2,
    pub text_style: FontId,
    pub focus_stroke: Stroke,
    pub invalid_stroke: Stroke,
}

/// Тема, построенная на токенах shadcn.
#[derive(Clone, Debug)]
pub struct Theme {
    pub palette: ColorPalette,
}

impl Theme {
    pub fn new(palette: ColorPalette) -> Self {
        info!("Инициализация темы egui-shadcn");
        Self { palette }
    }

    pub fn control(&self, variant: ControlVariant, size: ControlSize) -> ControlVisuals {
        trace!("Формируем стиль для варианта {:?} размера {:?}", variant, size);
        let tokens = variant_tokens(&self.palette, variant);
        let rounding = size.rounding();
        let expansion = size.expansion();
        ControlVisuals {
            widgets: widgets_from_variant(&tokens, rounding, expansion),
            padding: size.padding(),
            text_style: size.font(),
            expansion,
        }
    }

    pub fn input(&self, size: ControlSize) -> InputVisuals {
        trace!("Формируем стиль для input размера {:?}", size);
        let tokens = input_tokens(&self.palette);
        let rounding = size.rounding();
        let expansion = size.expansion();
        InputVisuals {
            widgets: widgets_from_input(&tokens, rounding, expansion),
            padding: size.padding(),
            text_style: size.font(),
            focus_stroke: tokens.focused.border,
            invalid_stroke: tokens.invalid.border,
        }
    }

    /// Применяет visuals на время выполнения замыкания.
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

fn widget_visuals(state: &StateColors, rounding: Rounding, expansion: f32) -> WidgetVisuals {
    WidgetVisuals {
        bg_fill: state.bg_fill,
        weak_bg_fill: state.bg_fill,
        bg_stroke: state.border,
        fg_stroke: state.fg_stroke,
        rounding,
        expansion,
    }
}

fn widgets_from_variant(tokens: &VariantTokens, rounding: Rounding, expansion: f32) -> Widgets {
    Widgets {
        noninteractive: widget_visuals(&tokens.disabled, rounding, expansion),
        inactive: widget_visuals(&tokens.idle, rounding, expansion),
        hovered: widget_visuals(&tokens.hovered, rounding, expansion),
        active: widget_visuals(&tokens.active, rounding, expansion),
        open: widget_visuals(&tokens.hovered, rounding, expansion),
    }
}

fn widgets_from_input(tokens: &InputTokens, rounding: Rounding, expansion: f32) -> Widgets {
    Widgets {
        noninteractive: widget_visuals(&tokens.disabled, rounding, expansion),
        inactive: widget_visuals(&tokens.idle, rounding, expansion),
        hovered: widget_visuals(&tokens.hovered, rounding, expansion),
        active: widget_visuals(&tokens.focused, rounding, expansion),
        open: widget_visuals(&tokens.hovered, rounding, expansion),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn primary_visuals_use_palette() {
        init_logger();
        let theme = Theme::default();
        let visuals = theme.control(ControlVariant::Primary, ControlSize::Md);
        assert_eq!(
            visuals.widgets.inactive.bg_fill,
            theme.palette.primary,
            "Кнопка primary использует цвет из палитры"
        );
    }

    #[test]
    fn input_focus_stroke_differs() {
        init_logger();
        let theme = Theme::default();
        let visuals = theme.input(ControlSize::Sm);
        assert!(visuals.focus_stroke.width > 1.0);
    }
}

