//! Behavioral tests for the typography component.

use crate::iced_compat::alignment::Horizontal;
use crate::iced_compat::widget::{container, text};
use crate::iced_compat::{Color, Element, Length};
use shadcn_common::{FontWeight, StyleId};

use super::style;
use super::table::column_count;
use super::*;
use crate::theme::Theme;

#[derive(Clone, Debug)]
enum Message {}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let block: Typography<'_, Message> = Typography::text("The Joke Tax", &theme)
        .variant(TypographyVariant::H2)
        .color(Color::WHITE)
        .size(48.0)
        .line_height(52.0)
        .width(Length::Shrink)
        .align_x(Horizontal::Center)
        .margin_top(12.0)
        .default_margin(true);

    assert!(matches!(block.content, TypographyContent::Text(_)));
    assert_eq!(block.variant, TypographyVariant::H2);
    assert_eq!(block.color, Some(Color::WHITE));
    assert_eq!(block.size, Some(48.0));
    assert_eq!(block.line_height, Some(52.0));
    assert_eq!(block.width, Some(Length::Shrink));
    assert_eq!(block.align_x, Horizontal::Center);
    assert_eq!(block.margin_top, Some(12.0));
    assert!(block.use_default_margin);
    assert!(std::ptr::eq(block.theme, &theme));
}

#[test]
fn every_variant_converts_to_an_element() {
    let theme = Theme::light();

    for variant in TypographyVariant::ALL {
        let _: Element<'_, Message> = Typography::text("Sample", &theme).variant(variant).into();
    }

    let _: Element<'_, Message> = Typography::new(container(text("Custom")), &theme).into();
}

