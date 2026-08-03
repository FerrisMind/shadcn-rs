//! Shared action-slot markers for password trailing controls.

/// Which trailing action a password input should reserve padding for.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PasswordActionSlot {
    /// Visibility toggle (`Eye` / `EyeOff`).
    Toggle,
    /// Copy-to-clipboard control.
    Copy,
}
