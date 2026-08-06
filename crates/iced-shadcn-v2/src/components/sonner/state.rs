//! Process-wide toast queue and lifecycle management.

use std::fmt;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use super::types::{RawCallback, SonnerToast, ToastAction, ToastId, ToastOptions, ToastType};

// `ToastCallback` is only needed by the `dismiss_all` regression test below;
// importing it under `#[cfg(test)]` keeps the non-test library build clean.
#[cfg(test)]
use super::types::ToastCallback;

/// The animation window used to keep a dismissed toast available for drawing.
pub(super) const DEFAULT_ANIMATION: Duration = Duration::from_millis(180);

static TOASTS_CHANGED: AtomicBool = AtomicBool::new(false);
static TOASTS: Mutex<Vec<RawToast>> = Mutex::new(Vec::new());
/// `on_dismiss` callbacks queued by [`dismiss_all_toasts`]. They cannot be
/// published from the free function (no `Shell` is available there), so the
/// Toaster overlay drains and publishes them on its next redraw — matching
/// svelte-sonner, which fires `onDismiss` for each toast dismissed by
/// `toast.dismiss()` (no id).
static PENDING_DISMISS_CALLBACKS: Mutex<Vec<RawCallback>> = Mutex::new(Vec::new());

#[cfg(test)]
pub(super) static TEST_LOCK: Mutex<()> = Mutex::new(());

/// Runtime data stored for one toast.
pub(super) struct RawToast {
    pub(super) id: ToastId,
    pub(super) title: String,
    pub(super) toast_type: ToastType,
    pub(super) description: Option<String>,
    pub(super) duration_ms: Option<u64>,
    pub(super) dismissible: bool,
    pub(super) close_button: bool,
    pub(super) rich_colors: bool,
    pub(super) invert: bool,
    pub(super) position: Option<super::types::ToastPosition>,
    pub(super) important: bool,
    pub(super) action_label: Option<String>,
    pub(super) action_callback: Option<RawCallback>,
    pub(super) cancel_label: Option<String>,
    pub(super) cancel_callback: Option<RawCallback>,
    pub(super) on_dismiss: Option<RawCallback>,
    pub(super) on_auto_close: Option<RawCallback>,
    pub(super) created_at: Instant,
    pub(super) paused_at: Option<Instant>,
    pub(super) paused_total: Duration,
    pub(super) dismissed_at: Option<Instant>,
    pub(super) open: bool,
}

impl fmt::Debug for RawToast {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RawToast")
            .field("id", &self.id)
            .field("title", &self.title)
            .field("toast_type", &self.toast_type)
            .field("description", &self.description)
            .field("duration_ms", &self.duration_ms)
            .field("dismissible", &self.dismissible)
            .field("close_button", &self.close_button)
            .field("rich_colors", &self.rich_colors)
            .field("invert", &self.invert)
            .field("position", &self.position)
            .field("important", &self.important)
            .field("action_label", &self.action_label)
            .field("action_callback", &self.action_callback.is_some())
            .field("cancel_label", &self.cancel_label)
            .field("cancel_callback", &self.cancel_callback.is_some())
            .field("on_dismiss", &self.on_dismiss.is_some())
            .field("on_auto_close", &self.on_auto_close.is_some())
            .field("created_at", &self.created_at)
            .field("paused_at", &self.paused_at)
            .field("paused_total", &self.paused_total)
            .field("dismissed_at", &self.dismissed_at)
            .field("open", &self.open)
            .finish()
    }
}

/// A render-only copy of the data that must be available without exposing
/// callbacks to the renderer.
#[derive(Debug, Clone)]
pub(super) struct ToastSnapshot {
    pub(super) id: ToastId,
    pub(super) title: String,
    pub(super) toast_type: ToastType,
    pub(super) description: Option<String>,
    pub(super) dismissible: bool,
    pub(super) close_button: bool,
    pub(super) rich_colors: bool,
    pub(super) invert: bool,
    pub(super) position: Option<super::types::ToastPosition>,
    pub(super) important: bool,
    pub(super) action_label: Option<String>,
    pub(super) cancel_label: Option<String>,
    pub(super) created_at: Instant,
    pub(super) dismissed_at: Option<Instant>,
    pub(super) open: bool,
}

/// Result of one timer/animation lifecycle pass.
#[derive(Default)]
pub(super) struct LifecycleResult {
    pub(super) changed: bool,
    pub(super) auto_callbacks: Vec<RawCallback>,
}

impl RawToast {
    fn snapshot(&self) -> ToastSnapshot {
        ToastSnapshot {
            id: self.id,
            title: self.title.clone(),
            toast_type: self.toast_type,
            description: self.description.clone(),
            dismissible: self.dismissible,
            close_button: self.close_button,
            rich_colors: self.rich_colors,
            invert: self.invert,
            position: self.position,
            important: self.important,
            action_label: self.action_label.clone(),
            cancel_label: self.cancel_label.clone(),
            created_at: self.created_at,
            dismissed_at: self.dismissed_at,
            open: self.open,
        }
    }
}

