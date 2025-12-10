use crate::theme::Theme;
use crate::tokens::{ControlSize, ControlVariant, ToggleVariant, toggle_button_tokens};
use egui::{Button, Response, TextStyle, TextWrapMode, Ui, Vec2, WidgetText};
use log::trace;

fn min_size(size: ControlSize) -> Vec2 {
    match size {
        ControlSize::Sm => Vec2::new(32.0, 32.0),
        ControlSize::Md | ControlSize::IconSm => Vec2::new(36.0, 36.0),
        ControlSize::Lg | ControlSize::Icon => Vec2::new(40.0, 40.0),
        ControlSize::IconLg => Vec2::new(44.0, 44.0),
    }
}

pub fn toggle(
    ui: &mut Ui,
    theme: &Theme,
    on: &mut bool,
    label: impl Into<WidgetText>,
    variant: ToggleVariant,
    size: ControlSize,
    enabled: bool,
) -> Response {
    trace!(
        "Rendering toggle(button) variant={:?} size={:?} enabled={}",
        variant, size, enabled
    );

    let tokens = toggle_button_tokens(&theme.palette, variant);
    let visuals = theme.control(
        match variant {
            ToggleVariant::Default => ControlVariant::Ghost,
            ToggleVariant::Outline => ControlVariant::Outline,
        },
        size,
    );

    let anim_id = ui.id().with(on as *const bool);
    let t = ui.ctx().animate_bool(anim_id, *on);
    let lerp_state = |off: &crate::tokens::StateColors, on_state: &crate::tokens::StateColors| {
        crate::tokens::StateColors {
            bg_fill: crate::tokens::mix(off.bg_fill, on_state.bg_fill, t),
            fg_stroke: egui::Stroke {
                width: off.fg_stroke.width + (on_state.fg_stroke.width - off.fg_stroke.width) * t,
                color: crate::tokens::mix(off.fg_stroke.color, on_state.fg_stroke.color, t),
            },
            border: egui::Stroke {
                width: off.border.width + (on_state.border.width - off.border.width) * t,
                color: crate::tokens::mix(off.border.color, on_state.border.color, t),
            },
        }
    };
    let blended = crate::tokens::VariantTokens {
        idle: lerp_state(&tokens.off.idle, &tokens.on.idle),
        hovered: lerp_state(&tokens.off.hovered, &tokens.on.hovered),
        active: lerp_state(&tokens.off.active, &tokens.on.active),
        disabled: tokens.off.disabled,
    };
    let widgets = crate::theme::widgets_from_variant(&blended, size.rounding(), size.expansion());

    theme.scoped(ui, widgets, |scoped_ui| {
        let mut style = scoped_ui.style().as_ref().clone();
        style.spacing.button_padding = match size {
            ControlSize::Sm => visuals.padding * 0.55,
            ControlSize::Md | ControlSize::IconSm => visuals.padding * 0.6,
            ControlSize::Lg | ControlSize::Icon => visuals.padding * 0.65,
            ControlSize::IconLg => visuals.padding * 0.7,
        };
        style
            .text_styles
            .insert(TextStyle::Button, visuals.text_style.clone());
        style.visuals.selection.bg_fill = tokens.on.idle.bg_fill;
        style.visuals.selection.stroke = tokens.on.idle.fg_stroke;
        style.visuals.override_text_color = Some(theme.palette.foreground);
        scoped_ui.set_style(style);

        let label_colored: WidgetText = label.into().color(theme.palette.foreground);
        let is_icon_size = matches!(
            size,
            ControlSize::IconSm | ControlSize::Icon | ControlSize::IconLg
        );

        if is_icon_size {
            let resp = scoped_ui.add_enabled(
                enabled,
                Button::new("")
                    .min_size(min_size(size))
                    .sense(egui::Sense::click()),
            );
            if resp.clicked() && enabled {
                *on = !*on;
            }

            let icon_color = if enabled {
                theme.palette.foreground
            } else {
                tokens.off.disabled.fg_stroke.color
            };
            let galley = label_colored.clone().into_galley(
                scoped_ui,
                Some(TextWrapMode::Extend),
                f32::INFINITY,
                TextStyle::Button,
            );
            let icon_pos = resp.rect.center() - 0.5 * galley.size();
            scoped_ui.painter().galley(icon_pos, galley, icon_color);
            resp
        } else {
            let resp = scoped_ui.add_enabled(
                enabled,
                Button::new(label_colored)
                    .min_size(min_size(size))
                    .sense(egui::Sense::click()),
            );
            if resp.clicked() && enabled {
                *on = !*on;
            }
            resp
        }
    })
}
