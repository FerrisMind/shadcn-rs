use egui::Color32;
use egui_shadcn::tokens::ColorPalette;
use egui_shadcn::{
    DEFAULT_FOCUS, ScrollAreaColors, ScrollAreaDir, ScrollAreaProps, ScrollAreaRadius,
    ScrollAreaSize, ScrollAreaType, ScrollDirection, Theme, scroll_area,
};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn scroll_area_props_builder_sets_fields() {
    let id = egui::Id::new("scroll");
    let props = ScrollAreaProps::default()
        .with_id(id)
        .with_direction(ScrollDirection::Both)
        .with_size(ScrollAreaSize::Size3)
        .with_radius(ScrollAreaRadius::Large)
        .with_type(ScrollAreaType::Always)
        .with_hide_delay_ms(500.0)
        .with_auto_shrink([true, false]);

    assert_eq!(props.id_source, Some(id));
    assert_eq!(props.direction, ScrollDirection::Both);
    assert_eq!(props.size, ScrollAreaSize::Size3);
    assert_eq!(props.radius, ScrollAreaRadius::Large);
    assert_eq!(props.scroll_type, ScrollAreaType::Always);
    assert_eq!(props.scroll_hide_delay_ms, Some(500.0));
    assert_eq!(props.auto_shrink, [true, false]);
}

#[test]
fn scroll_area_radix_defaults_match_reference() {
    let defaults = ScrollAreaProps::default();

    assert_eq!(defaults.scroll_type, ScrollAreaType::Hover);
    assert_eq!(defaults.scroll_hide_delay_ms, Some(600.0));
    assert!(!defaults.as_child);
    assert_eq!(defaults.dir, None);
    assert_eq!(defaults.nonce, None);
    assert_eq!(defaults.force_mount, [false, false]);
}

#[test]
fn scroll_area_radix_api_fields_can_be_set() {
    let props = ScrollAreaProps::default()
        .as_child(true)
        .with_dir(ScrollAreaDir::Rtl)
        .with_nonce("abc123")
        .with_force_mount([true, false])
        .with_scroll_hide_delay(450.0);

    assert!(props.as_child);
    assert_eq!(props.dir, Some(ScrollAreaDir::Rtl));
    assert_eq!(props.nonce.as_deref(), Some("abc123"));
    assert_eq!(props.force_mount, [true, false]);
    assert_eq!(props.scroll_hide_delay_ms, Some(450.0));
}

#[test]
fn scroll_area_radius_corner_values() {
    use egui::CornerRadius;

    assert_eq!(
        ScrollAreaRadius::None.corner_radius(),
        CornerRadius::same(0)
    );
    assert_eq!(
        ScrollAreaRadius::Small.corner_radius(),
        CornerRadius::same(2)
    );
    assert_eq!(
        ScrollAreaRadius::Medium.corner_radius(),
        CornerRadius::same(4)
    );
    assert_eq!(
        ScrollAreaRadius::Large.corner_radius(),
        CornerRadius::same(6)
    );
    assert_eq!(
        ScrollAreaRadius::Full.corner_radius(),
        CornerRadius::same(255)
    );
}

#[test]
fn scroll_area_focus_ring_matches_shadcn_tokens() {
    let palette = ColorPalette::default();
    let colors = ScrollAreaColors::from_palette(&palette, None, false);
    assert_eq!(
        colors.focus_ring,
        DEFAULT_FOCUS.stroke(Color32::from_rgba_unmultiplied(
            palette.ring.r(),
            palette.ring.g(),
            palette.ring.b(),
            128
        ))
    );
}

#[test]
fn scroll_area_size_thickness_is_ordered() {
    let s1 = ScrollAreaSize::Size1.metrics();
    let s2 = ScrollAreaSize::Size2.metrics();
    let s3 = ScrollAreaSize::Size3.metrics();

    assert!(s2.bar_thickness > s1.bar_thickness);
    assert!(s3.bar_thickness > s2.bar_thickness);
    assert!(s3.handle_min_length >= s1.handle_min_length);
}

#[test]
fn scroll_area_smoke_returns_inner_value() {
    init_logger();
    let ctx = egui::Context::default();
    let theme = Theme::default();

    ctx.begin_pass(egui::RawInput::default());
    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            scroll_area(
                ui,
                &theme,
                ScrollAreaProps::default()
                    .with_direction(ScrollDirection::Vertical)
                    .with_type(ScrollAreaType::Hover),
                |_scroll_ui| 123usize,
            )
        })
        .inner;
    let _ = ctx.end_pass();

    assert_eq!(inner, 123);
}
