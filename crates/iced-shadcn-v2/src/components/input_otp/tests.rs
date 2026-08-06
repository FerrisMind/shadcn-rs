//! Behavioral tests for the input-otp component.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::iced_compat::Element;
use shadcn_common::AccentColor;

use super::geometry::{self, OtpMetrics};
use super::render;
use super::style;
use super::*;
use crate::theme::Theme;

#[derive(Clone, Debug)]
enum Message {
    Changed(String),
    Completed(String),
    Submitted,
}

#[test]
fn builder_updates_semantic_fields() {
    let theme = Theme::light();
    let otp: InputOtp<'_, Message> = InputOtp::new(&theme)
        .value("12")
        .max_length(4)
        .groups([2, 2])
        .pattern(InputOtpPattern::Digits)
        .radius(InputOtpRadius::Full)
        .color(AccentColor::Blue)
        .slot_size(48)
        .text_size(18)
        .disabled(true)
        .invalid(true)
        .on_input(Message::Changed)
        .on_complete(Message::Completed)
        .on_submit(Message::Submitted)
        .paste_transformer(|text| text.replace('-', ""))
        .style_override(|style, _| style);

    assert_eq!(otp.value.as_ref(), "12");
    assert_eq!(otp.max_length, 4);
    assert_eq!(otp.groups, vec![2, 2]);
    assert_eq!(otp.pattern, InputOtpPattern::Digits);
    assert_eq!(otp.radius, Some(InputOtpRadius::Full));
    assert_eq!(otp.color, Some(AccentColor::Blue));
    assert_eq!(otp.slot_size, Some(48.0));
    assert_eq!(otp.text_size, Some(18.0));
    assert!(otp.disabled);
    assert!(otp.invalid);
    assert!(otp.on_input.is_some());
    assert!(otp.on_complete.is_some());
    assert!(otp.on_submit.is_some());
    assert!(otp.paste_transformer.is_some());
    assert!(otp.style_override.is_some());
    assert!(std::ptr::eq(otp.theme, &theme));

    let callback = otp.on_input.as_ref().expect("on_input was set");
    assert!(matches!(callback("7".to_owned()), Message::Changed(text) if text == "7"));
    let completed = otp.on_complete.as_ref().expect("on_complete was set");
    assert!(matches!(completed("1234".to_owned()), Message::Completed(text) if text == "1234"));
    assert!(matches!(otp.on_submit, Some(Message::Submitted)));
    let transform = otp
        .paste_transformer
        .as_ref()
        .expect("paste_transformer was set");
    assert_eq!(transform("123-456".to_owned()), "123456");
}

#[test]
fn builder_and_helper_convert_to_elements() {
    let theme = Theme::light();

    let _: Element<'_, Message> = InputOtp::new(&theme).on_input(Message::Changed).into();

    let _: Element<'_, Message> = input_otp("123", &theme)
        .on_input(Message::Changed)
        .into_element();
}

#[test]
fn max_length_is_clamped_to_at_least_one_slot() {
    let theme = Theme::light();
    let otp = InputOtp::<Message>::new(&theme).max_length(0);

    assert_eq!(otp.max_length, 1);
}

#[test]
fn patterns_match_the_bits_ui_regexes() {
    assert!(InputOtpPattern::Any.accepts('-'));
    assert!(!InputOtpPattern::Any.accepts('\u{8}'));

    assert!(InputOtpPattern::Digits.accepts('0'));
    assert!(!InputOtpPattern::Digits.accepts('a'));

    assert!(InputOtpPattern::Chars.accepts('z'));
    assert!(!InputOtpPattern::Chars.accepts('1'));

    assert!(InputOtpPattern::DigitsAndChars.accepts('a'));
    assert!(InputOtpPattern::DigitsAndChars.accepts('9'));
    assert!(!InputOtpPattern::DigitsAndChars.accepts('-'));
}

#[test]
fn groups_normalize_against_max_length() {
    // The shadcn demo layout survives untouched.
    assert_eq!(geometry::normalize_groups(6, &[3, 3]), vec![3, 3]);
    // Zero-sized groups are dropped, leftovers become a trailing group.
    assert_eq!(geometry::normalize_groups(6, &[0, 2, 2]), vec![2, 2, 2]);
    assert_eq!(geometry::normalize_groups(6, &[4]), vec![4, 2]);
    // Oversized layouts are truncated.
    assert_eq!(geometry::normalize_groups(4, &[3, 3]), vec![3, 1]);
    // No groups = one group with every slot.
    assert_eq!(geometry::normalize_groups(6, &[]), vec![6]);
}

