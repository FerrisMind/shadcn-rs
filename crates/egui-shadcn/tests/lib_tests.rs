#[test]
fn crate_inits() {
    let _ = env_logger::builder().is_test(true).try_init();
    log::info!("egui-shadcn crate initialized for testing");
    let debug = format!("{:?}", egui_shadcn::Theme::default());
    assert!(!debug.is_empty(), "Theme::default should be printable");
}
