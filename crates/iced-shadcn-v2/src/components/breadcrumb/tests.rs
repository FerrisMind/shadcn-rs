//! Behavioral tests for the breadcrumb component.

use crate::iced_compat::widget::text;
use crate::iced_compat::{Color, Element, Length, Padding};
use shadcn_common::StyleId;

use super::geometry;
use super::style;
use super::*;
use crate::theme::Theme;

struct NoDebugMessage;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Message {
    Home,
    Components,
}

#[test]
fn root_defaults_match_the_source_nav() {
    let theme = Theme::light();
    let breadcrumb: Breadcrumb<'_, NoDebugMessage> = Breadcrumb::new(&theme);

    assert_eq!(breadcrumb.width, Length::Shrink);
    assert_eq!(breadcrumb.height, Length::Shrink);
    assert_eq!(breadcrumb.padding, Padding::ZERO);
    assert_eq!(breadcrumb.accessible_label(), "breadcrumb");
    assert!(breadcrumb.list.is_empty());
    assert!(std::ptr::eq(breadcrumb.theme, &theme));
}

#[test]
fn list_defaults_wrap_like_the_source_flex_row() {
    let theme = Theme::light();
    let list: BreadcrumbList<'_, NoDebugMessage> = BreadcrumbList::new(&theme);

    assert!(list.wrap);
    assert_eq!(list.width, Length::Shrink);
    assert!(list.spacing.is_none());
    assert!(list.line_spacing.is_none());
    assert!(list.color.is_none());
    assert_eq!(list.len(), 0);
}

#[test]
fn root_forwards_entries_into_the_wrapped_list() {
    let theme = Theme::light();
    let breadcrumb = Breadcrumb::new(&theme)
        .push(BreadcrumbLink::text("Home", &theme).on_press(Message::Home))
        .push_separator()
        .push(BreadcrumbPage::text("Breadcrumb", &theme));

    assert_eq!(breadcrumb.list.len(), 3);

    let _: Element<'_, Message> = breadcrumb.into();
}

#[test]
fn replacing_the_list_keeps_entries_already_pushed() {
    let theme = Theme::light();
    let breadcrumb = Breadcrumb::<NoDebugMessage>::new(&theme)
        .push(BreadcrumbPage::text("Breadcrumb", &theme))
        .list(BreadcrumbList::new(&theme).spacing(10.0).wrap(false));

    assert_eq!(breadcrumb.list.len(), 1);
    assert_eq!(breadcrumb.list.spacing, Some(10.0));
    assert!(!breadcrumb.list.wrap);
}

#[test]
fn entries_report_the_source_data_slot() {
    let theme = Theme::light();

    let item: BreadcrumbEntry<'_, NoDebugMessage> = BreadcrumbItem::new(&theme).into();
    let separator: BreadcrumbEntry<'_, NoDebugMessage> = BreadcrumbSeparator::new(&theme).into();
    let link: BreadcrumbEntry<'_, NoDebugMessage> = BreadcrumbLink::text("Home", &theme).into();
    let page: BreadcrumbEntry<'_, NoDebugMessage> = BreadcrumbPage::text("Now", &theme).into();
    let ellipsis: BreadcrumbEntry<'_, NoDebugMessage> = BreadcrumbEllipsis::new(&theme).into();
    let element: BreadcrumbEntry<'_, NoDebugMessage> = BreadcrumbEntry::element(text("raw"));

    assert_eq!(item.slot(), "breadcrumb-item");
    assert_eq!(separator.slot(), "breadcrumb-separator");
    assert_eq!(link.slot(), "breadcrumb-link");
    assert_eq!(page.slot(), "breadcrumb-page");
    assert_eq!(ellipsis.slot(), "breadcrumb-ellipsis");
    assert_eq!(element.slot(), "element");
}

#[test]
fn items_compose_links_pages_and_arbitrary_widgets() {
    let theme = Theme::light();
    let item = BreadcrumbItem::new(&theme)
        .push(BreadcrumbEllipsis::new(&theme))
        .push(BreadcrumbLink::text("Components", &theme).on_press(Message::Components))
        .push(BreadcrumbEntry::element(text("extra")));

    assert_eq!(item.len(), 3);
    assert!(!item.is_empty());

    let _: Element<'_, Message> = item.into();
}

