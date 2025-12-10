use egui::{Context, RawInput};
use egui_shadcn::label::{Label, LabelProps, LabelStyle, LabelVariant};
use egui_shadcn::tokens::ColorPalette;
use egui_shadcn::{ControlSize, Theme};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn label_props_builder_defaults() {
    init_logger();
    let props = LabelProps::new("Name");
    assert_eq!(props.size, ControlSize::Md);
    assert_eq!(props.variant, LabelVariant::Default);
    assert!(!props.required);
    assert!(!props.disabled);
    assert!(props.for_id.is_none());
    assert!(props.description.is_none());
}

#[test]
fn label_props_builder_with_for_id() {
    init_logger();
    let target_id = egui::Id::new("input");
    let props = LabelProps::new("Email").for_id(target_id);
    assert_eq!(props.for_id, Some(target_id));
}

#[test]
fn label_props_builder_with_size() {
    init_logger();
    let props_sm = LabelProps::new("Text").size(ControlSize::Sm);
    assert_eq!(props_sm.size, ControlSize::Sm);

    let props_lg = LabelProps::new("Text").size(ControlSize::Lg);
    assert_eq!(props_lg.size, ControlSize::Lg);
}

#[test]
fn label_props_builder_with_variant() {
    init_logger();
    let props_default = LabelProps::new("Text").variant(LabelVariant::Default);
    assert_eq!(props_default.variant, LabelVariant::Default);

    let props_secondary = LabelProps::new("Text").variant(LabelVariant::Secondary);
    assert_eq!(props_secondary.variant, LabelVariant::Secondary);

    let props_muted = LabelProps::new("Text").variant(LabelVariant::Muted);
    assert_eq!(props_muted.variant, LabelVariant::Muted);

    let props_destructive = LabelProps::new("Text").variant(LabelVariant::Destructive);
    assert_eq!(props_destructive.variant, LabelVariant::Destructive);
}

#[test]
fn label_props_builder_with_disabled() {
    init_logger();
    let props = LabelProps::new("Text").disabled(true);
    assert!(props.disabled);
}

#[test]
fn label_props_builder_with_required() {
    init_logger();
    let props = LabelProps::new("Text").required(true);
    assert!(props.required);
}

#[test]
fn label_props_builder_with_description() {
    init_logger();
    let props = LabelProps::new("Text").description("Helper text");
    assert!(props.description.is_some());
}

#[test]
fn label_props_builder_chaining() {
    init_logger();
    let target_id = egui::Id::new("field");
    let props = LabelProps::new("Full Name")
        .for_id(target_id)
        .size(ControlSize::Lg)
        .variant(LabelVariant::Secondary)
        .required(true)
        .description("Enter your full legal name");

    assert_eq!(props.for_id, Some(target_id));
    assert_eq!(props.size, ControlSize::Lg);
    assert_eq!(props.variant, LabelVariant::Secondary);
    assert!(props.required);
    assert!(props.description.is_some());
}

#[test]
fn label_widget_builder_defaults() {
    init_logger();

    let _label = Label::new("Name");
}

#[test]
fn label_widget_builder_chaining() {
    init_logger();
    let target_id = egui::Id::new("field");

    let _label = Label::new("Username")
        .for_id(target_id)
        .size(ControlSize::Sm)
        .variant(LabelVariant::Muted)
        .required(true)
        .description("Unique identifier");
}

#[test]
fn label_style_from_palette_default_variant() {
    init_logger();
    let palette = ColorPalette::default();
    let style = LabelStyle::from_palette(&palette, LabelVariant::Default, ControlSize::Md);
    assert_eq!(style.text, palette.foreground);
    assert_eq!(style.required, palette.destructive);
}

#[test]
fn label_style_from_palette_secondary_variant() {
    init_logger();
    let palette = ColorPalette::default();
    let style = LabelStyle::from_palette(&palette, LabelVariant::Secondary, ControlSize::Md);
    assert_eq!(style.text, palette.secondary_foreground);
}

