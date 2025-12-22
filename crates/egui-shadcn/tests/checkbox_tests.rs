use egui_shadcn::Theme;
use egui_shadcn::checkbox::checkbox;
use egui_shadcn::checkbox::{CheckboxCycle, CheckboxState};
use egui_shadcn::tokens::{
    ColorPalette, ControlSize, ControlVariant, checkbox_metrics, checkbox_tokens,
};
use egui_shadcn::{CheckboxProps, CheckboxSize, CheckboxVariant, checkbox_with_props};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn checkbox_renders() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut value = false;
    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            checkbox(
                ui,
                &theme,
                &mut value,
                "Accept",
                ControlVariant::Secondary,
                ControlSize::Sm,
                true,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(!value);
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn checkbox_state_binary_toggle() {
    let mut state = CheckboxState::Unchecked;
    state.toggle(CheckboxCycle::Binary);
    assert_eq!(state, CheckboxState::Checked);
    state.toggle(CheckboxCycle::Binary);
    assert_eq!(state, CheckboxState::Unchecked);
}

#[test]
fn checkbox_state_tri_state_cycle() {
    let mut state = CheckboxState::Unchecked;
    state.toggle(CheckboxCycle::TriState);
    assert_eq!(state, CheckboxState::Checked);
    state.toggle(CheckboxCycle::TriState);
    assert_eq!(state, CheckboxState::Indeterminate);
    state.toggle(CheckboxCycle::TriState);
    assert_eq!(state, CheckboxState::Unchecked);
}

#[test]
fn checkbox_metrics_match_reference_sizes() {
    let sm = checkbox_metrics(ControlSize::Sm);
    let md = checkbox_metrics(ControlSize::Md);
    let lg = checkbox_metrics(ControlSize::Lg);
    assert_eq!(sm.track_size, egui::Vec2::splat(16.0));
    assert_eq!(md.track_size, egui::Vec2::splat(16.0));
    assert_eq!(lg.track_size, egui::Vec2::splat(18.0));
}

#[test]
fn checkbox_tokens_use_input_background_when_unchecked() {
    let palette = ColorPalette::default();
    let tokens = checkbox_tokens(&palette, ControlVariant::Secondary);
    assert_eq!(tokens.off.idle.bg_fill, palette.input);
    assert_eq!(tokens.off.idle.border.color, palette.border);
}

#[test]
fn checkbox_tokens_high_contrast_adjusts_colors() {
    let palette = ColorPalette::default();
    let normal = checkbox_tokens(&palette, ControlVariant::Secondary);
    let high = egui_shadcn::tokens::checkbox_tokens_with_high_contrast(
        &palette,
        ControlVariant::Secondary,
        true,
    );

    assert_ne!(normal.on.idle.bg_fill, high.on.idle.bg_fill);
    assert_ne!(normal.off.idle.bg_fill, high.off.idle.bg_fill);
    assert_ne!(normal.disabled.bg_fill, high.disabled.bg_fill);
}

#[test]
fn checkbox_props_default_matches_radix_api() {
    let props = CheckboxProps::default();
    assert_eq!(
        props.size,
        CheckboxSize::Size2,
        "Radix checkbox defaults to size 2"
    );
    assert_eq!(
        props.variant,
        CheckboxVariant::Surface,
        "Radix checkbox defaults to surface variant"
    );
    assert!(!props.high_contrast, "highContrast is opt-in");
    assert!(props.color.is_none(), "color defaults to palette accent");
}

#[test]
fn checkbox_props_apply_color_override() {
    let mut checked = false;
    let ctx = egui::Context::default();
    let theme = Theme::default();
    let accent = egui::Color32::from_rgb(0, 200, 255);

    ctx.begin_pass(egui::RawInput::default());
    let _ = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            checkbox(
                ui,
                &theme,
                &mut checked,
                "Color override",
                ControlVariant::Secondary,
                ControlSize::Md,
                true,
            );
            let mut state = CheckboxState::Unchecked;
            checkbox_with_props(
                ui,
                &theme,
                &mut state,
                "Props",
                CheckboxProps::default()
                    .with_color(accent)
                    .with_variant(CheckboxVariant::Soft),
            )
        })
        .inner;
    let _ = ctx.end_pass();

    assert!(!checked);
}
