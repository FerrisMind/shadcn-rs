//! Presence lifecycle without DOM (Zag `@zag-js/presence` states).
//!
//! Backends drive enter/exit animations and call [`Presence::animation_complete`]
//! when an exit transition finishes. No CSS animation-name inspection here.

/// Finite states matching Zag's presence machine.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PresenceState {
    /// Content is shown.
    Mounted,
    /// Waiting for an exit animation before unmount.
    UnmountSuspended,
    /// Content is not shown.
    #[default]
    Unmounted,
}

/// Events accepted by [`Presence`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresenceEvent {
    /// Request to show content.
    Mount,
    /// Request to hide immediately.
    Unmount,
    /// Request to hide after an exit animation.
    UnmountSuspend,
    /// Exit animation finished.
    AnimationComplete,
}

/// Controlled presence flag + lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Presence {
    present: bool,
    state: PresenceState,
}

impl Default for Presence {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Presence {
    /// Creates presence from an initial `present` flag.
    #[must_use]
    pub const fn new(present: bool) -> Self {
        Self {
            present,
            state: if present {
                PresenceState::Mounted
            } else {
                PresenceState::Unmounted
            },
        }
    }

    /// Whether the controlled present flag is true.
    #[must_use]
    pub const fn present(&self) -> bool {
        self.present
    }

    /// Current lifecycle state.
    #[must_use]
    pub const fn state(&self) -> PresenceState {
        self.state
    }

    /// Whether content should be kept in the tree (mounted or exiting).
    #[must_use]
    pub const fn is_present(&self) -> bool {
        matches!(
            self.state,
            PresenceState::Mounted | PresenceState::UnmountSuspended
        )
    }

    /// Synchronizes with an external controlled `present` prop.
    #[must_use]
    pub fn set_present(mut self, present: bool) -> Self {
        if present == self.present {
            return self;
        }
        self.present = present;
        if present {
            self.apply(PresenceEvent::Mount)
        } else if matches!(self.state, PresenceState::Mounted) {
            self.apply(PresenceEvent::UnmountSuspend)
        } else {
            self
        }
    }

    /// Applies a lifecycle event.
    #[must_use]
    pub fn apply(mut self, event: PresenceEvent) -> Self {
        self.state = match (self.state, event) {
            (PresenceState::Unmounted, PresenceEvent::Mount) => {
                self.present = true;
                PresenceState::Mounted
            }
            (PresenceState::Mounted, PresenceEvent::Unmount) => {
                self.present = false;
                PresenceState::Unmounted
            }
            (PresenceState::Mounted, PresenceEvent::UnmountSuspend) => {
                self.present = false;
                PresenceState::UnmountSuspended
            }
            (PresenceState::UnmountSuspended, PresenceEvent::Mount) => {
                self.present = true;
                PresenceState::Mounted
            }
            (
                PresenceState::UnmountSuspended,
                PresenceEvent::Unmount | PresenceEvent::AnimationComplete,
            ) => {
                self.present = false;
                PresenceState::Unmounted
            }
            (state, _) => state,
        };
        self
    }

    /// Completes a suspended exit animation.
    #[must_use]
    pub fn animation_complete(self) -> Self {
        self.apply(PresenceEvent::AnimationComplete)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mount_and_immediate_unmount() {
        let presence = Presence::new(false).apply(PresenceEvent::Mount);
        assert_eq!(presence.state(), PresenceState::Mounted);
        assert!(presence.is_present());

        let gone = presence.apply(PresenceEvent::Unmount);
        assert_eq!(gone.state(), PresenceState::Unmounted);
        assert!(!gone.is_present());
    }

    #[test]
    fn suspended_exit_waits_for_animation() {
        let presence = Presence::new(true)
            .apply(PresenceEvent::UnmountSuspend)
            .animation_complete();
        assert_eq!(presence.state(), PresenceState::Unmounted);
    }

    #[test]
    fn set_present_drives_suspend_on_hide() {
        let presence = Presence::new(true).set_present(false);
        assert_eq!(presence.state(), PresenceState::UnmountSuspended);
        assert!(presence.is_present());
    }
}
