//! Behavioral tests for the toggle component.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::iced_compat::widget::{button as button_widget, container};
use crate::iced_compat::{Background, Color, Element, Length};
use shadcn_common::StyleId;

use super::geometry;
use super::style;
use super::*;

#[derive(Debug, Clone)]
enum Message {
    Toggled(bool),
    Pressed,
}

fn resolve(
    theme: &Theme,
    variant: ToggleVariant,
    pressed: bool,
    invalid: bool,
    disabled: bool,
    status: button_widget::Status,
) -> button_widget::Style {
    style::resolve_toggle_style(theme, variant, pressed, None, invalid, disabled, status)
}

fn background_color(style: &button_widget::Style) -> Option<Color> {
    style.background.map(|background| match background {
        Background::Color(color) => color,
        _ => panic!("toggle backgrounds are plain colors"),
    })
}

#[test]
fn builder_defaults_match_the_web_component() {
    let theme = Theme::light();
    let toggle = Toggle::<Message>::text("Bold", &theme);

    assert!(matches!(toggle.content, ToggleContent::Label(_)));
    assert_eq!(toggle.variant, ToggleVariant::Default);
    assert_eq!(toggle.size, ToggleSize::Default);
    assert_eq!(toggle.radius, None);
    assert!(toggle.icon_start.is_none());
    assert!(toggle.icon_end.is_none());
    assert!(!toggle.pressed);
    assert!(!toggle.invalid);
    assert!(!toggle.disabled);
    assert!(!toggle.full_width);
    assert!(toggle.on_toggle.is_none());
    assert!(std::ptr::eq(toggle.theme, &theme));
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let toggle = Toggle::<Message>::text("Bold", &theme)
        .variant(ToggleVariant::Outline)
        .size(ToggleSize::Lg)
        .radius(ToggleRadius::Full)
        .pressed(true)
        .invalid(true)
        .disabled(true)
        .width(Length::Fixed(120.0))
        .height(Length::Fixed(48.0))
        .padding([0, 16])
        .full_width();

    assert_eq!(toggle.variant, ToggleVariant::Outline);
    assert_eq!(toggle.size, ToggleSize::Lg);
    assert_eq!(toggle.radius, Some(ToggleRadius::Full));
    assert!(toggle.pressed);
    assert!(toggle.invalid);
    assert!(toggle.disabled);
    assert_eq!(toggle.width, Length::Fixed(120.0));
    assert_eq!(toggle.height, Some(Length::Fixed(48.0)));
    assert_eq!(
        toggle.padding,
        Some(crate::iced_compat::Padding::from([0, 16]))
    );
    assert!(toggle.full_width);
}

#[test]
fn all_constructors_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = Toggle::new(container("Custom"), &theme)
        .on_toggle(Message::Toggled)
        .into();
    let _: Element<'_, Message> = Toggle::text("Bold", &theme)
        .on_toggle(Message::Toggled)
        .into();
    let _: Element<'_, Message> = Toggle::icon(container("B"), &theme)
        .on_press(Message::Pressed)
        .into();
    let _: Element<'_, Message> = Toggle::text("Bold", &theme)
        .icon_start(container("B"))
        .icon_end(container("↗"))
        .on_toggle(Message::Toggled)
        .into();
}

#[test]
fn icon_slots_tighten_the_padding_on_their_own_side() {
    let theme = Theme::light();
    let recipe = ToggleSize::Default.recipe(&theme);
    assert!(
        recipe.pad_x_icon_px < recipe.pad_x_px,
        "vega tightens padding"
    );

    let leading = ToggleSize::Default.default_padding(&theme, false, true, false);
    assert_eq!(leading.left, recipe.pad_x_icon_px);
    assert_eq!(leading.right, recipe.pad_x_px);

    let trailing = ToggleSize::Default.default_padding(&theme, false, false, true);
    assert_eq!(trailing.left, recipe.pad_x_px);
    assert_eq!(trailing.right, recipe.pad_x_icon_px);

    let both = ToggleSize::Default.default_padding(&theme, false, true, true);
    assert_eq!(both.left, recipe.pad_x_icon_px);
    assert_eq!(both.right, recipe.pad_x_icon_px);
}

