use egui::Color32;
use egui_shadcn::Theme;
use egui_shadcn::select::{
    ContentVariant, PopupPosition, SelectItem, SelectProps, SelectPropsSimple, SelectRadius,
    SelectSize, SelectStyle, TriggerVariant, select, select_with_items,
};
use egui_shadcn::tokens::{ColorPalette, ControlSize};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn select_size_aliases_are_equivalent() {
    init_logger();

    assert_eq!(
        SelectSize::Sm.trigger_height(),
        SelectSize::Size2.trigger_height()
    );
    assert_eq!(
        SelectSize::Default.trigger_height(),
        SelectSize::Size3.trigger_height()
    );
}

#[test]
fn select_size_trigger_heights_increase() {
    init_logger();
    let size1 = SelectSize::Size1.trigger_height();
    let size2 = SelectSize::Size2.trigger_height();
    let size3 = SelectSize::Size3.trigger_height();

    assert!(size1 < size2);
    assert!(size2 < size3);
    assert!(size1 > 0.0);
}

#[test]
fn select_size_item_heights_increase() {
    init_logger();
    let size1 = SelectSize::Size1.item_height();
    let size2 = SelectSize::Size2.item_height();
    let size3 = SelectSize::Size3.item_height();

    assert!(size1 < size2);
    assert!(size2 < size3);
}

#[test]
fn select_size_font_sizes_increase() {
    init_logger();
    let size1 = SelectSize::Size1.font_size();
    let size2 = SelectSize::Size2.font_size();
    let size3 = SelectSize::Size3.font_size();

    assert!(size1 < size2);
    assert!(size2 < size3);
}

#[test]
fn select_size_icon_sizes_increase() {
    init_logger();
    let size1 = SelectSize::Size1.icon_size();
    let size2 = SelectSize::Size2.icon_size();
    let size3 = SelectSize::Size3.icon_size();

    assert!(size1 < size2);
    assert!(size2 < size3);
}

#[test]
fn select_size_gaps_increase() {
    init_logger();
    let size1 = SelectSize::Size1.gap();
    let size2 = SelectSize::Size2.gap();
    let size3 = SelectSize::Size3.gap();

    assert!(size1 < size2);
    assert!(size2 < size3);
}

#[test]
fn select_size_trigger_padding_values() {
    init_logger();
    let padding1 = SelectSize::Size1.trigger_padding();
    let padding2 = SelectSize::Size2.trigger_padding();
    let padding3 = SelectSize::Size3.trigger_padding();

    assert!(padding1.x > 0.0);
    assert!(padding1.y > 0.0);
    assert!(padding2.x > 0.0);
    assert!(padding3.x > 0.0);
}

#[test]
fn select_size_from_control_size_conversion() {
    init_logger();
    assert_eq!(SelectSize::from(ControlSize::Sm), SelectSize::Sm);
    assert_eq!(SelectSize::from(ControlSize::Md), SelectSize::Default);
    assert_eq!(SelectSize::from(ControlSize::Lg), SelectSize::Default);
}

#[test]
fn select_radius_corner_values_increase() {
    init_logger();
    let none = SelectRadius::None.corner_radius();
    let small = SelectRadius::Small.corner_radius();
    let medium = SelectRadius::Medium.corner_radius();
    let large = SelectRadius::Large.corner_radius();
    let full = SelectRadius::Full.corner_radius();

    assert_eq!(none.nw, 0);
    assert_eq!(none.ne, 0);

    assert!(small.nw < medium.nw);
    assert!(medium.nw < large.nw);
    assert!(large.nw < full.nw);
}

#[test]
fn select_style_default_has_valid_colors() {
    init_logger();
    let style = SelectStyle::default();

    assert_ne!(style.trigger_bg, Color32::TRANSPARENT);
    assert_ne!(style.trigger_text, Color32::TRANSPARENT);

    assert_ne!(style.content_bg, Color32::TRANSPARENT);
    assert_ne!(style.item_text, Color32::TRANSPARENT);
}

#[test]
fn select_style_from_palette_creates_valid_style() {
    init_logger();
    let palette = egui_shadcn::tokens::ColorPalette::default();
    let style = SelectStyle::from_palette(&palette);

    assert_ne!(style.trigger_bg, Color32::TRANSPARENT);
    assert_ne!(style.content_bg, Color32::TRANSPARENT);
    assert_ne!(style.item_text, Color32::TRANSPARENT);
    assert_ne!(style.trigger_placeholder, Color32::TRANSPARENT);
}

