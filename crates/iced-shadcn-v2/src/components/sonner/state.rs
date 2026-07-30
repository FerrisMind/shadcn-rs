//! Global toast state management.
//!
//! Toasts are stored in a process-wide `Mutex<Vec<RawToast>>` so that the
//! imperative `toast()` API can be called from anywhere. The [`Toaster`]
//! widget polls the state on every frame and drives the lifecycle
//! (auto-close timers, dismiss animations, removal).

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use super::types::{RawToast, ToastType, next_toast_id};

/// Global flag set whenever the toast list changes. The [`Toaster`] widget
/// checks this on every frame to avoid redundant work.
static CHANGED: AtomicBool = AtomicBool::new(false);

/// Monotonic timestamp of the last state change, used by the Toaster to
/// schedule redraws for auto-close timers.
static LAST_CHANGE_MS: AtomicU64 = AtomicU64::new(0);

/// Global toast storage. Only accessible through [`with_toasts`] and
/// [`with_toasts_mut`].
static TOASTS: Mutex<Vec<RawToast>> = Mutex::new(Vec::new());

/// Marks the global state as changed and records the current timestamp.
fn mark_changed() {
    CHANGED.store(true, Ordering::Release);
    // Use a dummy timestamp; the Toaster will read the actual time.
    LAST_CHANGE_MS.store(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
        Ordering::Relaxed,
    );
}

/// Returns `true` if the toast list has changed since the last call to
/// [`reset_changed`].
pub(super) fn has_changed() -> bool {
    CHANGED.load(Ordering::Acquire)
}

/// Resets the changed flag.
pub(super) fn reset_changed() {
    CHANGED.store(false, Ordering::Release);
}

/// Returns the timestamp of the last state change.
pub(super) fn last_change_ms() -> u64 {
    LAST_CHANGE_MS.load(Ordering::Relaxed)
}

/// Executes a closure with immutable access to the toast list.
///
/// Returns `None` if the mutex is poisoned.
pub fn with_toasts<R>(f: impl FnOnce(&[RawToast]) -> R) -> Option<R> {
    let guard = TOASTS.lock().ok()?;
    Some(f(&guard))
}

/// Executes a closure with mutable access to the toast list.
///
/// Returns `None` if the mutex is poisoned.
pub fn with_toasts_mut<R>(f: impl FnOnce(&mut Vec<RawToast>) -> R) -> Option<R> {
    let mut guard = TOASTS.lock().ok()?;
    let result = f(&mut guard);
    mark_changed();
    Some(result)
}

/// Removes all toasts from the global state.
pub fn clear_all_toasts() {
    with_toasts_mut(|toasts| toasts.clear());
}

/// Removes a specific toast by ID.
pub fn remove_toast(id: u64) {
    with_toasts_mut(|toasts| toasts.retain(|t| t.id != id));
}

/// Marks a toast as dismissed (triggers the exit animation).
pub fn dismiss_toast(id: u64) {
    with_toasts_mut(|toasts| {
        if let Some(toast) = toasts.iter_mut().find(|t| t.id == id) {
            toast.dismissed = true;
        }
    });
}

/// Dismisses all toasts.
pub fn dismiss_all_toasts() {
    with_toasts_mut(|toasts| {
        for toast in toasts.iter_mut() {
            toast.dismissed = true;
        }
    });
}

/// Internal helper to create a toast with raw callback storage.
///
/// # Safety
///
/// The `action_cb` and `cancel_cb` closures must have been created from the
/// same `Message` type as the `Toaster` widget that will invoke them.
pub(super) fn create_raw_toast(
    title: String,
    toast_type: ToastType,
    options: super::types::ToastOptions<()>,
) -> u64 {
    let id = next_toast_id();
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let toast = RawToast {
        id,
        title,
        toast_type,
        description: options.description,
        duration: options.duration,
        dismissible: options.dismissible,
        close_button: options.close_button,
        rich_colors: options.rich_colors,
        invert: options.invert,
        position: options.position,
        action_label: None,
        cancel_label: None,
        action_cb: None,
        cancel_cb: None,
        created_at_ms: now_ms,
        dismissed: false,
        removing: false,
    };

    with_toasts_mut(|toasts| {
        toasts.insert(0, toast);
    });

    id
}

