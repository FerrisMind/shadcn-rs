use egui_shadcn::{
    CardProps, CardSize, CardVariant, ControlSize, ControlVariant, DialogProps, DialogSize,
    PopoverAlign, PopoverCollisionPadding, PopoverPlacement, PopoverProps, PopoverSide,
    PopoverSticky, PopoverUpdatePositionStrategy, ScrollAreaProps, ScrollDirection,
    SeparatorOrientation, SeparatorProps, TabItem, TabsOrientation, TabsProps, TabsVariant, Theme,
    button, card, card::card_tokens, dialog, dialog::compute_dialog_rect,
    dialog::dialog_layout_tokens, popover, popover::compute_popover_rect, scroll_area, separator,
    tabs,
};
fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn card_and_separator_render() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            let card_resp = card(
                ui,
                &theme,
                CardProps::default()
                    .with_heading("Card")
                    .with_description("Structural container")
                    .with_variant(CardVariant::Surface),
                |card_ui| {
                    card_ui.label("Inside");
                },
            );
            let sep_resp = separator(
                ui,
                &theme,
                SeparatorProps {
                    orientation: SeparatorOrientation::Horizontal,
                    ..SeparatorProps::default()
                },
            );
            (card_resp, sep_resp)
        })
        .inner;
    let _ = ctx.end_pass();

    assert!(inner.0.rect.width() >= 0.0);
    assert!(inner.1.rect.width() >= 0.0);
}

#[test]
fn tabs_switch_and_scroll() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let mut active = "one".to_string();
    let tabs_items = vec![TabItem::new("one", "One"), TabItem::new("two", "Two")];

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            let tabs_result = tabs(
                ui,
                &theme,
                TabsProps::new(ui.make_persistent_id("tabs"), &tabs_items, &mut active)
                    .with_variant(TabsVariant::Soft)
                    .with_orientation(TabsOrientation::Horizontal)
                    .compact(false),
                |content_ui, active_tab| {
                    content_ui.label(format!("Content for {}", active_tab.id));
                },
            );

            scroll_area(
                ui,
                &theme,
                ScrollAreaProps::default().with_direction(ScrollDirection::Vertical),
                |scroll_ui| {
                    for _ in 0..3 {
                        scroll_ui.label("Line");
                    }
                },
            );

            (tabs_result.bar_response, ())
        })
        .inner;
    let _ = ctx.end_pass();

    assert_eq!(active, "one".to_string());
    assert!(inner.0.rect.width() >= 0.0);
    assert_eq!(inner.1, ());
}

#[test]
fn tabs_arrow_key_navigation_skips_disabled() {
    init_logger();
    let ctx = egui::Context::default();
    let theme = Theme::default();
    let mut active = "one".to_string();
    let tabs_items = vec![
        TabItem::new("one", "One"),
        TabItem::new("two", "Two").disabled(true),
        TabItem::new("three", "Three"),
    ];

    ctx.begin_pass(egui::RawInput::default());
    egui::CentralPanel::default().show(&ctx, |ui| {
        let id = ui.make_persistent_id("tabs-nav");
        ui.memory_mut(|m| m.request_focus(id.with("trigger").with("one")));
        let _ = tabs(
            ui,
            &theme,
            TabsProps::new(id, &tabs_items, &mut active),
            |_content_ui, _active_tab| {},
        );
    });
    let _ = ctx.end_pass();

    let mut input = egui::RawInput::default();
    input.events.push(egui::Event::Key {
        key: egui::Key::ArrowRight,
        physical_key: Some(egui::Key::ArrowRight),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    });

    ctx.begin_pass(input);
    egui::CentralPanel::default().show(&ctx, |ui| {
        let id = ui.make_persistent_id("tabs-nav");
        let _ = tabs(
            ui,
            &theme,
            TabsProps::new(id, &tabs_items, &mut active),
            |_content_ui, _active_tab| {},
        );
    });
    let _ = ctx.end_pass();

    assert_eq!(
        active,
        "three".to_string(),
        "arrow navigation should skip disabled tabs"
    );
}

#[test]
fn popover_and_dialog_open_close() {
    init_logger();
    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();
    let mut popover_open = true;
    let mut dialog_open = true;

    let inner = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            let (trigger_resp, pop_content) = popover(
                ui,
                &theme,
                PopoverProps::new(ui.make_persistent_id("popover"), &mut popover_open)
                    .with_placement(PopoverPlacement::Below)
                    .with_width(240.0),
                |trigger_ui| {
                    button(
                        trigger_ui,
                        &theme,
                        "Open popover",
                        ControlVariant::Secondary,
                        ControlSize::Md,
                        true,
                    )
                },
                |content_ui| {
                    content_ui.label("Popover body");
                },
            );

            let dialog_content = dialog(
                ui,
                &theme,
                DialogProps::new(ui.make_persistent_id("dialog"), &mut dialog_open)
                    .with_title("Dialog")
                    .with_description("Modal with overlay"),
                |content_ui| {
                    content_ui.label("Dialog body");
                },
            );

            (
                trigger_resp,
                pop_content.is_some(),
                dialog_content.is_some(),
            )
        })
        .inner;
    let _ = ctx.end_pass();

    assert!(inner.0.rect.width() >= 0.0);
    assert!(inner.1 || !popover_open);
    assert!(inner.2 || !dialog_open);
}

