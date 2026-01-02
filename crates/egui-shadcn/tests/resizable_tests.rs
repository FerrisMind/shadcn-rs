//! Tests for Resizable panel component
//!
//! TDD test suite for resizable panel groups.

use egui_shadcn::Theme;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

// =============================================================================
// ResizableDirection enum tests
// =============================================================================

#[test]
fn resizable_direction_horizontal_is_default() {
    use egui_shadcn::ResizableDirection;
    assert_eq!(
        ResizableDirection::default(),
        ResizableDirection::Horizontal
    );
}

#[test]
fn resizable_direction_has_both_variants() {
    use egui_shadcn::ResizableDirection;
    let _h = ResizableDirection::Horizontal;
    let _v = ResizableDirection::Vertical;
}

// =============================================================================
// ResizablePanelGroupProps tests
// =============================================================================

#[test]
fn resizable_panel_group_props_default() {
    use egui_shadcn::{ResizableDirection, ResizablePanelGroupProps};

    let props = ResizablePanelGroupProps::new("test-group");
    assert_eq!(props.direction, ResizableDirection::Horizontal);
    assert!(props.auto_save_id.is_none());
}

#[test]
fn resizable_panel_group_props_builder() {
    use egui_shadcn::{ResizableDirection, ResizablePanelGroupProps};

    let props = ResizablePanelGroupProps::new("test-group")
        .direction(ResizableDirection::Vertical)
        .auto_save_id("my-layout");

    assert_eq!(props.direction, ResizableDirection::Vertical);
    assert_eq!(props.auto_save_id, Some("my-layout".to_string()));
}

// =============================================================================
// ResizablePanelProps tests
// =============================================================================

#[test]
fn resizable_panel_props_default() {
    use egui_shadcn::ResizablePanelProps;

    let props = ResizablePanelProps::new(50.0);
    assert_eq!(props.default_size, 50.0);
    assert!(props.min_size.is_none());
    assert!(props.max_size.is_none());
    assert!(!props.collapsible);
}

#[test]
fn resizable_panel_props_builder() {
    use egui_shadcn::ResizablePanelProps;

    let props = ResizablePanelProps::new(30.0)
        .min_size(10.0)
        .max_size(80.0)
        .collapsible(true);

    assert_eq!(props.default_size, 30.0);
    assert_eq!(props.min_size, Some(10.0));
    assert_eq!(props.max_size, Some(80.0));
    assert!(props.collapsible);
}

#[test]
fn resizable_panel_props_clamps_values() {
    use egui_shadcn::ResizablePanelProps;

    // Size should be clamped to 0-100 range
    let props = ResizablePanelProps::new(150.0);
    assert_eq!(props.default_size, 100.0);

    let props = ResizablePanelProps::new(-10.0);
    assert_eq!(props.default_size, 0.0);
}

// =============================================================================
// ResizableHandleProps tests
// =============================================================================

#[test]
fn resizable_handle_props_default() {
    use egui_shadcn::ResizableHandleProps;

    let props = ResizableHandleProps::new();
    assert!(!props.with_handle);
    assert!(!props.disabled);
}

#[test]
fn resizable_handle_props_with_grip() {
    use egui_shadcn::ResizableHandleProps;

    let props = ResizableHandleProps::new().with_handle(true);
    assert!(props.with_handle);
}

// =============================================================================
// Rendering tests
// =============================================================================

#[test]
fn resizable_panel_group_renders() {
    init_logger();
    use egui_shadcn::{ResizablePanelGroupProps, resizable_panel_group};

    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let mut sizes = vec![50.0, 50.0];

    let _ = egui::CentralPanel::default().show(&ctx, |ui| {
        resizable_panel_group(
            ui,
            &theme,
            ResizablePanelGroupProps::new("test-group"),
            &mut sizes,
            |ui, _ctx| {
                ui.label("Panel content");
            },
        )
    });

    let _ = ctx.end_pass();
}

#[test]
fn resizable_with_two_panels_renders() {
    init_logger();
    use egui_shadcn::{
        ResizableHandleProps, ResizablePanelGroupProps, ResizablePanelProps, resizable_handle,
        resizable_panel, resizable_panel_group,
    };

    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let mut sizes = vec![50.0, 50.0];

    let _ = egui::CentralPanel::default().show(&ctx, |ui| {
        resizable_panel_group(
            ui,
            &theme,
            ResizablePanelGroupProps::new("two-panels"),
            &mut sizes,
            |ui, ctx| {
                resizable_panel(ui, ctx, ResizablePanelProps::new(50.0), 0, |ui| {
                    ui.label("Left panel");
                });

                resizable_handle(ui, &theme, ctx, ResizableHandleProps::new(), 0);

                resizable_panel(ui, ctx, ResizablePanelProps::new(50.0), 1, |ui| {
                    ui.label("Right panel");
                });
            },
        )
    });

    let _ = ctx.end_pass();

    // Sizes should be preserved
    assert_eq!(sizes.len(), 2);
}

#[test]
fn resizable_sizes_respect_min_max() {
    use egui_shadcn::ResizablePanelProps;

    let props = ResizablePanelProps::new(50.0).min_size(20.0).max_size(80.0);

    // Test clamping logic
    let clamped = props.clamp_size(10.0);
    assert_eq!(clamped, 20.0);

    let clamped = props.clamp_size(90.0);
    assert_eq!(clamped, 80.0);

    let clamped = props.clamp_size(50.0);
    assert_eq!(clamped, 50.0);
}