#[test]
fn typing_fills_slots_and_respects_pattern_and_capacity() {
    assert_eq!(
        render::append_text("12", "3", InputOtpPattern::Digits, 6),
        Some("123".to_owned())
    );
    // Rejected characters leave the value untouched.
    assert_eq!(
        render::append_text("12", "x", InputOtpPattern::Digits, 6),
        None
    );
    // A full value rejects further input.
    assert_eq!(
        render::append_text("123456", "7", InputOtpPattern::Digits, 6),
        None
    );
    // Multi-character input (IME, key repeat) is filtered per character.
    assert_eq!(
        render::append_text("", "1a2b", InputOtpPattern::Digits, 6),
        Some("12".to_owned())
    );
}

#[test]
fn paste_appends_to_partial_values_and_replaces_full_ones() {
    assert_eq!(
        render::apply_paste("12", "3456", InputOtpPattern::Digits, 6),
        Some("123456".to_owned())
    );
    // Overflow is truncated to the remaining slots.
    assert_eq!(
        render::apply_paste("12", "3456789", InputOtpPattern::Digits, 6),
        Some("123456".to_owned())
    );
    // A full value is replaced outright.
    assert_eq!(
        render::apply_paste("123456", "654321", InputOtpPattern::Digits, 6),
        Some("654321".to_owned())
    );
    // Pasting nothing usable changes nothing.
    assert_eq!(
        render::apply_paste("12", "ab", InputOtpPattern::Digits, 6),
        None
    );
    assert_eq!(
        render::apply_paste("123456", "123456", InputOtpPattern::Digits, 6),
        None
    );
}

#[test]
fn backspace_removes_the_last_character() {
    assert_eq!(render::without_last_char("123"), "12");
    assert_eq!(render::without_last_char(""), "");
}

#[test]
fn caret_blink_matches_the_web_keyframes() {
    // `0%,70%,100% → 1` and `20%,50% → 0` from `animate-caret-blink`.
    assert_eq!(render::caret_opacity(0.0), 1.0);
    assert_eq!(render::caret_opacity(0.2), 0.0);
    assert_eq!(render::caret_opacity(0.35), 0.0);
    assert_eq!(render::caret_opacity(0.5), 0.0);
    assert_eq!(render::caret_opacity(0.7), 1.0);
    assert_eq!(render::caret_opacity(0.9), 1.0);
    // Cycles wrap.
    assert_eq!(render::caret_opacity(1.0), 1.0);
}

#[test]
fn metrics_reproduce_the_pack_footprints() {
    // Vega: `size-9` slots, `ring-3` reserve, `gap-2` around a `size-4`
    // separator → 3 + 36*3 + 8 + 16 + 8 + 36*3 + 3.
    let metrics = OtpMetrics {
        slot_size: 36.0,
        slot_gap: 0.0,
        ring_width: 3.0,
        separator: true,
    };
    let size = metrics.total_size(&[3, 3]);
    assert_eq!(size.width, 254.0);
    assert_eq!(size.height, 42.0);

    // Sera: `size-10` slots with `gap-1` and no ring.
    let sera = OtpMetrics {
        slot_size: 40.0,
        slot_gap: 4.0,
        ring_width: 0.0,
        separator: false,
    };
    let size = sera.total_size(&[6]);
    assert_eq!(size.width, 40.0 * 6.0 + 4.0 * 5.0);
    assert_eq!(size.height, 40.0);
}

