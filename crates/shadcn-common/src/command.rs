//! Backend-agnostic command-palette filtering and keyboard helpers.
//!
//! Ports the bits-ui / cmdk filter contract used by shadcn-svelte `Command`:
//! empty queries match everything, and each item scores against its value and
//! keywords. Highlight stepping reuses [`crate::collection_navigation`].

use crate::collection_navigation::{first_enabled_index, last_enabled_index, step_index};

/// Filter callback used when `should_filter` is enabled.
///
/// Returns a score in `[0.0, 1.0]`; `0.0` hides the item.
pub type CommandFilter = fn(value: &str, search: &str, keywords: &[&str]) -> f32;

/// Default cmdk-style subsequence filter over `value` and `keywords`.
#[must_use]
pub fn default_command_filter(value: &str, search: &str, keywords: &[&str]) -> f32 {
    let mut best = fuzzy_score(search, value);
    for keyword in keywords {
        best = best.max(fuzzy_score(search, keyword));
    }
    best
}

/// Scores how well `query` is a subsequence of `text`.
///
/// A full subsequence match returns the matched-character ratio; a partial
/// match halves that ratio. Empty queries score `1.0`.
#[must_use]
pub fn fuzzy_score(query: &str, text: &str) -> f32 {
    let query = query.trim().to_lowercase();
    let text = text.to_lowercase();
    if query.is_empty() {
        return 1.0;
    }
    if text.is_empty() {
        return 0.0;
    }

    let mut matched = 0usize;
    let mut query_chars = query.chars();
    let mut target = query_chars.next();
    for ch in text.chars() {
        if Some(ch) == target {
            matched += 1;
            target = query_chars.next();
            if target.is_none() {
                break;
            }
        }
    }

    if matched == 0 {
        return 0.0;
    }

    let ratio = matched as f32 / query.chars().count() as f32;
    if target.is_none() { ratio } else { ratio * 0.5 }
}

/// Whether an item should stay visible for the current search.
#[must_use]
pub fn command_matches(
    query: &str,
    value: &str,
    keywords: &[&str],
    should_filter: bool,
    filter: CommandFilter,
) -> bool {
    if !should_filter || query.trim().is_empty() {
        return true;
    }
    filter(value, query, keywords) > 0.0
}

/// Selects the first enabled entry, used after the filter set changes.
#[must_use]
pub fn first_selectable_index(enabled: &[bool]) -> Option<usize> {
    first_enabled_index(enabled, |ok| *ok)
}

/// Selects the last enabled entry.
#[must_use]
pub fn last_selectable_index(enabled: &[bool]) -> Option<usize> {
    last_enabled_index(enabled, |ok| *ok)
}

/// Moves the highlight by `delta`, skipping disabled / hidden entries.
#[must_use]
pub fn step_selectable_index(
    enabled: &[bool],
    current: Option<usize>,
    delta: isize,
    looping: bool,
) -> Option<usize> {
    step_index(enabled, current, delta, looping, |ok| *ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzy_score_matches_subsequence() {
        assert!(fuzzy_score("set", "settings") > 0.0);
        assert_eq!(fuzzy_score("zzz", "settings"), 0.0);
    }

    #[test]
    fn default_filter_uses_keywords() {
        let score = default_command_filter("Billing", "pay", &["payments"]);
        assert!(score > 0.0);
    }

    #[test]
    fn empty_query_matches_everything() {
        assert!(command_matches(
            "  ",
            "Calendar",
            &[],
            true,
            default_command_filter
        ));
        assert!(command_matches(
            "",
            "Calendar",
            &[],
            true,
            default_command_filter
        ));
    }

    #[test]
    fn stepping_skips_disabled() {
        let enabled = [true, false, true];
        assert_eq!(step_selectable_index(&enabled, Some(0), 1, false), Some(2));
        assert_eq!(first_selectable_index(&enabled), Some(0));
        assert_eq!(last_selectable_index(&enabled), Some(2));
    }
}
