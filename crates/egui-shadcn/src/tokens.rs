//! Токены темы и размеры по мотивам shadcn/ui.

use egui::{Color32, FontId, Rounding, Stroke, Vec2};

/// Варианты управляющих элементов.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlVariant {
    Primary,
    Secondary,
    Ghost,
    Outline,
}

/// Размеры управляющих элементов.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlSize {
    Sm,
    Md,
    Lg,
}

impl ControlSize {
    /// Паддинги по аналогии с shadcn (px).
    pub fn padding(self) -> Vec2 {
        match self {
            ControlSize::Sm => Vec2::new(12.0, 8.0),
            ControlSize::Md => Vec2::new(14.0, 9.0),
            ControlSize::Lg => Vec2::new(16.0, 10.0),
        }
    }

    /// Закругление углов.
    pub fn rounding(self) -> Rounding {
        match self {
            ControlSize::Sm => Rounding::same(6.0),
            ControlSize::Md => Rounding::same(8.0),
            ControlSize::Lg => Rounding::same(10.0),
        }
    }

    /// Толщина расширения фона при нажатии.
    pub fn expansion(self) -> f32 {
        match self {
            ControlSize::Sm => 1.0,
            ControlSize::Md => 1.25,
            ControlSize::Lg => 1.5,
        }
    }

    /// Размер шрифта.
    pub fn font(self) -> FontId {
        FontId::proportional(match self {
            ControlSize::Sm => 13.0,
            ControlSize::Md => 14.0,
            ControlSize::Lg => 15.0,
        })
    }
}

/// Базовая палитра цветов для темы.
#[derive(Clone, Debug, PartialEq)]
pub struct ColorPalette {
    pub background: Color32,
    pub foreground: Color32,
    pub border: Color32,
    pub input: Color32,
    pub primary: Color32,
    pub primary_foreground: Color32,
    pub secondary: Color32,
    pub secondary_foreground: Color32,
    pub accent: Color32,
    pub accent_foreground: Color32,
    pub muted: Color32,
    pub muted_foreground: Color32,
    pub destructive: Color32,
    pub destructive_foreground: Color32,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            background: Color32::from_rgb(250, 250, 250),
            foreground: Color32::from_rgb(15, 23, 42),
            border: Color32::from_rgb(226, 232, 240),
            input: Color32::from_rgb(241, 245, 249),
            primary: Color32::from_rgb(59, 130, 246),
            primary_foreground: Color32::WHITE,
            secondary: Color32::from_rgb(226, 232, 240),
            secondary_foreground: Color32::from_rgb(30, 41, 59),
            accent: Color32::from_rgb(236, 72, 153),
            accent_foreground: Color32::WHITE,
            muted: Color32::from_rgb(248, 250, 252),
            muted_foreground: Color32::from_rgb(100, 116, 139),
            destructive: Color32::from_rgb(239, 68, 68),
            destructive_foreground: Color32::WHITE,
        }
    }
}

/// Цвета для конкретного состояния виджета.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StateColors {
    pub bg_fill: Color32,
    pub fg_stroke: Stroke,
    pub border: Stroke,
}

impl StateColors {
    pub fn new(bg_fill: Color32, fg: Color32, border: Color32) -> Self {
        Self {
            bg_fill,
            fg_stroke: Stroke::new(1.0, fg),
            border: Stroke::new(1.0, border),
        }
    }

    pub fn with_border(bg_fill: Color32, fg: Color32, border: Stroke) -> Self {
        Self {
            bg_fill,
            fg_stroke: Stroke::new(1.0, fg),
            border,
        }
    }
}

/// Описание цветов для варианта контролов.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VariantTokens {
    pub idle: StateColors,
    pub hovered: StateColors,
    pub active: StateColors,
    pub disabled: StateColors,
}

/// Токены для полей ввода.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputTokens {
    pub idle: StateColors,
    pub hovered: StateColors,
    pub focused: StateColors,
    pub disabled: StateColors,
    pub invalid: StateColors,
}

/// Линейная интерполяция двух цветов.
pub fn mix(color: Color32, other: Color32, t: f32) -> Color32 {
    let t_clamped = t.clamp(0.0, 1.0);
    let [r1, g1, b1, a1] = color.to_array();
    let [r2, g2, b2, a2] = other.to_array();
    let lerp = |c1: u8, c2: u8| -> u8 {
        ((c1 as f32 * (1.0 - t_clamped)) + (c2 as f32 * t_clamped)).round() as u8
    };
    Color32::from_rgba_unmultiplied(
        lerp(r1, r2),
        lerp(g1, g2),
        lerp(b1, b2),
        lerp(a1, a2),
    )
}

