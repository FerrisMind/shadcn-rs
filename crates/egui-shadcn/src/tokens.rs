use egui::{Color32, CornerRadius, FontId, Stroke, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlVariant {
    Primary,
    Secondary,
    Ghost,
    Outline,
    Destructive,
    Link,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlSize {
    Sm,
    Md,
    Lg,
    IconSm,
    Icon,
    IconLg,
}

impl ControlSize {
    pub fn padding(self) -> Vec2 {
        match self {
            ControlSize::Sm => Vec2::new(12.0, 8.0),
            ControlSize::Md => Vec2::new(14.0, 9.0),
            ControlSize::Lg => Vec2::new(16.0, 10.0),
            ControlSize::IconSm => Vec2::new(8.0, 8.0),
            ControlSize::Icon => Vec2::new(9.0, 9.0),
            ControlSize::IconLg => Vec2::new(10.0, 10.0),
        }
    }

    pub fn rounding(self) -> CornerRadius {
        match self {
            ControlSize::Sm => CornerRadius::same(6),
            ControlSize::Md => CornerRadius::same(8),
            ControlSize::Lg => CornerRadius::same(10),
            ControlSize::IconSm => CornerRadius::same(8),
            ControlSize::Icon => CornerRadius::same(9),
            ControlSize::IconLg => CornerRadius::same(10),
        }
    }

    pub fn expansion(self) -> f32 {
        match self {
            ControlSize::Sm => 1.0,
            ControlSize::Md => 1.25,
            ControlSize::Lg => 1.5,
            ControlSize::IconSm => 1.0,
            ControlSize::Icon => 1.0,
            ControlSize::IconLg => 1.0,
        }
    }

    pub fn font(self) -> FontId {
        FontId::proportional(match self {
            ControlSize::Sm => 13.0,
            ControlSize::Md => 14.0,
            ControlSize::Lg => 15.0,
            ControlSize::IconSm => 13.0,
            ControlSize::Icon => 14.0,
            ControlSize::IconLg => 15.0,
        })
    }
}

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
            background: Color32::from_rgb(15, 15, 15),
            foreground: Color32::from_rgb(249, 249, 249),
            border: Color32::from_rgb(42, 42, 42),
            input: Color32::from_rgb(29, 29, 29),
            primary: Color32::from_rgb(250, 250, 250),
            primary_foreground: Color32::from_rgb(15, 15, 15),
            secondary: Color32::from_rgb(42, 42, 42),
            secondary_foreground: Color32::from_rgb(249, 249, 249),
            accent: Color32::from_rgb(42, 42, 42),
            accent_foreground: Color32::from_rgb(249, 249, 249),
            muted: Color32::from_rgb(42, 42, 42),
            muted_foreground: Color32::from_rgb(143, 143, 143),
            destructive: Color32::from_rgb(242, 95, 92),
            destructive_foreground: Color32::from_rgb(15, 15, 15),
        }
    }
}

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VariantTokens {
    pub idle: StateColors,
    pub hovered: StateColors,
    pub active: StateColors,
    pub disabled: StateColors,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputTokens {
    pub idle: StateColors,
    pub hovered: StateColors,
    pub focused: StateColors,
    pub disabled: StateColors,
    pub invalid: StateColors,

    pub selection_bg: Color32,

    pub selection_fg: Color32,

    pub placeholder: Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToggleState {
    pub idle: StateColors,
    pub hovered: StateColors,
    pub active: StateColors,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToggleTokens {
    pub off: ToggleState,
    pub on: ToggleState,
    pub disabled: StateColors,
    pub thumb_on: Color32,
    pub thumb_off: Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToggleMetrics {
    pub track_size: Vec2,
    pub thumb_size: Vec2,
}

pub fn mix(color: Color32, other: Color32, t: f32) -> Color32 {
    let t_clamped = t.clamp(0.0, 1.0);
    let [r1, g1, b1, a1] = color.to_array();
    let [r2, g2, b2, a2] = other.to_array();
    let lerp = |c1: u8, c2: u8| -> u8 {
        ((c1 as f32 * (1.0 - t_clamped)) + (c2 as f32 * t_clamped)).round() as u8
    };
    Color32::from_rgba_unmultiplied(lerp(r1, r2), lerp(g1, g2), lerp(b1, b2), lerp(a1, a2))
}

fn disabled_state(palette: &ColorPalette) -> StateColors {
    StateColors::new(
        mix(palette.muted, palette.background, 0.6),
        mix(palette.muted_foreground, palette.foreground, 0.5),
        mix(palette.border, palette.muted_foreground, 0.5),
    )
}

pub fn variant_tokens(palette: &ColorPalette, variant: ControlVariant) -> VariantTokens {
    let disabled = disabled_state(palette);
    match variant {
        ControlVariant::Primary => VariantTokens {
            idle: StateColors::new(palette.primary, palette.primary_foreground, palette.border),
            hovered: StateColors::new(
                mix(palette.primary, Color32::WHITE, 0.06),
                palette.primary_foreground,
                mix(palette.border, Color32::WHITE, 0.08),
            ),
            active: StateColors::new(
                mix(palette.primary, Color32::WHITE, 0.1),
                palette.primary_foreground,
                mix(palette.border, Color32::WHITE, 0.12),
            ),
            disabled,
        },
        ControlVariant::Destructive => VariantTokens {
            idle: StateColors::new(
                palette.destructive,
                palette.destructive_foreground,
                palette.border,
            ),
            hovered: StateColors::new(
                mix(palette.destructive, Color32::WHITE, 0.08),
                palette.destructive_foreground,
                mix(palette.border, Color32::WHITE, 0.12),
            ),
            active: StateColors::new(
                mix(palette.destructive, Color32::from_rgb(127, 29, 29), 0.12),
                palette.destructive_foreground,
                mix(palette.border, Color32::from_rgb(127, 29, 29), 0.2),
            ),
            disabled,
        },
        ControlVariant::Secondary => VariantTokens {
            idle: StateColors::new(
                palette.secondary,
                palette.secondary_foreground,
                palette.border,
            ),
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
        ControlVariant::Link => VariantTokens {
            idle: StateColors::new(Color32::TRANSPARENT, palette.primary, palette.border),
            hovered: StateColors::new(
                mix(palette.muted, palette.background, 0.5),
                palette.primary,
                palette.border,
            ),
            active: StateColors::new(
                mix(palette.muted, palette.primary, 0.4),
                palette.primary,
                palette.border,
            ),
            disabled,
        },
        ControlVariant::Ghost => VariantTokens {
            idle: StateColors::new(Color32::TRANSPARENT, palette.foreground, palette.border),
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

pub fn input_tokens(palette: &ColorPalette) -> InputTokens {
    let disabled = disabled_state(palette);
    let selection_bg = mix(palette.primary, Color32::WHITE, 0.12);
    InputTokens {
        idle: StateColors::new(palette.primary, palette.foreground, palette.border),
        hovered: StateColors::new(
            mix(palette.primary, Color32::WHITE, 0.06),
            palette.foreground,
            palette.border,
        ),
        focused: StateColors::with_border(
            mix(palette.primary, Color32::WHITE, 0.1),
            palette.foreground,
            Stroke::new(2.0, palette.border),
        ),
        disabled,
        invalid: StateColors::new(
            mix(palette.primary, palette.destructive, 0.12),
            palette.foreground,
            palette.destructive,
        ),
        selection_bg,
        selection_fg: palette.foreground,
        placeholder: mix(palette.foreground, palette.muted_foreground, 0.65),
    }
}

pub fn checkbox_tokens(palette: &ColorPalette, variant: ControlVariant) -> ToggleTokens {
    let input = input_tokens(palette);
    let on = variant_tokens(palette, variant);
    let disabled = disabled_state(palette);
    ToggleTokens {
        off: ToggleState {
            idle: input.idle,
            hovered: input.hovered,
            active: input.focused,
        },
        on: ToggleState {
            idle: on.idle,
            hovered: on.hovered,
            active: on.active,
        },
        disabled,
        thumb_on: on.idle.fg_stroke.color,
        thumb_off: input.idle.fg_stroke.color,
    }
}

pub fn switch_tokens(palette: &ColorPalette, _variant: ControlVariant) -> ToggleTokens {
    let off_bg = Color32::from_rgb(0x28, 0x28, 0x28);
    let on_bg = Color32::from_rgb(0xE5, 0xE5, 0xE5);
    let thumb_off = Color32::from_rgb(0xFA, 0xFA, 0xFA);
    let thumb_on = Color32::from_rgb(0x17, 0x17, 0x17);

    let off = InputTokens {
        idle: StateColors::new(off_bg, palette.foreground, Color32::TRANSPARENT),
        hovered: StateColors::new(
            mix(off_bg, Color32::WHITE, 0.06),
            palette.foreground,
            Color32::TRANSPARENT,
        ),
        focused: StateColors::new(
            mix(off_bg, Color32::WHITE, 0.1),
            palette.foreground,
            Color32::TRANSPARENT,
        ),
        disabled: disabled_state(palette),
        invalid: StateColors::new(
            mix(off_bg, palette.destructive, 0.1),
            palette.destructive_foreground,
            Color32::TRANSPARENT,
        ),
        selection_bg: palette.primary,
        selection_fg: palette.primary_foreground,
        placeholder: palette.muted_foreground,
    };
    let on = VariantTokens {
        idle: StateColors::new(on_bg, palette.foreground, Color32::TRANSPARENT),
        hovered: StateColors::new(
            mix(on_bg, palette.foreground, 0.08),
            palette.foreground,
            Color32::TRANSPARENT,
        ),
        active: StateColors::new(
            mix(on_bg, palette.foreground, 0.14),
            palette.foreground,
            Color32::TRANSPARENT,
        ),
        disabled: disabled_state(palette),
    };
    let disabled = disabled_state(palette);
    ToggleTokens {
        off: ToggleState {
            idle: off.idle,
            hovered: off.hovered,
            active: off.focused,
        },
        on: ToggleState {
            idle: on.idle,
            hovered: on.hovered,
            active: on.active,
        },
        disabled,
        thumb_on,
        thumb_off,
    }
}

pub fn checkbox_metrics(size: ControlSize) -> ToggleMetrics {
    let track = match size {
        ControlSize::Sm | ControlSize::Md | ControlSize::IconSm => 16.0,
        ControlSize::Lg | ControlSize::Icon | ControlSize::IconLg => 18.0,
    };
    ToggleMetrics {
        track_size: Vec2::splat(track),
        thumb_size: Vec2::splat(track * 0.6),
    }
}

pub fn toggle_metrics(size: ControlSize) -> ToggleMetrics {
    let (track_w, track_h): (f32, f32) = match size {
        ControlSize::Sm => (38.0, 20.0),
        ControlSize::Md | ControlSize::IconSm => (44.0, 24.0),
        ControlSize::Lg | ControlSize::Icon => (50.0, 28.0),
        ControlSize::IconLg => (54.0, 30.0),
    };
    let thumb = (track_h - 6.0_f32).max(12.0_f32);
    ToggleMetrics {
        track_size: Vec2::new(track_w, track_h),
        thumb_size: Vec2::splat(thumb),
    }
}

pub fn switch_metrics(_size: ControlSize) -> ToggleMetrics {
    let (track_w, track_h): (f32, f32) = (32.0, 18.4);
    let thumb = 16.0_f32;
    ToggleMetrics {
        track_size: Vec2::new(track_w, track_h),
        thumb_size: Vec2::splat(thumb),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToggleVariant {
    Default,
    Outline,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToggleButtonTokens {
    pub off: VariantTokens,
    pub on: VariantTokens,
}

fn accent_tokens(palette: &ColorPalette) -> VariantTokens {
    let disabled = disabled_state(palette);
    VariantTokens {
        idle: StateColors::new(palette.accent, palette.accent_foreground, palette.accent),
        hovered: StateColors::new(
            mix(palette.accent, Color32::WHITE, 0.08),
            palette.accent_foreground,
            mix(palette.accent, Color32::WHITE, 0.1),
        ),
        active: StateColors::new(
            mix(palette.accent, Color32::from_rgb(136, 19, 55), 0.15),
            palette.accent_foreground,
            mix(palette.accent, Color32::from_rgb(136, 19, 55), 0.2),
        ),
        disabled,
    }
}

pub fn toggle_button_tokens(palette: &ColorPalette, variant: ToggleVariant) -> ToggleButtonTokens {
    let disabled = disabled_state(palette);
    let off = match variant {
        ToggleVariant::Default => VariantTokens {
            idle: StateColors::new(palette.primary, palette.foreground, palette.border),
            hovered: StateColors::new(
                mix(palette.primary, Color32::WHITE, 0.06),
                palette.foreground,
                palette.border,
            ),
            active: StateColors::new(
                mix(palette.primary, Color32::from_rgb(30, 58, 138), 0.12),
                palette.foreground,
                palette.border,
            ),
            disabled,
        },
        ToggleVariant::Outline => VariantTokens {
            idle: StateColors::new(palette.primary, palette.foreground, palette.border),
            hovered: StateColors::new(
                mix(palette.primary, Color32::WHITE, 0.06),
                palette.foreground,
                palette.border,
            ),
            active: StateColors::new(
                mix(palette.primary, Color32::from_rgb(30, 58, 138), 0.12),
                palette.foreground,
                palette.border,
            ),
            disabled,
        },
    };
    let on = accent_tokens(palette);
    ToggleButtonTokens { off, on }
}
