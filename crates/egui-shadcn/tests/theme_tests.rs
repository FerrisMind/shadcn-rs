use egui_shadcn::Theme;
use egui_shadcn::tokens::{ControlSize, ControlVariant, InputVariant};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn primary_visuals_use_palette() {
    init_logger();
    let theme = Theme::default();
    let visuals = theme.control(ControlVariant::Primary, ControlSize::Md);
    assert_eq!(
        visuals.widgets.inactive.bg_fill, theme.palette.primary,
        "Primary button uses color from palette"
    );
}

#[test]
fn input_focus_stroke_differs() {
    init_logger();
    let theme = Theme::default();
    let visuals = theme.input(ControlSize::Sm);
    assert!(
        visuals.focus_stroke.width >= 3.0,
        "input focus stroke should be thick enough to mirror ring"
    );
}

#[test]
fn input_variants_customize_placeholder() {
    init_logger();
    let theme = Theme::default();
    let surface = theme.input_variant(ControlSize::Md, InputVariant::Surface);
    let soft = theme.input_variant(ControlSize::Md, InputVariant::Soft);
    assert_ne!(
        surface.placeholder, soft.placeholder,
        "variant should change placeholder color for parity with radix"
    );
}
