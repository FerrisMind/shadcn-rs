use egui_shadcn::{
    Button, ButtonSize, ButtonVariant, CollapsibleContentProps, CollapsibleProps, Theme,
    collapsible, icon_chevrons_up_down,
};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn collapsible_default_open_only_opens() {
    init_logger();
    let ctx = egui::Context::default();
    let theme = Theme::default();

    let mut open = false;
    ctx.begin_pass(egui::RawInput::default());
    egui::CentralPanel::default().show(&ctx, |ui| {
        let id = ui.make_persistent_id("collapsible-default-open");
        collapsible(
            ui,
            &theme,
            CollapsibleProps::new(id, &mut open).default_open(true),
            |_ui, _api| {},
        );
    });
    let _ = ctx.end_pass();

    assert!(open, "default_open(true) should set open on first render");
}

#[test]
fn collapsible_toggle_calls_on_open_change() {
    init_logger();
    let ctx = egui::Context::default();
    let theme = Theme::default();

    let mut open = false;
    let mut called: Vec<bool> = Vec::new();
    let mut on_change = |next: bool| called.push(next);

    ctx.begin_pass(egui::RawInput::default());
    egui::CentralPanel::default().show(&ctx, |ui| {
        let id = ui.make_persistent_id("collapsible-on-change");
        collapsible(
            ui,
            &theme,
            CollapsibleProps::new(id, &mut open).on_open_change(&mut on_change),
            |ui, api| {
                assert!(!api.is_open());
                api.toggle(ui);
                assert!(api.is_open());
            },
        );
    });
    let _ = ctx.end_pass();

    assert_eq!(called, vec![true]);
}

#[test]
fn collapsible_content_force_mount_controls_mounting() {
    init_logger();
    let ctx = egui::Context::default();
    let theme = Theme::default();

    let mut open = false;
    let mut ran = 0usize;

    ctx.begin_pass(egui::RawInput::default());
    egui::CentralPanel::default().show(&ctx, |ui| {
        let id = ui.make_persistent_id("collapsible-force-mount");
        collapsible(
            ui,
            &theme,
            CollapsibleProps::new(id, &mut open),
            |ui, api| {
                let none = api.content(ui, |_| {
                    ran += 1;
                });
                assert!(none.is_none());

                let some = api.content_with_props(
                    ui,
                    CollapsibleContentProps::new().force_mount(true),
                    |_| {
                        ran += 1;
                    },
                );
                assert!(some.is_some());
            },
        );
    });
    let _ = ctx.end_pass();

    assert_eq!(ran, 1);
}

#[test]
fn collapsible_content_renders_immediately_when_animation_disabled() {
    init_logger();
    let ctx = egui::Context::default();
    let theme = Theme::default();

    let mut open = true;
    let mut ran = 0usize;

    ctx.begin_pass(egui::RawInput::default());
    egui::CentralPanel::default().show(&ctx, |ui| {
        let id = ui.make_persistent_id("collapsible-no-anim");
        collapsible(
            ui,
            &theme,
            CollapsibleProps::new(id, &mut open).with_animation(false),
            |ui, api| {
                let mounted = api.content(ui, |_| {
                    ran += 1;
                });
                assert!(mounted.is_some());
            },
        );
    });
    let _ = ctx.end_pass();

    assert_eq!(ran, 1);
}

#[test]
fn collapsible_trigger_toggles_on_click() {
    init_logger();
    let ctx = egui::Context::default();
    let theme = Theme::default();

    let mut open = false;
    let mut trigger_rect: Option<egui::Rect> = None;

    // Frame 1: layout and capture trigger rect.
    ctx.begin_pass(egui::RawInput::default());
    egui::CentralPanel::default().show(&ctx, |ui| {
        let id = ui.make_persistent_id("collapsible-click");
        collapsible(
            ui,
            &theme,
            CollapsibleProps::new(id, &mut open),
            |ui, api| {
                let resp = api.trigger(ui, |ui| ui.button("Toggle"));
                trigger_rect = Some(resp.rect);
                let _ = api.content(ui, |_| {});
            },
        );
    });
    let _ = ctx.end_pass();
    assert!(!open);

    let rect = trigger_rect.expect("trigger rect should be captured");
    let pos = rect.center();

    // Frame 2: inject a click at the trigger position.
    let mut input = egui::RawInput::default();
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
        let id = ui.make_persistent_id("collapsible-click");
        collapsible(
            ui,
            &theme,
            CollapsibleProps::new(id, &mut open),
            |ui, api| {
                let _ = api.trigger(ui, |ui| ui.button("Toggle"));
                let _ = api.content(ui, |_| {});
            },
        );
    });
    let _ = ctx.end_pass();

    assert!(open, "clicking the trigger should toggle open=true");
}

#[test]
fn collapsible_trigger_toggles_on_click_with_shadcn_button() {
    init_logger();
    let ctx = egui::Context::default();
    let theme = Theme::default();

    let mut open = false;
    let mut trigger_rect: Option<egui::Rect> = None;

    // Frame 1: layout and capture trigger rect.
    ctx.begin_pass(egui::RawInput::default());
    egui::CentralPanel::default().show(&ctx, |ui| {
        let id = ui.make_persistent_id("collapsible-click-shadcn-button");
        collapsible(
            ui,
            &theme,
            CollapsibleProps::new(id, &mut open),
            |ui, api| {
                let button = Button::new("")
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::IconSm)
                    .icon(&icon_chevrons_up_down);
                let resp = api.trigger(ui, |ui| button.show(ui, &theme));
                trigger_rect = Some(resp.rect);
                let _ = api.content(ui, |_| {});
            },
        );
    });
    let _ = ctx.end_pass();
    assert!(!open);

    let rect = trigger_rect.expect("trigger rect should be captured");
    let pos = rect.center();

    // Frame 2: inject a click at the trigger position.
    let mut input = egui::RawInput::default();
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
        let id = ui.make_persistent_id("collapsible-click-shadcn-button");
        collapsible(
            ui,
            &theme,
            CollapsibleProps::new(id, &mut open),
            |ui, api| {
                let button = Button::new("")
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::IconSm)
                    .icon(&icon_chevrons_up_down);
                let _ = api.trigger(ui, |ui| button.show(ui, &theme));
                let _ = api.content(ui, |_| {});
            },
        );
    });
    let _ = ctx.end_pass();

    assert!(
        open,
        "clicking the shadcn button trigger should toggle open=true"
    );
}