#[test]
fn select_style_from_palette_with_accent_applies_accent_colors() {
    init_logger();
    let palette = ColorPalette::default();
    let base = SelectStyle::from_palette(&palette);
    let accent = Color32::from_rgb(210, 90, 120);
    let accent_style = SelectStyle::from_palette_with_accent(&palette, accent);

    assert_ne!(accent_style.trigger_bg, base.trigger_bg);
    assert_eq!(accent_style.trigger_text, accent);
    assert_eq!(accent_style.item_solid_bg_hover, accent);
    assert_eq!(accent_style.focus_ring_color.a(), 180);
}

#[test]
fn select_trigger_variants_have_distinct_styles() {
    init_logger();
    let palette = ColorPalette::default();
    let accent = Color32::from_rgb(120, 160, 255);

    let surface = SelectStyle::from_palette_for_variants(
        &palette,
        TriggerVariant::Surface,
        ContentVariant::Soft,
        None,
    );
    let ghost = SelectStyle::from_palette_for_variants(
        &palette,
        TriggerVariant::Ghost,
        ContentVariant::Soft,
        Some(accent),
    );
    let soft = SelectStyle::from_palette_for_variants(
        &palette,
        TriggerVariant::Soft,
        ContentVariant::Soft,
        Some(accent),
    );

    assert_eq!(ghost.trigger_bg, Color32::TRANSPARENT);
    assert_eq!(ghost.trigger_border, Color32::TRANSPARENT);
    assert_eq!(soft.trigger_text, accent);
    assert!(soft.trigger_bg.a() > surface.trigger_bg.a());
    assert_ne!(surface.trigger_border, Color32::TRANSPARENT);
}

#[test]
fn select_content_variant_solid_sets_solid_backgrounds() {
    init_logger();
    let palette = ColorPalette::default();

    let soft = SelectStyle::from_palette_for_variants(
        &palette,
        TriggerVariant::Surface,
        ContentVariant::Soft,
        None,
    );
    let solid = SelectStyle::from_palette_for_variants(
        &palette,
        TriggerVariant::Surface,
        ContentVariant::Solid,
        None,
    );

    assert_ne!(solid.content_bg, soft.content_bg);
    assert!(solid.item_bg_selected.a() > soft.item_bg_selected.a());
    assert_ne!(solid.item_bg_hover, soft.item_bg_hover);
    assert_ne!(solid.item_text_hover, soft.item_text_hover);
}

#[test]
fn select_style_high_contrast_adjusts_trigger_and_content() {
    init_logger();
    let palette = ColorPalette::default();
    let base = SelectStyle::from_palette(&palette);
    let hc = base.clone().with_high_contrast(&palette);

    assert_ne!(hc.trigger_bg, base.trigger_bg);
    assert_eq!(hc.trigger_text, palette.foreground);
    assert_ne!(hc.content_border, base.content_border);
}

#[test]
fn select_item_option_creates_enabled_item() {
    init_logger();
    let item = SelectItem::option("val", "Label");

    match item {
        SelectItem::Option {
            value,
            label,
            disabled,
        } => {
            assert_eq!(value, "val");
            assert_eq!(label, "Label");
            assert!(!disabled);
        }
        _ => panic!("Expected Option variant"),
    }
}

#[test]
fn select_item_option_disabled_creates_disabled_item() {
    init_logger();
    let item = SelectItem::option_disabled("val", "Label");

    match item {
        SelectItem::Option {
            value,
            label,
            disabled,
        } => {
            assert_eq!(value, "val");
            assert_eq!(label, "Label");
            assert!(disabled);
        }
        _ => panic!("Expected Option variant"),
    }
}

#[test]
fn select_item_group_creates_group() {
    init_logger();
    let items = vec![SelectItem::option("a", "A"), SelectItem::option("b", "B")];
    let group = SelectItem::group("Group Label", items);

    match group {
        SelectItem::Group { label, items } => {
            assert_eq!(label, "Group Label");
            assert_eq!(items.len(), 2);
        }
        _ => panic!("Expected Group variant"),
    }
}

#[test]
fn select_item_separator_creates_separator() {
    init_logger();
    let sep = SelectItem::separator();

    match sep {
        SelectItem::Separator => {}
        _ => panic!("Expected Separator variant"),
    }
}

#[test]
fn select_item_label_creates_label() {
    init_logger();
    let label = SelectItem::label("Section Header");

    match label {
        SelectItem::Label(text) => assert_eq!(text, "Section Header"),
        _ => panic!("Expected Label variant"),
    }
}

