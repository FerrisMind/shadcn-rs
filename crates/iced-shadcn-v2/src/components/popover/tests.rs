//! Behavioral tests for the popover component.

use shadcn_common::{
    FloatingAlign, FloatingConfig, FloatingPadding, FloatingRect, FloatingSide, FloatingSticky,
    PopoverShadow, StyleId, compute_floating, popover_recipe,
};

use super::style;
use super::types::PopoverState;
use super::*;
use crate::iced_compat::widget::container;
use crate::iced_compat::{Element, time::Duration};
use crate::theme::Theme;

#[derive(Clone, Debug)]
enum Message {
    Pressed,
    OpenChanged(bool),
}

impl Message {
    fn is_open(&self) -> bool {
        matches!(self, Self::OpenChanged(true))
    }
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let popover: Popover<'_, Message> = Popover::new(container("Open"), container("Body"), &theme)
        .side(PopoverSide::Top)
        .align(PopoverAlign::Start)
        .side_offset(10.0)
        .align_offset(2.0)
        .width(256.0)
        .content_padding(0.0)
        .radius(6.0)
        .duration(Duration::from_millis(200))
        .animated(false)
        .disabled(true)
        .open(true)
        .default_open(true)
        .on_open_change(Message::OpenChanged)
        .avoid_collisions(false)
        .collision_padding(16.0)
        .sticky(FloatingSticky::Always)
        .hide_when_detached(true)
        .close_on_click_outside(false)
        .close_on_escape(false);

    assert!(matches!(popover.content, PopoverContent::Element(_)));
    assert_eq!(popover.side, PopoverSide::Top);
    assert_eq!(popover.align, PopoverAlign::Start);
    assert_eq!(popover.side_offset, 10.0);
    assert_eq!(popover.align_offset, 2.0);
    assert_eq!(popover.width, Some(256.0));
    assert_eq!(popover.content_padding, Some(0.0));
    assert_eq!(popover.radius, Some(6.0));
    assert_eq!(popover.duration, Duration::from_millis(200));
    assert!(!popover.animated);
    assert!(popover.disabled);
    assert_eq!(popover.open, Some(true));
    assert!(popover.default_open);
    assert!(popover.on_open_change.is_some());
    assert!(!popover.avoid_collisions);
    assert_eq!(popover.collision_padding, FloatingPadding::all(16.0));
    assert_eq!(popover.sticky, FloatingSticky::Always);
    assert!(popover.hide_when_detached);
    assert!(!popover.close_on_click_outside);
    assert!(!popover.close_on_escape);
    assert!(std::ptr::eq(popover.theme, &theme));
}

#[test]
fn defaults_match_shadcn_svelte() {
    let theme = Theme::light();
    let popover: Popover<'_, Message> = Popover::text(container("Open"), "Body", &theme);

    // side = bottom, align = center, sideOffset = 4, w-72, dismissable.
    assert_eq!(popover.side, PopoverSide::Bottom);
    assert_eq!(popover.align, PopoverAlign::Center);
    assert_eq!(popover.side_offset, 4.0);
    assert_eq!(popover.align_offset, 0.0);
    assert_eq!(popover.width, None);
    assert_eq!(popover.content_padding, None);
    assert_eq!(popover.radius, None);
    assert_eq!(
        popover.duration,
        Duration::from_millis(POPOVER_ANIMATION_MS)
    );
    assert!(popover.animated);
    assert!(popover.avoid_collisions);
    assert_eq!(popover.open, None);
    assert!(!popover.default_open);
    assert!(!popover.disabled);
    assert!(popover.close_on_click_outside);
    assert!(popover.close_on_escape);
}

#[test]
fn text_and_generic_popovers_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = Popover::text(container("Open"), "Plain body", &theme).into();

    let _: Element<'_, Message> = Popover::new(
        container("Open"),
        PopoverHeader::new(&theme)
            .title(PopoverTitle::text("Dimensions", &theme))
            .description(PopoverDescription::text(
                "Set the dimensions for the layer.",
                &theme,
            )),
        &theme,
    )
    .open(true)
    .on_open_change(Message::OpenChanged)
    .into();

    let _ = Message::Pressed;
    assert!(Message::OpenChanged(true).is_open());
    assert!(!Message::OpenChanged(false).is_open());
}

