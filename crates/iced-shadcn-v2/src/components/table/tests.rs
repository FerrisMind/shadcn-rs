//! Behavioral tests for the table component.

use crate::iced_compat::alignment::{Horizontal, Vertical};
use crate::iced_compat::widget::{container, text};
use crate::iced_compat::{Color, Element, Length, Padding};
use crate::theme::Theme;
use shadcn_common::{FontWeight, StyleId};

use super::style::{cell_padding, hover_row_style, metrics, row_style, with_alpha};
use super::*;

#[test]
fn style_pack_metrics_match_shadcn_table_css() {
    let nova = Theme::light().with_style(StyleId::Nova);
    let nova_metrics = metrics(&nova);
    assert_eq!(nova_metrics.text_size, 14.0);
    assert_eq!(nova_metrics.line_height, 20.0);
    assert_eq!(nova_metrics.header_height, 40.0);
    assert_eq!(nova_metrics.header_padding_x, 8.0);
    assert_eq!(nova_metrics.cell_padding, 8.0);

    let sera = Theme::light().with_style(StyleId::Sera);
    let sera_metrics = metrics(&sera);
    assert_eq!(sera_metrics.header_height, 48.0);
    assert_eq!(sera_metrics.header_padding_x, 12.0);
    assert_eq!(sera_metrics.header_text_size, 12.0);
    assert!(sera_metrics.header_uppercase);
    assert!(sera_metrics.header_is_muted);

    let mira = Theme::light().with_style(StyleId::Mira);
    let mira_metrics = metrics(&mira);
    assert_eq!(mira_metrics.text_size, 12.0);
    assert_eq!(mira_metrics.line_height, 16.0);
}

#[test]
fn cell_padding_matches_header_and_body_rules() {
    let theme = Theme::light().with_style(StyleId::Nova);
    assert_eq!(
        cell_padding(&theme, true),
        Padding {
            top: 0.0,
            right: 8.0,
            bottom: 0.0,
            left: 8.0,
        }
    );
    assert_eq!(cell_padding(&theme, false), Padding::from(8.0));
}

#[test]
fn alignment_type_maps_to_iced_alignment() {
    assert_eq!(
        Horizontal::from(TableCellAlignment::Start),
        Horizontal::Left
    );
    assert_eq!(
        Horizontal::from(TableCellAlignment::Center),
        Horizontal::Center
    );
    assert_eq!(Horizontal::from(TableCellAlignment::End), Horizontal::Right);
    assert_eq!(TableCellAlignment::default(), TableCellAlignment::Start);
}

#[test]
fn row_style_uses_semantic_surfaces() {
    let theme = Theme::light().with_style(StyleId::Nova);
    let body = row_style(&theme, SectionKind::Body, false, true);
    assert_eq!(body.background, None);
    assert_eq!(body.border.width, 1.0);
    assert_eq!(body.border.color, theme.palette.border);

    let selected = row_style(&theme, SectionKind::Body, true, false);
    assert_eq!(
        selected.background,
        Some(crate::iced_compat::Background::Color(theme.palette.muted))
    );
    assert_eq!(selected.border.width, 0.0);

    let footer = row_style(&theme, SectionKind::Footer, false, false);
    assert_eq!(
        footer.background,
        Some(crate::iced_compat::Background::Color(with_alpha(
            theme.palette.muted,
            0.5,
        )))
    );
}

#[test]
fn hover_style_changes_unselected_body_surface() {
    let theme = Theme::light().with_style(StyleId::Vega);
    let resting = row_style(&theme, SectionKind::Body, false, true);
    let hovered = hover_row_style(&theme, SectionKind::Body, false, true);
    let selected = hover_row_style(&theme, SectionKind::Body, true, true);

    assert_ne!(hovered.background, resting.background);
    assert_eq!(
        hovered.background,
        Some(crate::iced_compat::Background::Color(with_alpha(
            theme.palette.muted,
            0.5,
        )))
    );
    assert_eq!(
        selected.background,
        Some(crate::iced_compat::Background::Color(theme.palette.muted))
    );
}

