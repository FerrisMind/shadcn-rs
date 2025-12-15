use egui::{Color32, Context, RawInput};
use egui_shadcn::tokens::ColorPalette;
use egui_shadcn::tooltip::{
    TooltipAlign, TooltipAnimationState, TooltipCollisionPadding, TooltipOpenState,
    TooltipPosition, TooltipProps, TooltipSide, TooltipState, TooltipSticky, TooltipStyle,
};
use egui_shadcn::{Theme, tooltip};

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn tooltip_style_from_palette_has_colors() {
    init_logger();
    let palette = ColorPalette::default();
    let style = TooltipStyle::from_palette(&palette, false);
    assert_ne!(style.bg, Color32::TRANSPARENT);
    assert_ne!(style.text, Color32::TRANSPARENT);
}

#[test]
fn tooltip_does_not_show_without_hover() {
    init_logger();
    let ctx = Context::default();
    ctx.begin_pass(RawInput::default());
    let theme = Theme::default();

    let shown = egui::CentralPanel::default()
        .show(&ctx, |ui| {
            let response = ui.label("anchor");
            tooltip(
                &response,
                ui,
                &theme,
                TooltipProps::new("Details").position(TooltipPosition::Bottom),
            )
        })
        .inner;
    let _ = ctx.end_pass();
    assert!(!shown);
}

#[test]
fn tooltip_props_builder_pattern_works() {
    let props = TooltipProps::new("Test tooltip")
        .delay_ms(300)
        .max_width(400.0)
        .position(TooltipPosition::Top)
        .show_arrow(true);

    assert_eq!(props.delay_ms, 300);
    assert_eq!(props.max_width, 400.0);
    assert_eq!(props.position, TooltipPosition::Top);
    assert!(props.show_arrow);
}

#[test]
fn tooltip_side_and_align_configuration() {
    let props = TooltipProps::new("Content")
        .side(TooltipSide::Top)
        .align(TooltipAlign::Center);

    assert_eq!(props.side, TooltipSide::Top);
    assert_eq!(props.align, TooltipAlign::Center);
}

#[test]
fn tooltip_collision_padding_applied() {
    let props = TooltipProps::new("Test").collision_padding(15.0);

    assert_eq!(props.collision_padding, TooltipCollisionPadding::all(15.0));
}

#[test]
fn tooltip_default_values() {
    let props = TooltipProps::new("Test");
    assert_eq!(props.delay_ms, 700);
    assert_eq!(props.max_width, 360.0);
    assert_eq!(props.position, TooltipPosition::Top);
    assert_eq!(props.side_offset, 4.0);
    assert!(!props.show_arrow);
    assert!(!props.force_mount);
}

#[test]
fn tooltip_side_offset_configuration() {
    let props = TooltipProps::new("Test").side_offset(8.0);

    assert_eq!(props.side_offset, 8.0);
}

#[test]
fn tooltip_skip_delay_configuration() {
    let props = TooltipProps::new("Test").skip_delay_ms(500);

    assert_eq!(props.skip_delay_ms, 500);
}

#[test]
fn tooltip_force_mount() {
    let props = TooltipProps::new("Test").force_mount(true);

    assert!(props.force_mount);
}

#[test]
fn tooltip_arrow_configuration() {
    let props = TooltipProps::new("Test")
        .show_arrow(true)
        .arrow_size(15.0, 7.0);

    assert!(props.show_arrow);
    assert_eq!(props.arrow_width, 15.0);
    assert_eq!(props.arrow_height, 7.0);
}

#[test]
fn tooltip_all_sides() {
    let sides = vec![
        TooltipSide::Top,
        TooltipSide::Right,
        TooltipSide::Bottom,
        TooltipSide::Left,
    ];

    for side in sides {
        let props = TooltipProps::new("Test").side(side);
        assert_eq!(props.side, side);
    }
}

#[test]
fn tooltip_all_alignments() {
    let aligns = vec![TooltipAlign::Start, TooltipAlign::Center, TooltipAlign::End];

    for align in aligns {
        let props = TooltipProps::new("Test").align(align);
        assert_eq!(props.align, align);
    }
}

#[test]
fn tooltip_animation_state_closed() {
    let state = TooltipAnimationState::Closed;
    assert_eq!(state, TooltipAnimationState::Closed);
}

#[test]
fn tooltip_animation_state_delayed_open() {
    let state = TooltipAnimationState::DelayedOpen;
    assert_eq!(state, TooltipAnimationState::DelayedOpen);
}

#[test]
fn tooltip_animation_state_instant_open() {
    let state = TooltipAnimationState::InstantOpen;
    assert_eq!(state, TooltipAnimationState::InstantOpen);
}

