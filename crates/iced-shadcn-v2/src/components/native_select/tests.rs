//! Behavioral tests for the native-select component.

use crate::iced_compat::{Element, Length};
use shadcn_common::{AccentColor, StyleId};

use super::style;
use super::types::Row;
use super::*;
use crate::theme::Theme;

#[derive(Clone, Debug, PartialEq)]
enum Message {
    Picked(&'static str),
    Opened,
    Closed,
}

fn fruits(theme: &Theme) -> NativeSelect<'_, &'static str, Message> {
    NativeSelect::new(theme)
        .placeholder("Select a fruit")
        .option(("apple", "Apple"))
        .option(NativeSelectOption::new("grapes", "Grapes").disabled(true))
        .option(("banana", "Banana"))
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let select = fruits(&theme)
        .size(NativeSelectSize::Sm)
        .radius(NativeSelectRadius::Full)
        .color(AccentColor::Blue)
        .width(Length::Fixed(200.0))
        .text_size(13.0)
        .selected("banana")
        .disabled(true)
        .invalid(true)
        .on_select(Message::Picked)
        .on_open(Message::Opened)
        .on_close(Message::Closed)
        .style_override(|style, _| style);

    assert_eq!(select.rows.len(), 3);
    assert_eq!(select.placeholder.as_deref(), Some("Select a fruit"));
    assert_eq!(select.size, NativeSelectSize::Sm);
    assert_eq!(select.radius, Some(NativeSelectRadius::Full));
    assert_eq!(select.color, Some(AccentColor::Blue));
    assert_eq!(select.width, Length::Fixed(200.0));
    assert_eq!(select.text_size, Some(13.0));
    assert_eq!(select.selected, Some("banana"));
    assert!(select.disabled);
    assert!(select.invalid);
    assert!(select.on_select.is_some());
    assert_eq!(select.on_open, Some(Message::Opened));
    assert_eq!(select.on_close, Some(Message::Closed));
    assert!(select.style_override.is_some());
    assert!(std::ptr::eq(select.theme, &theme));

    let callback = select.on_select.as_ref().expect("on_select was set");
    assert_eq!(callback("apple"), Message::Picked("apple"));
}

#[test]
fn builder_and_helper_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = fruits(&theme).on_select(Message::Picked).into();

    let _: Element<'_, Message> = native_select("Select a fruit", &theme)
        .options([("apple", "Apple"), ("banana", "Banana")])
        .on_select(Message::Picked)
        .into();
}

#[test]
fn groups_flatten_into_heading_and_indented_rows() {
    let theme = Theme::light();
    let select: NativeSelect<'_, &str, Message> = NativeSelect::new(&theme).group(
        NativeSelectGroup::new("Fruits")
            .option(("apple", "Apple"))
            .options([("banana", "Banana")]),
    );

    assert_eq!(select.rows.len(), 3);
    assert!(matches!(&select.rows[0], Row::GroupLabel { label } if label == "Fruits"));
    assert!(select.rows[1].is_indented());
    assert!(select.rows[1].is_selectable());
    assert!(!select.rows[0].is_selectable());
}

#[test]
fn disabled_group_disables_every_nested_option() {
    let theme = Theme::light();
    let select: NativeSelect<'_, &str, Message> = NativeSelect::new(&theme).group(
        NativeSelectGroup::new("Vegetables")
            .option(("carrot", "Carrot"))
            .option(NativeSelectOption::new("broccoli", "Broccoli"))
            .disabled(true),
    );

    assert!(select.rows.iter().skip(1).all(|row| !row.is_selectable()));
}

#[test]
fn disabled_option_is_not_selectable() {
    let theme = Theme::light();
    let select = fruits(&theme);

    assert!(select.rows[0].is_selectable());
    assert!(!select.rows[1].is_selectable());
    assert!(select.rows[2].is_selectable());
}