#[test]
fn select_props_new_creates_default_props() {
    init_logger();
    let mut selected = None;
    let props = SelectProps::new("test_id", &mut selected);

    assert_eq!(props.placeholder, "Select...");
    assert_eq!(props.size, SelectSize::Size2);
    assert_eq!(props.trigger_variant, TriggerVariant::Surface);
    assert_eq!(props.content_variant, ContentVariant::Soft);
    assert!(props.enabled);
    assert!(!props.is_invalid);
    assert!(props.width.is_none());
    assert!(props.style.is_none());
    assert!(props.accent_color.is_none());
    assert_eq!(props.radius, SelectRadius::Medium);
    assert!(!props.high_contrast);
    assert_eq!(props.position, PopupPosition::Popper);
}

#[test]
fn select_props_placeholder_sets_value() {
    init_logger();
    let mut selected = None;
    let props = SelectProps::new("test", &mut selected).placeholder("Choose an option...");

    assert_eq!(props.placeholder, "Choose an option...");
}

#[test]
fn select_props_size_sets_value() {
    init_logger();
    let mut selected = None;
    let props = SelectProps::new("test", &mut selected).size(SelectSize::Size1);

    assert_eq!(props.size, SelectSize::Size1);
}

#[test]
fn select_props_trigger_variant_sets_value() {
    init_logger();
    let mut selected = None;

    let surface = SelectProps::new("test1", &mut selected).trigger_variant(TriggerVariant::Surface);
    assert_eq!(surface.trigger_variant, TriggerVariant::Surface);

    let ghost = SelectProps::new("test2", &mut selected).trigger_variant(TriggerVariant::Ghost);
    assert_eq!(ghost.trigger_variant, TriggerVariant::Ghost);

    let classic = SelectProps::new("test3", &mut selected).trigger_variant(TriggerVariant::Classic);
    assert_eq!(classic.trigger_variant, TriggerVariant::Classic);

    let soft = SelectProps::new("test4", &mut selected).trigger_variant(TriggerVariant::Soft);
    assert_eq!(soft.trigger_variant, TriggerVariant::Soft);
}

#[test]
fn select_props_content_variant_sets_value() {
    init_logger();
    let mut selected = None;

    let soft = SelectProps::new("test1", &mut selected).content_variant(ContentVariant::Soft);
    assert_eq!(soft.content_variant, ContentVariant::Soft);

    let solid = SelectProps::new("test2", &mut selected).content_variant(ContentVariant::Solid);
    assert_eq!(solid.content_variant, ContentVariant::Solid);
}

#[test]
fn select_props_enabled_sets_value() {
    init_logger();
    let mut selected = None;
    let props = SelectProps::new("test", &mut selected).enabled(false);

    assert!(!props.enabled);
}

#[test]
fn select_props_invalid_sets_value() {
    init_logger();
    let mut selected = None;
    let props = SelectProps::new("test", &mut selected).invalid(true);

    assert!(props.is_invalid);
}

#[test]
fn select_props_width_sets_value() {
    init_logger();
    let mut selected = None;
    let props = SelectProps::new("test", &mut selected).width(250.0);

    assert_eq!(props.width, Some(250.0));
}

#[test]
fn select_props_style_sets_value() {
    init_logger();
    let mut selected = None;
    let custom = SelectStyle {
        trigger_bg: Color32::RED,
        ..SelectStyle::default()
    };
    let props = SelectProps::new("test", &mut selected).style(custom);

    assert!(props.style.is_some());
    assert_eq!(props.style.unwrap().trigger_bg, Color32::RED);
}

#[test]
fn select_props_accent_color_sets_value() {
    init_logger();
    let mut selected = None;
    let props = SelectProps::new("test", &mut selected).accent_color(Color32::BLUE);

    assert_eq!(props.accent_color, Some(Color32::BLUE));
}

#[test]
fn select_props_radius_sets_value() {
    init_logger();
    let mut selected = None;

    let none = SelectProps::new("t1", &mut selected).radius(SelectRadius::None);
    assert_eq!(none.radius, SelectRadius::None);

    let small = SelectProps::new("t2", &mut selected).radius(SelectRadius::Small);
    assert_eq!(small.radius, SelectRadius::Small);

    let full = SelectProps::new("t3", &mut selected).radius(SelectRadius::Full);
    assert_eq!(full.radius, SelectRadius::Full);
}

