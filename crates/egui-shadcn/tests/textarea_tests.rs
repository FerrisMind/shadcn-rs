use egui::Color32;
use egui_shadcn::Theme;
use egui_shadcn::tokens::ColorPalette;
use egui_shadcn::{
    Textarea, TextareaProps, TextareaRadius, TextareaSize, TextareaStyle, TextareaVariant,
    textarea_with_props,
};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn textarea_size_trigger_height() {
    assert_eq!(TextareaSize::Size1.min_height(), 64.0);
    assert_eq!(TextareaSize::Size2.min_height(), 80.0);
    assert_eq!(TextareaSize::Size3.min_height(), 96.0);
}

#[test]
fn textarea_size_font_size() {
    assert_eq!(TextareaSize::Size1.font_size(), 12.0);
    assert_eq!(TextareaSize::Size2.font_size(), 14.0);
    assert_eq!(TextareaSize::Size3.font_size(), 16.0);
}

#[test]
fn textarea_size_padding() {
    let p1 = TextareaSize::Size1.padding();
    let p2 = TextareaSize::Size2.padding();
    let p3 = TextareaSize::Size3.padding();
    assert!(p1.x < p2.x && p2.x < p3.x);
    assert!(p1.y < p2.y && p2.y < p3.y);
}

#[test]
fn textarea_radius_corner_radius() {
    assert_eq!(TextareaRadius::None.corner_radius().nw, 0);
    assert_eq!(TextareaRadius::Small.corner_radius().nw, 4);
    assert_eq!(TextareaRadius::Medium.corner_radius().nw, 6);
    assert_eq!(TextareaRadius::Large.corner_radius().nw, 8);
    assert_eq!(TextareaRadius::Full.corner_radius().nw, 255);
}

#[test]
fn textarea_variant_default() {
    assert_eq!(TextareaVariant::default(), TextareaVariant::Surface);
}

#[test]
fn textarea_size_default() {
    assert_eq!(TextareaSize::default(), TextareaSize::Size2);
}

#[test]
fn textarea_radius_default() {
    assert_eq!(TextareaRadius::default(), TextareaRadius::Medium);
}

#[test]
fn textarea_style_from_palette() {
    let palette = ColorPalette::default();
    let style = TextareaStyle::from_palette(&palette, TextareaVariant::Surface);
    assert_eq!(style.border, palette.input);
    assert_eq!(style.border_focus, palette.ring);
    assert_eq!(
        style.focus_ring,
        Color32::from_rgba_unmultiplied(palette.ring.r(), palette.ring.g(), palette.ring.b(), 128)
    );
    assert_eq!(style.selection_bg, palette.primary);
    assert_eq!(style.selection_fg, palette.primary_foreground);
    assert_ne!(style.text_color, Color32::TRANSPARENT);
    assert_ne!(style.placeholder_color, Color32::TRANSPARENT);
}

#[test]
fn textarea_style_variant_surface() {
    let palette = ColorPalette::default();
    let style = TextareaStyle::from_palette(&palette, TextareaVariant::Surface);
    assert_eq!(style.border, palette.input);
}

#[test]
fn textarea_style_variant_classic() {
    let palette = ColorPalette::default();
    let style = TextareaStyle::from_palette(&palette, TextareaVariant::Classic);
    assert_eq!(style.border, palette.input);
}

#[test]
fn textarea_style_variant_soft() {
    let palette = ColorPalette::default();
    let style = TextareaStyle::from_palette(&palette, TextareaVariant::Soft);
    assert_eq!(style.border, Color32::TRANSPARENT);
}

#[test]
fn textarea_props_builder_default() {
    let mut value = String::new();
    let props = TextareaProps::new("test_id", &mut value);
    assert_eq!(props.placeholder, "");
    assert_eq!(props.size, TextareaSize::Size2);
    assert_eq!(props.variant, TextareaVariant::Surface);
    assert_eq!(props.radius, TextareaRadius::Medium);
    assert!(props.enabled);
    assert!(!props.is_invalid);
    assert!(!props.read_only);
}

#[test]
fn textarea_props_builder_placeholder() {
    let mut value = String::new();
    let props = TextareaProps::new("test_id", &mut value).placeholder("Enter text...");
    assert_eq!(props.placeholder, "Enter text...");
}

