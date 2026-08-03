//! Behavioral tests for the meter component.

use std::time::Duration;

use crate::iced_compat::widget::canvas;
use crate::iced_compat::{Color, Element, Length, Rectangle, mouse, time, window};
use shadcn_common::{AccentColor, MeterFillTone, StyleId};

use super::geometry::{default_height, default_radius, display_ratio, radius_px, target_ratio};
use super::style::resolve_visual;
use super::*;
use crate::theme::Theme;

#[test]
fn defaults_match_shadcn_svelte_extras_meter() {
    let theme = Theme::light();
    let meter = Meter::new(&theme);

    assert_eq!(meter.value, 0.0);
    assert_eq!(meter.min, 0.0);
    assert_eq!(meter.max, 100.0);
    assert_eq!(meter.orientation, MeterOrientation::Horizontal);
    assert_eq!(meter.size, MeterSize::Default);
    assert_eq!(meter.radius, None);
    assert_eq!(meter.width, None);
    assert_eq!(meter.height, None);
    assert_eq!(default_height(&theme), 8.0);
    assert_eq!(default_radius(&theme), MeterRadius::Full);
    assert_eq!(theme.style.meter().track_alpha, 0.20);
}

#[test]
fn style_pack_meter_geometry_is_pack_invariant() {
    for style in StyleId::ALL {
        let theme = Theme::light().with_style(style);
        assert_eq!(default_height(&theme), 8.0);
        assert_eq!(default_radius(&theme), MeterRadius::Full);
    }
}

#[test]
fn meter_follows_theme_style_pack() {
    // Extras Meter markup is pack-invariant (like Form.json). Choosing a pack
    // on the shared Theme still surfaces that pack's chrome recipes (Button)
    // and fonts; Base/Accent/Mode drive the palette Meter paints with.
    let vega = Theme::light().with_style(StyleId::Vega);
    let rhea = Theme::light().with_style(StyleId::Rhea);

    assert_eq!(vega.style.meter(), rhea.style.meter());
    assert_ne!(vega.style.button_type(), rhea.style.button_type());
    assert_eq!(vega.font_pack(), rhea.font_pack());

    // Pack alone does not retint semantic primary — Base/Accent do — but Meter
    // still resolves fills from the shared Theme (Form pattern: one Theme).
    let vega_primary = resolve_visual(&Meter::new(&vega).value(50.0).theme_primary(), &vega);
    let rhea_primary = resolve_visual(&Meter::new(&rhea).value(50.0).theme_primary(), &rhea);
    assert_eq!(vega_primary.indicator, vega.palette.primary);
    assert_eq!(rhea_primary.indicator, rhea.palette.primary);

    let blue_base = Theme::light()
        .with_style(StyleId::Rhea)
        .with_accent(Some(AccentColor::Blue));
    let orange_base = Theme::light()
        .with_style(StyleId::Rhea)
        .with_accent(Some(AccentColor::Orange));
    let blue_fill = resolve_visual(
        &Meter::new(&blue_base).value(50.0).color(AccentColor::Blue),
        &blue_base,
    );
    let orange_fill = resolve_visual(
        &Meter::new(&orange_base)
            .value(50.0)
            .color(AccentColor::Orange),
        &orange_base,
    );
    assert_ne!(blue_fill.indicator, orange_fill.indicator);

    let danger = resolve_visual(&Meter::new(&rhea).value(100.0).auto_tone(true), &rhea);
    assert_eq!(danger.indicator, rhea.palette.destructive);

    let _meter = Meter::new(&rhea).value(66.0).theme_primary();
    let _button = crate::Button::<()>::text("Fill", &rhea);
    assert_eq!(rhea.style_id(), StyleId::Rhea);
}

#[test]
fn values_respect_min_max_and_invalid_bounds_are_safe() {
    let theme = Theme::light();
    let meter = Meter::new(&theme).min(10.0).max(50.0).value(30.0);
    assert!((target_ratio(&meter) - 0.5).abs() < f32::EPSILON);

    let meter = Meter::new(&theme).value(f32::NAN).max(-1.0);
    assert_eq!(meter.value, 0.0);
    assert!((target_ratio(&meter) - 0.0).abs() < f32::EPSILON);
}

#[test]
fn builder_exposes_bits_ui_and_extras_capabilities() {
    let theme = Theme::dark();
    let meter = Meter::new(&theme)
        .value(66.0)
        .min(0.0)
        .max(120.0)
        .size(MeterSize::Custom(10.0))
        .orientation(MeterOrientation::Vertical)
        .radius(MeterRadius::Custom(3.0))
        .color(AccentColor::Blue)
        .track_color(Color::from_rgb(0.1, 0.2, 0.3))
        .high_contrast(true)
        .animated(false)
        .width(Length::Fixed(18.0))
        .height(Length::Fixed(160.0))
        .transition_duration(Duration::from_millis(240))
        .warning_ratio(0.8);

    assert_eq!(meter.value, 66.0);
    assert_eq!(meter.max, 120.0);
    assert_eq!(meter.size, MeterSize::Custom(10.0));
    assert_eq!(meter.orientation, MeterOrientation::Vertical);
    assert_eq!(meter.radius, Some(MeterRadius::Custom(3.0)));
    assert_eq!(meter.color, Some(AccentColor::Blue));
    assert_eq!(meter.track_color, Some(Color::from_rgb(0.1, 0.2, 0.3)));
    assert!(meter.high_contrast);
    assert!(!meter.animated);
    assert_eq!(meter.width, Some(Length::Fixed(18.0)));
    assert_eq!(meter.height, Some(Length::Fixed(160.0)));
    assert_eq!(meter.transition_duration, Duration::from_millis(240));
    assert_eq!(meter.warning_ratio, 0.8);
}

