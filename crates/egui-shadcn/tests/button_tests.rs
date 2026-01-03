use egui::Color32;
use egui_shadcn::Theme;
use egui_shadcn::button::{
    Button, ButtonProps, ButtonRadius, ButtonSize, ButtonStyle, ButtonVariant, button,
};
use egui_shadcn::tokens::{ColorPalette, ControlSize, ControlVariant};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn button_renders_without_panic() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            button(
                ui,
                &theme,
                "Test",
                ControlVariant::Primary,
                ControlSize::Md,
                true,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn icon_sizes_and_link_variant_have_proper_padding() {
    init_logger();

    assert_eq!(ButtonSize::Icon.padding_x(), 0.0);
    assert_eq!(ButtonSize::IconSm.padding_x(), 0.0);
    assert_eq!(ButtonSize::IconLg.padding_x(), 0.0);

    assert!(ButtonSize::Sm.padding_x() > 0.0);
    assert!(ButtonSize::Default.padding_x() > 0.0);
    assert!(ButtonSize::Lg.padding_x() > 0.0);
}

#[test]
fn all_variants_have_distinct_styles() {
    init_logger();
    let palette = ColorPalette::default();

    let default_style = ButtonStyle::from_variant(&palette, ButtonVariant::Default);
    let destructive_style = ButtonStyle::from_variant(&palette, ButtonVariant::Destructive);
    let outline_style = ButtonStyle::from_variant(&palette, ButtonVariant::Outline);
    let secondary_style = ButtonStyle::from_variant(&palette, ButtonVariant::Secondary);
    let ghost_style = ButtonStyle::from_variant(&palette, ButtonVariant::Ghost);
    let link_style = ButtonStyle::from_variant(&palette, ButtonVariant::Link);

    assert_ne!(default_style.bg, destructive_style.bg);

    // Outline variant согласно Radix UI Themes: прозрачный фон, видимая граница на основе foreground
    assert_eq!(outline_style.bg, Color32::TRANSPARENT);
    assert_ne!(outline_style.bg_hover, Color32::TRANSPARENT, "hover должен иметь полупрозрачный фон");
    assert!(outline_style.bg_hover.a() < 50, "hover фон должен быть очень светлым");
    assert_eq!(outline_style.text, palette.foreground);
    assert_eq!(outline_style.text_hover, palette.foreground);
    assert_eq!(outline_style.text_active, palette.foreground);
    // Граница должна быть видимой (не полностью прозрачной и не слишком светлой)
    assert_ne!(outline_style.border, Color32::TRANSPARENT);
    assert!(outline_style.border.a() >= 100, "граница должна иметь достаточную непрозрачность для видимости");
    
    assert_eq!(ghost_style.bg, Color32::TRANSPARENT);
    assert_eq!(link_style.bg, Color32::TRANSPARENT);

    assert_ne!(secondary_style.bg, Color32::TRANSPARENT);
}

#[test]
fn outline_variant_border_is_visible() {
    init_logger();
    let palette = ColorPalette::default();
    let outline_style = ButtonStyle::from_variant(&palette, ButtonVariant::Outline);
    assert_ne!(outline_style.border, Color32::TRANSPARENT);
    assert!(outline_style.border.a() > 0);
}

#[test]
fn button_props_builder_works() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            ButtonProps::new("Test")
                .variant(ButtonVariant::Destructive)
                .size(ButtonSize::Lg)
                .enabled(true)
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn button_struct_builder_works() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            Button::new("Click me")
                .variant(ButtonVariant::Secondary)
                .size(ButtonSize::Sm)
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn loading_state_works() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            Button::new("Loading...").loading(true).show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn high_contrast_modifies_style() {
    init_logger();
    let palette = ColorPalette::default();
    let normal_style = ButtonStyle::from_variant(&palette, ButtonVariant::Default);
    let high_contrast_style = normal_style.clone().with_high_contrast(&palette);

    let bg_changed = normal_style.bg != high_contrast_style.bg
        || normal_style.bg_hover != high_contrast_style.bg_hover;
    if palette.foreground != normal_style.bg || palette.foreground != normal_style.bg_hover {
        assert!(
            bg_changed,
            "high-contrast should tweak background tones when foreground differs"
        );
    }
    assert_eq!(
        high_contrast_style.text, palette.foreground,
        "high-contrast should force text to foreground color"
    );
}

#[test]
fn size_conversion_from_control_size_works() {
    init_logger();
    assert_eq!(ButtonSize::from(ControlSize::Sm), ButtonSize::Sm);
    assert_eq!(ButtonSize::from(ControlSize::Md), ButtonSize::Default);
    assert_eq!(ButtonSize::from(ControlSize::Lg), ButtonSize::Lg);
    assert_eq!(ButtonSize::from(ControlSize::Icon), ButtonSize::Icon);
    assert_eq!(ButtonSize::from(ControlSize::IconSm), ButtonSize::IconSm);
    assert_eq!(ButtonSize::from(ControlSize::IconLg), ButtonSize::IconLg);
}

#[test]
fn variant_conversion_from_control_variant_works() {
    init_logger();
    assert_eq!(
        ButtonVariant::from(ControlVariant::Primary),
        ButtonVariant::Default
    );
    assert_eq!(
        ButtonVariant::from(ControlVariant::Destructive),
        ButtonVariant::Destructive
    );
    assert_eq!(
        ButtonVariant::from(ControlVariant::Outline),
        ButtonVariant::Outline
    );
    assert_eq!(
        ButtonVariant::from(ControlVariant::Secondary),
        ButtonVariant::Secondary
    );
    assert_eq!(
        ButtonVariant::from(ControlVariant::Ghost),
        ButtonVariant::Ghost
    );
    assert_eq!(
        ButtonVariant::from(ControlVariant::Link),
        ButtonVariant::Link
    );
}

#[test]
fn button_radius_corner_values() {
    init_logger();

    let none = ButtonRadius::None.corner_radius().nw;
    let small = ButtonRadius::Small.corner_radius().nw;
    let medium = ButtonRadius::Medium.corner_radius().nw;
    let large = ButtonRadius::Large.corner_radius().nw;
    let full = ButtonRadius::Full.corner_radius().nw;

    assert_eq!(none, 0);
    assert_eq!(small, 4);
    assert_eq!(medium, 8);
    assert_eq!(large, 12);
    assert_eq!(full, 255);

    assert!(none < small);
    assert!(small < medium);
    assert!(medium < large);
    assert!(large < full);
}

#[test]
fn new_radix_variants_have_styles() {
    init_logger();
    let palette = ColorPalette::default();

    let solid_style = ButtonStyle::from_variant(&palette, ButtonVariant::Solid);
    let classic_style = ButtonStyle::from_variant(&palette, ButtonVariant::Classic);
    let soft_style = ButtonStyle::from_variant(&palette, ButtonVariant::Soft);
    let surface_style = ButtonStyle::from_variant(&palette, ButtonVariant::Surface);

    assert_eq!(solid_style.bg, palette.primary);

    assert_ne!(classic_style.border, Color32::TRANSPARENT);

    assert!(soft_style.bg.a() < 255);
    assert_eq!(soft_style.border, Color32::TRANSPARENT);

    assert!(surface_style.bg.a() < 255);
    assert_ne!(surface_style.border, Color32::TRANSPARENT);
}

#[test]
fn button_with_radius_renders() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            Button::new("Rounded")
                .radius(ButtonRadius::Full)
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn button_with_accent_color_renders() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            Button::new("Custom Color")
                .accent_color(Color32::from_rgb(255, 100, 100))
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn accent_color_modifies_style() {
    init_logger();
    let palette = ColorPalette::default();
    let accent = Color32::from_rgb(255, 0, 0);

    let normal_style = ButtonStyle::from_variant(&palette, ButtonVariant::Default);
    let accent_style =
        ButtonStyle::from_variant_with_accent(&palette, ButtonVariant::Default, accent);

    assert_ne!(normal_style.bg, accent_style.bg);
    assert_eq!(accent_style.bg, accent);
}

#[test]
fn soft_variant_with_accent_uses_accent_color() {
    init_logger();
    let palette = ColorPalette::default();
    let accent = Color32::from_rgb(0, 128, 255);

    let style = ButtonStyle::from_variant_with_accent(&palette, ButtonVariant::Soft, accent);

    assert_eq!(style.text, accent);
    assert!(style.bg.a() < 255);
}

#[test]
fn classic_variant_renders() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            Button::new("Classic")
                .variant(ButtonVariant::Classic)
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn surface_variant_renders() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            Button::new("Surface")
                .variant(ButtonVariant::Surface)
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn chained_builder_with_all_new_features() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            Button::new("Full Featured")
                .variant(ButtonVariant::Soft)
                .size(ButtonSize::Lg)
                .radius(ButtonRadius::Large)
                .accent_color(Color32::from_rgb(100, 200, 100))
                .high_contrast(true)
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}