#[test]
fn select_props_high_contrast_sets_value() {
    init_logger();
    let mut selected = None;
    let props = SelectProps::new("test", &mut selected).high_contrast(true);

    assert!(props.high_contrast);
}

#[test]
fn select_props_position_sets_value() {
    init_logger();
    let mut selected = None;

    let popper = SelectProps::new("t1", &mut selected).position(PopupPosition::Popper);
    assert_eq!(popper.position, PopupPosition::Popper);

    let item = SelectProps::new("t2", &mut selected).position(PopupPosition::ItemAligned);
    assert_eq!(item.position, PopupPosition::ItemAligned);
}

#[test]
fn select_props_chained_builder() {
    init_logger();
    let mut selected = Some("option1".to_string());
    let props = SelectProps::new("select_chained", &mut selected)
        .placeholder("Pick one...")
        .size(SelectSize::Size3)
        .trigger_variant(TriggerVariant::Ghost)
        .content_variant(ContentVariant::Solid)
        .enabled(true)
        .invalid(false)
        .width(300.0)
        .radius(SelectRadius::Large)
        .high_contrast(true)
        .position(PopupPosition::ItemAligned);

    assert_eq!(props.placeholder, "Pick one...");
    assert_eq!(props.size, SelectSize::Size3);
    assert_eq!(props.trigger_variant, TriggerVariant::Ghost);
    assert_eq!(props.content_variant, ContentVariant::Solid);
    assert!(props.enabled);
    assert!(!props.is_invalid);
    assert_eq!(props.width, Some(300.0));
    assert_eq!(props.radius, SelectRadius::Large);
    assert!(props.high_contrast);
    assert_eq!(props.position, PopupPosition::ItemAligned);
}

#[test]
fn select_renders_with_placeholder() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;
    let options = vec!["One".to_string(), "Two".to_string()];
    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select(
                ui,
                &theme,
                SelectPropsSimple {
                    id_source: "select_test",
                    selected: &mut selected,
                    options: &options,
                    placeholder: "Choose",
                    size: ControlSize::Md,
                    enabled: true,
                    is_invalid: false,
                },
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(selected.is_none());
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_handles_invalid_state() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;
    let options = vec!["One".to_string()];
    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select(
                ui,
                &theme,
                SelectPropsSimple {
                    id_source: "select_invalid",
                    selected: &mut selected,
                    options: &options,
                    placeholder: "Choose",
                    size: ControlSize::Sm,
                    enabled: true,
                    is_invalid: true,
                },
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_with_items_renders() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;

    let items = vec![
        SelectItem::option("apple", "Apple"),
        SelectItem::option("banana", "Banana"),
        SelectItem::separator(),
        SelectItem::option("orange", "Orange"),
    ];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_items", &mut selected)
                    .placeholder("Select fruit...")
                    .size(SelectSize::Default),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(selected.is_none());
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_groups_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = Some("apple".to_string());

    let items = vec![
        SelectItem::group(
            "Fruits",
            vec![
                SelectItem::option("apple", "Apple"),
                SelectItem::option("banana", "Banana"),
            ],
        ),
        SelectItem::separator(),
        SelectItem::group(
            "Vegetables",
            vec![
                SelectItem::option("carrot", "Carrot"),
                SelectItem::option_disabled("potato", "Potato (out of stock)"),
            ],
        ),
    ];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_groups", &mut selected)
                    .placeholder("Select item...")
                    .width(200.0),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert_eq!(selected, Some("apple".to_string()));
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_size_sm_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;

    let items = vec![SelectItem::option("one", "One")];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_sm", &mut selected).size(SelectSize::Sm),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.height() >= 32.0);
}

#[test]
fn select_size1_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;
    let items = vec![SelectItem::option("x", "X")];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_size1", &mut selected).size(SelectSize::Size1),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();

    assert!(inner.rect.height() > 0.0);
}

#[test]
fn select_size3_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;
    let items = vec![SelectItem::option("x", "X")];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_size3", &mut selected).size(SelectSize::Size3),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();

    assert!(inner.rect.height() >= 36.0);
}