#[test]
fn style_uses_popover_pair_with_foreground_ring() {
    let theme = Theme::light();
    let resolved = style::resolve_style(&theme);

    assert_eq!(resolved.background, theme.palette.popover);
    assert_eq!(resolved.text_color, theme.palette.popover_foreground);
    assert_eq!(resolved.border_width, 1.0);
    // `ring-foreground/10`: the foreground color at a tenth of its alpha.
    assert_eq!(
        resolved.border_color,
        theme.palette.foreground.scale_alpha(0.10)
    );
    assert!(resolved.shadow.blur_radius > 0.0);
}

#[test]
fn surface_radius_matches_each_style_pack() {
    let cases = [
        (StyleId::Vega, 8.0),
        (StyleId::Nova, 10.0),
        (StyleId::Maia, 18.0),
        (StyleId::Lyra, 0.0),
        (StyleId::Mira, 10.0),
        (StyleId::Luma, 22.0),
        (StyleId::Sera, 0.0),
        (StyleId::Rhea, 22.0),
    ];

    for (style_id, expected) in cases {
        let theme = Theme::light().with_style(style_id);
        assert_eq!(style::surface_radius(&theme), expected, "{style_id:?}");
        assert_eq!(
            style::resolve_style(&theme).radius,
            expected,
            "{style_id:?}"
        );
    }
}

#[test]
fn style_override_patches_resolved_style() {
    let theme = Theme::light();
    let popover: Popover<'_, Message> = Popover::text(container("Open"), "Body", &theme)
        .style_override(|style| PopoverStyle {
            radius: 0.0,
            ..style
        });

    let resolved =
        (popover.style_override.as_ref().expect("override set"))(style::resolve_style(&theme));
    assert_eq!(resolved.radius, 0.0);
}

#[test]
fn recipe_tracks_style_pack_tokens() {
    // Vega: `p-4 gap-4 rounded-md text-sm shadow-md ring-foreground/10`.
    let vega = popover_recipe(StyleId::Vega);
    assert_eq!(vega.pad_px, 16.0);
    assert_eq!(vega.gap_px, 16.0);
    assert_eq!(vega.width_px, 288.0);
    assert_eq!(vega.ring_alpha, 0.10);
    assert_eq!(vega.shadow, PopoverShadow::MD);
    assert_eq!(vega.typography.size_px, 14.0);

    // Nova: `p-2.5 gap-2.5 rounded-lg`, header `gap-0.5`.
    let nova = popover_recipe(StyleId::Nova);
    assert_eq!(nova.pad_px, 10.0);
    assert_eq!(nova.gap_px, 10.0);
    assert_eq!(nova.header_gap_px, 2.0);

    // Maia: `shadow-2xl ring-foreground/5`, title `text-base`.
    let maia = popover_recipe(StyleId::Maia);
    assert_eq!(maia.shadow, PopoverShadow::XXL);
    assert_eq!(maia.ring_alpha, 0.05);
    assert_eq!(maia.title.size_px, 16.0);

    // Sera: uppercase semibold title.
    let sera = popover_recipe(StyleId::Sera);
    assert!(sera.title.uppercase);
}

#[test]
fn popover_recipe_differs_across_style_packs() {
    // popover.json is pack-specific (unlike form.json). Rhea/Luma use
    // rounded-3xl + shadow-lg; Vega uses rounded-md + shadow-md; Lyra is sharp.
    use shadcn_common::ComponentRadius;

    assert_eq!(popover_recipe(StyleId::Vega).radius, ComponentRadius::Md);
    assert_eq!(popover_recipe(StyleId::Rhea).radius, ComponentRadius::S3xl);
    assert_eq!(popover_recipe(StyleId::Lyra).radius, ComponentRadius::None);
    assert_ne!(
        popover_recipe(StyleId::Vega).shadow,
        popover_recipe(StyleId::Rhea).shadow
    );
    assert_ne!(
        popover_recipe(StyleId::Vega).title.size_px,
        popover_recipe(StyleId::Rhea).title.size_px
    );
}

