use egui::Color32;
use egui_shadcn::tokens::{
    ColorPalette, ControlSize, ControlVariant, ToggleVariant, checkbox_tokens, input_tokens, mix,
    switch_metrics, toggle_button_tokens, toggle_metrics, variant_tokens,
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
    let tokens = input_tokens(&palette);
    assert_eq!(
        tokens.selection_bg,
        mix(palette.primary, Color32::WHITE, 0.12)
    );
    assert_eq!(tokens.selection_fg, palette.foreground);
    assert_ne!(tokens.placeholder, palette.foreground);
}

#[test]
fn switch_metrics_match_reference() {
    init_logger();
    let sm = switch_metrics(ControlSize::Sm);
    assert_eq!(sm.track_size, egui::Vec2::new(32.0, 18.4));
    assert_eq!(sm.thumb_size, egui::Vec2::splat(16.0));
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
