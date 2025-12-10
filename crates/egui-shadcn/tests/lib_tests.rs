#[test]
fn crate_inits() {
    let _ = env_logger::builder().is_test(true).try_init();
    log::info!("egui-shadcn crate initialized for testing");
    assert!(true);
}
