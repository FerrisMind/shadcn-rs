use egui_shadcn::Theme;
use egui_shadcn::tokens::{ControlSize, ControlVariant};

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
    assert!(visuals.focus_stroke.width > 1.0);
}
