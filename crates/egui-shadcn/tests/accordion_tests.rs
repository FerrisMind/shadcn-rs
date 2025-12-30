//! Tests for Accordion component
//!
//! TDD test suite based on Radix Accordion API.

use egui_shadcn::Theme;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

// =============================================================================
// AccordionType enum tests
// =============================================================================

#[test]
fn accordion_type_single_is_default() {
    use egui_shadcn::AccordionType;
    assert_eq!(AccordionType::default(), AccordionType::Single);
}

#[test]
fn accordion_type_has_both_variants() {
    use egui_shadcn::AccordionType;
    let _single = AccordionType::Single;
    let _multiple = AccordionType::Multiple;
}

// =============================================================================
// AccordionProps tests
// =============================================================================

#[test]
fn accordion_props_default_values() {
    use egui_shadcn::AccordionProps;

    let mut value: Option<String> = None;
    let props = AccordionProps::new("test-id", &mut value);

    assert!(!props.collapsible, "collapsible defaults to false");
    assert!(!props.disabled, "disabled defaults to false");
    assert!(props.animate, "animate defaults to true");
}

#[test]
fn accordion_props_builder_methods() {
    use egui_shadcn::{AccordionProps, AccordionType};

    let mut value: Option<String> = None;
    let props = AccordionProps::new("test-id", &mut value)
        .accordion_type(AccordionType::Multiple)
        .collapsible(true)
        .disabled(true)
        .default_value("item-1")
        .animate(false)
        .animation_ms(300.0);

    assert!(props.collapsible);
    assert!(props.disabled);
    assert!(!props.animate);
    assert_eq!(props.animation_ms, Some(300.0));
}

// =============================================================================
// AccordionItemProps tests
// =============================================================================

#[test]
fn accordion_item_props_default() {
    use egui_shadcn::AccordionItemProps;

    let props = AccordionItemProps::new("item-1");
    assert_eq!(props.value, "item-1");
    assert!(!props.disabled);
}

#[test]
fn accordion_item_props_disabled() {
    use egui_shadcn::AccordionItemProps;

    let props = AccordionItemProps::new("item-1").disabled(true);
    assert!(props.disabled);
}

// =============================================================================
// AccordionState tests
// =============================================================================

#[test]
fn accordion_state_single_mode_allows_one_open() {
    use egui_shadcn::AccordionState;

    let mut state = AccordionState::single(None);

    // Open first item
    state.toggle("item-1", false);
    assert!(state.is_open("item-1"));
    assert!(!state.is_open("item-2"));

    // Open second item should close first
    state.toggle("item-2", false);
    assert!(!state.is_open("item-1"));
    assert!(state.is_open("item-2"));
}

#[test]
fn accordion_state_single_collapsible_allows_close() {
    use egui_shadcn::AccordionState;

    let mut state = AccordionState::single(Some("item-1".to_string()));
    assert!(state.is_open("item-1"));

    // With collapsible=true, clicking open item should close it
    state.toggle("item-1", true);
    assert!(!state.is_open("item-1"));
}

#[test]
fn accordion_state_single_non_collapsible_keeps_one_open() {
    use egui_shadcn::AccordionState;

    let mut state = AccordionState::single(Some("item-1".to_string()));
    assert!(state.is_open("item-1"));

    // With collapsible=false, clicking open item should keep it open
    state.toggle("item-1", false);
    assert!(state.is_open("item-1"));
}

#[test]
fn accordion_state_multiple_mode_allows_many_open() {
    use egui_shadcn::AccordionState;

    let mut state = AccordionState::multiple(vec![]);

    state.toggle("item-1", true);
    state.toggle("item-2", true);

    assert!(state.is_open("item-1"));
    assert!(state.is_open("item-2"));

    // Toggle off one
    state.toggle("item-1", true);
    assert!(!state.is_open("item-1"));
    assert!(state.is_open("item-2"));
}

#[test]
fn accordion_state_multiple_default_values() {
    use egui_shadcn::AccordionState;

    let state = AccordionState::multiple(vec!["item-1".to_string(), "item-3".to_string()]);

    assert!(state.is_open("item-1"));
    assert!(!state.is_open("item-2"));
    assert!(state.is_open("item-3"));
}

// =============================================================================
// Accordion rendering tests
// =============================================================================

#[test]
fn accordion_renders_without_panic() {
    init_logger();
    use egui_shadcn::{accordion, AccordionProps};

    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let mut value: Option<String> = None;

    let _ = egui::CentralPanel::default().show(&ctx, |ui| {
        accordion(
            ui,
            &theme,
            AccordionProps::new("test-accordion", &mut value),
            |ui, _ctx| {
                ui.label("Empty accordion");
            },
        )
    });

    let _ = ctx.end_pass();
}

#[test]
fn accordion_with_items_renders() {
    init_logger();
    use egui_shadcn::{accordion, accordion_item, AccordionItemProps, AccordionProps};

    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let mut value: Option<String> = Some("item-1".to_string());

    let _ = egui::CentralPanel::default().show(&ctx, |ui| {
        accordion(
            ui,
            &theme,
            AccordionProps::new("test-accordion", &mut value),
            |ui, acc_ctx| {
                accordion_item(
                    ui,
                    &theme,
                    acc_ctx,
                    AccordionItemProps::new("item-1"),
                    |ui, item_ctx| {
                        let response = ui.label(format!("Trigger 1 (open={})", item_ctx.is_open));
                        response
                    },
                    |ui| {
                        ui.label("Content 1");
                    },
                );

                accordion_item(
                    ui,
                    &theme,
                    acc_ctx,
                    AccordionItemProps::new("item-2"),
                    |ui, item_ctx| {
                        let response = ui.label(format!("Trigger 2 (open={})", item_ctx.is_open));
                        response
                    },
                    |ui| {
                        ui.label("Content 2");
                    },
                );
            },
        )
    });

    let _ = ctx.end_pass();

    // After rendering, item-1 should still be open
    assert_eq!(value, Some("item-1".to_string()));
}

#[test]
fn accordion_default_value_applied() {
    init_logger();
    use egui_shadcn::{accordion, AccordionProps};

    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let mut value: Option<String> = None;

    let _ = egui::CentralPanel::default().show(&ctx, |ui| {
        accordion(
            ui,
            &theme,
            AccordionProps::new("test-default", &mut value).default_value("default-item"),
            |ui, _ctx| {
                ui.label("Test");
            },
        )
    });

    let _ = ctx.end_pass();

    // Default value should be applied
    assert_eq!(value, Some("default-item".to_string()));
}
