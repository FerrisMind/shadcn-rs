//! Behavioral tests for the scroll-area component.

use crate::iced_compat::widget::scrollable::{self, Anchor, Direction};
use crate::iced_compat::widget::text;
use crate::iced_compat::{Color, Element, Length};
use shadcn_common::StyleId;
use twill_core::prelude::{Padding, PaddingValue, PaddingVar, Spacing};

use super::style::Tokens;
use super::*;
use crate::theme::Theme;

#[derive(Clone, Debug)]
enum Message {
    Scrolled,
}

/// The status iced reports for an idle scroll area whose content overflows.
const ACTIVE: scrollable::Status = scrollable::Status::Active {
    is_horizontal_scrollbar_disabled: false,
    is_vertical_scrollbar_disabled: false,
};

fn tokens(theme: &Theme) -> Tokens<'_> {
    Tokens {
        theme,
        frame_radius: 0.0,
        bordered: false,
        background: None,
        thumb_radius: geometry::thumb_radius_px(theme, ScrollAreaRadius::Theme),
        track_color: None,
        thumb_color: None,
    }
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let area: ScrollArea<'_, Message> = ScrollArea::new(text("Notes"), &theme)
        .orientation(ScrollAreaOrientation::Both)
        .width(Length::Fixed(350.0))
        .height(Length::Fixed(200.0))
        .radius(ScrollAreaRadius::Medium)
        .thumb_radius(ScrollAreaRadius::None)
        .bordered(true)
        .background(Color::WHITE)
        .track_color(theme.palette.muted)
        .thumb_color(theme.palette.primary)
        .auto_scroll(true)
        .id(crate::iced_compat::widget::Id::new("notes"))
        .on_scroll(|_viewport| Message::Scrolled);

    assert_eq!(area.orientation, ScrollAreaOrientation::Both);
    assert_eq!(area.width, Some(Length::Fixed(350.0)));
    assert_eq!(area.height, Some(Length::Fixed(200.0)));
    assert_eq!(area.radius, ScrollAreaRadius::Medium);
    assert_eq!(area.thumb_radius, ScrollAreaRadius::None);
    assert!(area.bordered);
    assert_eq!(area.background, Some(Color::WHITE));
    assert_eq!(area.track_color, Some(theme.palette.muted));
    assert_eq!(area.thumb_color, Some(theme.palette.primary));
    assert!(area.auto_scroll);
    assert!(area.id.is_some());
    assert!(area.on_scroll.is_some());
    assert!(std::ptr::eq(area.theme, &theme));
}

#[test]
fn defaults_match_the_reference_component() {
    let theme = Theme::light();
    let area = ScrollArea::<Message>::new(text("Notes"), &theme);

    assert_eq!(area.orientation, ScrollAreaOrientation::Vertical);
    assert_eq!(area.vertical, ScrollAreaScrollbar::default());
    assert_eq!(area.horizontal, ScrollAreaScrollbar::default());
    assert_eq!(area.radius, ScrollAreaRadius::Theme);
    assert!(!area.bordered);
    assert!(!area.auto_scroll);
    assert_eq!(area.width, None);
    assert_eq!(area.height, None);
    assert_eq!(area.padding, None);
}

#[test]
fn scroll_areas_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = ScrollArea::new(text("Notes"), &theme)
        .height(Length::Fixed(200.0))
        .into();

    let _ = ScrollArea::<Message>::new(text("Notes"), &theme).into_scrollable();
}

#[test]
fn scrollbar_defaults_reproduce_the_source_geometry() {
    // `.cn-scroll-area-scrollbar` is `w-2.5` with `p-px` around the thumb.
    let scrollbar = ScrollAreaScrollbar::default();
    assert_eq!(scrollbar.width, 10.0);
    assert_eq!(scrollbar.padding, 1.0);
    assert_eq!(scrollbar.thumb_width(), 8.0);
    assert!(!scrollbar.is_hidden());
    assert_eq!(scrollbar.spacing, None);
}

