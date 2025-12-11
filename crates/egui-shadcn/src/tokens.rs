use egui::{Color32, CornerRadius, FontId, Stroke, Vec2};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionTokens {
    pub fast_ms: f32,
    pub base_ms: f32,
    pub slow_ms: f32,
    pub easing: &'static str,
}

impl Default for MotionTokens {
    fn default() -> Self {
        Self {
            fast_ms: 150.0,
            base_ms: 200.0,
            slow_ms: 250.0,
            easing: "cubic-bezier(0.16, 1, 0.3, 1)",
        }
    }
}

pub fn ease_out_cubic(t: f32) -> f32 {
    let clamped = t.clamp(0.0, 1.0);
    let inv = 1.0 - clamped;
    1.0 - inv * inv * inv
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RadiusScale {
    pub r1: f32,
    pub r2: f32,
    pub r3: f32,
    pub r4: f32,
    pub r5: f32,
    pub r6: f32,
}

impl RadiusScale {
    pub fn step(&self, index: u8) -> f32 {
        match index {
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            _ => self.r6,
        }
    }
}

impl Default for RadiusScale {
    fn default() -> Self {
        Self {
            r1: 4.0,
            r2: 6.0,
            r3: 8.0,
            r4: 10.0,
            r5: 12.0,
            r6: 16.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FocusTokens {
    pub ring_width: f32,
}

impl FocusTokens {
    pub fn stroke(&self, color: Color32) -> Stroke {
        Stroke::new(self.ring_width, color)
    }
}

impl Default for FocusTokens {
    fn default() -> Self {
        Self { ring_width: 3.0 }
    }
}

pub const DEFAULT_MOTION: MotionTokens = MotionTokens {
    fast_ms: 150.0,
    base_ms: 200.0,
    slow_ms: 250.0,
    easing: "cubic-bezier(0.16, 1, 0.3, 1)",
};

pub const DEFAULT_RADIUS: RadiusScale = RadiusScale {
    r1: 4.0,
    r2: 6.0,
    r3: 8.0,
    r4: 10.0,
    r5: 12.0,
    r6: 16.0,
};

pub const DEFAULT_FOCUS: FocusTokens = FocusTokens { ring_width: 3.0 };

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwitchSize {
    One,
    Two,
    Three,
}

impl From<ControlSize> for SwitchSize {
    fn from(size: ControlSize) -> Self {
        match size {
            ControlSize::Sm => SwitchSize::One,
            ControlSize::Md | ControlSize::IconSm => SwitchSize::Two,
            ControlSize::Lg | ControlSize::Icon | ControlSize::IconLg => SwitchSize::Three,
        }
    }
}

impl From<SwitchSize> for ControlSize {
    fn from(size: SwitchSize) -> Self {
        match size {
            SwitchSize::One => ControlSize::Sm,
            SwitchSize::Two => ControlSize::Md,
            SwitchSize::Three => ControlSize::Lg,
        }
    }
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
        self.rounding_with_scale(&DEFAULT_RADIUS)
    }

    pub fn rounding_with_scale(self, scale: &RadiusScale) -> CornerRadius {
        let radius = match self {
            ControlSize::Sm => scale.r2,
            ControlSize::Md => scale.r3,
            ControlSize::Lg => scale.r4,
            ControlSize::IconSm => scale.r3,
            ControlSize::Icon => (scale.r3 + scale.r4) * 0.5,
            ControlSize::IconLg => scale.r4,
        };
        let radius_clamped = radius.round().clamp(0.0, u8::MAX as f32) as u8;
        CornerRadius::same(radius_clamped)
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
        Self::dark()
    }
}

impl ColorPalette {
    pub fn dark() -> Self {
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

    pub fn light() -> Self {
        Self {
            background: Color32::from_rgb(255, 255, 255),
            foreground: Color32::from_rgb(37, 37, 37),
            border: Color32::from_rgb(235, 235, 235),
            input: Color32::from_rgb(235, 235, 235),
            primary: Color32::from_rgb(52, 52, 52),
            primary_foreground: Color32::from_rgb(251, 251, 251),
            secondary: Color32::from_rgb(247, 247, 247),
            secondary_foreground: Color32::from_rgb(52, 52, 52),
            accent: Color32::from_rgb(247, 247, 247),
            accent_foreground: Color32::from_rgb(52, 52, 52),
            muted: Color32::from_rgb(247, 247, 247),
            muted_foreground: Color32::from_rgb(142, 142, 142),
            destructive: Color32::from_rgb(220, 53, 53),
            destructive_foreground: Color32::from_rgb(255, 255, 255),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwitchVariant {
    Surface,
    Classic,
    Soft,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputVariant {
    Surface,
    Classic,
    Soft,
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
    pub thumb_disabled: Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwitchTokens {
    pub toggle: ToggleTokens,
    pub focus_ring: Stroke,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwitchTokenOptions {
    pub variant: SwitchVariant,
    pub high_contrast: bool,
    pub accent: Color32,
    pub thumb_color: Option<Color32>,
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

pub fn input_tokens(palette: &ColorPalette, variant: InputVariant) -> InputTokens {
    let disabled = disabled_state(palette);
    let focus = FocusTokens::default();

    match variant {
        InputVariant::Surface => {
            let base_bg = palette.input;
            let border = mix(palette.border, palette.foreground, 0.08);
            let hover_border = mix(border, palette.primary, 0.15);
            let focus_color = mix(palette.primary, palette.foreground, 0.35);
            InputTokens {
                idle: StateColors::with_border(
                    base_bg,
                    palette.foreground,
                    Stroke::new(1.0, border),
                ),
                hovered: StateColors::with_border(
                    mix(base_bg, Color32::WHITE, 0.06),
                    palette.foreground,
                    Stroke::new(1.0, hover_border),
                ),
                focused: StateColors::with_border(
                    mix(base_bg, focus_color, 0.14),
                    palette.foreground,
                    focus.stroke(focus_color),
                ),
                disabled,
                invalid: StateColors::with_border(
                    mix(base_bg, palette.destructive, 0.18),
                    palette.foreground,
                    focus.stroke(palette.destructive),
                ),
                selection_bg: palette.primary,
                selection_fg: palette.primary_foreground,
                placeholder: mix(palette.muted_foreground, palette.foreground, 0.35),
            }
        }
        InputVariant::Classic => {
            let base_bg = mix(palette.input, palette.background, 0.1);
            let border = mix(palette.border, palette.foreground, 0.25);
            let focus_color = mix(palette.primary, palette.border, 0.3);
            InputTokens {
                idle: StateColors::with_border(
                    base_bg,
                    palette.foreground,
                    Stroke::new(1.0, border),
                ),
                hovered: StateColors::with_border(
                    mix(base_bg, Color32::WHITE, 0.08),
                    palette.foreground,
                    Stroke::new(1.0, mix(border, palette.primary, 0.12)),
                ),
                focused: StateColors::with_border(
                    mix(base_bg, Color32::WHITE, 0.12),
                    palette.foreground,
                    focus.stroke(focus_color),
                ),
                disabled,
                invalid: StateColors::with_border(
                    mix(base_bg, palette.destructive, 0.16),
                    palette.foreground,
                    focus.stroke(palette.destructive),
                ),
                selection_bg: palette.primary,
                selection_fg: palette.primary_foreground,
                placeholder: mix(palette.foreground, palette.muted_foreground, 0.55),
            }
        }
        InputVariant::Soft => {
            let base_bg = mix(palette.accent, palette.background, 0.4);
            let border = mix(palette.accent, palette.foreground, 0.12);
            let focus_color = mix(palette.accent, palette.foreground, 0.25);
            InputTokens {
                idle: StateColors::with_border(
                    base_bg,
                    palette.accent_foreground,
                    Stroke::new(0.0, Color32::TRANSPARENT),
                ),
                hovered: StateColors::with_border(
                    mix(base_bg, Color32::WHITE, 0.12),
                    palette.accent_foreground,
                    Stroke::new(1.0, border),
                ),
                focused: StateColors::with_border(
                    mix(base_bg, focus_color, 0.2),
                    palette.accent_foreground,
                    focus.stroke(focus_color),
                ),
                disabled,
                invalid: StateColors::with_border(
                    mix(base_bg, palette.destructive, 0.2),
                    palette.accent_foreground,
                    focus.stroke(palette.destructive),
                ),
                selection_bg: mix(palette.accent, Color32::WHITE, 0.12),
                selection_fg: palette.accent_foreground,
                placeholder: mix(palette.accent_foreground, palette.background, 0.45),
            }
        }
    }
}

pub fn checkbox_tokens(palette: &ColorPalette, variant: ControlVariant) -> ToggleTokens {
    checkbox_tokens_with_high_contrast(palette, variant, false)
}

pub fn checkbox_tokens_with_high_contrast(
    palette: &ColorPalette,
    variant: ControlVariant,
    high_contrast: bool,
) -> ToggleTokens {
    let fg_off = mix(palette.foreground, palette.muted_foreground, 0.4);
    let off_idle =
        StateColors::with_border(palette.input, fg_off, Stroke::new(1.0, palette.border));
    let off_hovered = StateColors::with_border(
        mix(palette.input, Color32::WHITE, 0.04),
        fg_off,
        Stroke::new(1.0, mix(palette.border, palette.foreground, 0.1)),
    );
    let off_active = StateColors::with_border(
        mix(palette.input, Color32::WHITE, 0.08),
        fg_off,
        Stroke::new(1.5, mix(palette.border, palette.primary, 0.25)),
    );

    let on = variant_tokens(palette, variant);
    let disabled = disabled_state(palette);
    let thumb_disabled = mix(palette.muted_foreground, palette.background, 0.5);
    let mut tokens = ToggleTokens {
        off: ToggleState {
            idle: off_idle,
            hovered: off_hovered,
            active: off_active,
        },
        on: ToggleState {
            idle: on.idle,
            hovered: on.hovered,
            active: on.active,
        },
        disabled,
        thumb_on: on.idle.fg_stroke.color,
        thumb_off: fg_off,
        thumb_disabled,
    };

    if high_contrast {
        tokens.on.idle.bg_fill = mix(tokens.on.idle.bg_fill, Color32::WHITE, 0.2);
        tokens.on.hovered.bg_fill = mix(tokens.on.hovered.bg_fill, Color32::WHITE, 0.25);
        tokens.on.active.bg_fill = mix(tokens.on.active.bg_fill, Color32::WHITE, 0.3);

        tokens.off.idle.bg_fill = mix(tokens.off.idle.bg_fill, Color32::WHITE, 0.25);
        tokens.off.hovered.bg_fill = mix(tokens.off.hovered.bg_fill, Color32::WHITE, 0.3);
        tokens.off.active.bg_fill = mix(tokens.off.active.bg_fill, Color32::WHITE, 0.35);

        tokens.disabled.bg_fill = mix(tokens.disabled.bg_fill, palette.background, 0.35);
        tokens.thumb_on = mix(tokens.thumb_on, palette.background, 0.12);
        tokens.thumb_off = mix(tokens.thumb_off, palette.background, 0.12);
    }

    tokens
}

pub fn switch_tokens(palette: &ColorPalette, variant: ControlVariant) -> SwitchTokens {
    let accent = match variant {
        ControlVariant::Primary | ControlVariant::Link => palette.primary,
        ControlVariant::Destructive => palette.destructive,
        ControlVariant::Secondary => palette.secondary,
        ControlVariant::Ghost | ControlVariant::Outline => palette.accent,
    };
    switch_tokens_with_options(
        palette,
        SwitchTokenOptions {
            variant: SwitchVariant::Surface,
            high_contrast: false,
            accent,
            thumb_color: None,
        },
    )
}

pub fn switch_tokens_with_options(
    palette: &ColorPalette,
    options: SwitchTokenOptions,
) -> SwitchTokens {
    let disabled = disabled_state(palette);

    let accent_luminance = (options.accent.r() as f32 * 0.299
        + options.accent.g() as f32 * 0.587
        + options.accent.b() as f32 * 0.114)
        / 255.0;
    let accent_is_light = accent_luminance > 0.5;

    let thumb_base = options.thumb_color.unwrap_or_else(|| {
        mix(
            palette.background,
            mix(palette.foreground, Color32::WHITE, 0.15),
            0.9,
        )
    });

    let thumb_on = if let Some(custom) = options.thumb_color {
        custom
    } else if options.high_contrast {
        mix(options.accent, palette.foreground, 0.65)
    } else if accent_is_light {
        mix(palette.primary_foreground, palette.background, 0.15)
    } else {
        thumb_base
    };

    let thumb_off = if options.high_contrast {
        mix(thumb_base, palette.background, 0.25)
    } else {
        thumb_base
    };

    let off_fg = mix(palette.foreground, palette.muted_foreground, 0.35);
    let (off_idle, off_hovered, off_active) = match options.variant {
        SwitchVariant::Soft => {
            let base = mix(
                palette.background,
                palette.accent,
                if options.high_contrast { 0.35 } else { 0.25 },
            );
            let border = Stroke::new(1.0, mix(palette.accent, palette.foreground, 0.18));
            let idle = StateColors::with_border(base, off_fg, border);
            let hovered = StateColors::with_border(
                mix(base, Color32::WHITE, 0.08),
                off_fg,
                Stroke::new(1.0, mix(border.color, palette.foreground, 0.15)),
            );
            let active = StateColors::with_border(
                mix(base, options.accent, 0.2),
                off_fg,
                Stroke::new(1.2, mix(border.color, options.accent, 0.25)),
            );
            (idle, hovered, active)
        }
        _ => {
            let off_bg = palette.input;
            let off_border = Stroke::new(1.0, mix(palette.border, palette.input, 0.4));
            let idle = StateColors::with_border(off_bg, off_fg, off_border);
            let hovered = StateColors::with_border(
                mix(off_bg, Color32::WHITE, 0.06),
                off_fg,
                Stroke::new(1.0, mix(off_border.color, palette.foreground, 0.1)),
            );
            let active = StateColors::with_border(
                mix(off_bg, Color32::WHITE, 0.12),
                off_fg,
                Stroke::new(1.2, mix(off_border.color, options.accent, 0.2)),
            );
            (idle, hovered, active)
        }
    };

    let (on_idle_bg, on_hover_bg, on_active_bg, on_border) = match options.variant {
        SwitchVariant::Surface => (
            options.accent,
            mix(options.accent, Color32::WHITE, 0.08),
            mix(options.accent, Color32::WHITE, 0.14),
            Stroke::new(1.0, mix(options.accent, palette.foreground, 0.08)),
        ),
        SwitchVariant::Classic => (
            mix(options.accent, palette.background, 0.06),
            mix(options.accent, Color32::WHITE, 0.12),
            mix(options.accent, Color32::WHITE, 0.2),
            Stroke::new(1.0, mix(options.accent, palette.foreground, 0.18)),
        ),
        SwitchVariant::Soft => (
            mix(options.accent, palette.background, 0.45),
            mix(options.accent, Color32::WHITE, 0.16),
            mix(options.accent, Color32::WHITE, 0.22),
            Stroke::new(1.0, mix(options.accent, palette.foreground, 0.12)),
        ),
    };

    let on_idle = StateColors::with_border(on_idle_bg, palette.foreground, on_border);
    let on_hovered = StateColors::with_border(
        on_hover_bg,
        palette.foreground,
        Stroke::new(on_border.width, on_border.color),
    );
    let on_active = StateColors::with_border(
        on_active_bg,
        palette.foreground,
        Stroke::new(on_border.width * 1.05, on_border.color),
    );

    let thumb_disabled = mix(palette.muted_foreground, palette.background, 0.5);

    let toggle = ToggleTokens {
        off: ToggleState {
            idle: off_idle,
            hovered: off_hovered,
            active: off_active,
        },
        on: ToggleState {
            idle: on_idle,
            hovered: on_hovered,
            active: on_active,
        },
        disabled,
        thumb_on,
        thumb_off,
        thumb_disabled,
    };

    let focus_color = if options.high_contrast {
        mix(options.accent, palette.foreground, 0.35)
    } else {
        mix(options.accent, palette.foreground, 0.15)
    };
    let focus_ring = DEFAULT_FOCUS.stroke(focus_color);

    SwitchTokens { toggle, focus_ring }
}

pub fn checkbox_metrics(size: ControlSize) -> ToggleMetrics {
    let track = match size {
        ControlSize::Sm | ControlSize::IconSm => 16.0,
        ControlSize::Md => 16.0,
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

pub fn switch_metrics(size: SwitchSize) -> ToggleMetrics {
    let (height, thumb_inset): (f32, f32) = match size {
        SwitchSize::One => (16.0, 1.0),
        SwitchSize::Two => (20.0, 1.0),
        SwitchSize::Three => (24.0, 1.0),
    };
    let track_w = height * 1.75;
    let thumb = (height - thumb_inset * 2.0).max(12.0);
    ToggleMetrics {
        track_size: Vec2::new(track_w, height),
        thumb_size: Vec2::splat(thumb),
    }
}

pub fn switch_metrics_for_control_size(size: ControlSize) -> ToggleMetrics {
    switch_metrics(SwitchSize::from(size))
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
    let border = Stroke::new(1.0, palette.accent);
    VariantTokens {
        idle: StateColors::with_border(palette.accent, palette.accent_foreground, border),
        hovered: StateColors::with_border(
            mix(palette.accent, Color32::WHITE, 0.08),
            palette.accent_foreground,
            Stroke::new(border.width, mix(palette.accent, Color32::WHITE, 0.1)),
        ),
        active: StateColors::with_border(
            mix(palette.accent, palette.foreground, 0.14),
            palette.accent_foreground,
            Stroke::new(border.width, mix(palette.accent, palette.foreground, 0.14)),
        ),
        disabled,
    }
}

pub fn toggle_button_tokens(palette: &ColorPalette, variant: ToggleVariant) -> ToggleButtonTokens {
    let disabled = disabled_state(palette);
    let off = match variant {
        ToggleVariant::Default => VariantTokens {
            idle: StateColors::with_border(
                Color32::TRANSPARENT,
                palette.foreground,
                Stroke::new(0.0, Color32::TRANSPARENT),
            ),
            hovered: StateColors::with_border(
                palette.muted,
                palette.muted_foreground,
                Stroke::new(0.0, Color32::TRANSPARENT),
            ),
            active: StateColors::with_border(
                mix(palette.muted, palette.foreground, 0.12),
                palette.muted_foreground,
                Stroke::new(0.0, Color32::TRANSPARENT),
            ),
            disabled,
        },
        ToggleVariant::Outline => {
            let border = Stroke::new(1.0, palette.border);
            VariantTokens {
                idle: StateColors::with_border(Color32::TRANSPARENT, palette.foreground, border),
                hovered: StateColors::with_border(
                    palette.accent,
                    palette.accent_foreground,
                    Stroke::new(border.width, palette.accent),
                ),
                active: StateColors::with_border(
                    mix(palette.accent, palette.foreground, 0.12),
                    palette.accent_foreground,
                    Stroke::new(border.width, mix(palette.accent, palette.foreground, 0.12)),
                ),
                disabled,
            }
        }
    };
    let on = accent_tokens(palette);
    ToggleButtonTokens { off, on }
}