#[test]
fn label_style_from_palette_muted_variant() {
    init_logger();
    let palette = ColorPalette::default();
    let style = LabelStyle::from_palette(&palette, LabelVariant::Muted, ControlSize::Md);
    assert_eq!(style.text, palette.muted_foreground);
}

#[test]
fn label_style_from_palette_destructive_variant() {
    init_logger();
    let palette = ColorPalette::default();
    let style = LabelStyle::from_palette(&palette, LabelVariant::Destructive, ControlSize::Md);
    assert_eq!(style.text, palette.destructive);
}

#[test]
fn label_style_handles_disabled() {
    init_logger();
    let palette = ColorPalette::default();
    let base = LabelStyle::from_palette(&palette, LabelVariant::Default, ControlSize::Sm);
    let disabled = base.clone().disabled();

    assert!(
        disabled.text.a() < base.text.a(),
        "Disabled text should have lower alpha: {} < {}",
        disabled.text.a(),
        base.text.a()
    );
    assert!(
        disabled.description.a() < base.description.a(),
        "Disabled description should have lower alpha: {} < {}",
        disabled.description.a(),
        base.description.a()
    );

    let expected_disabled_alpha = (base.text.a() as f32 * 0.55) as u8;
    assert!(
        (disabled.text.a() as i32 - expected_disabled_alpha as i32).abs() <= 2,
        "Alpha reduction should be ~55%: {} vs {}",
        disabled.text.a(),
        expected_disabled_alpha
    );
}

#[test]
fn label_style_description_font_is_smaller() {
    init_logger();
    let palette = ColorPalette::default();
    let style = LabelStyle::from_palette(&palette, LabelVariant::Default, ControlSize::Md);

    assert!(style.description_font.size < style.font.size);
    assert!(style.description_font.size >= style.font.size * 0.85);
}

#[test]
fn label_style_description_font_minimum_size() {
    init_logger();
    let palette = ColorPalette::default();
    let style = LabelStyle::from_palette(&palette, LabelVariant::Default, ControlSize::Sm);

    assert!(style.description_font.size >= 10.0);
}

#[test]
fn label_renders_and_links_without_panic() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();
    let target_id = egui::Id::new("input");

    let response = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            Label::new("Email")
                .for_id(target_id)
                .required(true)
                .description("We never share your email.")
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(response.rect.width() >= 0.0);
}

#[test]
fn label_renders_all_variants() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();

    let variants = [
        LabelVariant::Default,
        LabelVariant::Secondary,
        LabelVariant::Muted,
        LabelVariant::Destructive,
    ];

    for variant in variants {
        let response = egui::CentralPanel::default()
            .show(&ctx, |ui| {
                Label::new("Test").variant(variant).show(ui, &theme)
            })
            .inner;
        assert!(
            response.rect.width() >= 0.0,
            "Failed to render variant {:?}",
            variant
        );
    }

    let _ = ctx.end_pass();
}

#[test]
fn label_renders_all_sizes() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();

    let sizes = [ControlSize::Sm, ControlSize::Md, ControlSize::Lg];

    for size in sizes {
        let response = egui::CentralPanel::default()
            .show(&ctx, |ui| Label::new("Test").size(size).show(ui, &theme))
            .inner;
        assert!(
            response.rect.width() >= 0.0,
            "Failed to render size {:?}",
            size
        );
    }

    let _ = ctx.end_pass();
}

#[test]
fn label_renders_disabled_state() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();

    let response = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            Label::new("Disabled Label").disabled(true).show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(response.rect.width() >= 0.0);
}

#[test]
fn label_renders_required_indicator() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();

    let response = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            Label::new("Required Field").required(true).show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();

    assert!(response.rect.width() > 0.0);
}

#[test]
fn label_renders_with_description() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();

    let response = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            Label::new("Main Label")
                .description("This is a helper text")
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(response.rect.width() >= 0.0);
}

#[test]
fn label_renders_complex_configuration() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();
    let target_id = egui::Id::new("complex_input");

    let response = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            Label::new("Complex Label")
                .for_id(target_id)
                .size(ControlSize::Lg)
                .variant(LabelVariant::Destructive)
                .required(true)
                .description("This is a destructive action that is required")
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(response.rect.width() >= 0.0);
}
