use egui_shadcn::{LabelProps, LabelVariant};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn label_defaults_match_radix_api() {
    init_logger();
    let props = LabelProps::new("Name");

    assert!(!props.as_child, "as_child should default to false");
    assert!(props.html_for.is_none(), "html_for should default to None");
    assert_eq!(
        props.variant,
        LabelVariant::Default,
        "variant default stays default"
    );
}

#[test]
fn label_html_for_sets_id_and_is_opt_in() {
    init_logger();
    let props = LabelProps::new("Name").with_html_for("input-id");
    assert_eq!(
        props.html_for.as_deref(),
        Some("input-id"),
        "html_for should store provided target"
    );
}

#[test]
fn label_as_child_is_opt_in() {
    init_logger();
    let props = LabelProps::new("Name").with_as_child(true);
    assert!(props.as_child, "with_as_child should enable flag");
}
