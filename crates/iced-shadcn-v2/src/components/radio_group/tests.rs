//! Tests for the radio-group component.

use shadcn_common::StyleId;

use super::geometry;
use super::style::resolve_style;
use super::types::{
    RadioGroupOrientation, RadioGroupRadius, RadioGroupSize, RadioGroupStatus, RadioGroupStyle,
};
use super::{RadioGroup, RadioGroupItem};
use crate::iced_compat::{Element, Length};
use crate::theme::Theme;

#[derive(Debug, Clone)]
enum Message {
    Changed(String),
    Pressed,
}

fn style_for(theme: &Theme, status: RadioGroupStatus) -> RadioGroupStyle {
    let metrics = geometry::resolve_metrics(theme, RadioGroupSize::Default, None);

    resolve_style(theme, metrics, status)
}

fn checked() -> RadioGroupStatus {
    RadioGroupStatus {
        checked: true,
        ..RadioGroupStatus::default()
    }
}

#[test]
fn defaults_match_the_web_component() {
    assert_eq!(
        RadioGroupOrientation::default(),
        RadioGroupOrientation::Vertical
    );
    assert_eq!(RadioGroupSize::default(), RadioGroupSize::Default);
    assert_eq!(RadioGroupRadius::default(), RadioGroupRadius::Full);

    let theme = Theme::light();
    let group = RadioGroup::<Message>::new(&theme);

    assert!(group.is_empty());
    assert_eq!(group.len(), 0);
    assert_eq!(group.selected_value(), None);
    assert!(!group.is_required());
    assert_eq!(group.field_name(), None);
}

#[test]
fn every_style_pack_keeps_the_dot_inside_the_indicator() {
    for style in StyleId::ALL {
        for mode in [Theme::light(), Theme::dark()] {
            let theme = mode.with_style(style);

            for size in [
                RadioGroupSize::Sm,
                RadioGroupSize::Default,
                RadioGroupSize::Lg,
                RadioGroupSize::Custom(48.0),
            ] {
                let metrics = geometry::resolve_metrics(&theme, size, None);

                assert!(metrics.indicator >= 1.0, "{style:?} {size:?}");
                assert!(
                    metrics.dot + metrics.border_width * 2.0 <= metrics.indicator,
                    "{style:?} {size:?} dot {} overflows {}",
                    metrics.dot,
                    metrics.indicator,
                );
                assert!(metrics.footprint() > metrics.indicator);
            }
        }
    }
}

#[test]
fn size_presets_scale_the_pack_footprint() {
    let theme = Theme::light().with_style(StyleId::Vega);
    let small = geometry::resolve_metrics(&theme, RadioGroupSize::Sm, None);
    let default = geometry::resolve_metrics(&theme, RadioGroupSize::Default, None);
    let large = geometry::resolve_metrics(&theme, RadioGroupSize::Lg, None);

    assert_eq!(default.indicator, 16.0);
    assert_eq!(small.indicator, 14.0);
    assert_eq!(large.indicator, 20.0);
    assert!(small.dot < default.dot && default.dot < large.dot);

    // Sera is the one pack with a larger footprint (`size-4.5`).
    let sera = Theme::light().with_style(StyleId::Sera);
    assert_eq!(
        geometry::resolve_metrics(&sera, RadioGroupSize::Default, None).indicator,
        18.0,
    );
}

#[test]
fn custom_size_and_radius_stay_within_valid_bounds() {
    let theme = Theme::light();

    let custom = geometry::resolve_metrics(&theme, RadioGroupSize::Custom(24.0), None);
    assert_eq!(custom.indicator, 24.0);

    // Non-finite and negative inputs are normalized instead of panicking.
    let degenerate = geometry::resolve_metrics(&theme, RadioGroupSize::Custom(f32::NAN), None);
    assert_eq!(degenerate.indicator, 1.0);

    // `Full` is capped to a circle and no explicit radius may exceed it.
    let full = geometry::resolve_metrics(&theme, RadioGroupSize::Default, None);
    assert_eq!(full.radius, full.indicator / 2.0);

    let clamped = geometry::resolve_metrics(
        &theme,
        RadioGroupSize::Default,
        Some(RadioGroupRadius::Custom(999.0)),
    );
    assert_eq!(clamped.radius, clamped.indicator / 2.0);

    let square = geometry::resolve_metrics(
        &theme,
        RadioGroupSize::Default,
        Some(RadioGroupRadius::None),
    );
    assert_eq!(square.radius, 0.0);
}