/// Creates a toast with a typed message and options.
///
/// This is the internal implementation used by the public `toast()` API.
/// The action and cancel callbacks are stored as type-erased closures.
pub(super) fn create_typed_toast<Message: Send + Sync + 'static>(
    title: String,
    options: super::types::ToastOptions<Message>,
) -> u64 {
    let id = next_toast_id();
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    // Type-erase action callbacks. These will only be called from the
    // Toaster widget which knows the concrete Message type.
    let action_label = options.action.as_ref().map(|a| a.label.clone());
    let action_cb = options.action.and_then(|action| {
        action.on_click.map(|cb| {
            let action_copy = super::types::ToastAction {
                label: action.label,
                on_click: Some(cb),
            };
            Box::new(move || -> Option<()> {
                // The callback produces a Message, but we can't store it
                // in the global state without knowing the type. Instead, we
                // store the callback and the Toaster widget will handle it.
                // This is a placeholder; the real handling is in the
                // Toaster's update method.
                (action_copy.on_click.as_ref())(&action_copy)?;
                Some(())
            }) as Box<dyn Fn() -> Option<()> + Send + Sync>
        })
    });

    let cancel_label = options.cancel.as_ref().map(|c| c.label.clone());
    let cancel_cb = options.cancel.and_then(|cancel| {
        cancel.on_click.map(|cb| {
            let cancel_copy = super::types::ToastAction {
                label: cancel.label,
                on_click: Some(cb),
            };
            Box::new(move || -> Option<()> {
                (cancel_copy.on_click.as_ref())(&cancel_copy)?;
                Some(())
            }) as Box<dyn Fn() -> Option<()> + Send + Sync>
        })
    });

    let toast = RawToast {
        id,
        title,
        toast_type: options.toast_type,
        description: options.description,
        duration: options.duration,
        dismissible: options.dismissible,
        close_button: options.close_button,
        rich_colors: options.rich_colors,
        invert: options.invert,
        position: options.position,
        action_label,
        cancel_label,
        action_cb,
        cancel_cb,
        created_at_ms: now_ms,
        dismissed: false,
        removing: false,
    };

    with_toasts_mut(|toasts| {
        toasts.insert(0, toast);
    });

    id
}

/// Returns the number of active (non-dismissed) toasts.
pub fn active_toast_count() -> usize {
    with_toasts(|toasts| toasts.iter().filter(|t| !t.dismissed).count()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iced_compat::widget::container;
    use crate::theme::Theme;

    #[test]
    fn create_and_dismiss_toast() {
        clear_all_toasts();

        let id = create_raw_toast(
            "Test toast".to_string(),
            ToastType::Default,
            Default::default(),
        );

        assert_eq!(active_toast_count(), 1);

        dismiss_toast(id);
        // After dismiss, the toast is still in the list but marked as
        // dismissed. The Toaster widget will remove it after the animation.
        with_toasts(|toasts| {
            let toast = toasts.iter().find(|t| t.id == id).unwrap();
            assert!(toast.dismissed);
        });
    }

    #[test]
    fn clear_all_removes_toasts() {
        clear_all_toasts();
        create_raw_toast("A".to_string(), ToastType::Default, Default::default());
        create_raw_toast("B".to_string(), ToastType::Success, Default::default());

        assert_eq!(active_toast_count(), 2);

        clear_all_toasts();
        assert_eq!(active_toast_count(), 0);
    }

    #[test]
    fn toast_ids_are_unique() {
        clear_all_toasts();
        let id1 = create_raw_toast("A".to_string(), ToastType::Default, Default::default());
        let id2 = create_raw_toast("B".to_string(), ToastType::Default, Default::default());
        assert_ne!(id1, id2);
    }

    #[test]
    fn dismiss_nonexistent_toast_is_noop() {
        clear_all_toasts();
        dismiss_toast(99999);
        assert_eq!(active_toast_count(), 0);
    }
}