#[test]
fn slot_and_separator_regions_tile_the_bounds() {
    let metrics = OtpMetrics {
        slot_size: 36.0,
        slot_gap: 0.0,
        ring_width: 3.0,
        separator: true,
    };
    let groups = [3usize, 3];
    let total = metrics.total_size(&groups);
    let bounds = crate::iced_compat::Rectangle::new(crate::iced_compat::Point::ORIGIN, total);
    let regions = metrics.regions(bounds, &groups);

    assert_eq!(regions.group_bounds.len(), 2);
    assert_eq!(regions.separator_bounds.len(), 1);
    assert_eq!(regions.group_bounds[0].x, 3.0);
    assert_eq!(regions.group_bounds[0].width, 108.0);
    // Separator sits `gap-2` after the first group.
    assert_eq!(regions.separator_bounds[0].x, 3.0 + 108.0 + 8.0);
    // The second group starts `gap-2` after the separator.
    assert_eq!(regions.group_bounds[1].x, 3.0 + 108.0 + 8.0 + 16.0 + 8.0);
    // Slots tile each group edge to edge.
    let slot = metrics.slot_bounds(regions.group_bounds[0], 2);
    assert_eq!(slot.x, 3.0 + 72.0);
    assert_eq!(slot.width, 36.0);
}

#[test]
fn style_resolution_covers_all_packs_and_states() {
    for style_id in [
        shadcn_common::StyleId::Vega,
        shadcn_common::StyleId::Nova,
        shadcn_common::StyleId::Maia,
        shadcn_common::StyleId::Lyra,
        shadcn_common::StyleId::Mira,
        shadcn_common::StyleId::Luma,
        shadcn_common::StyleId::Sera,
        shadcn_common::StyleId::Rhea,
    ] {
        for theme in [
            Theme::light().with_style(style_id),
            Theme::dark().with_style(style_id),
        ] {
            for (disabled, invalid, focused) in [
                (false, false, false),
                (false, false, true),
                (false, true, true),
                (true, false, false),
            ] {
                let status = InputOtpStatus {
                    focused,
                    hovered: false,
                    disabled,
                    invalid,
                };
                let resolved = style::resolve_style(&theme, None, None, status);

                assert!(resolved.slot_text.a.is_finite());
                assert!(resolved.ring_width >= 0.0);
                assert!(resolved.radius >= 0.0);

                if invalid {
                    // `aria-invalid` recolors the resting border.
                    let destructive = theme
                        .semantic_color(twill_core::prelude::theme::SemanticColor::Destructive);
                    assert_eq!(resolved.slot_border.r, destructive.r);
                }
                if disabled {
                    // `has-disabled:opacity-50`.
                    assert!(resolved.slot_text.a <= 0.5 + f32::EPSILON);
                }
            }
        }
    }
}

#[test]
fn sera_is_underline_only_without_a_ring() {
    let theme = Theme::light().with_style(shadcn_common::StyleId::Sera);
    let resolved = style::resolve_style(&theme, None, None, InputOtpStatus::default());

    assert!(resolved.underline_only);
    assert_eq!(resolved.ring_width, 0.0);

    let pack = style::pack_recipe(shadcn_common::StyleId::Sera);
    assert_eq!(pack.slot_size_px, 40.0);
    assert_eq!(pack.slot_gap, 4.0);
}

#[test]
fn accent_color_recolors_the_active_treatment() {
    let theme = Theme::light();
    let status = InputOtpStatus {
        focused: true,
        ..InputOtpStatus::default()
    };

    let neutral = style::resolve_style(&theme, None, None, status);
    let accented = style::resolve_style(&theme, None, Some(AccentColor::Blue), status);

    assert_ne!(neutral.active_border, accented.active_border);
}

#[test]
fn tone_is_an_alias_for_color() {
    let theme = Theme::light();
    let otp: InputOtp<'_, Message> = InputOtp::new(&theme).tone(AccentColor::Blue);

    assert_eq!(otp.color, Some(AccentColor::Blue));
}

#[test]
fn debug_does_not_require_message_debug() {
    struct NoDebugMessage;

    let theme = Theme::light();
    let otp = InputOtp::<NoDebugMessage>::new(&theme).value("42");
    let debug = format!("{otp:?}");

    assert!(debug.contains("InputOtp"));
    assert!(debug.contains("max_length"));
}

#[test]
fn configuration_enums_support_hashing_and_expected_order() {
    fn hash<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    let _ = hash(&InputOtpPattern::Digits);
    let _ = hash(&InputOtpRadius::Medium);
    let _ = hash(&InputOtpStatus::default());
    assert!(InputOtpRadius::None < InputOtpRadius::Full);
    assert_eq!(InputOtpPattern::default(), InputOtpPattern::Any);
}