#[test]
fn auto_tone_matches_extras_tokens_demo() {
    let theme = Theme::light();
    let meter = Meter::new(&theme).value(50.0).auto_tone(true);
    assert_eq!(meter.resolved_tone(), MeterFillTone::Default);

    let meter = Meter::new(&theme).value(80.0).auto_tone(true);
    assert_eq!(meter.resolved_tone(), MeterFillTone::Warning);

    let meter = Meter::new(&theme).value(100.0).auto_tone(true);
    assert_eq!(meter.resolved_tone(), MeterFillTone::Danger);
}

#[test]
fn auto_tone_colors_match_extras_tokens_thresholds() {
    let theme = Theme::light();

    let default = resolve_visual(
        &Meter::new(&theme)
            .value(50.0)
            .color(AccentColor::Blue)
            .auto_tone(true),
        &theme,
    );
    let warning = resolve_visual(
        &Meter::new(&theme)
            .value(80.0)
            .color(AccentColor::Blue)
            .auto_tone(true),
        &theme,
    );
    let danger = resolve_visual(
        &Meter::new(&theme)
            .value(100.0)
            .color(AccentColor::Blue)
            .auto_tone(true),
        &theme,
    );

    assert_eq!(
        default.indicator,
        theme.color_with_accent(
            AccentColor::Blue,
            twill_core::prelude::theme::SemanticColor::Primary
        )
    );
    assert_eq!(
        warning.indicator,
        theme.color_with_accent(
            AccentColor::Orange,
            twill_core::prelude::theme::SemanticColor::Primary
        )
    );
    assert_eq!(danger.indicator, theme.palette.destructive);
    assert!((default.track.a - 0.20).abs() < 1e-5);
    assert!((danger.track.a - 0.20).abs() < 1e-5);
}

#[test]
fn radius_is_capped_to_the_track_bounds() {
    let theme = Theme::light();
    let meter = Meter::new(&theme).radius(MeterRadius::Full);
    assert_eq!(radius_px(&theme, &meter, 200.0, 8.0), 4.0);

    let meter = Meter::new(&theme).radius(MeterRadius::Custom(-4.0));
    assert_eq!(radius_px(&theme, &meter, 200.0, 8.0), 0.0);
}

#[test]
fn track_derives_from_indicator_alpha_by_default() {
    let theme = Theme::light();
    let meter = Meter::new(&theme).custom_color(Color::from_rgba(0.2, 0.4, 0.8, 1.0));
    let visual = resolve_visual(&meter, &theme);

    assert_eq!(visual.indicator, Color::from_rgba(0.2, 0.4, 0.8, 1.0));
    assert!((visual.track.a - 0.20).abs() < 1e-5);
    assert!((visual.track.r - 0.2).abs() < 1e-5);
}

#[test]
fn custom_colors_override_theme_resolution() {
    let theme = Theme::light();
    let custom_indicator = Color::from_rgb(0.1, 0.2, 0.3);
    let custom_track = Color::from_rgb(0.3, 0.2, 0.1);
    let meter = Meter::new(&theme)
        .custom_color(custom_indicator)
        .track_color(custom_track);
    let visual = resolve_visual(&meter, &theme);

    assert_eq!(visual.indicator, custom_indicator);
    assert_eq!(visual.track, custom_track);
}

#[test]
fn meter_converts_to_canvas_and_element() {
    let theme = Theme::light();
    let meter = Meter::new(&theme).value(33.0);
    let _ = meter.into_canvas::<()>();

    let _: Element<'_, ()> = Meter::new(&theme).value(33.0).into();
}

#[test]
fn public_debug_is_non_empty() {
    let theme = Theme::light();
    let debug = format!("{:?}", Meter::new(&theme));
    assert!(debug.contains("Meter"));
}

#[test]
fn display_ratio_tracks_live_value_when_idle() {
    let theme = Theme::light();
    let idle = MeterState::default();
    let meter = Meter::new(&theme).value(30.0);
    assert!((display_ratio(&idle, &meter) - 0.30).abs() < f32::EPSILON);

    let meter = Meter::new(&theme).value(90.0);
    assert!((display_ratio(&idle, &meter) - 0.90).abs() < f32::EPSILON);
}

#[test]
fn settled_meter_does_not_schedule_redraws() {
    let theme = Theme::light();
    let meter = Meter::new(&theme).value(50.0).animated(true);
    let mut state = MeterState::default();
    let now = time::Instant::now();
    let event = canvas::Event::Window(window::Event::RedrawRequested(now));

    let action = <Meter<'_> as canvas::Program<()>>::update(
        &meter,
        &mut state,
        &event,
        Rectangle::with_size(crate::iced_compat::Size::new(200.0, 8.0)),
        mouse::Cursor::Unavailable,
    );

    assert!(action.is_none());
    assert!(!state.transition.is_running());
}