#[test]
fn popover_and_composed_parts_follow_theme_style_pack() {
    // Popover owns its recipe; Button / Label / Input resolve through the
    // same Theme.style_id() — composite rule when a host has no pack deltas.
    use crate::components::button::Button;
    use crate::components::input::Input;
    use crate::components::label::Label;
    use shadcn_common::{ComponentRadius, ControlSize, LabelContext};

    let vega = Theme::light().with_style(StyleId::Vega);
    let rhea = Theme::light().with_style(StyleId::Rhea);
    let mira = Theme::light().with_style(StyleId::Mira);

    assert_eq!(style::recipe(&vega).radius, ComponentRadius::Md);
    assert_eq!(style::recipe(&rhea).radius, ComponentRadius::S3xl);

    let vega_style = style::resolve_style(&vega);
    let rhea_style = style::resolve_style(&rhea);
    assert_ne!(vega_style.radius, rhea_style.radius);
    assert_ne!(vega_style.shadow.blur_radius, rhea_style.shadow.blur_radius);

    // Trigger / with-form parts: pack deltas live on Button / Label recipes.
    assert_ne!(vega.style.button_type(), rhea.style.button_type());
    assert_ne!(
        vega.style.button_size(ControlSize::Md).height_px,
        rhea.style.button_size(ControlSize::Md).height_px
    );
    // Vega/Rhea share field-label typography; Mira proves Theme drives Label.
    assert_eq!(
        vega.style.label(LabelContext::Field),
        rhea.style.label(LabelContext::Field)
    );
    assert_ne!(
        vega.style.label(LabelContext::Field),
        mira.style.label(LabelContext::Field)
    );

    let trigger = Button::text("Open Popover", &rhea);
    let _header = PopoverHeader::<()>::new(&rhea)
        .title(PopoverTitle::text("Dimensions", &rhea))
        .description(PopoverDescription::text("Set the dimensions.", &rhea));
    let _label = Label::<()>::text("Width", &rhea);
    let _input = Input::<()>::new(&rhea).value("100%");
    let _popover: Popover<'_, ()> =
        Popover::new(trigger, container("Body"), &rhea).align(PopoverAlign::Start);
    assert_eq!(rhea.style_id(), StyleId::Rhea);
}

#[test]
fn state_visibility_follows_open_and_transition() {
    let mut state = PopoverState::new(false);
    assert!(!state.is_visible());
    assert!(!state.requested_open);

    state.open = true;
    assert!(state.is_visible());

    state.open = false;
    state.transition.reset(0.4);
    assert!(state.is_visible());

    state.transition.reset(0.0);
    assert!(!state.is_visible());

    // `defaultOpen` seeds the uncontrolled intent only.
    let state = PopoverState::new(true);
    assert!(state.requested_open);
    assert!(!state.open);
}

#[test]
fn side_and_align_map_to_floating_tokens() {
    assert_eq!(PopoverSide::Top.to_floating(), FloatingSide::Top);
    assert_eq!(PopoverSide::Right.to_floating(), FloatingSide::Right);
    assert_eq!(PopoverSide::Bottom.to_floating(), FloatingSide::Bottom);
    assert_eq!(PopoverSide::Left.to_floating(), FloatingSide::Left);
    assert_eq!(PopoverAlign::Start.to_floating(), FloatingAlign::Start);
    assert_eq!(PopoverAlign::Center.to_floating(), FloatingAlign::Center);
    assert_eq!(PopoverAlign::End.to_floating(), FloatingAlign::End);
}

#[test]
fn floating_pipeline_places_surface_below_trigger() {
    let anchor = FloatingRect::new(300.0, 200.0, 100.0, 36.0);
    let boundary = FloatingRect::new(0.0, 0.0, 800.0, 600.0);
    let config = FloatingConfig::default()
        .side(FloatingSide::Bottom)
        .side_offset(4.0);

    let placement = compute_floating(anchor, 288.0, 120.0, boundary, &config);

    assert_eq!(placement.side, FloatingSide::Bottom);
    assert_eq!(placement.y, 200.0 + 36.0 + 4.0);
    // align = center on the trigger edge.
    assert_eq!(placement.x, anchor.center_x() - 288.0 / 2.0);
}
