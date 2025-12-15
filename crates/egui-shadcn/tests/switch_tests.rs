use egui::{Context, Id, RawInput};
use egui_shadcn::switch::{
    SwitchOptions, SwitchProps, switch, switch_with_options, switch_with_props,
};
use egui_shadcn::{ControlVariant, SwitchSize, SwitchVariant, Theme};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn switch_props_defaults_match_radix() {
    init_logger();
    let mut checked = false;
    let id = Id::new("switch_defaults");
    let props = SwitchProps::new(id, &mut checked, "Airplane mode");

    assert_eq!(props.default_checked, None);
    assert_eq!(props.disabled, false);
    assert_eq!(props.required, false);
    assert_eq!(props.name, None);
    assert_eq!(props.value, Some("on".to_string()));
    assert_eq!(props.as_child, false);
    assert_eq!(props.thumb_as_child, false);
    assert_eq!(props.size, SwitchSize::Two);
    assert_eq!(props.style, SwitchVariant::Surface);
    assert!(props.on_checked_change.is_none());
}

#[test]
fn switch_default_checked_applied_once_and_callback_fires() {
    init_logger();
    let ctx = Context::default();
    let theme = Theme::default();
    let mut checked = false;
    let id = Id::new("switch_default_checked");
    let mut calls = 0usize;

    ctx.begin_pass(RawInput::default());
    egui::CentralPanel::default()
        .show(&ctx, |ui| {
            switch_with_props(
                ui,
                &theme,
                SwitchProps::new(id, &mut checked, "Wi-Fi")
                    .with_default_checked(true)
                    .with_on_checked_change(|state| {
                        calls += 1;
                        assert!(state);
                    }),
            )
        })
        .inner;
    let _ = ctx.end_pass();

    assert!(checked);
    assert_eq!(calls, 1);

    // second pass should not reapply
    checked = false;
    ctx.begin_pass(RawInput::default());
    egui::CentralPanel::default()
        .show(&ctx, |ui| {
            switch_with_props(
                ui,
                &theme,
                SwitchProps::new(id, &mut checked, "Wi-Fi").with_default_checked(true),
            )
        })
        .inner;
    let _ = ctx.end_pass();

    assert!(!checked);
    assert_eq!(calls, 1);
}

#[test]
fn switch_disabled_prevents_toggle() {
    init_logger();
    let ctx = Context::default();
    let theme = Theme::default();
    let mut checked = false;
    let id = Id::new("switch_disabled");

    ctx.begin_pass(RawInput::default());
    let response = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            switch_with_props(
                ui,
                &theme,
                SwitchProps::new(id, &mut checked, "Disabled").with_disabled(true),
            )
        })
        .inner;
    let _ = ctx.end_pass();

    assert!(response.rect.width() >= 0.0);
    assert!(!checked);
}
#[test]
fn switch_renders() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut value = false;
    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            switch(
                ui,
                &theme,
                &mut value,
                "Alias",
                ControlVariant::Outline,
                egui_shadcn::tokens::ControlSize::Sm,
                true,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(!value);
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn switch_with_options_supports_sizes_and_high_contrast() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut value = true;
    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            switch_with_options(
                ui,
                &theme,
                &mut value,
                "HC Switch",
                SwitchOptions {
                    size: SwitchSize::Three,
                    style: SwitchVariant::Classic,
                    high_contrast: true,
                    animate: false,
                    ..SwitchOptions::default()
                },
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(value);
    assert!(inner.rect.width() >= 40.0);
}