#[test]
fn option_accessors_report_configuration() {
    let option = NativeSelectOption::new("apple", "Apple").disabled(true);
    assert_eq!(option.value(), &"apple");
    assert_eq!(option.label(), "Apple");
    assert!(option.is_disabled());

    let group = NativeSelectGroup::<&str>::new("Fruits");
    assert_eq!(group.label(), "Fruits");
    assert!(group.is_empty());
    assert_eq!(group.option(("apple", "Apple")).len(), 1);
}

#[test]
fn default_width_shrinks_like_w_fit() {
    let theme = Theme::light();
    let select: NativeSelect<'_, &str, Message> = NativeSelect::new(&theme);

    assert_eq!(select.width, Length::Shrink);
}

#[test]
fn control_heights_match_the_pack_ladder() {
    let vega = Theme::light();
    assert_eq!(NativeSelectSize::Sm.control_height(&vega), 32.0);
    assert_eq!(NativeSelectSize::Default.control_height(&vega), 36.0);
    assert_eq!(NativeSelectSize::Lg.control_height(&vega), 40.0);

    // `.cn-native-select` heights: Mira h-7, Sera h-10.
    let mira = Theme::light().with_style(StyleId::Mira);
    assert_eq!(NativeSelectSize::Default.control_height(&mira), 28.0);

    let sera = Theme::light().with_style(StyleId::Sera);
    assert_eq!(NativeSelectSize::Default.control_height(&sera), 40.0);
}

#[test]
fn pack_text_and_icon_sizes_follow_the_css() {
    let vega = Theme::light();
    assert_eq!(
        style::pack_text_size(&vega, NativeSelectSize::Default),
        14.0
    );
    assert_eq!(style::pack_icon_size(&vega, NativeSelectSize::Sm), 16.0);

    // Mira: `text-xs`, sm drops to 10px text and a `size-3` icon.
    let mira = Theme::light().with_style(StyleId::Mira);
    assert_eq!(
        style::pack_text_size(&mira, NativeSelectSize::Default),
        12.0
    );
    assert_eq!(style::pack_text_size(&mira, NativeSelectSize::Sm), 10.0);
    assert_eq!(
        style::pack_icon_size(&mira, NativeSelectSize::Default),
        14.0
    );
    assert_eq!(style::pack_icon_size(&mira, NativeSelectSize::Sm), 12.0);
}

#[test]
fn field_style_uses_input_border_and_muted_icon() {
    let theme = Theme::light();
    let resolved = style::resolve_field_style(
        &theme,
        NativeSelectSize::Default,
        None,
        None,
        false,
        false,
        NativeSelectStatus::Active,
    );

    assert_eq!(resolved.border_color, theme.palette.input);
    assert_eq!(resolved.text_color, theme.palette.foreground);
    assert_eq!(resolved.placeholder_color, theme.palette.muted_foreground);
    assert_eq!(resolved.icon_color, theme.palette.muted_foreground);
    // Vega light: `bg-transparent`.
    assert_eq!(resolved.background.a, 0.0);
}

#[test]
fn opened_status_recolors_the_border_with_ring() {
    let theme = Theme::light();
    let resolved = style::resolve_field_style(
        &theme,
        NativeSelectSize::Default,
        None,
        None,
        false,
        false,
        NativeSelectStatus::Opened,
    );

    assert_eq!(resolved.border_color, theme.palette.ring);
}

#[test]
fn invalid_outranks_the_focus_border() {
    let theme = Theme::light();
    let resolved = style::resolve_field_style(
        &theme,
        NativeSelectSize::Default,
        None,
        None,
        true,
        false,
        NativeSelectStatus::Opened,
    );

    assert_eq!(resolved.border_color, theme.palette.destructive);

    // `dark:aria-invalid:border-destructive/50`.
    let dark = Theme::dark();
    let resolved = style::resolve_field_style(
        &dark,
        NativeSelectSize::Default,
        None,
        None,
        true,
        false,
        NativeSelectStatus::Active,
    );

    assert!((resolved.border_color.a - dark.palette.destructive.a * 0.5).abs() < f32::EPSILON);
}

