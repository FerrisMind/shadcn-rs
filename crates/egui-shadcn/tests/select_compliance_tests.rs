use egui::{Pos2, Rect, Vec2};
use egui_shadcn::select::{
    PopupPosition, SelectAlign, SelectCollisionPadding, SelectDirection, SelectPortalContainer,
    SelectProps, SelectSide, SelectSticky, SelectUpdatePositionStrategy,
};

#[test]
fn select_radix_api_surface_exists() {
    let mut selected: Option<String> = None;

    let mut open_changes = Vec::<bool>::new();
    let mut on_open_change = |open: bool| open_changes.push(open);

    let mut value_changes = Vec::<String>::new();
    let mut on_value_change = |value: &str| value_changes.push(value.to_string());

    let mut close_auto_focus_calls = 0usize;
    let mut on_close_auto_focus =
        |_evt: &mut egui_shadcn::select::SelectAutoFocusEvent| close_auto_focus_calls += 1;

    let mut escape_calls = 0usize;
    let mut on_escape_key_down =
        |_evt: &mut egui_shadcn::select::SelectEscapeKeyDownEvent| escape_calls += 1;

    let mut outside_calls = 0usize;
    let mut on_pointer_down_outside =
        |_evt: &mut egui_shadcn::select::SelectPointerDownOutsideEvent| outside_calls += 1;

    let boundary = Rect::from_min_size(Pos2::ZERO, Vec2::new(100.0, 100.0));

    let props = SelectProps::new("select-api", &mut selected)
        .open(false)
        .default_open(false)
        .on_open_change(&mut on_open_change)
        .value("b")
        .default_value("a")
        .on_value_change(&mut on_value_change)
        .dir(SelectDirection::Ltr)
        .name("field")
        .auto_complete("shipping")
        .disabled(false)
        .required(true)
        .form("form-1")
        .position(PopupPosition::Popper)
        .side(SelectSide::Bottom)
        .side_offset(4.0)
        .align(SelectAlign::Start)
        .align_offset(0.0)
        .avoid_collisions(true)
        .collision_boundary(boundary)
        .collision_padding(SelectCollisionPadding::all(10.0))
        .arrow_padding(0.0)
        .sticky(SelectSticky::Partial)
        .hide_when_detached(false)
        .update_position_strategy(SelectUpdatePositionStrategy::Optimized)
        .container(SelectPortalContainer::Foreground)
        .on_close_auto_focus(&mut on_close_auto_focus)
        .on_escape_key_down(&mut on_escape_key_down)
        .on_pointer_down_outside(&mut on_pointer_down_outside);

    assert_eq!(props.open, Some(false));
    assert!(!props.default_open);
    assert_eq!(props.value.as_deref(), Some("b"));
    assert_eq!(props.default_value.as_deref(), Some("a"));
    assert_eq!(props.position, PopupPosition::Popper);
    assert_eq!(props.side, SelectSide::Bottom);
    assert_eq!(props.align, SelectAlign::Start);
    assert_eq!(props.collision_boundary, Some(boundary));
    assert_eq!(props.collision_padding, SelectCollisionPadding::all(10.0));
    assert_eq!(props.sticky, SelectSticky::Partial);
    assert_eq!(props.container, Some(SelectPortalContainer::Foreground));

    assert!(open_changes.is_empty());
    assert!(value_changes.is_empty());
    assert_eq!(close_auto_focus_calls, 0);
    assert_eq!(escape_calls, 0);
    assert_eq!(outside_calls, 0);
}
