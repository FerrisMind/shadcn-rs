use egui_shadcn::Theme;
use egui_shadcn::checkbox::checkbox;
use egui_shadcn::tokens::{ControlSize, ControlVariant};

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
