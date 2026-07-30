//! Backend-agnostic navigation helpers for ordered collections.
//!
//! Components use these functions for roving focus, popup hover, and similar
//! interactions where disabled entries must be skipped. The callbacks inspect
//! component-owned item types, so this module does not depend on iced, egui,
//! or a particular item representation.

/// Finds the first item for which `is_enabled` returns `true`.
#[must_use]
pub fn first_enabled_index<T, F>(items: &[T], is_enabled: F) -> Option<usize>
where
    F: FnMut(&T) -> bool,
{
    items.iter().position(is_enabled)
}

/// Finds the last item for which `is_enabled` returns `true`.
#[must_use]
pub fn last_enabled_index<T, F>(items: &[T], is_enabled: F) -> Option<usize>
where
    F: FnMut(&T) -> bool,
{
    items.iter().rposition(is_enabled)
}

/// Moves from `current` in one direction, skipping disabled entries.
///
/// `delta` is interpreted by sign: positive values move forward and negative
/// values move backward. A `None` current position starts before the first
/// item for forward navigation and after the last item for backward
/// navigation. With `looping` enabled, reaching an edge wraps to the opposite
/// edge. A zero delta or an empty collection returns `None`.
#[must_use]
pub fn step_index<T, F>(
    items: &[T],
    current: Option<usize>,
    delta: isize,
    looping: bool,
    mut is_enabled: F,
) -> Option<usize>
where
    F: FnMut(&T) -> bool,
{
    if items.is_empty() || delta == 0 {
        return None;
    }

    let len = items.len();
    let direction = delta.signum();
    let mut index = match current {
        Some(index) if index < len => index as isize,
        Some(_) => return None,
        None if direction > 0 => -1,
        None => len as isize,
    };

    for _ in 0..len {
        index += direction;

        if index < 0 || index >= len as isize {
            if !looping {
                return None;
            }

            index = if direction > 0 { 0 } else { len as isize - 1 };
        }

        let candidate = index as usize;
        if is_enabled(&items[candidate]) {
            return Some(candidate);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_edges_while_skipping_disabled_entries() {
        let items = [false, true, false, true];

        assert_eq!(first_enabled_index(&items, |enabled| *enabled), Some(1));
        assert_eq!(last_enabled_index(&items, |enabled| *enabled), Some(3));
    }

    #[test]
    fn steps_forward_and_backward_without_looping() {
        let items = [true, false, true];

        assert_eq!(step_index(&items, Some(0), 1, false, |item| *item), Some(2));
        assert_eq!(step_index(&items, Some(2), 1, false, |item| *item), None);
        assert_eq!(
            step_index(&items, Some(2), -1, false, |item| *item),
            Some(0)
        );
        assert_eq!(step_index(&items, Some(0), -1, false, |item| *item), None);
    }

    #[test]
    fn starts_at_the_nearest_edge_without_a_current_item() {
        let items = [false, true, false, true];

        assert_eq!(step_index(&items, None, 1, false, |item| *item), Some(1));
        assert_eq!(step_index(&items, None, -1, false, |item| *item), Some(3));
    }

    #[test]
    fn looping_wraps_and_does_not_loop_forever_when_all_are_disabled() {
        let items = [true, false, true];

        assert_eq!(step_index(&items, Some(2), 1, true, |item| *item), Some(0));
        assert_eq!(step_index(&items, Some(0), -1, true, |item| *item), Some(2));

        let disabled = [false, false];
        assert_eq!(step_index(&disabled, Some(0), 1, true, |item| *item), None);
    }
}
