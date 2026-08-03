//! Backend-agnostic keyboard navigation policy.
//!
//! Backends translate their native key type into [`NavKey`], apply this
//! policy, and then map the resulting [`NavAction`] to component behavior.
//! Keeping that translation separate makes RTL and orientation rules
//! consistent between iced and egui.

/// Axis used by arrow-key navigation.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Orientation {
    /// Left/right arrows navigate.
    #[default]
    Horizontal,
    /// Up/down arrows navigate.
    Vertical,
}

/// Inline direction used to resolve horizontal arrow keys.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction {
    /// Left-to-right layout: right is forward.
    #[default]
    Ltr,
    /// Right-to-left layout: left is forward.
    Rtl,
}

/// Key identities needed by roving-focus components.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavKey {
    /// The left arrow key.
    ArrowLeft,
    /// The right arrow key.
    ArrowRight,
    /// The up arrow key.
    ArrowUp,
    /// The down arrow key.
    ArrowDown,
    /// The Home key.
    Home,
    /// The End key.
    End,
    /// The Enter key.
    Enter,
    /// The Space key.
    Space,
}

/// Semantic action produced by [`resolve_nav_action`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavAction {
    /// Move to the previous enabled item.
    Previous,
    /// Move to the next enabled item.
    Next,
    /// Move to the first enabled item.
    First,
    /// Move to the last enabled item.
    Last,
    /// Activate the currently focused item.
    Activate,
}

/// Resolves a backend-neutral key into a navigation action.
#[must_use]
pub const fn resolve_nav_action(
    key: NavKey,
    orientation: Orientation,
    direction: Direction,
) -> Option<NavAction> {
    match key {
        NavKey::ArrowLeft if matches!(orientation, Orientation::Horizontal) => {
            Some(match direction {
                Direction::Ltr => NavAction::Previous,
                Direction::Rtl => NavAction::Next,
            })
        }
        NavKey::ArrowRight if matches!(orientation, Orientation::Horizontal) => {
            Some(match direction {
                Direction::Ltr => NavAction::Next,
                Direction::Rtl => NavAction::Previous,
            })
        }
        NavKey::ArrowUp if matches!(orientation, Orientation::Vertical) => {
            Some(NavAction::Previous)
        }
        NavKey::ArrowDown if matches!(orientation, Orientation::Vertical) => Some(NavAction::Next),
        NavKey::Home => Some(NavAction::First),
        NavKey::End => Some(NavAction::Last),
        NavKey::Enter | NavKey::Space => Some(NavAction::Activate),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_keys_follow_text_direction() {
        assert_eq!(
            resolve_nav_action(NavKey::ArrowRight, Orientation::Horizontal, Direction::Ltr),
            Some(NavAction::Next)
        );
        assert_eq!(
            resolve_nav_action(NavKey::ArrowRight, Orientation::Horizontal, Direction::Rtl),
            Some(NavAction::Previous)
        );
        assert_eq!(
            resolve_nav_action(NavKey::ArrowUp, Orientation::Horizontal, Direction::Ltr),
            None
        );
    }

    #[test]
    fn vertical_and_boundary_keys_are_orientation_independent() {
        assert_eq!(
            resolve_nav_action(NavKey::ArrowDown, Orientation::Vertical, Direction::Rtl),
            Some(NavAction::Next)
        );
        assert_eq!(
            resolve_nav_action(NavKey::Home, Orientation::Horizontal, Direction::Ltr),
            Some(NavAction::First)
        );
        assert_eq!(
            resolve_nav_action(NavKey::Space, Orientation::Vertical, Direction::Rtl),
            Some(NavAction::Activate)
        );
    }
}
