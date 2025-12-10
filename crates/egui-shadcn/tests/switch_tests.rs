use egui_shadcn::Theme;
use egui_shadcn::switch::{SwitchOptions, switch, switch_with_options};
use egui_shadcn::tokens::{ControlSize, ControlVariant, SwitchSize, SwitchVariant};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
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
                    enabled: true,
                    high_contrast: true,
                    animate: false,
                    accent: None,
                    corner_radius: None,
                    thumb_color: None,
                },
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(value);
    assert!(inner.rect.width() >= 40.0);
}
