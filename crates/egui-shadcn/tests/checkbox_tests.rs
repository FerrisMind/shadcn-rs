use egui_shadcn::Theme;
use egui_shadcn::checkbox::checkbox;
use egui_shadcn::checkbox::{CheckboxCycle, CheckboxState};
use egui_shadcn::tokens::{
    ColorPalette, ControlSize, ControlVariant, checkbox_metrics, checkbox_tokens,
};

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