#[test]
fn select_style_customization() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;

    let custom_style = SelectStyle {
        trigger_bg: Color32::from_rgb(30, 30, 30),
        trigger_border: Color32::from_rgb(100, 100, 100),
        ..SelectStyle::default()
    };

    let items = vec![SelectItem::option("test", "Test")];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_custom", &mut selected).style(custom_style),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_trigger_variant_ghost_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;
    let items = vec![SelectItem::option("x", "X")];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_ghost", &mut selected)
                    .trigger_variant(TriggerVariant::Ghost),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_trigger_variant_soft_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;
    let items = vec![SelectItem::option("x", "X")];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_soft", &mut selected)
                    .trigger_variant(TriggerVariant::Soft),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_trigger_variant_classic_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;
    let items = vec![SelectItem::option("x", "X")];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_classic", &mut selected)
                    .trigger_variant(TriggerVariant::Classic),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_content_variant_solid_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;
    let items = vec![SelectItem::option("x", "X")];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_solid", &mut selected)
                    .content_variant(ContentVariant::Solid),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_high_contrast_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;
    let items = vec![SelectItem::option("x", "X")];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_hc", &mut selected)
                    .high_contrast(true)
                    .content_variant(ContentVariant::Solid),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_disabled_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;
    let items = vec![SelectItem::option("x", "X")];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_disabled", &mut selected).enabled(false),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_with_preselected_value_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = Some("banana".to_string());
    let items = vec![
        SelectItem::option("apple", "Apple"),
        SelectItem::option("banana", "Banana"),
        SelectItem::option("cherry", "Cherry"),
    ];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_preselected", &mut selected),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert_eq!(selected, Some("banana".to_string()));
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_with_labels_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;
    let items = vec![
        SelectItem::label("Section A"),
        SelectItem::option("a1", "Option A1"),
        SelectItem::option("a2", "Option A2"),
        SelectItem::separator(),
        SelectItem::label("Section B"),
        SelectItem::option("b1", "Option B1"),
    ];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_labels", &mut selected),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_with_all_radius_variants() {
    init_logger();
    let ctx = egui::Context::default();
    let theme = Theme::default();
    let items = vec![SelectItem::option("x", "X")];

    for radius in [
        SelectRadius::None,
        SelectRadius::Small,
        SelectRadius::Medium,
        SelectRadius::Large,
        SelectRadius::Full,
    ] {
        ctx.begin_pass(egui::RawInput::default());
        let mut selected = None;

        let inner = egui::CentralPanel::default()
            .show(&ctx, |ui| {
                select_with_items(
                    ui,
                    &theme,
                    SelectProps::new(format!("select_radius_{:?}", radius), &mut selected)
                        .radius(radius),
                    &items,
                )
            })
            .inner;
        let _ = ctx.end_pass();
        assert!(inner.rect.width() >= 0.0);
    }
}

#[test]
fn select_with_accent_color_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;
    let items = vec![SelectItem::option("x", "X")];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_accent", &mut selected)
                    .accent_color(Color32::from_rgb(255, 100, 100)),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_empty_items_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;
    let items: Vec<SelectItem> = vec![];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_empty", &mut selected),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_nested_groups_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;

    let items = vec![SelectItem::group(
        "Level 1",
        vec![
            SelectItem::option("1a", "Option 1A"),
            SelectItem::group(
                "Level 2",
                vec![
                    SelectItem::option("2a", "Option 2A"),
                    SelectItem::option("2b", "Option 2B"),
                ],
            ),
        ],
    )];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_nested", &mut selected),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_many_options_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;

    let items: Vec<SelectItem> = (0..50)
        .map(|i| SelectItem::option(format!("opt_{}", i), format!("Option {}", i)))
        .collect();

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_many", &mut selected),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_long_labels_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut selected = None;

    let items = vec![
        SelectItem::option("short", "X"),
        SelectItem::option(
            "long",
            "This is a very long label that should handle overflow correctly",
        ),
    ];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            select_with_items(
                ui,
                &theme,
                SelectProps::new("select_long", &mut selected).width(150.0),
                &items,
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(inner.rect.width() >= 0.0);
}

#[test]
fn select_typeahead_matches_prefix_case_insensitive() {
    let items = vec![
        SelectItem::option("apple", "Apple"),
        SelectItem::option("banana", "Banana"),
        SelectItem::option_disabled("blueberry", "Blueberry"),
        SelectItem::option("cherry", "Cherry"),
    ];

    let match_a = egui_shadcn::select::find_typeahead_match(&items, "ap");
    let match_b = egui_shadcn::select::find_typeahead_match(&items, "B");
    let match_c = egui_shadcn::select::find_typeahead_match(&items, "ch");

    assert_eq!(match_a, Some(0));
    assert_eq!(match_b, Some(1));
    assert_eq!(match_c, Some(3));
}