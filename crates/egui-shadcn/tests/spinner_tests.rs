use egui::{Context, RawInput, Sense, vec2};
use egui_shadcn::{
    SpinnerProps, SpinnerSize, SpinnerVariant, Theme, spinner, spinner_with_content,
};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn spinner_props_defaults_match_reference() {
    init_logger();
    let props = SpinnerProps::default();
    assert_eq!(props.size, SpinnerSize::Size2);
    assert!(props.color.is_none());
    assert!(props.loading);
    assert_eq!(props.variant, SpinnerVariant::RadixLeaf);
    assert!((props.opacity - 0.65).abs() < f32::EPSILON);
    assert!((props.duration_ms - 800.0).abs() < f32::EPSILON);
}

#[test]
fn spinner_props_builders_customize_all_fields() {
    init_logger();
    let props = SpinnerProps::default()
        .with_size(SpinnerSize::Size3)
        .with_color(egui::Color32::from_rgb(255, 0, 0))
        .with_loading(false)
        .with_opacity(0.8)
        .with_duration_ms(1200.0)
        .with_variant(SpinnerVariant::LucideLoaderCircle);

    assert_eq!(props.size, SpinnerSize::Size3);
    assert_eq!(props.color, Some(egui::Color32::from_rgb(255, 0, 0)));
    assert!(!props.loading);
    assert!((props.opacity - 0.8).abs() < f32::EPSILON);
    assert!((props.duration_ms - 1200.0).abs() < f32::EPSILON);
    assert_eq!(props.variant, SpinnerVariant::LucideLoaderCircle);
}

#[test]
fn spinner_renders_with_expected_side_length() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();

    let response = egui::CentralPanel::default()
        .show(&ctx, |ui| spinner(ui, &theme, SpinnerProps::default()))
        .inner;

    let _ = ctx.end_pass();
    let expected = SpinnerSize::Size2.side_px();
    assert!((response.rect.width() - expected).abs() < 0.5);
    assert!((response.rect.height() - expected).abs() < 0.5);
}

#[test]
fn spinner_with_content_returns_child_when_not_loading() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();

    let (value, response) = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            spinner_with_content(
                ui,
                &theme,
                SpinnerProps::default().with_loading(false),
                |ui| {
                    let (rect, _response) =
                        ui.allocate_exact_size(vec2(120.0, 32.0), Sense::hover());
                    (rect.size(), 42)
                },
            )
        })
        .inner;

    let _ = ctx.end_pass();
    let (child_size, number) = value;
    assert_eq!(number, 42);
    assert!((response.rect.width() - 120.0).abs() < 0.5);
    assert!((response.rect.height() - 32.0).abs() < 0.5);
    assert!((child_size.x - 120.0).abs() < 0.5);
    assert!((child_size.y - 32.0).abs() < 0.5);
}

#[test]
fn spinner_with_content_overlays_on_child_when_loading() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();

    let response = egui::CentralPanel::default()
        .frame(egui::Frame::NONE)
        .show(&ctx, |ui| {
            spinner_with_content(ui, &theme, SpinnerProps::default(), |ui| {
                ui.allocate_exact_size(vec2(96.0, 24.0), Sense::hover());
            })
            .1
        })
        .inner;

    let _ = ctx.end_pass();
    assert!((response.rect.width() - 96.0).abs() < 0.5);
    assert!((response.rect.height() - 24.0).abs() < 0.5);
}

#[test]
fn spinner_with_content_paints_only_when_loading() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();

    let _inner = egui::CentralPanel::default()
        .frame(egui::Frame::NONE)
        .show(&ctx, |ui| {
            spinner_with_content(
                ui,
                &theme,
                SpinnerProps::default().with_loading(false),
                |ui| {
                    ui.allocate_exact_size(vec2(48.0, 24.0), Sense::hover());
                },
            );
        });

    let baseline_shapes = ctx.end_pass().shapes.len();

    ctx.begin_pass(RawInput::default());
    let _inner = egui::CentralPanel::default()
        .frame(egui::Frame::NONE)
        .show(&ctx, |ui| {
            spinner_with_content(ui, &theme, SpinnerProps::default(), |ui| {
                ui.allocate_exact_size(vec2(48.0, 24.0), Sense::hover());
            });
        });

    let output = ctx.end_pass();
    assert!(
        output.shapes.len() > baseline_shapes,
        "spinner should paint additional shapes when loading"
    );
}

#[test]
fn spinner_lucide_paints_arc() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();

    let _response = egui::CentralPanel::default()
        .frame(egui::Frame::NONE)
        .show(&ctx, |ui| {
            spinner(
                ui,
                &theme,
                SpinnerProps::default().with_variant(SpinnerVariant::LucideLoaderCircle),
            );
        });

    let output = ctx.end_pass();
    assert!(
        !output.shapes.is_empty(),
        "lucide loader-circle should produce painted shapes"
    );
}
