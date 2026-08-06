//! Behavioral tests for the textarea component.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::iced_compat::widget::text_editor;
use crate::iced_compat::{Color, Element, Length};
use shadcn_common::AccentColor;
use twill_core::prelude::theme::SemanticColor;

use super::geometry;
use super::style;
use super::*;
use crate::theme::Theme;

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Message {
    Edited(text_editor::Action),
}

const FOCUSED: text_editor::Status = text_editor::Status::Focused { is_hovered: false };

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let content = text_editor::Content::new();
    let textarea: Textarea<'_, Message> = Textarea::new(&content, &theme)
        .placeholder("Type here")
        .size(TextareaSize::Lg)
        .radius(TextareaRadius::Full)
        .color(AccentColor::Blue)
        .padding([8.0, 12.0])
        .text_size(16.0_f32)
        .rows(4)
        .max_rows(10)
        .resize(TextareaResize::Both)
        .wrapping(iced_core::text::Wrapping::Word)
        .max_len(280)
        .disabled(true)
        .read_only(true)
        .invalid(true)
        .id("bio")
        .on_action(Message::Edited)
        .style_override(|style, _| style);

    assert_eq!(textarea.placeholder.as_ref(), "Type here");
    assert_eq!(textarea.size, TextareaSize::Lg);
    assert_eq!(textarea.radius, Some(TextareaRadius::Full));
    assert_eq!(textarea.color, Some(AccentColor::Blue));
    assert_eq!(textarea.padding, Some([8.0, 12.0]));
    assert_eq!(textarea.text_size, Some(16.0));
    assert_eq!(textarea.rows, Some(4));
    assert_eq!(textarea.max_rows, Some(10));
    assert_eq!(textarea.resize, TextareaResize::Both);
    assert_eq!(textarea.wrapping, iced_core::text::Wrapping::Word);
    assert_eq!(textarea.max_len, Some(280));
    assert!(textarea.disabled);
    assert!(textarea.read_only);
    assert!(textarea.invalid);
    assert!(textarea.id.is_some());
    assert!(textarea.on_action.is_some());
    assert!(textarea.style_override.is_some());
    assert!(std::ptr::eq(textarea.theme, &theme));

    let callback = textarea.on_action.as_ref().expect("on_action was set");
    assert!(matches!(
        callback(text_editor::Action::Edit(text_editor::Edit::Insert('x'))),
        Message::Edited(_)
    ));
}

#[test]
fn builder_and_helper_convert_to_elements() {
    let theme = Theme::light();
    let content = text_editor::Content::new();

    let _: Element<'_, Message> = Textarea::new(&content, &theme)
        .placeholder("Bio")
        .on_action(Message::Edited)
        .into();

    let _: Element<'_, Message> = textarea(&content, "Bio", &theme)
        .on_action(Message::Edited)
        .into();
}

#[test]
fn default_width_fills_like_w_full() {
    let theme = Theme::light();
    let content = text_editor::Content::new();
    let textarea = Textarea::<Message>::new(&content, &theme);

    assert_eq!(textarea.width, Length::Fill);
}