#[test]
fn textarea_props_builder_size() {
    let mut value = String::new();
    let props = TextareaProps::new("test_id", &mut value).size(TextareaSize::Size3);
    assert_eq!(props.size, TextareaSize::Size3);
}

#[test]
fn textarea_props_builder_variant() {
    let mut value = String::new();
    let props = TextareaProps::new("test_id", &mut value).variant(TextareaVariant::Classic);
    assert_eq!(props.variant, TextareaVariant::Classic);
}

#[test]
fn textarea_props_builder_radius() {
    let mut value = String::new();
    let props = TextareaProps::new("test_id", &mut value).radius(TextareaRadius::Large);
    assert_eq!(props.radius, TextareaRadius::Large);
}

#[test]
fn textarea_props_builder_enabled() {
    let mut value = String::new();
    let props = TextareaProps::new("test_id", &mut value).enabled(false);
    assert!(!props.enabled);
}

#[test]
fn textarea_props_builder_invalid() {
    let mut value = String::new();
    let props = TextareaProps::new("test_id", &mut value).invalid(true);
    assert!(props.is_invalid);
}

#[test]
fn textarea_props_builder_read_only() {
    let mut value = String::new();
    let props = TextareaProps::new("test_id", &mut value).read_only(true);
    assert!(props.read_only);
}

#[test]
fn textarea_props_builder_max_len() {
    let mut value = String::new();
    let props = TextareaProps::new("test_id", &mut value).max_len(100);
    assert_eq!(props.max_len, Some(100));
}

#[test]
fn textarea_props_builder_show_counter() {
    let mut value = String::new();
    let props = TextareaProps::new("test_id", &mut value).show_counter(true);
    assert!(props.show_counter);
}

#[test]
fn textarea_props_builder_accent_color() {
    let mut value = String::new();
    let accent = Color32::from_rgb(100, 150, 200);
    let props = TextareaProps::new("test_id", &mut value).accent_color(accent);
    assert_eq!(props.accent_color, Some(accent));
}

#[test]
fn textarea_props_builder_high_contrast() {
    let mut value = String::new();
    let props = TextareaProps::new("test_id", &mut value).high_contrast(true);
    assert!(props.high_contrast);
}

#[test]
fn textarea_props_builder_rows() {
    let mut value = String::new();
    let props = TextareaProps::new("test_id", &mut value).rows(5);
    assert_eq!(props.rows, Some(5));
}

#[test]
fn textarea_builder_new() {
    let textarea = Textarea::new("builder_test");
    assert_eq!(textarea.size, TextareaSize::Size2);
    assert_eq!(textarea.variant, TextareaVariant::Surface);
}

#[test]
fn textarea_builder_chained() {
    let textarea = Textarea::new("builder_chained")
        .placeholder("Type here...")
        .variant(TextareaVariant::Soft)
        .size(TextareaSize::Size1)
        .radius(TextareaRadius::Small);
    assert_eq!(textarea.size, TextareaSize::Size1);
    assert_eq!(textarea.variant, TextareaVariant::Soft);
    assert_eq!(textarea.radius, TextareaRadius::Small);
}

#[test]
fn textarea_renders_basic() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut text = "hello".to_string();

    egui::CentralPanel::default().show(&ctx, |ui| {
        let response = textarea_with_props(
            ui,
            &theme,
            TextareaProps::new("render_basic", &mut text)
                .placeholder("Enter text...")
                .size(TextareaSize::Size2),
        );
        assert!(response.rect.width() > 0.0);
        assert!(response.rect.height() > 0.0);
    });
    let _ = ctx.end_pass();
}

#[test]
fn textarea_renders_all_variants() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let variants = [
        TextareaVariant::Surface,
        TextareaVariant::Classic,
        TextareaVariant::Soft,
    ];

    for variant in variants {
        let mut text = String::new();
        egui::CentralPanel::default().show(&ctx, |ui| {
            let response = textarea_with_props(
                ui,
                &theme,
                TextareaProps::new(format!("variant_{:?}", variant), &mut text).variant(variant),
            );
            assert!(response.rect.width() > 0.0);
        });
    }
    let _ = ctx.end_pass();
}