#[test]
fn gaps_follow_the_style_pack_until_overridden() {
    let vega = Theme::light().with_style(StyleId::Vega);
    let nova = Theme::light().with_style(StyleId::Nova);

    assert_eq!(geometry::gap_px(&vega, None), 12.0);
    assert_eq!(geometry::gap_px(&nova, None), 8.0);
    assert_eq!(geometry::gap_px(&vega, Some(2.0)), 8.0);

    // The reserved ring width already separates indicator and label.
    let metrics = geometry::resolve_metrics(&vega, RadioGroupSize::Default, None);
    assert_eq!(
        geometry::label_gap_px(&vega, metrics, None),
        8.0 - metrics.ring_width,
    );
    assert_eq!(geometry::label_gap_px(&vega, metrics, Some(0.0)), 0.0);
}

#[test]
fn checked_items_fill_with_primary_and_paint_a_dot() {
    let theme = Theme::light().with_style(StyleId::Vega);
    let unchecked = style_for(&theme, RadioGroupStatus::default());
    let checked = style_for(&theme, checked());

    assert_eq!(checked.indicator, theme.palette.primary);
    assert_eq!(checked.border, theme.palette.primary);
    assert_eq!(checked.dot, theme.palette.primary_foreground);
    assert!(checked.dot_size > 0.0);

    // An unchecked light-mode outline radio is a bordered hole.
    assert_eq!(unchecked.indicator.a, 0.0);
    assert_eq!(unchecked.border, theme.palette.input);
    assert_eq!(unchecked.dot_size, 0.0);
}

#[test]
fn sera_keeps_the_box_transparent_and_paints_with_foreground() {
    let theme = Theme::light().with_style(StyleId::Sera);
    let checked = style_for(&theme, checked());

    assert_eq!(checked.indicator.a, 0.0);
    assert_eq!(checked.border, theme.palette.foreground);
    assert_eq!(checked.dot, theme.palette.foreground);
}

#[test]
fn filled_packs_hide_the_border_and_keep_an_unchecked_fill() {
    for style in [StyleId::Luma, StyleId::Rhea] {
        let theme = Theme::light().with_style(style);
        let unchecked = style_for(&theme, RadioGroupStatus::default());
        let checked = style_for(&theme, checked());

        assert!(unchecked.indicator.a > 0.0, "{style:?}");
        assert_eq!(unchecked.border.a, 0.0, "{style:?}");
        assert_eq!(checked.border.a, 0.0, "{style:?}");
        assert_eq!(checked.indicator, theme.palette.primary, "{style:?}");
    }
}

#[test]
fn invalid_outranks_focus_and_spares_a_checked_bordered_radio() {
    let theme = Theme::light().with_style(StyleId::Vega);
    let invalid = style_for(
        &theme,
        RadioGroupStatus {
            invalid: true,
            focused: true,
            ..RadioGroupStatus::default()
        },
    );

    assert_eq!(invalid.border, theme.palette.destructive);
    assert_eq!(invalid.ring.map(|ring| ring.a > 0.0), Some(true));

    // `aria-invalid:aria-checked:border-primary` keeps the checked border.
    let invalid_checked = style_for(
        &theme,
        RadioGroupStatus {
            invalid: true,
            checked: true,
            ..RadioGroupStatus::default()
        },
    );
    assert_eq!(invalid_checked.border, theme.palette.primary);

    // Filled packs have no such override.
    let filled = Theme::light().with_style(StyleId::Rhea);
    let filled_invalid_checked = style_for(
        &filled,
        RadioGroupStatus {
            invalid: true,
            checked: true,
            ..RadioGroupStatus::default()
        },
    );
    assert_eq!(filled_invalid_checked.border, filled.palette.destructive);
}

#[test]
fn focus_paints_a_ring_and_recolors_the_border() {
    for style in StyleId::ALL {
        let theme = Theme::light().with_style(style);
        let resting = style_for(&theme, RadioGroupStatus::default());
        let focused = style_for(
            &theme,
            RadioGroupStatus {
                focused: true,
                ..RadioGroupStatus::default()
            },
        );

        assert!(resting.ring.is_none(), "{style:?}");
        assert_eq!(focused.border, theme.palette.ring, "{style:?}");

        let ring = focused.ring.expect("focused radios paint a ring");
        assert!(ring.a > 0.0 && focused.ring_width > 0.0, "{style:?}");
    }
}