#[test]
fn recipe_tokens_match_the_css() {
    // Vega: px-2.5 py-2 text-sm rounded-md, transparent in light, /30 in dark.
    let vega = style::pack_recipe(&Theme::light());
    assert_eq!(vega.pad_x_px, 10.0);
    assert_eq!(vega.pad_y_px, 8.0);
    assert_eq!(vega.text_size_px, 14.0);
    assert!(vega.bordered);
    assert!(!vega.disabled_fill);
    assert!(vega.shadow);

    // Nova: rounded-lg + disabled:bg-input/50, no shadow.
    let nova = style::pack_recipe(&Theme::light().with_style(shadcn_common::StyleId::Nova));
    assert!(nova.disabled_fill);
    assert!(!nova.shadow);

    // Maia: px-3 py-3 rounded-xl bg-input/30.
    let maia = style::pack_recipe(&Theme::light().with_style(shadcn_common::StyleId::Maia));
    assert_eq!(maia.pad_x_px, 12.0);
    assert_eq!(maia.pad_y_px, 12.0);
    assert!((maia.fill_alpha_light - 0.3).abs() < f32::EPSILON);

    // Lyra: text-xs rounded-none focus-ring-1.
    let lyra = style::pack_recipe(&Theme::light().with_style(shadcn_common::StyleId::Lyra));
    assert_eq!(lyra.text_size_px, 12.0);
    assert_eq!(lyra.focus_ring_px, 1.0);

    // Sera: underline-only, px-0 py-3, no focus ring.
    let sera = style::pack_recipe(&Theme::light().with_style(shadcn_common::StyleId::Sera));
    assert_eq!(sera.pad_x_px, 0.0);
    assert_eq!(sera.pad_y_px, 12.0);
    assert!(sera.underline_only);
    assert_eq!(sera.focus_ring_px, 0.0);

    // Luma / Rhea: border-transparent + bg-input/50.
    let luma = style::pack_recipe(&Theme::light().with_style(shadcn_common::StyleId::Luma));
    assert!(!luma.bordered);
    assert!((luma.fill_alpha_light - 0.5).abs() < f32::EPSILON);
}

#[test]
fn pack_text_sizes_follow_the_css() {
    let recipe = style::pack_recipe(&Theme::light());
    assert_eq!(
        geometry::pack_text_size(recipe, TextareaSize::Default),
        14.0
    );
    assert_eq!(geometry::pack_text_size(recipe, TextareaSize::Sm), 13.0);
    assert_eq!(geometry::pack_text_size(recipe, TextareaSize::Lg), 16.0);

    let lyra = style::pack_recipe(&Theme::light().with_style(shadcn_common::StyleId::Lyra));
    assert_eq!(geometry::pack_text_size(lyra, TextareaSize::Default), 12.0);
}

#[test]
fn min_height_matches_min_h_16() {
    let recipe = style::pack_recipe(&Theme::light());
    let padding = geometry::pack_padding(recipe, TextareaSize::Default);
    assert_eq!(
        geometry::min_height(TextareaSize::Default, 14.0, padding, None),
        64.0
    );
    assert_eq!(
        geometry::min_height(TextareaSize::Lg, 16.0, padding, None),
        96.0
    );

    // Explicit rows override the default minimum height.
    let rows_height = geometry::min_height(TextareaSize::Default, 14.0, padding, Some(3));
    let expected = 14.0 * 1.4 * 3.0 + padding[0] * 2.0;
    assert!((rows_height - expected).abs() < f32::EPSILON);
}

#[test]
fn base_style_uses_the_input_border() {
    let theme = Theme::light();
    let recipe = style::pack_recipe(&theme);
    let style = style::resolve_textarea_style(
        &theme,
        recipe,
        None,
        None,
        false,
        false,
        false,
        text_editor::Status::Active,
    );

    assert_eq!(style.border.width, 1.0);
    assert_eq!(
        style.border.color,
        theme.semantic_color(SemanticColor::Input)
    );
    assert_eq!(style.value, theme.semantic_color(SemanticColor::Foreground));
    assert_eq!(
        style.placeholder,
        theme.semantic_color(SemanticColor::MutedForeground)
    );
}

#[test]
fn focused_style_recolors_the_border_with_ring() {
    let theme = Theme::light();
    let recipe = style::pack_recipe(&theme);
    let style =
        style::resolve_textarea_style(&theme, recipe, None, None, false, false, false, FOCUSED);

    assert_eq!(
        style.border.color,
        theme.semantic_color(SemanticColor::Ring)
    );
}

#[test]
fn invalid_outranks_the_focus_treatment() {
    let theme = Theme::light();
    let recipe = style::pack_recipe(&theme);
    let style =
        style::resolve_textarea_style(&theme, recipe, None, None, true, false, false, FOCUSED);

    assert_eq!(
        style.border.color,
        theme.semantic_color(SemanticColor::Destructive)
    );

    // `dark:aria-invalid:border-destructive/50`.
    let dark = Theme::dark();
    let dark_recipe = style::pack_recipe(&dark);
    let dark_style =
        style::resolve_textarea_style(&dark, dark_recipe, None, None, true, false, false, FOCUSED);
    let destructive = dark.semantic_color(SemanticColor::Destructive);
    assert!((dark_style.border.color.a - destructive.a * 0.5).abs() < f32::EPSILON);
}