#[test]
fn textarea_renders_all_sizes() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let sizes = [
        TextareaSize::Size1,
        TextareaSize::Size2,
        TextareaSize::Size3,
    ];

    for size in sizes {
        let mut text = String::new();
        egui::CentralPanel::default().show(&ctx, |ui| {
            let response = textarea_with_props(
                ui,
                &theme,
                TextareaProps::new(format!("size_{:?}", size), &mut text).size(size),
            );

            assert!(response.rect.height() > 0.0);
        });
    }
    let _ = ctx.end_pass();
}

#[test]
fn textarea_renders_all_radii() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let radii = [
        TextareaRadius::None,
        TextareaRadius::Small,
        TextareaRadius::Medium,
        TextareaRadius::Large,
        TextareaRadius::Full,
    ];

    for radius in radii {
        let mut text = String::new();
        egui::CentralPanel::default().show(&ctx, |ui| {
            let response = textarea_with_props(
                ui,
                &theme,
                TextareaProps::new(format!("radius_{:?}", radius), &mut text).radius(radius),
            );
            assert!(response.rect.width() > 0.0);
        });
    }
    let _ = ctx.end_pass();
}

#[test]
fn textarea_renders_disabled() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut text = "disabled".to_string();

    egui::CentralPanel::default().show(&ctx, |ui| {
        let response = textarea_with_props(
            ui,
            &theme,
            TextareaProps::new("disabled_test", &mut text).enabled(false),
        );
        assert!(response.rect.width() > 0.0);
    });
    let _ = ctx.end_pass();
}

#[test]
fn textarea_renders_invalid() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut text = String::new();

    egui::CentralPanel::default().show(&ctx, |ui| {
        let response = textarea_with_props(
            ui,
            &theme,
            TextareaProps::new("invalid_test", &mut text).invalid(true),
        );
        assert!(response.rect.width() > 0.0);
    });
    let _ = ctx.end_pass();
}

#[test]
fn textarea_renders_read_only() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut text = "read only text".to_string();

    egui::CentralPanel::default().show(&ctx, |ui| {
        let response = textarea_with_props(
            ui,
            &theme,
            TextareaProps::new("readonly_test", &mut text).read_only(true),
        );
        assert!(response.rect.width() > 0.0);
    });
    let _ = ctx.end_pass();
}

#[test]
fn textarea_renders_with_counter() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut text = "abc".to_string();

    egui::CentralPanel::default().show(&ctx, |ui| {
        let response = textarea_with_props(
            ui,
            &theme,
            TextareaProps::new("counter_test", &mut text)
                .show_counter(true)
                .max_len(10),
        );
        assert!(response.rect.width() > 0.0);
    });
    let _ = ctx.end_pass();
    assert_eq!(text, "abc");
}

#[test]
fn textarea_respects_limit() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut text = "abc".to_string();

    egui::CentralPanel::default().show(&ctx, |ui| {
        textarea_with_props(
            ui,
            &theme,
            TextareaProps::new("limit_test", &mut text)
                .max_len(5)
                .show_counter(true),
        );
    });
    let _ = ctx.end_pass();
    assert_eq!(text, "abc");
}

#[test]
fn textarea_renders_with_accent_color() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut text = String::new();

    egui::CentralPanel::default().show(&ctx, |ui| {
        let response = textarea_with_props(
            ui,
            &theme,
            TextareaProps::new("accent_test", &mut text)
                .accent_color(Color32::from_rgb(100, 150, 200)),
        );
        assert!(response.rect.width() > 0.0);
    });
    let _ = ctx.end_pass();
}

#[test]
fn textarea_renders_high_contrast() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut text = String::new();

    egui::CentralPanel::default().show(&ctx, |ui| {
        let response = textarea_with_props(
            ui,
            &theme,
            TextareaProps::new("high_contrast_test", &mut text)
                .variant(TextareaVariant::Soft)
                .high_contrast(true),
        );
        assert!(response.rect.width() > 0.0);
    });
    let _ = ctx.end_pass();
}

#[test]
fn textarea_style_with_accent() {
    let palette = ColorPalette::default();
    let accent = Color32::from_rgb(50, 100, 150);
    let style = TextareaStyle::from_palette_with_accent(&palette, TextareaVariant::Soft, accent);
    assert_ne!(style.bg, Color32::TRANSPARENT);
}

#[test]
fn textarea_style_default() {
    let style = TextareaStyle::default();
    assert_ne!(style.text_color, Color32::TRANSPARENT);
}