#[test]
fn tooltip_position_to_side_conversion() {
    assert_eq!(
        TooltipSide::from_position(TooltipPosition::Top),
        TooltipSide::Top
    );
    assert_eq!(
        TooltipSide::from_position(TooltipPosition::Bottom),
        TooltipSide::Bottom
    );
    assert_eq!(
        TooltipSide::from_position(TooltipPosition::Left),
        TooltipSide::Left
    );
    assert_eq!(
        TooltipSide::from_position(TooltipPosition::Right),
        TooltipSide::Right
    );
}

#[test]
fn tooltip_side_offset_direction() {
    assert_eq!(TooltipSide::Top.offset_direction().y, -1.0);
    assert_eq!(TooltipSide::Bottom.offset_direction().y, 1.0);
    assert_eq!(TooltipSide::Left.offset_direction().x, -1.0);
    assert_eq!(TooltipSide::Right.offset_direction().x, 1.0);
}

#[test]
fn tooltip_align_factor() {
    assert_eq!(TooltipAlign::Center.factor(), 0.0);
    assert_eq!(TooltipAlign::Start.factor(), -1.0);
    assert_eq!(TooltipAlign::End.factor(), 1.0);
}

#[test]
fn tooltip_high_contrast_styling() {
    let palette = ColorPalette::default();
    let style = TooltipStyle::from_palette(&palette, true);
    let style_normal = TooltipStyle::from_palette(&palette, false);

    assert_ne!(style.bg, style_normal.bg);
}

#[test]
fn tooltip_disable_hoverable_content() {
    let props = TooltipProps::new("Test").disable_hoverable_content(true);

    assert!(props.disable_hoverable_content);
}

#[test]
fn tooltip_show_when_disabled() {
    let props = TooltipProps::new("Test").show_when_disabled(true);

    assert!(props.show_when_disabled);
}

#[test]
fn tooltip_animation_state_transitions() {
    assert_eq!(TooltipAnimationState::Closed, TooltipAnimationState::Closed);
    assert_eq!(
        TooltipAnimationState::DelayedOpen,
        TooltipAnimationState::DelayedOpen
    );
    assert_eq!(
        TooltipAnimationState::InstantOpen,
        TooltipAnimationState::InstantOpen
    );
}

#[test]
fn tooltip_open_state_default() {
    let state = TooltipOpenState::default();
    assert!(!state.is_open);
    assert_eq!(state.animation_progress, 0.0);
}

#[test]
fn tooltip_open_state_fully_open() {
    let state = TooltipOpenState {
        is_open: true,
        animation_progress: 1.0,
        hover_start_time: Some(0.0),
        last_close_time: None,
    };
    assert!(state.is_visible());
}

#[test]
fn tooltip_open_state_animation_in_progress() {
    let state = TooltipOpenState {
        is_open: true,
        animation_progress: 0.5,
        hover_start_time: Some(0.0),
        last_close_time: None,
    };
    assert!(state.is_animating());
}

#[test]
fn tooltip_state_skip_delay_applies() {
    let state = TooltipOpenState {
        is_open: false,
        animation_progress: 0.0,
        hover_start_time: None,
        last_close_time: Some(0.1),
    };

    let current_time = 0.2;
    let skip_delay_ms = 300;
    assert!(state.should_skip_delay(current_time, skip_delay_ms));
}

#[test]
fn tooltip_state_skip_delay_expired() {
    let state = TooltipOpenState {
        is_open: false,
        animation_progress: 0.0,
        hover_start_time: None,
        last_close_time: Some(0.0),
    };

    let current_time = 1.0;
    let skip_delay_ms = 300;
    assert!(!state.should_skip_delay(current_time, skip_delay_ms));
}

#[test]
fn tooltip_controlled_open_state() {
    let props = TooltipProps::new("Controlled tooltip").open(true);

    assert_eq!(props.open, Some(true));
}

#[test]
fn tooltip_default_open_state() {
    let props = TooltipProps::new("Default open tooltip").default_open(true);

    assert!(props.default_open);
}

#[test]
fn tooltip_animation_duration_configuration() {
    let props = TooltipProps::new("Test").animation_duration_ms(200);

    assert_eq!(props.animation_duration_ms, 200);
}

#[test]
fn tooltip_default_animation_duration() {
    let props = TooltipProps::new("Test");
    assert_eq!(props.animation_duration_ms, 200);
}

#[test]
fn tooltip_sticky_configuration() {
    let props = TooltipProps::new("Sticky tooltip").sticky(true);

    assert_eq!(props.sticky, TooltipSticky::Always);
}

