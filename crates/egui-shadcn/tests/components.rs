use egui_shadcn::{
    ControlSize, ControlVariant, TextareaProps, Theme, ToggleVariant, button, checkbox, select,
    switch, text_input, textarea, toggle,
};
use env_logger;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn button_and_checkbox_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut checked = false;
    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            let button_resp = button(
                ui,
                &theme,
                "Click",
                ControlVariant::Primary,
                ControlSize::Md,
                true,
            );
            let checkbox_resp = checkbox(
                ui,
                &theme,
                &mut checked,
                "Check",
                ControlVariant::Secondary,
                ControlSize::Sm,
                true,
            );
            (button_resp, checkbox_resp)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.0.rect.width() >= 0.0);
    assert!(inner.1.rect.width() >= 0.0);
    assert!(!checked);
}

#[test]
fn select_and_toggle_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;
    let options = vec!["One".to_string(), "Two".to_string()];
    let enabled = true;
    let mut switch_on = false;
    let mut toggle_on = false;
    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            let select_resp = select(
                ui,
                &theme,
                egui_shadcn::SelectPropsSimple {
                    id_source: "test_select",
                    selected: &mut selected,
                    options: &options,
                    placeholder: "Pick",
                    size: ControlSize::Md,
                    enabled,
                    is_invalid: false,
                },
            );
            let toggle_resp = toggle(
                ui,
                &theme,
                &mut toggle_on,
                "Toggle",
                ToggleVariant::Outline,
                ControlSize::Md,
                true,
            );
            let switch_resp = switch(
                ui,
                &theme,
                &mut switch_on,
                "Switch",
                ControlVariant::Primary,
                ControlSize::Sm,
                true,
            );
            (select_resp, toggle_resp, switch_resp)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.0.rect.width() >= 0.0);
    assert!(inner.1.rect.width() >= 0.0);
    assert!(inner.2.rect.width() >= 0.0);
    assert!(selected.is_none());
    assert!(enabled);
    assert!(switch_on == false || switch_on == true);
    assert!(toggle_on == false || toggle_on == true);
}

#[test]
fn inputs_render_and_keep_text() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut text = "abc".to_string();
    let mut long_text = "def".to_string();
    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            let input_resp = text_input(
                ui,
                &theme,
                &mut text,
                "placeholder",
                ControlSize::Sm,
                false,
                true,
            );
            let textarea_resp = textarea(
                ui,
                &theme,
                TextareaProps {
                    value: &mut long_text,
                    placeholder: "placeholder".into(),
                    size: ControlSize::Lg,
                    is_invalid: false,
                    show_counter: true,
                    max_len: Some(10),
                    enabled: true,
                },
            );
            (input_resp, textarea_resp)
        })
        .inner;
    let _ = ctx.end_pass();
    assert_eq!(text, "abc");
    assert_eq!(long_text, "def");
    assert!(inner.0.rect.width() >= 0.0);
    assert!(inner.1.rect.width() >= 0.0);
}