fn mark_changed() {
    TOASTS_CHANGED.store(true, Ordering::Release);
}

pub(super) fn has_changed() -> bool {
    TOASTS_CHANGED.load(Ordering::Acquire)
}

pub(super) fn reset_changed() {
    TOASTS_CHANGED.store(false, Ordering::Release);
}

fn with_toasts<R>(operation: impl FnOnce(&[RawToast]) -> R) -> R {
    let guard = TOASTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    operation(&guard)
}

fn with_toasts_mut<R>(operation: impl FnOnce(&mut Vec<RawToast>) -> R) -> R {
    let mut guard = TOASTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    operation(&mut guard)
}

fn raw_callback(action: Option<&ToastAction>) -> Option<RawCallback> {
    action.and_then(ToastAction::callback)
}

fn raw_toast(id: ToastId, title: String, options: ToastOptions, now: Instant) -> RawToast {
    let action_label = options
        .action
        .as_ref()
        .map(|action| action.label_text().to_owned());
    let cancel_label = options
        .cancel
        .as_ref()
        .map(|action| action.label_text().to_owned());

    RawToast {
        id,
        title,
        toast_type: options.toast_type,
        description: options.description,
        duration_ms: options.duration_ms,
        dismissible: options.dismissible,
        close_button: options.close_button,
        rich_colors: options.rich_colors,
        invert: options.invert,
        position: options.position,
        important: options.important,
        action_label,
        action_callback: raw_callback(options.action.as_ref()),
        cancel_label,
        cancel_callback: raw_callback(options.cancel.as_ref()),
        on_dismiss: options
            .on_dismiss
            .as_ref()
            .map(|callback| callback.callback()),
        on_auto_close: options
            .on_auto_close
            .as_ref()
            .map(|callback| callback.callback()),
        created_at: now,
        paused_at: None,
        paused_total: Duration::ZERO,
        dismissed_at: None,
        open: true,
    }
}

pub(super) fn create_toast(toast: SonnerToast) -> ToastId {
    let (generated_id, title, options) = toast.into_parts();
    let id = options.id.unwrap_or(generated_id);
    let entry = raw_toast(id, title, options, Instant::now());

    with_toasts_mut(|toasts| {
        if let Some(existing) = toasts.iter_mut().find(|toast| toast.id == id) {
            *existing = entry;
        } else {
            toasts.insert(0, entry);
        }
    });
    mark_changed();

    id
}

pub(super) fn update_toast(id: ToastId, toast: SonnerToast) -> ToastId {
    let (_, title, mut options) = toast.into_parts();
    options.id = Some(id);
    let entry = raw_toast(id, title, options, Instant::now());

    with_toasts_mut(|toasts| {
        if let Some(existing) = toasts.iter_mut().find(|toast| toast.id == id) {
            *existing = entry;
        } else {
            toasts.insert(0, entry);
        }
    });
    mark_changed();

    id
}

pub(super) fn snapshots() -> Vec<ToastSnapshot> {
    with_toasts(|toasts| toasts.iter().map(RawToast::snapshot).collect())
}

pub(super) fn has_toasts() -> bool {
    with_toasts(|toasts| !toasts.is_empty())
}

pub(super) fn active_toast_count() -> usize {
    with_toasts(|toasts| toasts.iter().filter(|toast| toast.open).count())
}

#[cfg(test)]
pub(super) fn clear_all_toasts() {
    with_toasts_mut(Vec::clear);
}

pub(super) fn dismiss_toast(id: ToastId) -> Option<RawCallback> {
    dismiss_toast_with_reason(id, false)
}

pub(super) fn dismiss_toast_with_reason(id: ToastId, automatic: bool) -> Option<RawCallback> {
    let mut callback = None;
    let mut dismissed = false;

    with_toasts_mut(|toasts| {
        if let Some(toast) = toasts.iter_mut().find(|toast| toast.id == id)
            && toast.open
        {
            toast.open = false;
            toast.dismissed_at = Some(Instant::now());
            dismissed = true;
            callback = if automatic {
                toast.on_auto_close.take()
            } else {
                toast.on_dismiss.take()
            };
        }
    });

    if dismissed {
        mark_changed();
    }

    callback
}

pub(super) fn dismiss_all_toasts() {
    let pending: Vec<RawCallback> = with_toasts_mut(|toasts| {
        let now = Instant::now();
        let mut pending = Vec::new();
        let mut changed = false;
        for toast in toasts {
            if toast.open {
                toast.open = false;
                toast.dismissed_at = Some(now);
                if let Some(callback) = toast.on_dismiss.take() {
                    pending.push(callback);
                }
                changed = true;
            }
        }
        if changed {
            mark_changed();
        }
        pending
    });

    if !pending.is_empty() {
        let mut queue = PENDING_DISMISS_CALLBACKS
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        queue.extend(pending);
    }
}

