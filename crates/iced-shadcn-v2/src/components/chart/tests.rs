//! Behavioral checks for the chart builder and configuration types.

use super::*;
use crate::theme::Theme;

fn sample_chart(theme: &Theme) -> Chart<'_> {
    Chart::bar(theme)
        .categories(["January", "February", "March"])
        .series(ChartSeries::new("Desktop", [186.0, 305.0, 237.0]))
}

#[test]
fn defaults_match_shadcn() {
    let theme = Theme::light();
    let chart = Chart::bar(&theme);

    assert_eq!(chart.kind(), ChartKind::Bar);
    assert_eq!(chart.axis, ChartAxis::Both);
    assert!(chart.grid);
    assert!(chart.tooltip);
    assert!(chart.highlight);
    assert!(chart.animated);
    assert!(!chart.legend);
    assert!(!chart.stacked);
    assert_eq!(chart.curve, ChartCurve::Linear);
    assert_eq!(chart.tooltip_indicator, ChartIndicator::Dot);
}

#[test]
fn sample_count_spans_series_and_categories() {
    let theme = Theme::light();
    let chart = Chart::line(&theme)
        .categories(["a", "b"])
        .series(ChartSeries::new("One", [1.0, 2.0, 3.0]));

    assert_eq!(chart.sample_count(), 3);
    assert_eq!(chart.category(1), "b");
    assert_eq!(chart.category(9), "");
}

#[test]
fn colors_cycle_by_series_index() {
    assert_eq!(ChartColor::from_index(0), ChartColor::Chart1);
    assert_eq!(ChartColor::from_index(4), ChartColor::Chart5);
    assert_eq!(ChartColor::from_index(5), ChartColor::Chart1);

    let series = ChartSeries::new("One", [1.0]);
    assert_eq!(series.resolved_color(2), ChartColor::Chart3);
    assert_eq!(
        series.clone().color(ChartColor::Chart5).resolved_color(2),
        ChartColor::Chart5,
    );
}

#[test]
fn point_colors_override_series_color() {
    let series = ChartSeries::new("One", [1.0, -2.0])
        .color(ChartColor::Chart1)
        .point_colors([None, Some(ChartColor::Chart2)]);

    assert_eq!(series.resolved_point_color(0, 0), ChartColor::Chart1);
    assert_eq!(series.resolved_point_color(1, 0), ChartColor::Chart2);
    assert_eq!(series.resolved_point_color(9, 0), ChartColor::Chart1);
}

#[test]
fn chart_colors_resolve_to_palette() {
    let theme = Theme::light();

    assert_eq!(ChartColor::Chart1.resolve(&theme), theme.palette.chart_1);
    assert_eq!(ChartColor::Chart5.resolve(&theme), theme.palette.chart_5);

    let custom = crate::iced_compat::Color::from_rgb(0.1, 0.2, 0.3);
    assert_eq!(ChartColor::Custom(custom).resolve(&theme), custom);
}

#[test]
fn numeric_knobs_are_clamped() {
    let theme = Theme::light();
    let chart = sample_chart(&theme)
        .bar_radius(-3.0)
        .donut(2.0)
        .band_padding(f32::NAN)
        .tick_count(0);

    assert_eq!(chart.bar_radius, Some(0.0));
    assert_eq!(chart.donut_fraction, 0.95);
    assert_eq!(
        chart.band_padding,
        shadcn_common::CHART_BAND_PADDING_FRACTION
    );
    assert_eq!(chart.tick_count, 1);

    let chart = sample_chart(&theme)
        .bar_radius(f32::INFINITY)
        .donut(f32::NAN);
    assert_eq!(chart.bar_radius, Some(0.0));
    assert_eq!(chart.donut_fraction, 0.0);
}

#[test]
fn style_pack_drives_default_bar_radius() {
    let vega = Theme::light();
    let lyra = Theme::from_resolved(
        shadcn_common::ResolvedTheme::default().with_style(shadcn_common::StyleId::Lyra),
    );

    assert!(sample_chart(&vega).resolved_bar_radius() > 0.0);
    assert_eq!(sample_chart(&lyra).resolved_bar_radius(), 0.0);
    assert_eq!(
        sample_chart(&lyra).bar_radius(8.0).resolved_bar_radius(),
        8.0,
    );
}

#[test]
fn formatters_apply() {
    let theme = Theme::light();
    let chart = sample_chart(&theme)
        .category_format(|label| label.chars().take(3).collect())
        .value_format(|value| format!("{value}%"));

    assert_eq!(chart.format_category("January"), "Jan");
    assert_eq!(chart.format_value(5.0), "5%");
    assert_eq!(chart.format_tooltip_label("January"), "January");
}

#[test]
fn default_value_format_groups_thousands() {
    let theme = Theme::light();
    let chart = sample_chart(&theme);

    assert_eq!(chart.format_value(1234.0), "1,234");
}

#[test]
fn debug_output_is_not_empty() {
    let theme = Theme::light();
    let chart = sample_chart(&theme);
    let debug = format!("{chart:?}");

    assert!(debug.contains("Chart"));
    assert!(debug.contains("Bar"));
}

#[test]
fn chart_converts_into_element() {
    let theme = Theme::light();
    let _: crate::iced_compat::Element<'_, ()> = sample_chart(&theme).into();
}
