use egui_shadcn::{DialogAlign, DialogProps, DialogSize};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn dialog_defaults_match_radix_content_api() {
    init_logger();
    let mut open = true;
    let props = DialogProps::new(egui::Id::new("dialog-default"), &mut open);

    assert_eq!(
        props.dialog_size,
        DialogSize::Size3,
        "default size should be 3"
    );
    assert_eq!(
        props.align,
        DialogAlign::Center,
        "default align should be center"
    );
    assert_eq!(
        props.max_width,
        Some(600.0),
        "default max_width should be 600px"
    );
    assert!(!props.as_child, "as_child must be opt-in");
}

#[test]
fn dialog_as_child_is_opt_in() {
    init_logger();
    let mut open = true;
    let props = DialogProps::new(egui::Id::new("dialog-as-child"), &mut open).with_as_child(true);

    assert!(props.as_child, "with_as_child should enable flag");
}