#[test]
fn disabled_halves_every_alpha() {
    let theme = Theme::light();
    let active = style::resolve_field_style(
        &theme,
        NativeSelectSize::Default,
        None,
        None,
        false,
        false,
        NativeSelectStatus::Active,
    );
    let disabled = style::resolve_field_style(
        &theme,
        NativeSelectSize::Default,
        None,
        None,
        false,
        true,
        NativeSelectStatus::Disabled,
    );

    assert!((disabled.text_color.a - active.text_color.a * 0.5).abs() < f32::EPSILON);
    assert!((disabled.border_color.a - active.border_color.a * 0.5).abs() < f32::EPSILON);
    assert!((disabled.icon_color.a - active.icon_color.a * 0.5).abs() < f32::EPSILON);
}

#[test]
fn accent_color_recolors_the_open_border() {
    let theme = Theme::light();
    let resolved = style::resolve_field_style(
        &theme,
        NativeSelectSize::Default,
        None,
        Some(AccentColor::Blue),
        false,
        false,
        NativeSelectStatus::Opened,
    );

    assert_eq!(
        resolved.border_color,
        theme.color_with_accent(
            AccentColor::Blue,
            twill_core::prelude::theme::SemanticColor::Primary
        )
    );
}

#[test]
fn maia_field_uses_rounded_4xl() {
    // Maia's trigger is `rounded-4xl` → `--radius-4xl` = base + 16
    // (default base 0.625rem → 26px), not a true `rounded-full` pill.
    let maia = Theme::light().with_style(StyleId::Maia);
    let field = style::resolve_field_style(
        &maia,
        NativeSelectSize::Default,
        None,
        None,
        false,
        false,
        NativeSelectStatus::Active,
    );

    assert_eq!(field.radius, 26.0);

    // Lyra stays square.
    let lyra = Theme::light().with_style(StyleId::Lyra);
    let field = style::resolve_field_style(
        &lyra,
        NativeSelectSize::Default,
        None,
        None,
        false,
        false,
        NativeSelectStatus::Active,
    );
    assert_eq!(field.radius, 0.0);
}

#[test]
fn sera_paints_only_the_bottom_hairline() {
    let sera = Theme::light().with_style(StyleId::Sera);
    let resolved = style::resolve_field_style(
        &sera,
        NativeSelectSize::Default,
        None,
        None,
        false,
        false,
        NativeSelectStatus::Active,
    );

    // `border-b-input pl-0 rounded-none`.
    assert!(resolved.underline_only);
    assert_eq!(resolved.border_color, sera.palette.input);
    assert_eq!(resolved.radius, 0.0);
    assert_eq!(style::recipe(&sera).pad_left_px, 0.0);

    // Every other pack keeps the full border box.
    let vega = Theme::light();
    let resolved = style::resolve_field_style(
        &vega,
        NativeSelectSize::Default,
        None,
        None,
        false,
        false,
        NativeSelectStatus::Active,
    );
    assert!(!resolved.underline_only);
}

#[test]
fn borderless_packs_drop_the_resting_border() {
    let luma = Theme::light().with_style(StyleId::Luma);
    let resolved = style::resolve_field_style(
        &luma,
        NativeSelectSize::Default,
        None,
        None,
        false,
        false,
        NativeSelectStatus::Active,
    );

    assert_eq!(resolved.border_color.a, 0.0);
    // `bg-input/50`.
    assert!(resolved.background.a > 0.0);
}

#[test]
fn debug_never_panics_and_reports_row_count() {
    let theme = Theme::light();
    let select = fruits(&theme).on_select(Message::Picked);
    let debug = format!("{select:?}");

    assert!(debug.contains("NativeSelect"));
    assert!(debug.contains("rows: 3"));
}
