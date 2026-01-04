use std::sync::OnceLock;

#[cfg_attr(not(feature = "plot"), allow(dead_code))]
pub fn screenshot_mode() -> bool {
    static FLAG: OnceLock<bool> = OnceLock::new();
    *FLAG.get_or_init(|| std::env::args().any(|arg| arg == "--screenshot"))
}

#[cfg_attr(not(feature = "plot"), allow(dead_code))]
pub fn apply_screenshot_scale(ctx: &egui::Context) {
    if !screenshot_mode() {
        return;
    }

    let applied_id = egui::Id::new("screenshot_scale_applied");
    let already_applied = ctx
        .data(|d| d.get_temp::<bool>(applied_id))
        .unwrap_or(false);
    if already_applied {
        return;
    }

    ctx.set_pixels_per_point(ctx.pixels_per_point() * 2.0);
    ctx.data_mut(|d| d.insert_temp(applied_id, true));
}