#[test]
fn disabled_style_halves_the_text_opacity() {
    let theme = Theme::light();
    let recipe = style::pack_recipe(&theme);
    let base = style::resolve_textarea_style(
        &theme,
        recipe,
        None,
        None,
        false,
        false,
        false,
        text_editor::Status::Active,
    );
    let disabled = style::resolve_textarea_style(
        &theme,
        style::pack_recipe(&theme),
        None,
        None,
        false,
        true,
        false,
        text_editor::Status::Disabled,
    );

    assert!((disabled.value.a - base.value.a * 0.5).abs() < f32::EPSILON);
    assert!((disabled.placeholder.a - base.placeholder.a * 0.5).abs() < f32::EPSILON);
}

#[test]
fn read_only_mutes_the_value_color() {
    let theme = Theme::light();
    let recipe = style::pack_recipe(&theme);
    let style = style::resolve_textarea_style(
        &theme,
        recipe,
        None,
        None,
        false,
        false,
        true,
        text_editor::Status::Active,
    );

    assert_eq!(
        style.value,
        theme.semantic_color(SemanticColor::MutedForeground)
    );
}

#[test]
fn dark_mode_fills_follow_the_pack_alpha() {
    // Vega: `bg-transparent` in light, `dark:bg-input/30`.
    let light = style::resolve_textarea_style(
        &Theme::light(),
        style::pack_recipe(&Theme::light()),
        None,
        None,
        false,
        false,
        false,
        text_editor::Status::Active,
    );
    let dark_theme = Theme::dark();
    let dark = style::resolve_textarea_style(
        &dark_theme,
        style::pack_recipe(&dark_theme),
        None,
        None,
        false,
        false,
        false,
        text_editor::Status::Active,
    );

    let crate::iced_compat::Background::Color(light_fill) = light.background else {
        panic!("textarea backgrounds are plain colors");
    };
    let crate::iced_compat::Background::Color(dark_fill) = dark.background else {
        panic!("textarea backgrounds are plain colors");
    };
    let input = dark_theme.semantic_color(SemanticColor::Input);

    assert!(light_fill.a.abs() < f32::EPSILON);
    assert!((dark_fill.a - input.a * 0.3).abs() < f32::EPSILON);
}

#[test]
fn accent_color_recolors_the_focus_ring() {
    let theme = Theme::light();
    let recipe = style::pack_recipe(&theme);
    let plain =
        style::resolve_textarea_style(&theme, recipe, None, None, false, false, false, FOCUSED);
    let accented = style::resolve_textarea_style(
        &theme,
        style::pack_recipe(&theme),
        None,
        Some(AccentColor::Blue),
        false,
        false,
        false,
        FOCUSED,
    );

    assert_eq!(
        accented.border.color,
        theme.color_with_accent(AccentColor::Blue, SemanticColor::Primary)
    );
    assert_ne!(plain.border.color, accented.border.color);
}

#[test]
fn sera_underline_only_clears_the_box_border() {
    let sera = Theme::light().with_style(shadcn_common::StyleId::Sera);
    assert!(style::uses_underline_only(&sera));

    let recipe = style::pack_recipe(&sera);
    let style = style::resolve_textarea_style(
        &sera,
        recipe,
        None,
        None,
        false,
        false,
        false,
        text_editor::Status::Active,
    );

    assert_eq!(style.border.width, 0.0);
    assert_eq!(style.border.color, Color::TRANSPARENT);
}

