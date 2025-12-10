use egui_shadcn::Theme;
use egui_shadcn::input::text_input;
use egui_shadcn::tokens::ControlSize;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn input_renders_and_keeps_text() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut text = String::from("value");
    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            text_input(
                ui,
                &theme,
                &mut text,
                "placeholder",
                ControlSize::Sm,
                false,
                true,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert_eq!(text, "value");
    assert!(inner.rect.width() >= 0.0);
}