#[test]
fn popover_does_not_close_on_open_click_same_frame() {
    init_logger();
    let ctx = egui::Context::default();
    let theme = Theme::default();
    let mut popover_open = false;
    let mut trigger_rect = egui::Rect::NOTHING;

    ctx.begin_pass(egui::RawInput::default());
    egui::CentralPanel::default().show(&ctx, |ui| {
        let (trigger_resp, _) = popover(
            ui,
            &theme,
            PopoverProps::new(ui.make_persistent_id("popover-click"), &mut popover_open),
            |trigger_ui| {
                button(
                    trigger_ui,
                    &theme,
                    "Open popover",
                    ControlVariant::Secondary,
                    ControlSize::Md,
                    true,
                )
            },
            |_content_ui| (),
        );
        trigger_rect = trigger_resp.rect;
    });
    let _ = ctx.end_pass();
    assert!(!popover_open, "popover should start closed");

    let mut input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::pos2(0.0, 0.0),
            egui::vec2(1024.0, 768.0),
        )),
        ..Default::default()
    };
    let pos = trigger_rect.center();
    input.events.push(egui::Event::PointerMoved(pos));
    input.events.push(egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed: true,
        modifiers: egui::Modifiers::NONE,
    });
    input.events.push(egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::NONE,
    });

    ctx.begin_pass(input);
    egui::CentralPanel::default().show(&ctx, |ui| {
        let _ = popover(
            ui,
            &theme,
            PopoverProps::new(ui.make_persistent_id("popover-click"), &mut popover_open),
            |trigger_ui| {
                button(
                    trigger_ui,
                    &theme,
                    "Open popover",
                    ControlVariant::Secondary,
                    ControlSize::Md,
                    true,
                )
            },
            |_content_ui| (),
        );
    });
    let _ = ctx.end_pass();

    assert!(
        popover_open,
        "popover should remain open after opening click"
    );
}

#[test]
fn dialog_respects_close_on_escape_flag() {
    init_logger();
    let mut dialog_open = true;
    let mut input = egui::RawInput::default();
    input.events.push(egui::Event::Key {
        key: egui::Key::Escape,
        physical_key: Some(egui::Key::Escape),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    });

    let ctx = egui::Context::default();
    ctx.begin_pass(input);
    egui::CentralPanel::default().show(&ctx, |ui| {
        let _ = dialog(
            ui,
            &Theme::default(),
            DialogProps::new(ui.make_persistent_id("dialog-escape"), &mut dialog_open)
                .with_close_on_escape(true)
                .with_close_on_background(false),
            |content_ui| {
                content_ui.label("Body");
            },
        );
    });
    let _ = ctx.end_pass();
    assert!(
        !dialog_open,
        "escape should close dialog when flag is enabled"
    );

    let mut dialog_open_blocked = true;
    let mut input_again = egui::RawInput::default();
    input_again.events.push(egui::Event::Key {
        key: egui::Key::Escape,
        physical_key: Some(egui::Key::Escape),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    });
    let ctx2 = egui::Context::default();
    ctx2.begin_pass(input_again);
    egui::CentralPanel::default().show(&ctx2, |ui| {
        let _ = dialog(
            ui,
            &Theme::default(),
            DialogProps::new(
                ui.make_persistent_id("dialog-escape-off"),
                &mut dialog_open_blocked,
            )
            .with_close_on_escape(false)
            .with_close_on_background(false),
            |content_ui| {
                content_ui.label("Body");
            },
        );
    });
    let _ = ctx2.end_pass();
    assert!(
        dialog_open_blocked,
        "escape must be ignored when close_on_escape is false"
    );
}

#[test]
fn popover_geometry_aligns_and_clamps() {
    let screen = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(400.0, 320.0));
    let trigger = egui::Rect::from_min_size(egui::pos2(360.0, 150.0), egui::vec2(32.0, 20.0));

    let clamped = compute_popover_rect(
        trigger,
        screen,
        PopoverPlacement::Below,
        PopoverAlign::End,
        8.0,
        0.0,
        220.0,
        180.0,
        true,
    );
    assert!(
        clamped.right() <= screen.right() && clamped.left() >= screen.left(),
        "popover should clamp to screen bounds when constrain_to_screen = true"
    );

    let centered = compute_popover_rect(
        trigger,
        screen,
        PopoverPlacement::Above,
        PopoverAlign::Center,
        4.0,
        6.0,
        120.0,
        120.0,
        false,
    );
    let expected_center_x = trigger.center().x + 6.0;
    assert!(
        (centered.center().x - expected_center_x).abs() < 0.5,
        "align_offset should shift popover center"
    );

    let left_side = compute_popover_rect(
        trigger,
        screen,
        PopoverPlacement::Left,
        PopoverAlign::Center,
        8.0,
        0.0,
        220.0,
        180.0,
        true,
    );
    assert!(
        left_side.right() <= trigger.left() + 0.5,
        "left placement should place popover to the left of trigger when space allows"
    );

    let right_side = compute_popover_rect(
        trigger,
        screen,
        PopoverPlacement::Right,
        PopoverAlign::Center,
        8.0,
        0.0,
        220.0,
        180.0,
        true,
    );
    assert!(
        right_side.right() <= screen.right() && right_side.left() >= screen.left(),
        "right placement should clamp within screen bounds"
    );
}