fn disabled_state(palette: &ColorPalette) -> StateColors {
    StateColors::new(
        mix(palette.muted, palette.background, 0.6),
        mix(palette.muted_foreground, palette.foreground, 0.5),
        mix(palette.border, palette.muted_foreground, 0.5),
    )
}

/// Генерация токенов для кнопок.
pub fn variant_tokens(palette: &ColorPalette, variant: ControlVariant) -> VariantTokens {
    let disabled = disabled_state(palette);
    match variant {
        ControlVariant::Primary => VariantTokens {
            idle: StateColors::new(palette.primary, palette.primary_foreground, palette.primary),
            hovered: StateColors::new(
                mix(palette.primary, Color32::WHITE, 0.08),
                palette.primary_foreground,
                mix(palette.primary, Color32::WHITE, 0.12),
            ),
            active: StateColors::new(
                mix(palette.primary, Color32::from_rgb(30, 58, 138), 0.12),
                palette.primary_foreground,
                mix(palette.primary, Color32::from_rgb(30, 58, 138), 0.2),
            ),
            disabled,
        },
        ControlVariant::Secondary => VariantTokens {
            idle: StateColors::new(palette.secondary, palette.secondary_foreground, palette.border),
            hovered: StateColors::new(
                mix(palette.secondary, Color32::WHITE, 0.08),
                palette.secondary_foreground,
                mix(palette.border, Color32::WHITE, 0.1),
            ),
            active: StateColors::new(
                mix(palette.secondary, palette.border, 0.3),
                palette.secondary_foreground,
                mix(palette.border, palette.foreground, 0.15),
            ),
            disabled,
        },
        ControlVariant::Ghost => VariantTokens {
            idle: StateColors::new(Color32::TRANSPARENT, palette.foreground, Color32::TRANSPARENT),
            hovered: StateColors::new(
                mix(palette.muted, palette.background, 0.5),
                palette.foreground,
                mix(palette.border, palette.foreground, 0.05),
            ),
            active: StateColors::new(
                mix(palette.muted, palette.border, 0.4),
                palette.foreground,
                palette.border,
            ),
            disabled,
        },
        ControlVariant::Outline => VariantTokens {
            idle: StateColors::new(Color32::TRANSPARENT, palette.foreground, palette.border),
            hovered: StateColors::new(
                mix(palette.background, palette.border, 0.25),
                palette.foreground,
                mix(palette.border, palette.foreground, 0.1),
            ),
            active: StateColors::new(
                mix(palette.background, palette.primary, 0.1),
                palette.foreground,
                mix(palette.primary, palette.border, 0.2),
            ),
            disabled,
        },
    }
}

/// Токены для текстовых полей и select.
pub fn input_tokens(palette: &ColorPalette) -> InputTokens {
    let disabled = disabled_state(palette);
    InputTokens {
        idle: StateColors::new(palette.background, palette.foreground, palette.border),
        hovered: StateColors::new(
            mix(palette.background, palette.input, 0.4),
            palette.foreground,
            mix(palette.border, palette.foreground, 0.1),
        ),
        focused: StateColors::with_border(
            mix(palette.background, palette.input, 0.6),
            palette.foreground,
            Stroke::new(1.25, palette.primary),
        ),
        disabled,
        invalid: StateColors::new(
            mix(palette.background, palette.destructive, 0.05),
            palette.foreground,
            palette.destructive,
        ),
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
    fn mix_respects_bounds() {
        init_logger();
        let c1 = Color32::from_rgb(0, 0, 0);
        let c2 = Color32::from_rgb(255, 255, 255);
        let mid = mix(c1, c2, 0.5);
        assert_eq!(mid, Color32::from_rgb(128, 128, 128));
    }

    #[test]
    fn primary_variant_uses_palette() {
        init_logger();
        let palette = ColorPalette::default();
        let variant = variant_tokens(&palette, ControlVariant::Primary);
        assert_eq!(variant.idle.bg_fill, palette.primary);
        assert_eq!(variant.idle.fg_stroke.color, palette.primary_foreground);
    }

    #[test]
    fn control_size_padding_grows() {
        init_logger();
        let sm = ControlSize::Sm.padding();
        let lg = ControlSize::Lg.padding();
        assert!(lg.x > sm.x);
        assert!(lg.y > sm.y);
    }
}

