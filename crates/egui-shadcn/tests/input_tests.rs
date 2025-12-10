use egui::Color32;
use egui_shadcn::tokens::ColorPalette;
use egui_shadcn::{
    Input, InputProps, InputRadius, InputSize, InputStyle, InputType, InputVariant,
};

#[test]
fn input_size_height() {
    assert_eq!(InputSize::Size1.height(), 24.0);
    assert_eq!(InputSize::Size2.height(), 32.0);
    assert_eq!(InputSize::Size3.height(), 40.0);
}

#[test]
fn input_size_font_size() {
    assert_eq!(InputSize::Size1.font_size(), 12.0);
    assert_eq!(InputSize::Size2.font_size(), 14.0);
    assert_eq!(InputSize::Size3.font_size(), 16.0);
}

#[test]
fn input_size_padding() {
    let p1 = InputSize::Size1.padding();
    let p2 = InputSize::Size2.padding();
    let p3 = InputSize::Size3.padding();
    assert!(p1.x < p2.x && p2.x < p3.x);
    assert!(p1.y < p2.y && p2.y < p3.y);
}

#[test]
fn input_size_rounding() {
    let r1 = InputSize::Size1.rounding();
    let r2 = InputSize::Size2.rounding();
    let r3 = InputSize::Size3.rounding();
    assert!(r1.nw > 0);
    assert!(r2.nw > 0);
    assert!(r3.nw > 0);
}

#[test]
fn input_size_default() {
    assert_eq!(InputSize::default(), InputSize::Size2);
}

#[test]
fn input_radius_corner_radius() {
    assert_eq!(InputRadius::None.corner_radius().nw, 0);
    assert_eq!(InputRadius::Small.corner_radius().nw, 4);
    assert_eq!(InputRadius::Medium.corner_radius().nw, 6);
    assert_eq!(InputRadius::Large.corner_radius().nw, 8);
    assert_eq!(InputRadius::Full.corner_radius().nw, 255);
}

#[test]
fn input_radius_default() {
    assert_eq!(InputRadius::default(), InputRadius::Medium);
}

#[test]
fn input_variant_default() {
    assert_eq!(InputVariant::default(), InputVariant::Surface);
}

#[test]
fn input_type_default() {
    assert_eq!(InputType::default(), InputType::Text);
}

#[test]
fn input_type_is_password() {
    assert!(!InputType::Text.is_password());
    assert!(InputType::Password.is_password());
    assert!(!InputType::Email.is_password());
    assert!(!InputType::Number.is_password());
    assert!(!InputType::Search.is_password());
    assert!(!InputType::Tel.is_password());
    assert!(!InputType::Url.is_password());
}

#[test]
fn input_style_from_palette() {
    let palette = ColorPalette::default();
    let style = InputStyle::from_palette(&palette, InputVariant::Surface);
    assert_ne!(style.bg, Color32::TRANSPARENT);
    assert_ne!(style.text_color, Color32::TRANSPARENT);
    assert_ne!(style.placeholder_color, Color32::TRANSPARENT);
}

#[test]
fn input_style_variant_surface() {
    let palette = ColorPalette::default();
    let style = InputStyle::from_palette(&palette, InputVariant::Surface);
    assert_ne!(style.border, Color32::TRANSPARENT);
}

#[test]
fn input_style_variant_classic() {
    let palette = ColorPalette::default();
    let style = InputStyle::from_palette(&palette, InputVariant::Classic);
    assert_ne!(style.border, Color32::TRANSPARENT);
}

#[test]
fn input_style_variant_soft() {
    let palette = ColorPalette::default();
    let style = InputStyle::from_palette(&palette, InputVariant::Soft);
    assert_eq!(style.border, Color32::TRANSPARENT);
}

#[test]
fn input_style_default() {
    let style = InputStyle::default();
    assert_ne!(style.bg, Color32::TRANSPARENT);
    assert_ne!(style.text_color, Color32::TRANSPARENT);
}

#[test]
fn input_style_with_accent() {
    let palette = ColorPalette::default();
    let accent = Color32::from_rgb(100, 150, 200);
    let style = InputStyle::from_palette_with_accent(&palette, InputVariant::Soft, accent);

    assert!(style.bg.r() > 0 || style.bg.g() > 0 || style.bg.b() > 0);
}

#[test]
fn input_style_high_contrast() {
    let palette = ColorPalette::default();
    let style = InputStyle::from_palette(&palette, InputVariant::Surface);
    let hc_style = style.with_high_contrast();

    assert_eq!(hc_style.text_color, Color32::WHITE);
}