#[test]
fn scrollbar_measurements_are_validated() {
    let scrollbar = ScrollAreaScrollbar::new()
        .width(f32::NAN)
        .padding(-4.0)
        .margin(f32::INFINITY)
        .spacing(-1.0);

    assert_eq!(scrollbar.width, 0.0);
    assert_eq!(scrollbar.padding, 0.0);
    assert_eq!(scrollbar.margin, 0.0);
    assert_eq!(scrollbar.spacing, Some(0.0));
    assert_eq!(scrollbar.thumb_width(), 0.0);

    // An inset wider than the rail collapses the thumb instead of inverting it.
    assert_eq!(ScrollAreaScrollbar::new().padding(9.0).thumb_width(), 0.0);
}

#[test]
fn orientation_maps_to_the_matching_iced_direction() {
    let vertical = ScrollAreaScrollbar::new().width(8.0).padding(1.0);
    let horizontal = ScrollAreaScrollbar::new().width(4.0).padding(0.0);

    assert!(matches!(
        geometry::direction(ScrollAreaOrientation::Vertical, vertical, horizontal),
        Direction::Vertical(_)
    ));
    assert!(matches!(
        geometry::direction(ScrollAreaOrientation::Horizontal, vertical, horizontal),
        Direction::Horizontal(_)
    ));

    let Direction::Both {
        vertical: y,
        horizontal: x,
    } = geometry::direction(ScrollAreaOrientation::Both, vertical, horizontal)
    else {
        panic!("`Both` maps onto a two-axis direction");
    };

    assert_eq!(
        y,
        scrollable::Scrollbar::new()
            .width(8.0)
            .scroller_width(6.0)
            .anchor(Anchor::Start)
    );
    assert_eq!(
        x,
        scrollable::Scrollbar::new()
            .width(4.0)
            .scroller_width(4.0)
            .anchor(Anchor::Start)
    );
}

#[test]
fn hidden_rails_keep_scrolling_without_reserving_space() {
    let hidden = ScrollAreaScrollbar::hidden();
    let direction = geometry::direction(ScrollAreaOrientation::Vertical, hidden, hidden);

    let Direction::Vertical(rail) = direction else {
        panic!("a vertical orientation maps onto a vertical direction");
    };

    assert_eq!(rail, scrollable::Scrollbar::hidden());
}

#[test]
fn end_anchored_rails_pin_content_to_the_trailing_edge() {
    let anchored = ScrollAreaScrollbar::new().anchor(ScrollAreaAnchor::End);
    let direction = geometry::direction(ScrollAreaOrientation::Vertical, anchored, anchored);

    let Direction::Vertical(rail) = direction else {
        panic!("a vertical orientation maps onto a vertical direction");
    };

    assert_eq!(
        rail,
        scrollable::Scrollbar::new()
            .width(10.0)
            .scroller_width(8.0)
            .anchor(Anchor::End)
    );
}

#[test]
fn per_axis_scrollbars_are_configured_independently() {
    let theme = Theme::light();
    let slim = ScrollAreaScrollbar::new().width(4.0);

    let both = ScrollArea::<Message>::new(text("Notes"), &theme).scrollbar(slim);
    assert_eq!(both.vertical, slim);
    assert_eq!(both.horizontal, slim);

    let per_axis = ScrollArea::<Message>::new(text("Notes"), &theme)
        .vertical_scrollbar(ScrollAreaScrollbar::hidden())
        .horizontal_scrollbar(slim);
    assert!(per_axis.vertical.is_hidden());
    assert_eq!(per_axis.horizontal, slim);
}

#[test]
fn padding_maps_all_four_sides() {
    let theme = Theme::light();
    let area = ScrollArea::<Message>::new(text("Notes"), &theme)
        .padding(Padding::individual(
            Spacing::S1,
            Spacing::S2,
            Spacing::S3,
            Spacing::S4,
        ))
        .expect("scale padding is supported");

    assert_eq!(
        area.padding,
        Some(crate::iced_compat::Padding {
            top: 4.0,
            right: 8.0,
            bottom: 12.0,
            left: 16.0,
        })
    );
}

