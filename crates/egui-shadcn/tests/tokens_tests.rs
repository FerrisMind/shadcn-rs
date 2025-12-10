use egui::Color32;
use egui_shadcn::tokens::{
    ColorPalette, ControlSize, ControlVariant, InputVariant, SwitchSize, SwitchTokenOptions,
    SwitchVariant, ToggleVariant, checkbox_tokens, input_tokens, mix, switch_metrics,
    switch_metrics_for_control_size, switch_tokens_with_options, toggle_button_tokens,
    toggle_metrics, variant_tokens,
};

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
fn destructive_variant_uses_palette() {
    init_logger();
    let palette = ColorPalette::default();
    let variant = variant_tokens(&palette, ControlVariant::Destructive);
    assert_eq!(variant.idle.bg_fill, palette.destructive);
    assert_eq!(variant.idle.fg_stroke.color, palette.destructive_foreground);
}

#[test]
fn control_size_padding_grows() {
    init_logger();
    let sm = ControlSize::Sm.padding();
    let lg = ControlSize::Lg.padding();
    assert!(lg.x > sm.x);
    assert!(lg.y > sm.y);
}

#[test]
fn checkbox_tokens_use_input_and_variant_bg() {
    init_logger();
    let palette = ColorPalette::default();
    let tokens = checkbox_tokens(&palette, ControlVariant::Primary);
    assert_eq!(tokens.off.idle.bg_fill, palette.input);
    assert_eq!(tokens.on.idle.bg_fill, palette.primary);
}

#[test]
fn input_tokens_expose_selection_and_placeholder() {
    init_logger();
    let palette = ColorPalette::default();
    let tokens = input_tokens(&palette, InputVariant::Surface);
    assert_eq!(tokens.selection_bg, palette.primary);
    assert_eq!(tokens.selection_fg, palette.primary_foreground);
    assert_ne!(tokens.placeholder, palette.foreground);
}

#[test]
fn input_variants_have_distinct_bg_and_selection() {
    init_logger();
    let palette = ColorPalette::default();
    let surface = input_tokens(&palette, InputVariant::Surface);
    let soft = input_tokens(&palette, InputVariant::Soft);
    assert_ne!(
        surface.idle.bg_fill, soft.idle.bg_fill,
        "variants must alter idle background"
    );
    assert_eq!(
        soft.selection_fg, palette.accent_foreground,
        "soft variant should follow accent foreground"
    );
}

#[test]
fn switch_metrics_match_reference() {
    init_logger();
    let s1 = switch_metrics(SwitchSize::One);
    assert_eq!(s1.track_size, egui::Vec2::new(28.0, 16.0));
    assert_eq!(s1.thumb_size, egui::Vec2::splat(14.0));

    let s2 = switch_metrics(SwitchSize::Two);
    assert_eq!(s2.track_size, egui::Vec2::new(35.0, 20.0));
    assert_eq!(s2.thumb_size, egui::Vec2::splat(18.0));

    let s3 = switch_metrics(SwitchSize::Three);
    assert_eq!(s3.track_size, egui::Vec2::new(42.0, 24.0));
    assert_eq!(s3.thumb_size, egui::Vec2::splat(22.0));

    let mapped = switch_metrics_for_control_size(ControlSize::Sm);
    assert_eq!(mapped.track_size, s1.track_size);
}

#[test]
fn toggle_metrics_increase_with_size() {
    init_logger();
    let sm = toggle_metrics(ControlSize::Sm);
    let lg = toggle_metrics(ControlSize::Lg);
    assert!(lg.track_size.x > sm.track_size.x);
    assert!(lg.thumb_size.x > sm.thumb_size.x);
}

#[test]
fn toggle_button_tokens_have_on_state() {
    init_logger();
    let palette = ColorPalette::default();
    let tokens = toggle_button_tokens(&palette, ToggleVariant::Default);
    assert_eq!(tokens.on.idle.bg_fill, palette.accent);
    assert_ne!(tokens.off.idle.bg_fill, tokens.on.idle.bg_fill);
}

#[test]
fn toggle_default_tokens_match_shadcn_reference() {
    init_logger();
    let palette = ColorPalette::default();
    let tokens = toggle_button_tokens(&palette, ToggleVariant::Default);
    assert_eq!(tokens.off.idle.bg_fill, Color32::TRANSPARENT);
    assert_eq!(tokens.off.hovered.bg_fill, palette.muted);
    assert_eq!(tokens.off.hovered.fg_stroke.color, palette.muted_foreground);
    assert_eq!(tokens.on.idle.bg_fill, palette.accent);
    assert_eq!(tokens.on.idle.fg_stroke.color, palette.accent_foreground);
}

#[test]
fn toggle_outline_tokens_apply_accent_on_hover() {
    init_logger();
    let palette = ColorPalette::default();
    let tokens = toggle_button_tokens(&palette, ToggleVariant::Outline);
    assert_eq!(tokens.off.idle.border.color, palette.border);
    assert_eq!(tokens.off.hovered.bg_fill, palette.accent);
    assert_eq!(
        tokens.off.hovered.fg_stroke.color,
        palette.accent_foreground
    );
}

#[test]
fn switch_tokens_respect_variant_and_high_contrast() {
    init_logger();
    let palette = ColorPalette::default();
    let surface = switch_tokens_with_options(
        &palette,
        SwitchTokenOptions {
            variant: SwitchVariant::Surface,
            high_contrast: false,
            accent: palette.primary,
            thumb_color: None,
        },
    );
    let soft_hc = switch_tokens_with_options(
        &palette,
        SwitchTokenOptions {
            variant: SwitchVariant::Soft,
            high_contrast: true,
            accent: palette.accent,
            thumb_color: None,
        },
    );
    assert_ne!(
        surface.toggle.on.idle.bg_fill,
        surface.toggle.off.idle.bg_fill
    );
    assert_ne!(
        soft_hc.toggle.on.idle.bg_fill,
        surface.toggle.on.idle.bg_fill
    );
    assert_ne!(
        soft_hc.toggle.off.idle.bg_fill,
        surface.toggle.off.idle.bg_fill
    );
    assert_ne!(soft_hc.toggle.thumb_on, surface.toggle.thumb_on);
}