#[test]
fn builders_cover_all_slots_and_arbitrary_content() {
    #[derive(Debug, Clone, PartialEq)]
    struct Clicked;

    let theme = Theme::dark().with_style(StyleId::Nova);
    let table = Table::new(&theme)
        .min_width(560.0)
        .column_widths([
            Length::Fixed(100.0),
            Length::Fill,
            Length::Fill,
            Length::Fill,
        ])
        .caption(TableCaption::text(
            "A list of your recent invoices.",
            &theme,
        ))
        .header(
            TableHeader::new(&theme).push(
                TableRow::new(&theme)
                    .head(TableHead::text("Invoice", &theme))
                    .head(TableHead::text("Status", &theme))
                    .head(TableHead::text("Method", &theme))
                    .head(TableHead::text("Amount", &theme).align_x(Horizontal::Right)),
            ),
        )
        .body(
            TableBody::new(&theme).extend([
                TableRow::new(&theme)
                    .cell(TableCell::text("INV001", &theme))
                    .cell(TableCell::text("Paid", &theme))
                    .cell(TableCell::new(text("Credit Card"), &theme))
                    .cell(TableCell::text("$250.00", &theme).align_x(Horizontal::Right)),
                TableRow::new(&theme)
                    .selected(true)
                    .hoverable(false)
                    .cell(TableCell::text("Total", &theme).span(3))
                    .cell(TableCell::text("$250.00", &theme).align_x(Horizontal::Right)),
            ]),
        )
        .footer(
            TableFooter::new(&theme).push(
                TableRow::new(&theme)
                    .cell(TableCell::text("Total", &theme).span(3))
                    .cell(TableCell::text("$250.00", &theme).align_x(Horizontal::Right)),
            ),
        )
        .style_override(|mut style| {
            style.border.width = 1.0;
            style
        });

    assert!(format!("{table:?}").contains("Table"));
    let _: Element<'_, Clicked> = table.into();
}

#[test]
fn builders_normalize_invalid_numeric_inputs() {
    let theme = Theme::light();
    let cell = TableCell::<()>::text("value", &theme)
        .span(0)
        .font_weight(FontWeight::Medium)
        .text_size(f32::NAN)
        .line_height(-4.0)
        .padding(Padding::ZERO);
    assert_eq!(cell.config.span, 1);
    assert_eq!(cell.config.font_weight, Some(FontWeight::Medium));
    assert_eq!(cell.config.text_size, Some(1.0));
    assert_eq!(cell.config.line_height, Some(1.0));

    let row = TableRow::<()>::new(&theme).height(f32::NAN);
    assert_eq!(row.height, Some(1.0));

    let caption = TableCaption::<()>::text("caption", &theme).margin_top(f32::NEG_INFINITY);
    assert_eq!(caption.margin_top, Some(0.0));

    let table = Table::<()>::new(&theme).min_width(f32::INFINITY);
    assert_eq!(table.min_width, 0.0);
}

#[test]
fn public_builders_have_non_empty_debug_output() {
    let theme = Theme::light();
    assert!(format!("{:?}", TableCell::<()>::text("a", &theme)).contains("TableCell"));
    assert!(format!("{:?}", TableHead::<()>::text("a", &theme)).contains("TableHead"));
    assert!(format!("{:?}", TableRow::<()>::new(&theme)).contains("TableRow"));
    assert!(format!("{:?}", TableBody::<()>::new(&theme)).contains("TableSection"));
    assert!(format!("{:?}", TableCaption::<()>::text("a", &theme)).contains("TableCaption"));
    assert!(format!("{:?}", Table::<()>::new(&theme)).contains("Table"));
    let element: Element<'_, ()> = text("a").into();
    assert!(format!("{:?}", TableRowCell::from(element)).contains("Element"));
}

#[test]
fn standalone_slots_convert_to_elements() {
    let theme = Theme::light();
    let _: Element<'_, ()> = TableCell::text("cell", &theme).into();
    let _: Element<'_, ()> = TableHead::text("head", &theme).into();
    let _: Element<'_, ()> = TableRow::new(&theme)
        .cell(TableCell::text("cell", &theme))
        .into();
    let _: Element<'_, ()> = TableRow::new(&theme).push_element(text("arbitrary")).into();
    let _: Element<'_, ()> = TableBody::new(&theme)
        .push(TableRow::new(&theme).cell(TableCell::text("cell", &theme)))
        .into();
    let _: Element<'_, ()> = TableCaption::text("caption", &theme).into();
}

#[test]
fn custom_content_preserves_message_type() {
    #[derive(Debug, Clone, PartialEq)]
    struct Message;

    let theme = Theme::light();
    let element: Element<'_, Message> = TableCell::new(
        container(text("custom"))
            .padding(Padding::from(4.0))
            .style(|_| container::Style {
                text_color: Some(Color::BLACK),
                ..container::Style::default()
            }),
        &theme,
    )
    .align_y(Vertical::Center)
    .into();
    let _ = element;
}
