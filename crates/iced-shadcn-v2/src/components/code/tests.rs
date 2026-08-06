//! Unit tests for the Code component's pure helpers.

use super::render::{LINE_NUMBER_GAP, LINE_NUMBER_WIDTH, split_lines};
use super::types::{CodeOverflow, CodeVariant};
use crate::components::button::{ButtonSize, ButtonVariant};
use crate::components::copy_button::CopyButtonStatus;

#[test]
fn split_lines_basic() {
    let lines = split_lines("fn main() {\n    println!();\n}");

    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0].number, 1);
    assert_eq!(lines[0].start, 0);
    assert_eq!(lines[0].end, 11);
    assert_eq!(lines[1].number, 2);
    assert_eq!(lines[1].start, 12);
    assert_eq!(lines[1].end, 27);
    assert_eq!(lines[2].number, 3);
    assert_eq!(lines[2].start, 28);
    assert_eq!(lines[2].end, 29);
}

#[test]
fn split_lines_ignores_trailing_newline() {
    // The Svelte component trims the code with `code.trimEnd()` before
    // rendering, so a trailing newline must not produce an empty last line.
    let lines = split_lines("a\nb\n");
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].number, 1);
    assert_eq!(lines[1].number, 2);
}

#[test]
fn split_lines_strips_carriage_returns() {
    let lines = split_lines("a\r\nb");
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].end, 1);
}

#[test]
fn split_lines_empty_source_yields_one_line() {
    let empty = split_lines("");
    assert_eq!(empty.len(), 1);
    assert_eq!(empty[0].number, 1);
    assert_eq!(empty[0].start, 0);
    assert_eq!(empty[0].end, 0);

    let spaces = split_lines("  ");
    assert_eq!(spaces.len(), 1);
    assert_eq!(spaces[0].end, 2);
}

#[test]
fn line_number_geometry_matches_reference_css() {
    // `width: 1.8rem` and `margin-right: 1.4rem` at the 16 px root.
    assert_eq!(LINE_NUMBER_WIDTH, 1.8 * 16.0);
    assert_eq!(LINE_NUMBER_GAP, 1.4 * 16.0);
}

#[test]
fn code_variant_defaults_to_card_surface() {
    assert_eq!(CodeVariant::default(), CodeVariant::Default);
}

#[test]
fn code_overflow_defaults() {
    let overflow: CodeOverflow<'_, ()> = CodeOverflow::new(true);
    assert_eq!(overflow.max_height, 300.0);
    assert!(overflow.default_collapsed);
    assert_eq!(overflow.collapsed_override, None);
    assert!(overflow.expand_button.is_none());
    assert!(overflow.on_collapse_change.is_none());
}

#[test]
fn code_overflow_can_be_overridden() {
    let overflow: CodeOverflow<'_, ()> = CodeOverflow::new(false)
        .collapsed_override(Some(true))
        .max_height(400.0);
    assert_eq!(overflow.collapsed_override, Some(true));
    assert_eq!(overflow.max_height, 400.0);
}

#[test]
fn code_copy_button_defaults_match_reference() {
    #[derive(Debug, Clone, PartialEq)]
    enum Message {
        Copy,
    }

    let button = super::CodeCopyButton::new(Message::Copy);
    assert_eq!(button.variant, ButtonVariant::Ghost);
    assert_eq!(button.size, ButtonSize::Icon);
    assert_eq!(button.radius, None);
    assert_eq!(button.status, CopyButtonStatus::Idle);
    assert!(button.icon.is_none());
    assert_eq!(button.on_copy, Message::Copy);
}

#[test]
fn language_names_resolve_with_fallback() {
    use shadcn_common::LanguageId;

    assert_eq!(LanguageId::from("rust"), LanguageId::Rust);
    assert_eq!(LanguageId::from("ts"), LanguageId::TypeScript);
    assert_eq!(LanguageId::from("json"), LanguageId::Json);
    assert_eq!(LanguageId::from("does-not-exist"), LanguageId::Text);
}