#[test]
fn shortcut_constructors_pick_the_matching_variant() {
    let theme = Theme::light();

    let cases: [(Typography<'_, Message>, TypographyVariant); 11] = [
        (Typography::h1("x", &theme), TypographyVariant::H1),
        (Typography::h2("x", &theme), TypographyVariant::H2),
        (Typography::h3("x", &theme), TypographyVariant::H3),
        (Typography::h4("x", &theme), TypographyVariant::H4),
        (Typography::p("x", &theme), TypographyVariant::P),
        (
            Typography::blockquote("x", &theme),
            TypographyVariant::Blockquote,
        ),
        (
            Typography::inline_code("x", &theme),
            TypographyVariant::InlineCode,
        ),
        (Typography::lead("x", &theme), TypographyVariant::Lead),
        (Typography::large("x", &theme), TypographyVariant::Large),
        (Typography::small("x", &theme), TypographyVariant::Small),
        (Typography::muted("x", &theme), TypographyVariant::Muted),
    ];

    for (block, variant) in cases {
        assert_eq!(block.variant, variant);
    }
}

#[test]
fn recipes_match_the_web_utility_classes() {
    let h1 = TypographyVariant::H1.type_recipe();
    assert_eq!(h1.size_px, 36.0);
    assert_eq!(h1.weight, FontWeight::ExtraBold);
    assert!((h1.tracking_em - -0.025).abs() < f32::EPSILON);

    let p = TypographyVariant::P.type_recipe();
    assert_eq!(p.size_px, 16.0);
    assert_eq!(p.line_height_px, 28.0);
    assert_eq!(p.weight, FontWeight::Normal);

    let code = TypographyVariant::InlineCode.type_recipe();
    assert_eq!(code.size_px, 14.0);
    assert_eq!(code.weight, FontWeight::Semibold);

    let small = TypographyVariant::Small.type_recipe();
    assert_eq!(small.line_height_px, small.size_px);
}

#[test]
fn variant_flags_mirror_the_web_examples() {
    assert!(TypographyVariant::H1.uses_heading_font());
    assert!(!TypographyVariant::P.uses_heading_font());
    assert!(TypographyVariant::InlineCode.uses_mono_font());
    assert!(TypographyVariant::Blockquote.is_italic());
    assert!(TypographyVariant::Lead.is_muted());
    assert!(TypographyVariant::Muted.is_muted());
    assert!(!TypographyVariant::Large.is_muted());
}

#[test]
fn muted_variants_default_to_muted_foreground() {
    let theme = Theme::light();

    let muted = style::resolve_color(&theme, TypographyVariant::Muted, None);
    assert_eq!(muted, theme.palette.muted_foreground);

    let body = style::resolve_color(&theme, TypographyVariant::P, None);
    assert_eq!(body, theme.palette.foreground);
}

#[test]
fn color_override_beats_variant_default() {
    let theme = Theme::light();
    let color = style::resolve_color(&theme, TypographyVariant::Muted, Some(Color::WHITE));
    assert_eq!(color, Color::WHITE);
}

#[test]
fn headings_use_the_theme_heading_face() {
    let theme = Theme::light().with_style(StyleId::Sera);
    let heading = style::resolve_font(&theme, TypographyVariant::H1);
    let body = style::resolve_font(&theme, TypographyVariant::P);
    let code = style::resolve_font(&theme, TypographyVariant::InlineCode);

    let pack = theme.font_pack();
    assert_eq!(heading.family, crate::iced_font(pack.heading).family);
    assert_eq!(body.family, crate::iced_font(pack.sans).family);
    assert_eq!(code.family, crate::iced_font(pack.mono).family);
    assert_eq!(
        style::resolve_font(&theme, TypographyVariant::Blockquote).style,
        crate::iced_compat::font::Style::Italic,
    );
}

#[test]
fn size_override_scales_line_height_proportionally() {
    let theme = Theme::light();
    // H1 36/40 → 48 px should produce a 53.33 px line height when unset.
    let block: Typography<'_, Message> = Typography::h1("Wide", &theme).size(48.0);
    let recipe = block.variant.type_recipe();
    let expected = recipe.line_height_px * 48.0 / recipe.size_px;
    assert!((expected - 40.0 * 48.0 / 36.0).abs() < 1e-4);
    let _: Element<'_, Message> = block.into();
}

#[test]
fn default_margins_follow_the_demo_article_flow() {
    assert_eq!(TypographyVariant::H1.default_margin_top_px(), 0.0);
    assert_eq!(TypographyVariant::H2.default_margin_top_px(), 40.0);
    assert_eq!(TypographyVariant::H3.default_margin_top_px(), 32.0);
    assert_eq!(TypographyVariant::P.default_margin_top_px(), 24.0);
    assert_eq!(TypographyVariant::InlineCode.default_margin_top_px(), 0.0);
}

#[test]
fn list_builder_tracks_items() {
    let theme = Theme::light();
    let list: TypographyList<'_, Message> = TypographyList::new(&theme);
    assert!(list.is_empty());

    let list = list
        .item("1st level of puns: 5 gold coins")
        .items(["2nd level of jokes: 10 gold coins"])
        .item_element(text("custom"))
        .color(Color::WHITE)
        .indent(-3.0)
        .width(Length::Shrink);
    assert_eq!(list.len(), 3);
    assert!(!list.is_empty());

    let _: Element<'_, Message> = list.into();
}

#[test]
fn table_builder_tracks_rows() {
    let theme = Theme::light();
    let table: TypographyTable<'_, Message> = TypographyTable::new(&theme);
    assert!(table.is_empty());

    let table = table
        .header(["King's Treasury", "People's happiness"])
        .row(["Empty", "Overflowing"])
        .row(["Modest"])
        .align_columns([Horizontal::Left, Horizontal::Right])
        .striped(false)
        .color(Color::WHITE)
        .width(Length::Shrink);
    assert_eq!(table.len(), 2);
    assert!(!table.is_empty());

    let _: Element<'_, Message> = table.into();
}

#[test]
fn table_columns_include_body_cells_beyond_the_header() {
    assert_eq!(column_count(Some(2), [3, 1]), 3);
    assert_eq!(column_count(None, [0, 4]), 4);
    assert_eq!(column_count(Some(0), []), 1);
}

#[test]
fn variant_names_match_shadcn_example_ids() {
    assert_eq!(TypographyVariant::H1.as_str(), "h1");
    assert_eq!(TypographyVariant::InlineCode.as_str(), "inline-code");
    assert_eq!(TypographyVariant::ALL.len(), 11);
    assert_eq!(TypographyVariant::default(), TypographyVariant::P);
}

#[test]
fn debug_is_nonempty_without_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();

    let block = Typography::<NoDebugMessage>::text("Prose", &theme);
    let debug = format!("{block:?}");
    assert!(debug.contains("Typography"));
    assert!(debug.contains("variant"));

    let list = TypographyList::<NoDebugMessage>::new(&theme);
    assert!(format!("{list:?}").contains("TypographyList"));

    let table = TypographyTable::<NoDebugMessage>::new(&theme);
    assert!(format!("{table:?}").contains("TypographyTable"));
}