#[test]
fn disabled_items_dim_every_painted_color() {
    let theme = Theme::light().with_style(StyleId::Vega);
    let enabled = style_for(&theme, checked());
    let disabled = style_for(
        &theme,
        RadioGroupStatus {
            checked: true,
            disabled: true,
            ..RadioGroupStatus::default()
        },
    );

    assert!(disabled.indicator.a < enabled.indicator.a);
    assert!(disabled.dot.a < enabled.dot.a);
    assert!(disabled.label.a < enabled.label.a);
    assert!(disabled.description.a < enabled.description.a);
}

#[test]
fn arrow_navigation_skips_disabled_items_and_honors_loop() {
    let theme = Theme::light();
    let group = || {
        RadioGroup::<Message>::new(&theme)
            .push(RadioGroupItem::new("a"))
            .push(RadioGroupItem::new("b").disabled(true))
            .push(RadioGroupItem::new("c"))
    };

    // A disabled item is never an answer.
    assert_eq!(group().value("a").next_value(), Some("c"));
    assert_eq!(group().value("c").previous_value(), Some("a"));

    // With no selection the nearest end wins.
    assert_eq!(group().next_value(), Some("a"));
    assert_eq!(group().previous_value(), Some("c"));

    // `loop` wraps; without it the edge value stays put.
    assert_eq!(group().value("c").next_value(), Some("a"));
    assert_eq!(
        group().value("c").loop_navigation(false).next_value(),
        Some("c"),
    );
    assert_eq!(
        group().value("a").loop_navigation(false).previous_value(),
        Some("a"),
    );

    // Empty, disabled, and read-only groups have nothing to move to.
    assert_eq!(RadioGroup::<Message>::new(&theme).next_value(), None);
    assert_eq!(group().disabled(true).next_value(), None);
    assert_eq!(group().readonly(true).next_value(), None);
}

#[test]
fn values_and_selection_queries_follow_the_controlled_value() {
    let theme = Theme::light();
    let group = RadioGroup::<Message>::new(&theme)
        .extend([
            RadioGroupItem::text("monthly", "Monthly"),
            RadioGroupItem::text("yearly", "Yearly").description("Save 17%"),
        ])
        .value("yearly");

    assert_eq!(group.values().collect::<Vec<_>>(), ["monthly", "yearly"]);
    assert_eq!(group.selected_value(), Some("yearly"));
    assert!(group.is_selected("yearly"));
    assert!(!group.is_selected("monthly"));
    assert_eq!(group.clear_value().selected_value(), None);
}

#[test]
fn item_metadata_is_readable_before_rendering() {
    let item = RadioGroupItem::<Message>::text("all", "All new messages")
        .description("Every message in every channel")
        .id_attr("notify-all");

    assert_eq!(item.value(), "all");
    assert_eq!(item.id(), Some("notify-all"));
}

#[test]
fn the_change_callback_receives_the_pressed_item_value() {
    let theme = Theme::light();
    let group = RadioGroup::<Message>::new(&theme).on_change(Message::Changed);
    let callback = group.on_change.as_ref().expect("callback was just set");

    let Message::Changed(value) = callback("yearly".to_owned()) else {
        panic!("on_change must map the value into its own message");
    };
    assert_eq!(value, "yearly");

    // Setting a press message clears the value callback, and vice versa.
    assert!(group.on_press(Message::Pressed).on_change.is_none());
}

#[test]
fn builder_supports_value_callbacks_press_messages_and_overrides() {
    let theme = Theme::light();

    let _: Element<'_, Message> = RadioGroup::new(&theme)
        .value("all")
        .push(RadioGroupItem::text("all", "All"))
        .push(
            RadioGroupItem::text("none", "None")
                .invalid(true)
                .focused(true),
        )
        .on_change(Message::Changed)
        .into();

    let _: Element<'_, Message> = RadioGroup::new(&theme)
        .push(RadioGroupItem::new("all"))
        .on_press(Message::Pressed)
        .into();

    let _: Element<'_, Message> = RadioGroup::new(&theme)
        .orientation(RadioGroupOrientation::Horizontal)
        .size(RadioGroupSize::Lg)
        .radius(RadioGroupRadius::Medium)
        .spacing(4.0)
        .label_spacing(3.0)
        .readonly(true)
        .required(true)
        .name("notify")
        .full_width()
        .item_width(Length::Fixed(240.0))
        .push(RadioGroupItem::with_content(
            "custom",
            crate::iced_compat::widget::text("Custom content"),
        ))
        .item_style_override(|style, _status| style)
        .style_override(|style| style)
        .on_change_maybe(Some(Message::Changed))
        .into();
}