#[test]
fn card_variants_produce_distinct_tokens() {
    let palette = Theme::default().palette;
    let surface = card_tokens(&palette, CardVariant::Surface, true);
    let classic = card_tokens(&palette, CardVariant::Classic, false);
    let ghost = card_tokens(&palette, CardVariant::Ghost, true);

    assert_ne!(
        surface.background, classic.background,
        "classic should differ from surface background"
    );
    assert!(
        classic.stroke.width >= surface.stroke.width,
        "classic stroke should be equal or stronger than surface"
    );
    assert!(
        ghost.stroke.width <= classic.stroke.width,
        "ghost should keep light stroke"
    );
    assert!(surface.shadow_alpha >= ghost.shadow_alpha);
}

#[test]
fn card_size_changes_padding_and_rounding_defaults() {
    let small = CardProps::default().with_size(CardSize::Size1);
    let large = CardProps::default().with_size(CardSize::Size5);

    assert!(
        large.padding.x > small.padding.x && large.padding.y > small.padding.y,
        "larger sizes should increase padding"
    );
    assert_ne!(large.rounding, small.rounding);
}

#[test]
fn card_high_contrast_strengthens_surface_border() {
    let palette = Theme::default().palette;
    let normal = egui_shadcn::card_tokens_with_options(
        &palette,
        CardVariant::Surface,
        true,
        CardSize::Size2,
        false,
    );
    let high_contrast = egui_shadcn::card_tokens_with_options(
        &palette,
        CardVariant::Surface,
        true,
        CardSize::Size2,
        true,
    );

    assert!(
        high_contrast.stroke.width >= normal.stroke.width,
        "high contrast should not weaken border"
    );
}

#[test]
fn dialog_geometry_centers_and_clamps() {
    let screen = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(800.0, 600.0));

    let centered = compute_dialog_rect(
        screen,
        Some(700.0),
        None,
        Some(600.0),
        None,
        None,
        None,
        egui::Vec2::ZERO,
        egui_shadcn::DialogAlign::Center,
    );
    assert!(
        (centered.center().x - screen.center().x).abs() < 0.5
            && (centered.center().y - screen.center().y).abs() < 0.5,
        "center align should center dialog"
    );
    assert!(
        centered.width() <= 600.0 + 0.5,
        "max_width default/override should clamp width"
    );

    let start = compute_dialog_rect(
        screen,
        Some(480.0),
        None,
        None,
        None,
        None,
        None,
        egui::vec2(10.0, 12.0),
        egui_shadcn::DialogAlign::Start,
    );
    assert!(
        start.left() >= screen.left() + 10.0,
        "start align should be near top-left with offset"
    );
}

#[test]
fn dialog_size_tokens_increase_padding() {
    let theme = Theme::default();
    let small = dialog_layout_tokens(&theme, DialogSize::Size1);
    let large = dialog_layout_tokens(&theme, DialogSize::Size4);

    assert!(
        large.padding.left > small.padding.left && large.padding.top > small.padding.top,
        "larger dialog sizes should increase padding"
    );
    assert_ne!(small.rounding, large.rounding);
}

#[test]
fn popover_api_surface_matches_radix_reference() {
    init_logger();

    let ctx = egui::Context::default();
    ctx.begin_pass(egui::RawInput::default());
    let theme = Theme::default();

    let mut open = false;
    let mut open_changes = Vec::<bool>::new();
    let mut on_open_change = |value: bool| open_changes.push(value);

    let id_source = egui::Id::new("popover-api");
    let _ = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            let (trigger_resp, _inner) = popover(
                ui,
                &theme,
                PopoverProps::new(id_source, &mut open)
                    .default_open(true)
                    .on_open_change(&mut on_open_change)
                    .modal(false)
                    .side(PopoverSide::Bottom)
                    .side_offset(5.0)
                    .align(PopoverAlign::Center)
                    .align_offset(2.0)
                    .avoid_collisions(true)
                    .collision_padding(PopoverCollisionPadding::all(8.0))
                    .sticky(PopoverSticky::Partial)
                    .hide_when_detached(false)
                    .update_position_strategy(PopoverUpdatePositionStrategy::Optimized)
                    .force_mount(false),
                |trigger_ui| {
                    button(
                        trigger_ui,
                        &theme,
                        "Trigger",
                        ControlVariant::Outline,
                        ControlSize::Md,
                        true,
                    )
                },
                |_content_ui| {},
            );
            trigger_resp
        })
        .inner;

    let _ = ctx.end_pass();

    assert!(
        open_changes.is_empty(),
        "no implicit open change on first render"
    );
}