#[test]
fn all_states_resolve_in_light_and_dark_themes() {
    for theme in [Theme::light(), Theme::dark()] {
        let recipe = style::pack_recipe(&theme);
        for invalid in [false, true] {
            for disabled in [false, true] {
                for read_only in [false, true] {
                    for status in [
                        text_editor::Status::Active,
                        text_editor::Status::Hovered,
                        FOCUSED,
                        text_editor::Status::Disabled,
                    ] {
                        let style = style::resolve_textarea_style(
                            &theme, recipe, None, None, invalid, disabled, read_only, status,
                        );
                        assert!(style.value.a.is_finite());
                        assert!(style.border.width.is_finite());
                    }
                }
            }
        }
    }
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let content = text_editor::Content::new();
    let textarea = Textarea::<NoDebugMessage>::new(&content, &theme).placeholder("hello");
    let debug = format!("{textarea:?}");
    assert!(debug.contains("Textarea"));
    assert!(debug.contains("hello"));
}

#[test]
fn configuration_enums_support_hashing_and_expected_order() {
    fn hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    let _ = hash(&TextareaSize::Default);
    let _ = hash(&TextareaRadius::Medium);
    let _ = hash(&TextareaResize::Both);
    assert!(TextareaRadius::None < TextareaRadius::Full);
    assert_eq!(TextareaSize::default(), TextareaSize::Default);
    assert_eq!(TextareaRadius::default(), TextareaRadius::Medium);
    assert_eq!(TextareaResize::default(), TextareaResize::None);
}

#[test]
fn tone_is_an_alias_for_color() {
    let theme = Theme::light();
    let content = text_editor::Content::new();
    let textarea: Textarea<'_, Message> = Textarea::new(&content, &theme).tone(AccentColor::Blue);

    assert_eq!(textarea.color, Some(AccentColor::Blue));
}

#[test]
fn apply_action_respects_disabled_and_read_only() {
    let mut content = text_editor::Content::new();
    let insert = text_editor::Action::Edit(text_editor::Edit::Insert('a'));

    assert!(!textarea_apply_action(
        &mut content,
        insert.clone(),
        true,
        false,
        None
    ));
    assert_eq!(content.text(), "");

    assert!(!textarea_apply_action(
        &mut content,
        insert.clone(),
        false,
        true,
        None
    ));
    assert_eq!(content.text(), "");

    assert!(textarea_apply_action(
        &mut content,
        insert,
        false,
        false,
        None
    ));
    assert_eq!(content.text(), "a");
}

#[test]
fn apply_action_enforces_max_len() {
    let mut content = text_editor::Content::new();
    content.perform(text_editor::Action::Edit(text_editor::Edit::Insert('a')));
    content.perform(text_editor::Action::Edit(text_editor::Edit::Insert('b')));

    let insert = text_editor::Action::Edit(text_editor::Edit::Insert('c'));
    assert!(!textarea_apply_action(
        &mut content,
        insert,
        false,
        false,
        Some(2)
    ));
    assert_eq!(content.text(), "ab");

    // Non-edit actions (movement) always pass through.
    let move_action = text_editor::Action::Move(text_editor::Motion::Right);
    assert!(textarea_apply_action(
        &mut content,
        move_action,
        false,
        false,
        Some(2)
    ));
}

#[test]
fn states_dimensions_and_style_override_are_configurable() {
    let theme = Theme::light();
    let content = text_editor::Content::new();
    let textarea = Textarea::editor_fixture(&content, &theme)
        .width(Length::Fixed(240.0))
        .wrapping(iced_core::text::Wrapping::Glyph)
        .id("bio")
        .style_override(|mut style, _| {
            style.value = Color::from_rgb(1.0, 0.0, 1.0);
            style
        })
        .on_action(Message::Edited);

    assert_eq!(textarea.width, Length::Fixed(240.0));
    assert_eq!(textarea.wrapping, iced_core::text::Wrapping::Glyph);
    assert!(textarea.id.is_some());
    assert!(textarea.style_override.is_some());

    let _ = textarea.into_text_editor();
}

impl<'a> Textarea<'a, Message> {
    /// Test fixture with a placeholder preset.
    fn editor_fixture(content: &'a text_editor::Content, theme: &'a Theme) -> Self {
        Textarea::new(content, theme).placeholder("placeholder")
    }
}