#[test]
fn tooltip_hoverable_content_default() {
    let props = TooltipProps::new("Test");

    assert!(!props.disable_hoverable_content);
}

#[test]
fn tooltip_avoid_collisions_configuration() {
    let props = TooltipProps::new("Test").avoid_collisions(false);

    assert!(!props.avoid_collisions);
}

#[test]
fn tooltip_avoid_collisions_default() {
    let props = TooltipProps::new("Test");
    assert!(props.avoid_collisions);
}

#[test]
fn tooltip_align_offset_configuration() {
    let props = TooltipProps::new("Test").align_offset(10.0);

    assert_eq!(props.align_offset, 10.0);
}

#[test]
fn tooltip_align_offset_default() {
    let props = TooltipProps::new("Test");
    assert_eq!(props.align_offset, 0.0);
}

#[test]
fn tooltip_arrow_padding_configuration() {
    let props = TooltipProps::new("Test").arrow_padding(5.0);

    assert_eq!(props.arrow_padding, 5.0);
}

#[test]
fn tooltip_arrow_padding_default() {
    let props = TooltipProps::new("Test");
    assert_eq!(props.arrow_padding, 0.0);
}

#[test]
fn tooltip_state_new() {
    let state = TooltipState::new();
    assert!(!state.open_state.is_open);
    assert_eq!(state.open_state.animation_progress, 0.0);
}

#[test]
fn tooltip_state_open_close() {
    let mut state = TooltipState::new();

    state.open_state.is_open = true;
    state.open_state.animation_progress = 1.0;
    assert!(state.open_state.is_visible());

    state.open_state.is_open = false;
    state.open_state.animation_progress = 0.5;
    assert!(state.open_state.is_animating());
}

#[test]
fn tooltip_complex_configuration() {
    let props = TooltipProps::new("Complex tooltip")
        .side(TooltipSide::Bottom)
        .align(TooltipAlign::Start)
        .side_offset(8.0)
        .align_offset(4.0)
        .collision_padding(12.0)
        .delay_ms(500)
        .skip_delay_ms(200)
        .animation_duration_ms(100)
        .show_arrow(true)
        .arrow_size(13.0, 6.0)
        .arrow_padding(3.0)
        .max_width(400.0)
        .sticky(true)
        .avoid_collisions(true)
        .high_contrast(true);

    assert_eq!(props.side, TooltipSide::Bottom);
    assert_eq!(props.align, TooltipAlign::Start);
    assert_eq!(props.side_offset, 8.0);
    assert_eq!(props.align_offset, 4.0);
    assert_eq!(props.collision_padding, TooltipCollisionPadding::all(12.0));
    assert_eq!(props.delay_ms, 500);
    assert_eq!(props.skip_delay_ms, 200);
    assert_eq!(props.animation_duration_ms, 100);
    assert!(props.show_arrow);
    assert_eq!(props.arrow_width, 13.0);
    assert_eq!(props.arrow_height, 6.0);
    assert_eq!(props.arrow_padding, 3.0);
    assert_eq!(props.max_width, 400.0);
    assert_eq!(props.sticky, TooltipSticky::Always);
    assert!(props.avoid_collisions);
    assert!(props.high_contrast);
}

#[test]
fn tooltip_side_flip() {
    assert_eq!(TooltipSide::Top.flip(), TooltipSide::Bottom);
    assert_eq!(TooltipSide::Bottom.flip(), TooltipSide::Top);
    assert_eq!(TooltipSide::Left.flip(), TooltipSide::Right);
    assert_eq!(TooltipSide::Right.flip(), TooltipSide::Left);
}

#[test]
fn tooltip_style_custom() {
    let custom_style = TooltipStyle {
        bg: Color32::from_rgb(30, 30, 30),
        border: Color32::from_rgb(60, 60, 60),
        border_width: 2.0,
        text: Color32::WHITE,
        rounding: egui::CornerRadius::same(8),
        shadow: egui::epaint::Shadow::NONE,
        arrow_fill: Color32::from_rgb(30, 30, 30),
    };

    let props = TooltipProps::new("Styled tooltip").style(custom_style.clone());

    assert!(props.style.is_some());
    let style = props.style.unwrap();
    assert_eq!(style.bg, custom_style.bg);
    assert_eq!(style.arrow_fill, custom_style.arrow_fill);
}

#[test]
fn tooltip_style_arrow_fill_matches_bg() {
    let palette = ColorPalette::default();
    let style = TooltipStyle::from_palette(&palette, false);

    assert_eq!(style.bg, style.arrow_fill);
}
