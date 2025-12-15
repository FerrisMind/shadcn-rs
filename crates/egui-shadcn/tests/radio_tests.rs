use egui::{Context, RawInput};
use egui_shadcn::radio::{
    GridLayout, RadioCardVariant, RadioDirection, RadioGroup, RadioGroupProps, RadioOption,
};
use egui_shadcn::{ControlSize, ControlVariant, Theme};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn radio_group_props_defaults() {
    init_logger();
    let mut value = "a".to_string();
    let options = vec![
        RadioOption::new("a".to_string(), "A"),
        RadioOption::new("b".to_string(), "B"),
    ];
    let props = RadioGroupProps::new("group", &mut value, &options);
    assert_eq!(props.size, ControlSize::Md);
    assert_eq!(props.variant, ControlVariant::Primary);
    assert_eq!(props.direction, RadioDirection::Vertical);
    assert_eq!(props.card_variant, RadioCardVariant::Button);
    assert!(props.grid_layout.is_none());
    assert!(!props.disabled);
    assert!(!props.high_contrast);
    assert!(!props.show_separators);
    assert!(props.name.is_none());
    assert!(!props.required);
    assert!(props.dir.is_none());
    assert!(props.default_value.is_none());
    assert!(props.on_value_change.is_none());
    assert!(props.loop_focus);
    assert!(!props.as_child);
}

#[test]
fn radio_group_renders() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();
    let mut value = "option_1".to_string();
    let options = vec![
        RadioOption::new("option_1".to_string(), "Option 1"),
        RadioOption::new("option_2".to_string(), "Option 2").description("Secondary label"),
    ];

    let response = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            RadioGroup::new("radio_demo", &mut value, &options)
                .size(ControlSize::Sm)
                .variant(ControlVariant::Secondary)
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(response.rect.width() >= 0.0);
}

#[test]
fn radio_option_builder_sets_description_and_disabled() {
    init_logger();
    let option = RadioOption::new(1, "One")
        .description("Desc")
        .disabled(true);
    assert!(option.description.is_some());
    assert!(option.disabled);
}

#[test]
fn radio_option_builder_sets_icon_and_color() {
    init_logger();
    let option = RadioOption::new(1, "One")
        .icon("📦")
        .accent_color(egui::Color32::LIGHT_BLUE);
    assert!(option.icon.is_some());
    assert!(option.accent_color.is_some());
}

#[test]
fn radio_option_supports_required_and_as_child() {
    init_logger();
    let option = RadioOption::new("value", "Label")
        .required(true)
        .as_child(true);
    assert!(option.required);
    assert!(option.as_child);
}

#[test]
fn radio_group_card_variant() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();
    let mut value = "card_1".to_string();
    let options = vec![
        RadioOption::new("card_1".to_string(), "Card 1").description("First card"),
        RadioOption::new("card_2".to_string(), "Card 2").description("Second card"),
    ];

    let response = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            RadioGroup::new("card_demo", &mut value, &options)
                .card_variant(RadioCardVariant::Card)
                .grid_layout(GridLayout::new(2))
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(response.rect.width() >= 0.0);
}

#[test]
fn radio_group_with_grid_layout() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();
    let mut value = "grid_1".to_string();
    let options = vec![
        RadioOption::new("grid_1".to_string(), "Grid 1"),
        RadioOption::new("grid_2".to_string(), "Grid 2"),
        RadioOption::new("grid_3".to_string(), "Grid 3"),
        RadioOption::new("grid_4".to_string(), "Grid 4"),
    ];

    let response = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            RadioGroup::new("grid_demo", &mut value, &options)
                .card_variant(RadioCardVariant::Card)
                .grid_layout(GridLayout::new(2).with_spacing(12.0))
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(response.rect.width() >= 0.0);
}

#[test]
fn radio_group_horizontal_with_separators() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();
    let mut value = "h_1".to_string();
    let options = vec![
        RadioOption::new("h_1".to_string(), "Horizontal 1"),
        RadioOption::new("h_2".to_string(), "Horizontal 2"),
        RadioOption::new("h_3".to_string(), "Horizontal 3"),
    ];

    let response = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            RadioGroup::new("horiz_demo", &mut value, &options)
                .direction(RadioDirection::Horizontal)
                .show_separators(true)
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(response.rect.width() >= 0.0);
}

#[test]
fn radio_group_default_value_applied_once_and_callback_fires() {
    init_logger();
    let ctx = Context::default();
    let theme = Theme::default();
    let mut value = String::new();
    let options = vec![
        RadioOption::new("one".to_string(), "One"),
        RadioOption::new("two".to_string(), "Two"),
    ];
    let mut callback_calls = 0usize;
    let default_value = "two".to_string();

    ctx.begin_pass(RawInput::default());
    egui::CentralPanel::default()
        .show(&ctx, |ui| {
            RadioGroup::new("default_demo", &mut value, &options)
                .default_value(default_value.clone())
                .on_value_change(|new_val| {
                    callback_calls += 1;
                    assert_eq!(new_val, &default_value);
                })
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert_eq!(value, default_value);
    assert_eq!(callback_calls, 1);

    value = "one".to_string();
    ctx.begin_pass(RawInput::default());
    egui::CentralPanel::default()
        .show(&ctx, |ui| {
            RadioGroup::new("default_demo", &mut value, &options)
                .default_value("ignored".to_string())
                .show(ui, &theme)
        })
        .inner;
    let _ = ctx.end_pass();
    assert_eq!(value, "one");
    assert_eq!(callback_calls, 1);
}

#[test]
fn radio_tokens_high_contrast_adjusts_colors() {
    let palette = egui_shadcn::tokens::ColorPalette::default();
    let normal = egui_shadcn::tokens::checkbox_tokens(&palette, ControlVariant::Primary);
    let high = egui_shadcn::tokens::checkbox_tokens_with_high_contrast(
        &palette,
        ControlVariant::Primary,
        true,
    );

    assert_ne!(normal.on.idle.bg_fill, high.on.idle.bg_fill);
    assert_ne!(normal.off.idle.bg_fill, high.off.idle.bg_fill);
}
