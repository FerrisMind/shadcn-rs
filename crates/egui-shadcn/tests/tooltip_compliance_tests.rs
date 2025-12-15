use egui::{Pos2, Rect, Vec2};
use egui_shadcn::tooltip::{
    TooltipCollisionPadding, TooltipPortalContainer, TooltipProps, TooltipSticky,
    TooltipUpdatePositionStrategy,
};

#[test]
fn tooltip_radix_api_aliases() {
    let props = TooltipProps::new("Label")
        .delay_duration(500)
        .skip_delay_duration(200);

    assert_eq!(props.delay_ms, 500);
    assert_eq!(props.skip_delay_ms, 200);
}

#[test]
fn tooltip_collision_boundary_config() {
    let boundary = Rect::from_min_size(Pos2::ZERO, Vec2::new(100.0, 100.0));
    let props = TooltipProps::new("Label").collision_boundary(boundary);

    assert_eq!(props.collision_boundary, Some(boundary));
}

#[test]
fn tooltip_api_reference_surface_exists() {
    let mut open_changes = Vec::<bool>::new();
    let mut on_open_change = |value: bool| open_changes.push(value);

    let mut escape_calls = 0usize;
    let mut on_escape_key_down = |_evt: &mut egui_shadcn::tooltip::TooltipEscapeKeyDownEvent| {
        escape_calls += 1;
    };

    let mut outside_calls = 0usize;
    let mut on_pointer_down_outside =
        |_evt: &mut egui_shadcn::tooltip::TooltipPointerDownOutsideEvent| {
            outside_calls += 1;
        };

    let props = TooltipProps::new("Label")
        .default_open(false)
        .open(false)
        .on_open_change(&mut on_open_change)
        .sticky(TooltipSticky::Partial)
        .collision_padding(TooltipCollisionPadding::all(8.0))
        .hide_when_detached(false)
        .update_position_strategy(TooltipUpdatePositionStrategy::Optimized)
        .container(TooltipPortalContainer::Tooltip)
        .aria_label("Accessible label")
        .on_escape_key_down(&mut on_escape_key_down)
        .on_pointer_down_outside(&mut on_pointer_down_outside);

    assert_eq!(props.open, Some(false));
    assert!(!props.default_open);
    assert_eq!(props.sticky, TooltipSticky::Partial);
    assert_eq!(props.collision_padding, TooltipCollisionPadding::all(8.0));
    assert!(!props.hide_when_detached);
    assert_eq!(
        props.update_position_strategy,
        TooltipUpdatePositionStrategy::Optimized
    );
    assert_eq!(props.container, Some(TooltipPortalContainer::Tooltip));
    assert_eq!(props.aria_label.as_deref(), Some("Accessible label"));

    assert!(open_changes.is_empty());
    assert_eq!(escape_calls, 0);
    assert_eq!(outside_calls, 0);
}
