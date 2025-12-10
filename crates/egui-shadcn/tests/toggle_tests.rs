use egui_shadcn::Theme;
use egui_shadcn::toggle::toggle;
use egui_shadcn::tokens::{ControlSize, ToggleVariant};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn toggle_button_renders() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut value = false;
    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            toggle(
                ui,
                &theme,
                &mut value,
                "Toggle btn",
                ToggleVariant::Outline,
                ControlSize::Sm,
                true,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
    assert!(!value);
}
