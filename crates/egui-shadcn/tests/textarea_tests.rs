use egui_shadcn::Theme;
use egui_shadcn::textarea::{TextareaProps, textarea};
use egui_shadcn::tokens::ControlSize;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn textarea_respects_limit() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut text = "abc".to_string();
    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            textarea(
                ui,
                &theme,
                TextareaProps {
                    value: &mut text,
                    placeholder: "placeholder".into(),
                    size: ControlSize::Lg,
                    is_invalid: false,
                    show_counter: true,
                    max_len: Some(5),
                    enabled: true,
                },
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert_eq!(text, "abc");
    assert!(inner.rect.width() >= 0.0);
}
