//! Backend-agnostic select value transitions.
//!
//! Mirrors bits-ui / shadcn-svelte `type="single" | "multiple"` behaviour so
//! iced and egui adapters share one toggle rule for controlled selection.

/// Selection mode of a custom select (`type` prop on the web root).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SelectMode {
    /// At most one value may be selected.
    #[default]
    Single,
    /// Any number of values may be selected; picks toggle membership.
    Multiple,
}

impl SelectMode {
    /// Returns `true` when this mode accepts multiple selected values.
    #[must_use]
    pub const fn is_multiple(self) -> bool {
        matches!(self, Self::Multiple)
    }
}

/// Computes the next single selection after `picked` is activated.
///
/// When `deselectable` is `true` and `picked` is already selected, the
/// selection clears (bits-ui default for single selects that allow empty).
#[must_use]
pub fn next_single_value<T>(current: Option<&T>, picked: &T, deselectable: bool) -> Option<T>
where
    T: Clone + PartialEq,
{
    if current.is_some_and(|value| value == picked) {
        if deselectable {
            None
        } else {
            Some(picked.clone())
        }
    } else {
        Some(picked.clone())
    }
}

/// Computes the next multiple selection after `picked` is activated.
///
/// Preserves insertion order and never inserts duplicates.
#[must_use]
pub fn next_multiple_values<T>(current: &[T], picked: &T) -> Vec<T>
where
    T: Clone + PartialEq,
{
    let mut next = Vec::with_capacity(current.len() + 1);

    let mut removed = false;
    for value in current {
        if value == picked {
            removed = true;
            continue;
        }
        next.push(value.clone());
    }

    if !removed {
        next.push(picked.clone());
    }

    next
}

/// Formats the closed-trigger label for a multiple selection, matching the
/// shadcn-svelte multiple demo (`N fruits selected`).
#[must_use]
pub fn multiple_selection_label(count: usize, empty: &str, singular: &str, plural: &str) -> String {
    match count {
        0 => empty.to_owned(),
        1 => singular.to_owned(),
        n => format!("{n} {plural}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_replaces_and_can_clear() {
        assert_eq!(next_single_value(None, &"a", true), Some("a"));
        assert_eq!(next_single_value(Some(&"a"), &"b", true), Some("b"));
        assert_eq!(next_single_value(Some(&"a"), &"a", true), None);
        assert_eq!(next_single_value(Some(&"a"), &"a", false), Some("a"));
    }

    #[test]
    fn multiple_toggles_membership() {
        assert_eq!(next_multiple_values(&[], &"a"), vec!["a"]);
        assert_eq!(next_multiple_values(&["a"], &"b"), vec!["a", "b"]);
        assert_eq!(next_multiple_values(&["a", "b"], &"a"), vec!["b"]);
    }

    #[test]
    fn multiple_label_matches_docs_demo() {
        assert_eq!(
            multiple_selection_label(0, "Select fruits", "Apple", "fruits selected"),
            "Select fruits"
        );
        assert_eq!(
            multiple_selection_label(1, "Select fruits", "Apple", "fruits selected"),
            "Apple"
        );
        assert_eq!(
            multiple_selection_label(3, "Select fruits", "Apple", "fruits selected"),
            "3 fruits selected"
        );
    }
}
