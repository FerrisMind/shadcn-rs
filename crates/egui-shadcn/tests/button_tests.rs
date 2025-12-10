use egui::Color32;
use egui_shadcn::Theme;
use egui_shadcn::button::{Button, ButtonProps, ButtonSize, ButtonStyle, ButtonVariant, button};
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

    assert_eq!(outline_style.bg, Color32::TRANSPARENT);
    assert_eq!(ghost_style.bg, Color32::TRANSPARENT);
    assert_eq!(link_style.bg, Color32::TRANSPARENT);

    assert_ne!(secondary_style.bg, Color32::TRANSPARENT);
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
    let high_contrast_style = normal_style.clone().with_high_contrast();

    assert_ne!(normal_style.bg, high_contrast_style.bg);
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
