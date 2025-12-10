use egui_shadcn::Theme;
use egui_shadcn::switch::switch;
use egui_shadcn::tokens::{ControlSize, ControlVariant};

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
