use crate::theme::Theme;
use crate::tokens::{ColorPalette, DEFAULT_RADIUS, RadiusScale, ease_out_cubic, mix};
use egui::{
    Color32, CornerRadius, Frame, Id, Margin, Response, RichText, Sense, Stroke, StrokeKind, Ui,
    Vec2, vec2,
};
use log::trace;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardVariant {
    Surface,

    Classic,

    Ghost,

    Outline,

    Subtle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CardSize {
    Size1,

    Size2,

    #[default]
    Size3,

    Size4,

    Size5,
}

impl CardSize {
    pub fn padding(self) -> Vec2 {
        match self {
            CardSize::Size1 => vec2(12.0, 10.0),
            CardSize::Size2 => vec2(16.0, 12.0),
            CardSize::Size3 => vec2(20.0, 16.0),
            CardSize::Size4 => vec2(24.0, 20.0),
            CardSize::Size5 => vec2(32.0, 24.0),
        }
    }

    pub fn rounding_with_scale(self, scale: &RadiusScale) -> CornerRadius {
        let radius = match self {
            CardSize::Size1 | CardSize::Size2 => scale.r4,
            CardSize::Size3 | CardSize::Size4 => scale.r5,
            CardSize::Size5 => scale.r6,
        };
        let radius_clamped = radius.round().clamp(0.0, u8::MAX as f32) as u8;
        CornerRadius::same(radius_clamped)
    }
}

#[derive(Clone, Debug)]
pub struct CardProps {
    pub id_source: Option<Id>,
    pub variant: CardVariant,
    pub size: CardSize,
    pub padding: Vec2,
    pub rounding: CornerRadius,
    pub show_shadow: bool,
    pub as_child: bool,
    pub interactive: bool,
    pub sense: Sense,
    pub high_contrast: bool,
    pub tokens_override: Option<CardTokens>,
    pub heading: Option<String>,
    pub description: Option<String>,
}

impl Default for CardProps {
    fn default() -> Self {
        let size = CardSize::Size3;
        Self {
            id_source: None,
            variant: CardVariant::Surface,
            size,
            padding: size.padding(),
            rounding: size.rounding_with_scale(&DEFAULT_RADIUS),
            show_shadow: true,
            as_child: false,
            interactive: false,
            sense: Sense::hover(),
            high_contrast: false,
            tokens_override: None,
            heading: None,
            description: None,
        }
    }
}

impl CardProps {
    pub fn with_id(mut self, id: Id) -> Self {
        self.id_source = Some(id);
        self
    }

    pub fn with_padding(mut self, padding: Vec2) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_rounding(mut self, rounding: CornerRadius) -> Self {
        self.rounding = rounding;
        self
    }

    pub fn with_size(mut self, size: CardSize) -> Self {
        self.size = size;
        self.padding = size.padding();
        self.rounding = size.rounding_with_scale(&DEFAULT_RADIUS);
        self
    }

    pub fn with_as_child(mut self, as_child: bool) -> Self {
        self.as_child = as_child;
        if as_child {
            self.interactive = true;
            self.sense = Sense::click();
        }
        self
    }

    pub fn with_interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        if interactive {
            self.sense = Sense::click();
        }
        self
    }

    pub fn with_sense(mut self, sense: Sense) -> Self {
        self.interactive = true;
        self.sense = sense;
        self
    }

    pub fn with_high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }

    pub fn with_tokens(mut self, tokens: CardTokens) -> Self {
        self.tokens_override = Some(tokens);
        self
    }

    pub fn with_heading(mut self, heading: impl Into<String>) -> Self {
        self.heading = Some(heading.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_shadow(mut self, show_shadow: bool) -> Self {
        self.show_shadow = show_shadow;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CardTokens {
    pub background: Color32,
    pub background_hover: Color32,
    pub background_active: Color32,
    pub stroke: Stroke,
    pub stroke_hover: Stroke,
    pub stroke_active: Stroke,
    pub shadow_idle: egui::epaint::Shadow,
    pub shadow_hover: egui::epaint::Shadow,
    pub shadow_active: egui::epaint::Shadow,
    pub shadow_alpha: u8,
}

pub fn card_tokens(palette: &ColorPalette, variant: CardVariant, show_shadow: bool) -> CardTokens {
    card_tokens_with_options(palette, variant, show_shadow, CardSize::Size3, false)
}

pub fn card_tokens_with_options(
    palette: &ColorPalette,
    variant: CardVariant,
    show_shadow: bool,
    _size: CardSize,
    high_contrast: bool,
) -> CardTokens {
    let resolved_variant = match variant {
        CardVariant::Surface | CardVariant::Subtle => CardVariant::Surface,
        CardVariant::Classic | CardVariant::Outline => CardVariant::Classic,
        CardVariant::Ghost => CardVariant::Ghost,
    };

    let transparent_shadow = egui::epaint::Shadow {
        offset: [0, 0],
        blur: 0,
        spread: 0,
        color: Color32::TRANSPARENT,
    };

    match resolved_variant {
        CardVariant::Surface => {
            let background = mix(
                palette.input,
                palette.background,
                if high_contrast { 0.75 } else { 0.65 },
            );
            let background_hover = mix(
                background,
                Color32::WHITE,
                if high_contrast { 0.06 } else { 0.04 },
            );
            let background_active = mix(background, palette.foreground, 0.06);

            let border_color = mix(
                palette.border,
                palette.background,
                if high_contrast { 0.3 } else { 0.4 },
            );
            let stroke = Stroke::new(if high_contrast { 1.0 } else { 0.75 }, border_color);
            let stroke_hover =
                Stroke::new(stroke.width, mix(border_color, palette.foreground, 0.08));
            let stroke_active =
                Stroke::new(stroke.width, mix(border_color, palette.foreground, 0.04));

            let idle_alpha = if show_shadow { 18 } else { 0 };
            let hover_alpha = if show_shadow { 22 } else { 0 };
            let active_alpha = if show_shadow { 14 } else { 0 };

            let shadow_idle = egui::epaint::Shadow {
                offset: [0, 1],
                blur: 4,
                spread: 0,
                color: Color32::from_rgba_unmultiplied(
                    palette.foreground.r(),
                    palette.foreground.g(),
                    palette.foreground.b(),
                    idle_alpha,
                ),
            };
            let shadow_hover = egui::epaint::Shadow {
                offset: [0, 2],
                blur: 6,
                spread: 0,
                color: Color32::from_rgba_unmultiplied(
                    palette.foreground.r(),
                    palette.foreground.g(),
                    palette.foreground.b(),
                    hover_alpha,
                ),
            };
            let shadow_active = egui::epaint::Shadow {
                offset: [0, 1],
                blur: 4,
                spread: 0,
                color: Color32::from_rgba_unmultiplied(
                    palette.foreground.r(),
                    palette.foreground.g(),
                    palette.foreground.b(),
                    active_alpha,
                ),
            };

            CardTokens {
                background,
                background_hover,
                background_active,
                stroke,
                stroke_hover,
                stroke_active,
                shadow_idle,
                shadow_hover,
                shadow_active,
                shadow_alpha: idle_alpha,
            }
        }
        CardVariant::Classic => {
            let background = palette.background;
            let background_hover = mix(
                background,
                Color32::WHITE,
                if high_contrast { 0.04 } else { 0.02 },
            );
            let background_active = mix(background, palette.foreground, 0.04);

            let border_color = mix(
                palette.border,
                palette.foreground,
                if high_contrast { 0.2 } else { 0.14 },
            );
            let stroke = Stroke::new(if high_contrast { 1.2 } else { 1.0 }, border_color);
            let stroke_hover =
                Stroke::new(stroke.width, mix(border_color, palette.foreground, 0.12));
            let stroke_active =
                Stroke::new(stroke.width, mix(border_color, palette.foreground, 0.06));

            let idle_alpha = if show_shadow { 32 } else { 0 };
            let hover_alpha = if show_shadow { 48 } else { 0 };
            let active_alpha = if show_shadow { 28 } else { 0 };

            let shadow_idle = egui::epaint::Shadow {
                offset: [0, 2],
                blur: 8,
                spread: 0,
                color: Color32::from_rgba_unmultiplied(
                    palette.foreground.r(),
                    palette.foreground.g(),
                    palette.foreground.b(),
                    idle_alpha,
                ),
            };
            let shadow_hover = egui::epaint::Shadow {
                offset: [0, 4],
                blur: 12,
                spread: 0,
                color: Color32::from_rgba_unmultiplied(
                    palette.foreground.r(),
                    palette.foreground.g(),
                    palette.foreground.b(),
                    hover_alpha,
                ),
            };
            let shadow_active = egui::epaint::Shadow {
                offset: [0, 2],
                blur: 8,
                spread: 0,
                color: Color32::from_rgba_unmultiplied(
                    palette.foreground.r(),
                    palette.foreground.g(),
                    palette.foreground.b(),
                    active_alpha,
                ),
            };

            CardTokens {
                background,
                background_hover,
                background_active,
                stroke,
                stroke_hover,
                stroke_active,
                shadow_idle,
                shadow_hover,
                shadow_active,
                shadow_alpha: idle_alpha,
            }
        }
        CardVariant::Ghost => {
            let background = Color32::TRANSPARENT;
            let background_hover = mix(
                palette.muted,
                palette.background,
                if high_contrast { 0.35 } else { 0.25 },
            );
            let background_active = mix(background_hover, palette.foreground, 0.08);

            let stroke = Stroke::new(0.0, Color32::TRANSPARENT);

            CardTokens {
                background,
                background_hover,
                background_active,
                stroke,
                stroke_hover: stroke,
                stroke_active: stroke,
                shadow_idle: transparent_shadow,
                shadow_hover: transparent_shadow,
                shadow_active: transparent_shadow,
                shadow_alpha: 0,
            }
        }
        _ => unreachable!("variant is normalized above"),
    }
}

pub fn card(
    ui: &mut Ui,
    theme: &Theme,
    props: CardProps,
    add_contents: impl FnOnce(&mut Ui),
) -> Response {
    let palette = &theme.palette;
    let tokens = props.tokens_override.unwrap_or_else(|| {
        card_tokens_with_options(
            palette,
            props.variant,
            props.show_shadow,
            props.size,
            props.high_contrast,
        )
    });

    let interactive = props.interactive || props.as_child;
    let sense = if props.as_child {
        Sense::click()
    } else {
        props.sense
    };

    let id = props
        .id_source
        .unwrap_or_else(|| ui.make_persistent_id("card"));
    trace!("render card {:?}", id);

    let ctx = ui.ctx().clone();
    let anim_duration = theme.motion.base_ms / 1000.0;

    let last_hovered = ctx
        .data(|d| d.get_temp::<bool>(id.with("hovered")))
        .unwrap_or(false);
    let last_pressed = ctx
        .data(|d| d.get_temp::<bool>(id.with("pressed")))
        .unwrap_or(false);

    let hover_t = if interactive {
        ctx.animate_bool_with_time_and_easing(
            id.with("hover-anim"),
            last_hovered,
            anim_duration,
            ease_out_cubic,
        )
    } else {
        0.0
    };
    let active_t = if interactive {
        ctx.animate_bool_with_time_and_easing(
            id.with("active-anim"),
            last_pressed,
            anim_duration,
            ease_out_cubic,
        )
    } else {
        0.0
    };

    let hover_mix = hover_t * (1.0 - active_t);

    let mut background = tokens.background;
    let mut stroke = tokens.stroke;
    let mut shadow = tokens.shadow_idle;

    if interactive {
        background = lerp_color(tokens.background, tokens.background_hover, hover_mix);
        background = lerp_color(background, tokens.background_active, active_t);

        stroke = lerp_stroke(tokens.stroke, tokens.stroke_hover, hover_mix);
        stroke = lerp_stroke(stroke, tokens.stroke_active, active_t);

        shadow = lerp_shadow(tokens.shadow_idle, tokens.shadow_hover, hover_mix);
        shadow = lerp_shadow(shadow, tokens.shadow_active, active_t);
    }

    let frame = Frame::default()
        .fill(background)
        .stroke(stroke)
        .corner_radius(props.rounding)
        .inner_margin(Margin::symmetric(
            props.padding.x.round() as i8,
            props.padding.y.round() as i8,
        ))
        .shadow(shadow);

    let inner = frame.show(ui, |card_ui| {
        if let Some(title) = &props.heading {
            card_ui.heading(title);
        }
        if let Some(description) = &props.description {
            let text = RichText::new(description).color(palette.muted_foreground);
            card_ui.label(text);
        }
        add_contents(card_ui);
    });

    let mut response = inner.response;

    if interactive {
        response = response.interact(sense);
        if response.clicked() {
            ui.memory_mut(|m| m.request_focus(response.id));
        }

        let hovered_now = response.hovered();
        let pressed_now = response.is_pointer_button_down_on();
        ctx.data_mut(|d| d.insert_temp(id.with("hovered"), hovered_now));
        ctx.data_mut(|d| d.insert_temp(id.with("pressed"), pressed_now));

        if response.has_focus() {
            let focus_color = if props.high_contrast {
                mix(palette.accent, palette.foreground, 0.35)
            } else {
                mix(palette.accent, palette.foreground, 0.2)
            };
            ui.painter().rect_stroke(
                response.rect,
                props.rounding,
                theme.focus.stroke(focus_color),
                StrokeKind::Outside,
            );
        }
    }

    response
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t)
        .round()
        .clamp(0.0, 255.0) as u8
}

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    Color32::from_rgba_unmultiplied(
        lerp_u8(a.r(), b.r(), t),
        lerp_u8(a.g(), b.g(), t),
        lerp_u8(a.b(), b.b(), t),
        lerp_u8(a.a(), b.a(), t),
    )
}

fn lerp_stroke(a: Stroke, b: Stroke, t: f32) -> Stroke {
    let t = t.clamp(0.0, 1.0);
    Stroke::new(
        a.width + (b.width - a.width) * t,
        lerp_color(a.color, b.color, t),
    )
}

fn lerp_i8(a: i8, b: i8, t: f32) -> i8 {
    (a as f32 + (b as f32 - a as f32) * t)
        .round()
        .clamp(i8::MIN as f32, i8::MAX as f32) as i8
}

fn lerp_shadow(a: egui::epaint::Shadow, b: egui::epaint::Shadow, t: f32) -> egui::epaint::Shadow {
    let t = t.clamp(0.0, 1.0);
    egui::epaint::Shadow {
        offset: [
            lerp_i8(a.offset[0], b.offset[0], t),
            lerp_i8(a.offset[1], b.offset[1], t),
        ],
        blur: lerp_u8(a.blur, b.blur, t),
        spread: lerp_u8(a.spread, b.spread, t),
        color: lerp_color(a.color, b.color, t),
    }
}