#[test]
fn padding_variable_returns_a_descriptive_error() {
    let theme = Theme::light();
    let error = ScrollArea::<Message>::new(text("Notes"), &theme)
        .padding(Padding::individual_value(
            PaddingValue::Var(PaddingVar::new("--scroll-area-padding")),
            PaddingValue::Px(2.0),
            PaddingValue::Px(3.0),
            PaddingValue::Px(4.0),
        ))
        .expect_err("padding variables are unsupported");

    assert_eq!(
        error,
        ScrollAreaBuildError::UnsupportedPaddingVariable {
            name: "--scroll-area-padding"
        }
    );
    assert!(error.to_string().contains("--scroll-area-padding"));
}

#[test]
fn padding_auto_returns_a_descriptive_error() {
    let theme = Theme::light();
    let error = ScrollArea::<Message>::new(text("Notes"), &theme)
        .padding(Padding::all(Spacing::Auto))
        .expect_err("auto padding is unsupported");

    assert_eq!(error, ScrollAreaBuildError::UnsupportedPaddingAuto);
    assert!(error.to_string().contains("auto"));
}

#[test]
fn thumb_radius_follows_the_style_pack() {
    // `.cn-scroll-area-thumb` is `rounded-full` everywhere except Lyra and Sera.
    for style in [StyleId::Vega, StyleId::Nova, StyleId::Maia, StyleId::Rhea] {
        let theme = Theme::light().with_style(style);
        assert!(geometry::thumb_radius_px(&theme, ScrollAreaRadius::Theme) > 100.0);
    }

    for style in [StyleId::Lyra, StyleId::Sera] {
        let theme = Theme::light().with_style(style);
        assert_eq!(
            geometry::thumb_radius_px(&theme, ScrollAreaRadius::Theme),
            0.0
        );
    }
}

#[test]
fn frame_radius_is_square_until_it_is_asked_for() {
    let theme = Theme::light();

    // The reference viewport is `rounded-[inherit]`: no radius of its own.
    assert_eq!(
        geometry::frame_radius_px(&theme, ScrollAreaRadius::Theme),
        0.0
    );
    assert_eq!(
        geometry::frame_radius_px(&theme, ScrollAreaRadius::None),
        0.0
    );
    assert_eq!(
        geometry::frame_radius_px(&theme, ScrollAreaRadius::Medium),
        theme.style.radius.md_px
    );
    assert_eq!(
        geometry::frame_radius_px(&theme, ScrollAreaRadius::Custom(12.0)),
        12.0
    );
    // Invalid custom radii fall back to square corners.
    assert_eq!(
        geometry::frame_radius_px(&theme, ScrollAreaRadius::Custom(f32::NAN)),
        0.0
    );
    assert_eq!(
        geometry::frame_radius_px(&theme, ScrollAreaRadius::Custom(-8.0)),
        0.0
    );
}

#[test]
fn default_style_paints_a_transparent_rail_with_a_border_thumb() {
    let theme = Theme::light();
    let style = style::resolve_scroll_area_style(tokens(&theme), ACTIVE);

    assert!(style.vertical_rail.background.is_none());
    assert_eq!(
        style.vertical_rail.scroller.background,
        crate::iced_compat::Background::Color(theme.palette.border)
    );
    assert!(style.gap.is_none());
    assert!(style.container.background.is_none());
    assert_eq!(style.container.border.width, 0.0);
}

#[test]
fn bordered_frame_paints_a_hairline_with_the_border_token() {
    let theme = Theme::light();
    let style = style::resolve_scroll_area_style(
        Tokens {
            bordered: true,
            frame_radius: 8.0,
            background: Some(theme.palette.card),
            ..tokens(&theme)
        },
        ACTIVE,
    );

    assert_eq!(style.container.border.width, 1.0);
    assert_eq!(style.container.border.color, theme.palette.border);
    assert_eq!(
        style.container.background,
        Some(crate::iced_compat::Background::Color(theme.palette.card))
    );
}