#[test]
fn input_props_builder_default() {
    let mut value = String::new();
    let props = InputProps::new("test_id", &mut value);
    assert_eq!(props.placeholder, "");
    assert_eq!(props.size, InputSize::Size2);
    assert_eq!(props.variant, InputVariant::Surface);
    assert_eq!(props.radius, InputRadius::Medium);
    assert_eq!(props.input_type, InputType::Text);
    assert!(props.enabled);
    assert!(!props.is_invalid);
    assert!(!props.read_only);
}

#[test]
fn input_props_builder_chain() {
    let mut value = String::new();
    let props = InputProps::new("test_id", &mut value)
        .placeholder("Enter text...")
        .size(InputSize::Size3)
        .variant(InputVariant::Classic)
        .radius(InputRadius::Large)
        .input_type(InputType::Email)
        .enabled(false)
        .read_only(true)
        .invalid(true)
        .max_len(100);

    assert_eq!(props.placeholder, "Enter text...");
    assert_eq!(props.size, InputSize::Size3);
    assert_eq!(props.variant, InputVariant::Classic);
    assert_eq!(props.radius, InputRadius::Large);
    assert_eq!(props.input_type, InputType::Email);
    assert!(!props.enabled);
    assert!(props.read_only);
    assert!(props.is_invalid);
    assert_eq!(props.max_len, Some(100));
}

#[test]
fn input_props_width() {
    let mut value = String::new();
    let props = InputProps::new("test_id", &mut value).width(300.0);
    assert_eq!(props.width, Some(300.0));
}

#[test]
fn input_props_accent_color() {
    let mut value = String::new();
    let accent = Color32::from_rgb(255, 100, 50);
    let props = InputProps::new("test_id", &mut value).accent_color(accent);
    assert_eq!(props.accent_color, Some(accent));
}

#[test]
fn input_props_high_contrast() {
    let mut value = String::new();
    let props = InputProps::new("test_id", &mut value).high_contrast(true);
    assert!(props.high_contrast);
}

#[test]
fn input_props_custom_style() {
    let mut value = String::new();
    let custom_style = InputStyle::default();
    let props = InputProps::new("test_id", &mut value).style(custom_style.clone());
    assert!(props.style.is_some());
}

#[test]
fn input_builder_default() {
    let input = Input::new("test_id");
    assert_eq!(input.placeholder, "");
    assert_eq!(input.size, InputSize::Size2);
    assert_eq!(input.variant, InputVariant::Surface);
}

#[test]
fn input_builder_chain() {
    let input = Input::new("test_id")
        .placeholder("Search...")
        .size(InputSize::Size1)
        .variant(InputVariant::Soft)
        .radius(InputRadius::Full)
        .input_type(InputType::Search);

    assert_eq!(input.placeholder, "Search...");
    assert_eq!(input.size, InputSize::Size1);
    assert_eq!(input.variant, InputVariant::Soft);
    assert_eq!(input.radius, InputRadius::Full);
    assert_eq!(input.input_type, InputType::Search);
}

#[test]
fn input_with_left_slot() {
    let mut value = String::new();
    let props = InputProps::new("test_id", &mut value)
        .left_slot(|_painter, _rect, _color| {

        });
    assert!(props.left_slot.is_some());
}

#[test]
fn input_with_right_slot() {
    let mut value = String::new();
    let props = InputProps::new("test_id", &mut value)
        .right_slot(|_painter, _rect, _color| {

        });
    assert!(props.right_slot.is_some());
}

#[test]
fn input_with_both_slots() {
    let mut value = String::new();
    let props = InputProps::new("test_id", &mut value)
        .left_slot(|_painter, _rect, _color| {})
        .right_slot(|_painter, _rect, _color| {});
    assert!(props.left_slot.is_some());
    assert!(props.right_slot.is_some());
}

#[test]
fn input_style_focus_ring_width() {
    let palette = ColorPalette::default();
    let style = InputStyle::from_palette(&palette, InputVariant::Surface);
    assert!(style.focus_ring_width > 0.0);
}

#[test]
fn input_style_invalid_colors() {
    let palette = ColorPalette::default();
    let style = InputStyle::from_palette(&palette, InputVariant::Surface);
    assert_ne!(style.invalid_border, Color32::TRANSPARENT);
    assert_ne!(style.invalid_ring, Color32::TRANSPARENT);
}

#[test]
fn input_style_selection_colors() {
    let palette = ColorPalette::default();
    let style = InputStyle::from_palette(&palette, InputVariant::Surface);
    assert_ne!(style.selection_bg, Color32::TRANSPARENT);
}

#[test]
fn input_style_disabled_opacity() {
    let palette = ColorPalette::default();
    let style = InputStyle::from_palette(&palette, InputVariant::Surface);
    assert!(style.disabled_opacity > 0.0 && style.disabled_opacity < 1.0);
}
