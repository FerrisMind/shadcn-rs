use egui_shadcn::{SeparatorOrientation, SeparatorProps, SeparatorSize, Theme, separator};
use env_logger;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn separator_size_lengths_match_radix_scale() {
    assert_eq!(SeparatorSize::Size1.length_px(100.0), 16.0);
    assert_eq!(SeparatorSize::Size2.length_px(100.0), 24.0);
    assert_eq!(SeparatorSize::Size3.length_px(100.0), 48.0);
    assert_eq!(SeparatorSize::Size4.length_px(123.0), 123.0);
}

#[test]
fn separator_horizontal_full_width_uses_available_when_length_none() {
    init_logger();
    let ctx = egui::Context::default();
    let theme = Theme::default();
    ctx.begin_pass(egui::RawInput::default());

    let (available, response) = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            let available = ui.available_width();
            let response = separator(ui, &theme, SeparatorProps::default());
            (available, response)
        })
        .inner;

    let _ = ctx.end_pass();
    assert!(
        (response.rect.width() - available).abs() < 0.5,
        "default Size4 horizontal separator should fill available width"
    );
}

#[test]
fn separator_vertical_size2_respects_fixed_length() {
    init_logger();
    let ctx = egui::Context::default();
    let theme = Theme::default();
    ctx.begin_pass(egui::RawInput::default());

    let response = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            separator(
                ui,
                &theme,
                SeparatorProps::default()
                    .with_orientation(SeparatorOrientation::Vertical)
                    .with_size(SeparatorSize::Size2),
            )
        })
        .inner;

    let _ = ctx.end_pass();
    assert!(
        (response.rect.height() - 24.0).abs() < 0.5,
        "Size2 vertical separator should have fixed length"
    );
}

#[test]
fn separator_length_override_takes_precedence_over_size() {
    init_logger();
    let ctx = egui::Context::default();
    let theme = Theme::default();
    ctx.begin_pass(egui::RawInput::default());

    let response = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            separator(
                ui,
                &theme,
                SeparatorProps::default()
                    .with_size(SeparatorSize::Size1)
                    .with_length(100.0),
            )
        })
        .inner;

    let _ = ctx.end_pass();
    assert!(
        (response.rect.width() - 100.0).abs() < 0.5,
        "explicit length must override size"
    );
}