/// Drains the `on_dismiss` callbacks queued by [`dismiss_all_toasts`] so the
/// Toaster overlay can publish them through its [`Shell`].
pub(super) fn take_pending_dismiss_callbacks() -> Vec<RawCallback> {
    let mut queue = PENDING_DISMISS_CALLBACKS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    std::mem::take(&mut *queue)
}

pub(super) fn take_action_callback(id: ToastId, cancel: bool) -> Option<RawCallback> {
    let mut callback = None;

    with_toasts_mut(|toasts| {
        // Only an open toast can publish its action: once a toast starts
        // dismissing (`open == false`) its buttons no longer fire. `take` (not
        // `clone`) makes a second click between the action press and the
        // dismissal a no-op instead of a duplicate publish.
        if let Some(toast) = toasts.iter_mut().find(|toast| toast.id == id)
            && toast.open
        {
            callback = if cancel {
                toast.cancel_callback.take()
            } else {
                toast.action_callback.take()
            };
        }
    });

    callback
}

/// Advances timers, hover pauses, and the post-dismiss animation window.
#[allow(clippy::too_many_arguments)]
pub(super) fn update_lifecycle(
    now: Instant,
    hovered: Option<ToastId>,
    pointer_over: bool,
    focused: bool,
    pause_on_hover: bool,
    pause_when_unfocused: bool,
    default_duration_ms: u64,
    animation: Duration,
) -> LifecycleResult {
    let mut result = LifecycleResult::default();

    with_toasts_mut(|toasts| {
        for toast in toasts.iter_mut() {
            if !toast.open {
                continue;
            }

            let should_pause = (pause_on_hover && (pointer_over || hovered == Some(toast.id)))
                || (pause_when_unfocused && !focused);

            if should_pause {
                if toast.paused_at.is_none() {
                    toast.paused_at = Some(now);
                    result.changed = true;
                }
                continue;
            }

            if let Some(paused_at) = toast.paused_at.take() {
                toast.paused_total += now.saturating_duration_since(paused_at);
                result.changed = true;
            }

            let duration_ms = toast.duration_ms.unwrap_or(default_duration_ms);
            let persistent_loading = toast.duration_ms.is_none() && toast.toast_type.is_loading();
            if duration_ms == 0 || persistent_loading {
                continue;
            }

            let elapsed = now
                .saturating_duration_since(toast.created_at)
                .saturating_sub(toast.paused_total);
            if elapsed >= Duration::from_millis(duration_ms) {
                toast.open = false;
                toast.dismissed_at = Some(now);
                toast.on_dismiss = None;
                if let Some(callback) = toast.on_auto_close.take() {
                    result.auto_callbacks.push(callback);
                }
                result.changed = true;
            }
        }

        let before = toasts.len();
        toasts.retain(|toast| {
            toast.open
                || toast
                    .dismissed_at
                    .is_none_or(|dismissed| now.saturating_duration_since(dismissed) < animation)
        });
        result.changed |= before != toasts.len();
    });

    if result.changed {
        mark_changed();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_update_and_dismiss_keep_toast_ids_stable() {
        let _guard = TEST_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        clear_all_toasts();

        let first = create_toast(SonnerToast::new("First"));
        let updated = update_toast(first, SonnerToast::new("Updated"));

        assert_eq!(updated, first);
        assert_eq!(active_toast_count(), 1);

        let _ = dismiss_toast(first);
        assert_eq!(active_toast_count(), 0);
        clear_all_toasts();
    }

    #[test]
    fn persistent_loading_toast_does_not_expire() {
        let _guard = TEST_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        clear_all_toasts();

        let id = create_toast(SonnerToast::new("Loading").toast_type(ToastType::Loading));
        let now = Instant::now() + Duration::from_secs(60);
        let result = update_lifecycle(now, None, false, true, false, false, 1, Duration::ZERO);

        assert!(!result.changed || active_toast_count() == 1);
        assert_eq!(active_toast_count(), 1);
        let _ = dismiss_toast(id);
        clear_all_toasts();
    }

    #[test]
    fn dismiss_all_queues_on_dismiss_callbacks() {
        let _guard = TEST_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        clear_all_toasts();
        let _ = take_pending_dismiss_callbacks();

        let _ = create_toast(
            SonnerToast::new("Queues on dismiss").on_dismiss(ToastCallback::new(|| 42u8)),
        );
        let _ = create_toast(SonnerToast::new("No callback"));
        assert_eq!(active_toast_count(), 2);

        dismiss_all_toasts();
        assert_eq!(active_toast_count(), 0, "all toasts should be dismissed");

        let pending = take_pending_dismiss_callbacks();
        assert_eq!(
            pending.len(),
            1,
            "only the toast with an on_dismiss callback should be queued"
        );
        let value = pending[0]().expect("callback produces a message");
        assert_eq!(
            value.downcast_ref::<u8>().expect("downcasts to u8"),
            &42,
            "the queued on_dismiss message survives type erasure"
        );

        // Draining twice yields nothing the second time around (idempotent).
        assert!(take_pending_dismiss_callbacks().is_empty());
        clear_all_toasts();
    }
}