#[test]
fn every_part_converts_to_an_element_standalone() {
    let theme = Theme::light();

    let _: Element<'_, Message> = BreadcrumbList::new(&theme).push_separator().into();
    let _: Element<'_, Message> = BreadcrumbItem::new(&theme).into();
    let _: Element<'_, Message> = BreadcrumbLink::text("Home", &theme).into();
    let _: Element<'_, Message> = BreadcrumbLink::new(text("Home"), &theme).into();
    let _: Element<'_, Message> = BreadcrumbPage::text("Now", &theme).into();
    let _: Element<'_, Message> = BreadcrumbPage::new(text("Now"), &theme).into();
    let _: Element<'_, Message> = BreadcrumbSeparator::new(&theme).into();
    let _: Element<'_, Message> = BreadcrumbSeparator::text("/", &theme).into();
    let _: Element<'_, Message> = BreadcrumbSeparator::with_content(text(">"), &theme).into();
    let _: Element<'_, Message> = BreadcrumbEllipsis::new(&theme).into();
}

#[test]
fn debug_does_not_require_message_debug() {
    let theme = Theme::light();
    let breadcrumb = Breadcrumb::<NoDebugMessage>::new(&theme)
        .push(BreadcrumbLink::text("Home", &theme))
        .push_separator()
        .push(BreadcrumbEllipsis::new(&theme));

    let debug = format!("{breadcrumb:?}");

    assert!(debug.contains("Breadcrumb"));
    assert!(debug.contains("breadcrumb-separator"));
    assert!(debug.contains("breadcrumb-ellipsis"));
}

#[test]
fn link_carries_its_href_and_suppresses_press_when_disabled() {
    let theme = Theme::light();
    let link = BreadcrumbLink::<Message>::text("Home", &theme)
        .href("/")
        .on_press(Message::Home);

    assert_eq!(link.associated_href(), Some("/"));
    assert_eq!(link.on_press, Some(Message::Home));

    let disabled = link.disabled(true);
    assert!(disabled.disabled);

    let cleared = BreadcrumbLink::<Message>::text("Home", &theme).on_press_maybe(None);
    assert!(cleared.on_press.is_none());
    assert!(cleared.associated_href().is_none());
}

#[test]
fn separator_defaults_to_the_chevron_glyph() {
    let theme = Theme::light();

    let default: BreadcrumbSeparator<'_, NoDebugMessage> = BreadcrumbSeparator::new(&theme);
    let custom: BreadcrumbSeparator<'_, NoDebugMessage> = BreadcrumbSeparator::text("/", &theme);

    assert!(default.is_default_glyph());
    assert!(!custom.is_default_glyph());
}

#[test]
fn ellipsis_defaults_to_the_source_screen_reader_label() {
    let theme = Theme::light();
    let ellipsis: BreadcrumbEllipsis<'_, NoDebugMessage> = BreadcrumbEllipsis::new(&theme);

    assert_eq!(ellipsis.screen_reader_label(), "More");
    assert_eq!(
        ellipsis.sr_label("Show more").screen_reader_label(),
        "Show more"
    );
}

#[test]
fn metrics_follow_the_source_style_css() {
    let cases = [
        (StyleId::Vega, 6.0, 6.0, 14.0, 20.0, 20.0, 16.0),
        (StyleId::Nova, 6.0, 4.0, 14.0, 20.0, 20.0, 16.0),
        (StyleId::Maia, 6.0, 6.0, 14.0, 20.0, 20.0, 16.0),
        (StyleId::Lyra, 6.0, 4.0, 12.0, 16.0, 20.0, 16.0),
        (StyleId::Mira, 6.0, 4.0, 12.0, 19.5, 16.0, 14.0),
        (StyleId::Luma, 6.0, 6.0, 14.0, 20.0, 20.0, 16.0),
        (StyleId::Sera, 6.0, 6.0, 12.0, 16.0, 20.0, 16.0),
        (StyleId::Rhea, 6.0, 6.0, 14.0, 20.0, 20.0, 16.0),
    ];

    for (style_id, list_gap, item_gap, text_size, line_height, ellipsis_box, ellipsis_icon) in cases
    {
        let theme = Theme::light().with_style(style_id);
        let metrics = geometry::metrics(&theme);

        assert_eq!(metrics.list_gap_px, list_gap, "{style_id:?}");
        assert_eq!(metrics.item_gap_px, item_gap, "{style_id:?}");
        assert_eq!(metrics.text_size_px, text_size, "{style_id:?}");
        assert_eq!(metrics.line_height_px, line_height, "{style_id:?}");
        assert_eq!(metrics.separator_icon_px, 14.0, "{style_id:?}");
        assert_eq!(metrics.ellipsis_box_px, ellipsis_box, "{style_id:?}");
        assert_eq!(metrics.ellipsis_icon_px, ellipsis_icon, "{style_id:?}");
    }
}

