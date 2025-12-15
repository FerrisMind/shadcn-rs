use egui::{Context, Id, RawInput};
use egui_shadcn::Theme;
use egui_shadcn::tabs::{
    TabItem, TabsActivationMode, TabsContentForceMount, TabsDirection, TabsListLoop, TabsProps,
    TabsVariant, tabs,
};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

fn sample_items() -> Vec<TabItem> {
    vec![
        TabItem::new("tab1", "Tab 1"),
        TabItem::new("tab2", "Tab 2"),
        TabItem::new("tab3", "Tab 3"),
    ]
}

#[test]
fn tabs_props_defaults_match_radix_root() {
    init_logger();
    let mut active = "tab1".to_string();
    let items = sample_items();
    let id = Id::new("tabs_defaults");
    let props = TabsProps::new(id, &items, &mut active);

    assert_eq!(props.orientation, TabsDirection::Horizontal);
    assert_eq!(props.activation_mode, TabsActivationMode::Automatic);
    assert!(props.dir.is_none());
    assert!(props.default_value.is_none());
    assert!(props.on_value_change.is_none());
    assert_eq!(props.list_loop, TabsListLoop::Enabled);
    assert!(!props.root_as_child);
    assert!(!props.list_as_child);
    assert!(!props.trigger_as_child);
    assert_eq!(props.content_force_mount, TabsContentForceMount::Off);
    assert!(!props.content_as_child);

    assert_eq!(props.variant, TabsVariant::Underline);
}

#[test]
fn tabs_default_value_applies_once_and_calls_callback() {
    init_logger();
    let mut active = String::new();
    let items = sample_items();
    let id = Id::new("tabs_default_value");
    let mut calls = 0usize;
    let default_value = "tab2".to_string();

    let ctx = Context::default();
    let theme = Theme::default();

    ctx.begin_pass(RawInput::default());
    egui::CentralPanel::default()
        .show(&ctx, |ui| {
            tabs(
                ui,
                &theme,
                TabsProps::new(id, &items, &mut active)
                    .with_default_value(default_value.clone())
                    .with_on_value_change(|val| {
                        calls += 1;
                        assert_eq!(val, &default_value);
                    }),
                |_ui, _tab| {},
            )
        })
        .inner;
    let _ = ctx.end_pass();

    assert_eq!(active, default_value);
    assert_eq!(calls, 1);

    active = "tab1".to_string();
    ctx.begin_pass(RawInput::default());
    egui::CentralPanel::default()
        .show(&ctx, |ui| {
            tabs(
                ui,
                &theme,
                TabsProps::new(id, &items, &mut active)
                    .with_default_value("should_not_apply".to_string())
                    .with_on_value_change(|_val| {
                        calls += 1;
                    }),
                |_ui, _tab| {},
            )
        })
        .inner;
    let _ = ctx.end_pass();

    assert_eq!(active, "tab1");
    assert_eq!(calls, 1);
}