#[test]
fn icon_slots_are_tracked_by_the_builder() {
    let theme = Theme::light();
    let toggle = Toggle::<Message>::text("Bold", &theme)
        .icon_start(container("B"))
        .icon_end(container("↗"));

    assert!(toggle.icon_start.is_some());
    assert!(toggle.icon_end.is_some());

    let debug = format!("{toggle:?}");
    assert!(debug.contains("icon_start: true"));
    assert!(debug.contains("icon_end: true"));
}

#[test]
fn toggle_callback_receives_the_next_state_and_press_ignores_it() {
    let theme = Theme::light();

    let toggled = Toggle::text("Bold", &theme)
        .pressed(true)
        .on_toggle(Message::Toggled);
    let on_toggle = toggled.on_toggle.as_ref().expect("callback is stored");
    assert!(matches!(on_toggle(false), Message::Toggled(false)));

    let pressed = Toggle::text("Bold", &theme).on_press(Message::Pressed);
    let on_press = pressed.on_toggle.as_ref().expect("callback is stored");
    assert!(matches!(on_press(true), Message::Pressed));

    assert!(
        Toggle::<Message>::text("Bold", &theme)
            .on_press_maybe(None)
            .on_toggle
            .is_none()
    );
    assert!(
        Toggle::text("Bold", &theme)
            .on_toggle_maybe(Some(Message::Toggled))
            .on_toggle
            .is_some()
    );
}

#[test]
fn resting_toggles_are_transparent_and_fill_when_pressed_on() {
    let theme = Theme::light();

    let off = resolve(
        &theme,
        ToggleVariant::Default,
        false,
        false,
        false,
        button_widget::Status::Active,
    );
    assert!(off.background.is_none());
    assert_eq!(off.border.width, 0.0);

    let on = resolve(
        &theme,
        ToggleVariant::Default,
        true,
        false,
        false,
        button_widget::Status::Active,
    );
    assert_eq!(
        background_color(&on),
        Some(theme.semantic_color(crate::SemanticColor::Muted)),
    );

    let hovered = resolve(
        &theme,
        ToggleVariant::Default,
        false,
        false,
        false,
        button_widget::Status::Hovered,
    );
    assert_eq!(background_color(&hovered), background_color(&on));
}

#[test]
fn outline_variant_uses_the_input_border() {
    let theme = Theme::light();

    let outline = resolve(
        &theme,
        ToggleVariant::Outline,
        false,
        false,
        false,
        button_widget::Status::Active,
    );
    assert_eq!(outline.border.width, 1.0);
    assert_eq!(
        outline.border.color,
        theme.semantic_color(crate::SemanticColor::Input),
    );
}

#[test]
fn only_vega_outline_carries_a_shadow() {
    for style_id in StyleId::ALL {
        let theme = Theme::light().with_style(style_id);
        let style = resolve(
            &theme,
            ToggleVariant::Outline,
            false,
            false,
            false,
            button_widget::Status::Active,
        );

        assert_eq!(
            style.shadow.color.a > 0.0,
            matches!(style_id, StyleId::Vega),
            "{style_id:?} shadow mismatch",
        );
    }
}

#[test]
fn invalid_state_paints_a_destructive_border() {
    let theme = Theme::light();

    let invalid = resolve(
        &theme,
        ToggleVariant::Outline,
        false,
        true,
        false,
        button_widget::Status::Active,
    );
    assert_eq!(
        invalid.border.color,
        theme.semantic_color(crate::SemanticColor::Destructive),
    );

    let invalid_default = resolve(
        &theme,
        ToggleVariant::Default,
        false,
        true,
        false,
        button_widget::Status::Active,
    );
    assert_eq!(invalid_default.border.width, 1.0);
    assert!(invalid_default.border.color.a < 1.0);
}