#[test]
fn hover_and_drag_emphasise_only_the_pointed_axis() {
    let theme = Theme::light();
    let tokens = tokens(&theme);
    let idle = style::resolve_scroll_area_style(tokens, ACTIVE)
        .vertical_rail
        .scroller
        .background;

    let hovered = style::resolve_scroll_area_style(
        tokens,
        scrollable::Status::Hovered {
            is_horizontal_scrollbar_hovered: false,
            is_vertical_scrollbar_hovered: true,
            is_horizontal_scrollbar_disabled: false,
            is_vertical_scrollbar_disabled: false,
        },
    );
    assert_ne!(hovered.vertical_rail.scroller.background, idle);
    assert_eq!(hovered.horizontal_rail.scroller.background, idle);

    let dragged = style::resolve_scroll_area_style(
        tokens,
        scrollable::Status::Dragged {
            is_horizontal_scrollbar_dragged: false,
            is_vertical_scrollbar_dragged: true,
            is_horizontal_scrollbar_disabled: false,
            is_vertical_scrollbar_disabled: false,
        },
    );
    assert_ne!(dragged.vertical_rail.scroller.background, idle);
    assert_ne!(
        dragged.vertical_rail.scroller.background,
        hovered.vertical_rail.scroller.background
    );
}

#[test]
fn a_disabled_axis_keeps_its_thumb_invisible() {
    let theme = Theme::light();
    let style = style::resolve_scroll_area_style(
        tokens(&theme),
        scrollable::Status::Active {
            is_horizontal_scrollbar_disabled: true,
            is_vertical_scrollbar_disabled: false,
        },
    );

    assert_eq!(
        style.horizontal_rail.scroller.background,
        crate::iced_compat::Background::Color(Color::TRANSPARENT)
    );
    assert_ne!(
        style.vertical_rail.scroller.background,
        crate::iced_compat::Background::Color(Color::TRANSPARENT)
    );
}

#[test]
fn thumb_and_track_overrides_beat_the_theme_tokens() {
    let theme = Theme::light();
    let style = style::resolve_scroll_area_style(
        Tokens {
            track_color: Some(theme.palette.muted),
            thumb_color: Some(theme.palette.primary),
            ..tokens(&theme)
        },
        ACTIVE,
    );

    assert_eq!(
        style.vertical_rail.background,
        Some(crate::iced_compat::Background::Color(theme.palette.muted))
    );
    assert_eq!(
        style.vertical_rail.scroller.background,
        crate::iced_compat::Background::Color(theme.palette.primary)
    );
}

#[test]
fn every_status_resolves_in_light_and_dark_themes() {
    for theme in [Theme::light(), Theme::dark()] {
        for disabled in [false, true] {
            for status in [
                scrollable::Status::Active {
                    is_horizontal_scrollbar_disabled: disabled,
                    is_vertical_scrollbar_disabled: disabled,
                },
                scrollable::Status::Hovered {
                    is_horizontal_scrollbar_hovered: true,
                    is_vertical_scrollbar_hovered: true,
                    is_horizontal_scrollbar_disabled: disabled,
                    is_vertical_scrollbar_disabled: disabled,
                },
                scrollable::Status::Dragged {
                    is_horizontal_scrollbar_dragged: true,
                    is_vertical_scrollbar_dragged: true,
                    is_horizontal_scrollbar_disabled: disabled,
                    is_vertical_scrollbar_disabled: disabled,
                },
            ] {
                let style = style::resolve_scroll_area_style(tokens(&theme), status);
                assert!(style.auto_scroll.icon.a.is_finite());
                assert!(style.horizontal_rail.scroller.border.radius.top_left >= 0.0);
            }
        }
    }
}

#[test]
fn style_override_runs_after_internal_resolution() {
    let theme = Theme::light();
    let area = ScrollArea::<Message>::new(text("Notes"), &theme).style_override(|mut style, _| {
        style.gap = Some(crate::iced_compat::Background::Color(Color::BLACK));
        style
    });

    assert!(area.style_override.is_some());
    let _ = area.into_scrollable();
}

#[test]
fn orientation_reports_the_axes_it_mounts() {
    assert!(ScrollAreaOrientation::Vertical.has_vertical());
    assert!(!ScrollAreaOrientation::Vertical.has_horizontal());
    assert!(ScrollAreaOrientation::Horizontal.has_horizontal());
    assert!(!ScrollAreaOrientation::Horizontal.has_vertical());
    assert!(ScrollAreaOrientation::Both.has_vertical());
    assert!(ScrollAreaOrientation::Both.has_horizontal());
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let area = ScrollArea::<NoDebugMessage>::new(text("Notes"), &theme);
    let debug = format!("{area:?}");

    assert!(debug.contains("ScrollArea"));
    assert!(debug.contains("orientation"));
}