#[test]
fn only_sera_uppercases_the_trail() {
    for style_id in [
        StyleId::Vega,
        StyleId::Nova,
        StyleId::Maia,
        StyleId::Lyra,
        StyleId::Mira,
        StyleId::Luma,
        StyleId::Rhea,
    ] {
        let theme = Theme::light().with_style(style_id);
        assert!(!geometry::metrics(&theme).uppercase, "{style_id:?}");
    }

    let sera = Theme::light().with_style(StyleId::Sera);
    let metrics = geometry::metrics(&sera);

    assert!(metrics.uppercase);
    assert_eq!(metrics.tracking_em, 0.025);
}

#[test]
fn list_overrides_replace_the_inherited_tokens() {
    let theme = Theme::light();
    let magenta = Color::from_rgb(1.0, 0.0, 1.0);
    let list = BreadcrumbList::<NoDebugMessage>::new(&theme)
        .color(magenta)
        .text_size(11.0)
        .line_height(13.0)
        .line_spacing(3.0);

    assert_eq!(list.color, Some(magenta));
    assert_eq!(list.text_size, Some(11.0));
    assert_eq!(list.line_height, Some(13.0));
    assert_eq!(list.line_spacing, Some(3.0));
}

#[test]
fn links_move_from_muted_to_foreground_on_hover() {
    let theme = Theme::light();
    let resting = style::muted_color(&theme);
    let hovered = style::current_color(&theme);

    assert_ne!(resting, hovered);
    assert_eq!(resting, theme.palette.muted_foreground);
    assert_eq!(hovered, theme.palette.foreground);

    for status in [
        button_widget::Status::Active,
        button_widget::Status::Disabled,
    ] {
        let style = style::resolve_link_style(resting, hovered, status);

        assert_eq!(style.text_color, resting);
        assert!(style.background.is_none());
        assert_eq!(style.border.width, 0.0);
    }

    for status in [
        button_widget::Status::Hovered,
        button_widget::Status::Pressed,
    ] {
        assert_eq!(
            style::resolve_link_style(resting, hovered, status).text_color,
            hovered
        );
    }
}

#[test]
fn layout_and_typography_values_normalize_invalid_input() {
    assert_eq!(geometry::normalize_px(-1.0), 0.0);
    assert_eq!(geometry::normalize_px(f32::NAN), 0.0);
    assert_eq!(geometry::normalize_px(f32::INFINITY), 0.0);
    assert_eq!(geometry::normalize_min_px(0.0), 1.0);
    assert_eq!(geometry::normalize_min_px(f32::NAN), 1.0);

    let theme = Theme::light();
    let list = BreadcrumbList::<NoDebugMessage>::new(&theme)
        .spacing(f32::NAN)
        .text_size(-4.0);
    let breadcrumb = Breadcrumb::<NoDebugMessage>::new(&theme).padding(Padding {
        top: -1.0,
        right: f32::NAN,
        bottom: f32::INFINITY,
        left: 4.0,
    });

    assert_eq!(list.spacing, Some(0.0));
    assert_eq!(list.text_size, Some(1.0));
    assert_eq!(breadcrumb.padding.top, 0.0);
    assert_eq!(breadcrumb.padding.right, 0.0);
    assert_eq!(breadcrumb.padding.bottom, 0.0);
    assert_eq!(breadcrumb.padding.left, 4.0);
}

#[test]
fn style_overrides_are_stored_without_forcing_message_debug() {
    let theme = Theme::light();
    let magenta = Color::from_rgb(1.0, 0.0, 1.0);

    let breadcrumb = Breadcrumb::<NoDebugMessage>::new(&theme).style_override(|mut style| {
        style.text_color = Some(magenta);
        style
    });
    let link = BreadcrumbLink::<NoDebugMessage>::text("Home", &theme).style_override(
        |mut style, _status| {
            style.text_color = magenta;
            style
        },
    );
    let ellipsis = BreadcrumbEllipsis::<NoDebugMessage>::new(&theme).button_style_override(
        |mut style, _status| {
            style.text_color = magenta;
            style
        },
    );

    assert!(breadcrumb.style_override.is_some());
    assert!(link.style_override.is_some());
    assert!(ellipsis.style_override.is_some());
}

#[test]
fn extend_appends_every_entry_of_an_iterator() {
    let theme = Theme::light();
    let list = BreadcrumbList::<NoDebugMessage>::new(&theme).extend([
        BreadcrumbEntry::from(BreadcrumbLink::text("Home", &theme)),
        BreadcrumbEntry::from(BreadcrumbSeparator::new(&theme)),
        BreadcrumbEntry::from(BreadcrumbPage::text("Now", &theme)),
    ]);
    let item = BreadcrumbItem::<NoDebugMessage>::with_children(
        &theme,
        [BreadcrumbEntry::from(BreadcrumbEllipsis::new(&theme))],
    );

    assert_eq!(list.len(), 3);
    assert_eq!(item.len(), 1);
}
