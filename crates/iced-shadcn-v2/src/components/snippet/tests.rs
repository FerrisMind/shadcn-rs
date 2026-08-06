//! Unit tests for the snippet types.

use crate::components::snippet::types::{SnippetRadius, SnippetText, SnippetVariant};

#[test]
fn single_text_lines_split_on_newlines() {
    assert_eq!(SnippetText::from("a\nb").lines(), vec!["a", "b"]);
    assert_eq!(SnippetText::from("a").lines(), vec!["a"]);
    assert_eq!(SnippetText::from("").lines(), vec![""]);
    assert_eq!(SnippetText::from("a\n").lines(), vec!["a", ""]);
}

#[test]
fn single_text_copy_keeps_raw_string() {
    assert_eq!(SnippetText::from("a\nb").copy_text(), "a\nb");
    assert_eq!(SnippetText::from("x").copy_text(), "x");
}

#[test]
fn lines_text_joins_entries_with_newline_for_copy() {
    let text = SnippetText::from(vec!["a".to_owned(), "b".to_owned()]);
    assert_eq!(text.lines(), vec!["a", "b"]);
    assert_eq!(text.copy_text(), "a\nb");
}

#[test]
fn lines_text_keeps_per_entry_newlines_in_rows() {
    let text = SnippetText::from(vec!["a\nb".to_owned()]);
    assert_eq!(text.lines(), vec!["a", "b"]);
    assert_eq!(text.copy_text(), "a\nb");
}

#[test]
fn lines_text_empty_list_renders_nothing_and_copies_empty() {
    let text = SnippetText::from(Vec::<String>::new());
    assert!(text.lines().is_empty());
    assert_eq!(text.copy_text(), "");
}

#[test]
fn str_slices_convert_to_owned_lines() {
    let text = SnippetText::from(vec!["a", "b"]);
    assert_eq!(text.lines(), vec!["a", "b"]);
}

#[test]
fn single_is_the_default_text_kind() {
    assert_eq!(SnippetText::default(), SnippetText::Single(String::new()));
}

#[test]
fn default_variant_is_default() {
    assert_eq!(SnippetVariant::default(), SnippetVariant::Default);
}

#[test]
fn radius_ordering_matches_corner_roundness() {
    assert!(SnippetRadius::None < SnippetRadius::Small);
    assert!(SnippetRadius::Small < SnippetRadius::Medium);
    assert!(SnippetRadius::Medium < SnippetRadius::Large);
    assert!(SnippetRadius::Large < SnippetRadius::Full);
    assert_eq!(SnippetRadius::default(), SnippetRadius::Medium);
}
