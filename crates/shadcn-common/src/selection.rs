//! Backend-agnostic selection set logic ported from Zag `@zag-js/collection`.
//!
//! Keys are opaque strings owned by the caller (item ids). Collection order and
//! disabled checks are supplied by callbacks so this module stays free of GUI
//! types.

use std::collections::BTreeSet;

/// How selection responds to [`Selection::select`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SelectionMode {
    /// Selection is disabled.
    None,
    /// At most one key may be selected.
    #[default]
    Single,
    /// Multiple keys may be selected without modifiers.
    Multiple,
    /// Multiple keys via replace / extend / toggle semantics.
    Extended,
}

/// Immutable selection snapshot.
///
/// Methods return a new [`Selection`] rather than mutating in place, matching
/// Zag's copy-on-write style and keeping adapters easy to reason about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    values: BTreeSet<String>,
    mode: SelectionMode,
    deselectable: bool,
}

impl Default for Selection {
    fn default() -> Self {
        Self {
            values: BTreeSet::new(),
            mode: SelectionMode::Single,
            deselectable: true,
        }
    }
}

impl Selection {
    /// Empty selection in single mode with deselection allowed.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a selection from an iterator of keys.
    #[must_use]
    pub fn from_values<I, S>(values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::default().set_values(values)
    }

    /// Current selection mode.
    #[must_use]
    pub const fn mode(&self) -> SelectionMode {
        self.mode
    }

    /// Whether the last selected item may be cleared.
    #[must_use]
    pub const fn deselectable(&self) -> bool {
        self.deselectable
    }

    /// Sets the selection mode.
    #[must_use]
    pub fn with_mode(mut self, mode: SelectionMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets whether clearing the last item is allowed.
    #[must_use]
    pub fn with_deselectable(mut self, deselectable: bool) -> Self {
        self.deselectable = deselectable;
        self
    }

    /// Number of selected keys.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether nothing is selected.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Iterates selected keys in lexicographic order.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.values.iter().map(String::as_str)
    }

    /// Whether `key` is selected under the current mode.
    #[must_use]
    pub fn is_selected(&self, key: &str) -> bool {
        !matches!(self.mode, SelectionMode::None) && self.values.contains(key)
    }

    /// Replaces the selection with `values` (truncated to one key in single mode).
    #[must_use]
    pub fn set_values<I, S>(self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        if matches!(self.mode, SelectionMode::None) {
            return self;
        }

        let mut next = BTreeSet::new();
        for value in values {
            next.insert(value.into());
            if matches!(self.mode, SelectionMode::Single) {
                break;
            }
        }

        Self {
            values: next,
            mode: self.mode,
            deselectable: self.deselectable,
        }
    }

    /// Clears the selection when deselection is allowed.
    #[must_use]
    pub fn clear(self) -> Self {
        if self.deselectable && !self.values.is_empty() {
            Self {
                values: BTreeSet::new(),
                mode: self.mode,
                deselectable: self.deselectable,
            }
        } else {
            self
        }
    }

    /// Removes one key.
    #[must_use]
    pub fn deselect(mut self, key: &str) -> Self {
        self.values.remove(key);
        self
    }

    /// Replaces the selection with a single selectable `key`.
    #[must_use]
    pub fn replace(self, key: &str, can_select: impl FnOnce(&str) -> bool) -> Self {
        if matches!(self.mode, SelectionMode::None) || !can_select(key) {
            return self;
        }

        Self {
            values: BTreeSet::from([key.to_owned()]),
            mode: self.mode,
            deselectable: self.deselectable,
        }
    }

    /// Toggles `key` in multiple modes; replaces in single mode when absent.
    #[must_use]
    pub fn toggle(self, key: &str, can_select: impl FnOnce(&str) -> bool) -> Self {
        if matches!(self.mode, SelectionMode::None) {
            return self;
        }

        if matches!(self.mode, SelectionMode::Single) && !self.is_selected(key) {
            return self.replace(key, can_select);
        }

        let mut next = self.clone();
        if next.values.contains(key) {
            next.values.remove(key);
        } else if can_select(key) {
            next.values.insert(key.to_owned());
        }
        next
    }

    /// Primary select gesture (Zag `Selection.select` without extended modifiers).
    #[must_use]
    pub fn select(self, key: &str, can_select: impl FnOnce(&str) -> bool) -> Self {
        match self.mode {
            SelectionMode::None => self,
            SelectionMode::Single => {
                if self.is_selected(key) && self.deselectable {
                    self.toggle(key, can_select)
                } else {
                    self.replace(key, can_select)
                }
            }
            SelectionMode::Multiple => self.toggle(key, can_select),
            SelectionMode::Extended => self.replace(key, can_select),
        }
    }

    /// Selects every key in `range` that passes `can_select`.
    #[must_use]
    pub fn select_range<'a, I>(self, range: I, can_select: impl Fn(&str) -> bool) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        if matches!(self.mode, SelectionMode::None | SelectionMode::Single) {
            return self;
        }

        let mut next = self;
        for key in range {
            if can_select(key) {
                next.values.insert(key.to_owned());
            }
        }
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_mode_replaces_and_can_deselect() {
        let selection = Selection::new().select("a", |_| true).select("b", |_| true);
        assert!(selection.is_selected("b"));
        assert!(!selection.is_selected("a"));

        let cleared = selection.select("b", |_| true);
        assert!(cleared.is_empty());
    }

    #[test]
    fn multiple_mode_toggles() {
        let selection = Selection::new()
            .with_mode(SelectionMode::Multiple)
            .select("a", |_| true)
            .select("b", |_| true)
            .select("a", |_| true);
        assert!(!selection.is_selected("a"));
        assert!(selection.is_selected("b"));
    }

    #[test]
    fn none_mode_is_inert() {
        let selection = Selection::new()
            .with_mode(SelectionMode::None)
            .select("a", |_| true);
        assert!(selection.is_empty());
    }

    #[test]
    fn select_range_fills_extended_selection() {
        let selection = Selection::new()
            .with_mode(SelectionMode::Extended)
            .select_range(["a", "b", "c"], |key| key != "b");
        assert!(selection.is_selected("a"));
        assert!(!selection.is_selected("b"));
        assert!(selection.is_selected("c"));
    }
}