#[test]
fn disabled_toggles_are_painted_at_half_opacity() {
    let theme = Theme::light();

    let enabled = resolve(
        &theme,
        ToggleVariant::Default,
        true,
        false,
        false,
        button_widget::Status::Active,
    );
    let disabled = resolve(
        &theme,
        ToggleVariant::Default,
        true,
        false,
        true,
        button_widget::Status::Disabled,
    );

    let enabled_bg = background_color(&enabled).expect("pressed-on has a fill");
    let disabled_bg = background_color(&disabled).expect("disabled keeps the fill");
    assert!((disabled_bg.a - enabled_bg.a * 0.5).abs() < f32::EPSILON);
    assert!((disabled.text_color.a - enabled.text_color.a * 0.5).abs() < f32::EPSILON);
}

#[test]
fn toggle_sizes_match_style_pack_recipes() {
    let vega = Theme::light();
    assert_eq!(ToggleSize::Sm.control_height(&vega), 32.0);
    assert_eq!(ToggleSize::Default.control_height(&vega), 36.0);
    assert_eq!(ToggleSize::Lg.control_height(&vega), 40.0);

    let mira = Theme::light().with_style(StyleId::Mira);
    assert_eq!(ToggleSize::Default.control_height(&mira), 28.0);
    assert_eq!(ToggleSize::Sm.label_text_size(&mira), 10.0);

    let sera = Theme::light().with_style(StyleId::Sera);
    assert_eq!(ToggleSize::Lg.control_height(&sera), 44.0);
    assert!(sera.style.toggle().typography.uppercase);
}

#[test]
fn icon_toggles_use_the_square_min_width_footprint() {
    let resolved =
        geometry::resolve_toggle_width(Length::Shrink, Length::Fixed(72.0), false, true, 36.0);
    assert_eq!(resolved, Length::Fixed(72.0));

    let default_height =
        geometry::resolve_toggle_width(Length::Shrink, Length::Shrink, false, true, 36.0);
    assert_eq!(default_height, Length::Fixed(36.0));

    let full = geometry::resolve_toggle_width(Length::Shrink, Length::Shrink, true, false, 36.0);
    assert_eq!(full, Length::Fill);
}

#[test]
fn icon_toggles_drop_the_horizontal_padding() {
    let theme = Theme::light();

    assert_eq!(
        ToggleSize::Default.default_padding(&theme, true, false, false),
        crate::iced_compat::Padding::ZERO,
    );

    let padded = ToggleSize::Default.default_padding(&theme, false, false, false);
    assert_eq!(padded.left, 10.0);
    assert_eq!(padded.right, 10.0);
}

#[test]
fn all_variants_resolve_in_light_and_dark_themes() {
    for theme in [Theme::light(), Theme::dark()] {
        for variant in [ToggleVariant::Default, ToggleVariant::Outline] {
            for pressed in [false, true] {
                for status in [
                    button_widget::Status::Active,
                    button_widget::Status::Hovered,
                    button_widget::Status::Pressed,
                    button_widget::Status::Disabled,
                ] {
                    let style = resolve(&theme, variant, pressed, false, false, status);
                    assert!(style.text_color.a.is_finite());
                }
            }
        }
    }
}

#[test]
fn states_dimensions_and_style_override_are_configurable() {
    let theme = Theme::light();
    let toggle = Toggle::text("Bold", &theme)
        .pressed(true)
        .style_override(|mut style, _| {
            style.text_color = Color::from_rgb(1.0, 0.0, 1.0);
            style
        })
        .on_toggle(Message::Toggled);

    assert!(toggle.style_override.is_some());

    let _ = toggle.into_button();
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let toggle = Toggle::<NoDebugMessage>::text("Bold", &theme).pressed(true);
    let debug = format!("{toggle:?}");

    assert!(debug.contains("Toggle"));
    assert!(debug.contains("label"));
    assert!(debug.contains("pressed: true"));
}

#[test]
fn configuration_enums_support_hashing_and_expected_order() {
    fn hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    let _ = hash(&ToggleVariant::Default);
    let _ = hash(&ToggleSize::Default);
    let _ = hash(&ToggleRadius::Medium);
    assert!(ToggleRadius::None < ToggleRadius::Full);
}
