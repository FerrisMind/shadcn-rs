//! File drop zone ported from shadcn-svelte-extras to iced-shadcn-v2.
//!
//! Composition mirrors the web component:
//!
//! ```text
//! FileDropZone (Root config)
//! ├── .trigger()          // default extras chrome
//! ├── .trigger_with(el)   // custom trigger content
//! └── .textarea(el)       // drop / click wrapper for arbitrary content
//! ```
//!
//! Extras markup has **no** pack-specific `.cn-*` tables (same idea as Form).
//! Choosing Rhea (or Nova, …) on the shared [`Theme`] still styles the zone:
//! dashed `rounded-lg` resolves through the pack radius scale, colors/fonts
//! come from the theme palette, and composed parts (e.g. Button in demos)
//! use their own pack-aware recipes via `theme.style_id()`.
//!
//! Validation, size formatting, and accept matching live in
//! [`shadcn_common`] so egui can reuse the same rules.
//!
//! ```rust,no_run
//! use iced::Element;
//! use iced_shadcn_v2::{
//!     ACCEPT_IMAGE, FileDropZone, FileDropZoneAction, FileDropZoneState, MEGABYTE, Theme,
//! };
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Zone(FileDropZoneAction),
//! }
//!
//! fn view<'a>(theme: &'a Theme, state: &'a FileDropZoneState, count: usize) -> Element<'a, Message> {
//!     FileDropZone::new(theme, state)
//!         .max_files(4)
//!         .file_count(count)
//!         .max_file_size(3 * MEGABYTE)
//!         .accept(ACCEPT_IMAGE)
//!         .on_action(Message::Zone)
//!         .trigger()
//! }
//! ```

mod geometry;
mod render;
mod style;
mod types;

#[cfg(test)]
mod tests;

#[cfg(feature = "rfd")]
pub use render::pick_files;
pub use render::{load_files, partition_paths};
pub use types::{
    FileDropZoneAction, FileDropZoneFile, FileDropZoneMode, FileDropZoneState, FileDropZoneVariant,
};

use std::fmt;
use std::rc::Rc;

use crate::iced_compat::{Element, Length};
use crate::theme::Theme;

use shadcn_common::FileDropZoneConfig;

/// Builder-first file drop zone styled from `shadcn-common` theme tokens.
///
/// The zone recipe itself is pack-invariant; pass the app [`Theme`] so
/// radius / palette / composed controls follow the active style pack
/// (Rhea, Nova, …). Call [`Self::trigger`] for the default extras UI,
/// [`Self::trigger_with`] for custom content, or [`Self::textarea`] for the
/// textarea composition pattern.
#[must_use = "builders do nothing unless turned into an iced Element"]
pub struct FileDropZone<'a, Message> {
    theme: &'a Theme,
    state: &'a FileDropZoneState,
    config: FileDropZoneConfig,
    variant: FileDropZoneVariant,
    width: Length,
    height: Option<Length>,
    on_action: Option<Rc<dyn Fn(FileDropZoneAction) -> Message + 'a>>,
}

impl<Message> fmt::Debug for FileDropZone<'_, Message> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FileDropZone")
            .field("theme", &self.theme)
            .field("state", &self.state)
            .field("config", &self.config)
            .field("variant", &self.variant)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("on_action", &self.on_action.is_some())
            .finish()
    }
}

impl<'a, Message> FileDropZone<'a, Message> {
    /// Creates a drop zone bound to `theme` and application-owned `state`.
    ///
    /// ```rust
    /// use iced_shadcn_v2::{FileDropZone, FileDropZoneState, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message {}
    /// let theme = Theme::light();
    /// let state = FileDropZoneState::new();
    /// let _zone = FileDropZone::<Message>::new(&theme, &state);
    /// ```
    pub fn new(theme: &'a Theme, state: &'a FileDropZoneState) -> Self {
        Self {
            theme,
            state,
            config: FileDropZoneConfig::new(),
            variant: FileDropZoneVariant::Default,
            width: Length::Fill,
            height: None,
            on_action: None,
        }
    }

    /// Disables interaction (`disabled` on the web component).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.config.disabled = disabled;
        self
    }

    /// Sets the maximum number of files (`maxFiles`).
    pub fn max_files(mut self, max_files: usize) -> Self {
        self.config.max_files = Some(max_files);
        self
    }

    /// Sets the current uploaded count (`fileCount`).
    pub fn file_count(mut self, file_count: usize) -> Self {
        self.config.file_count = Some(file_count);
        self
    }

    /// Sets the per-file size limit in bytes (`maxFileSize`).
    pub fn max_file_size(mut self, max_file_size: u64) -> Self {
        self.config.max_file_size = Some(max_file_size);
        self
    }

    /// Sets the HTML-style accept list (`accept`).
    pub fn accept(mut self, accept: impl Into<String>) -> Self {
        self.config.accept = Some(accept.into());
        self
    }

    /// Marks an upload in flight (blocks further picks, matching extras).
    pub fn uploading(mut self, uploading: bool) -> Self {
        self.config.uploading = uploading;
        self
    }

    /// Replaces the entire configuration at once.
    pub fn config(mut self, config: FileDropZoneConfig) -> Self {
        self.config = config;
        self
    }

    /// Sets the surface fill treatment.
    pub fn variant(mut self, variant: FileDropZoneVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the widget width.
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the widget height (defaults to the extras `h-48` for triggers).
    pub fn height(mut self, height: Length) -> Self {
        self.height = Some(height);
        self
    }

    /// Routes every [`FileDropZoneAction`] through `f`.
    pub fn on_action(mut self, f: impl Fn(FileDropZoneAction) -> Message + 'a) -> Self {
        self.on_action = Some(Rc::new(f));
        self
    }

    /// Builds the default extras trigger UI.
    ///
    /// ```rust,no_run
    /// use iced::Element;
    /// use iced_shadcn_v2::{FileDropZone, FileDropZoneAction, FileDropZoneState, Theme};
    ///
    /// # #[derive(Debug, Clone)]
    /// # enum Message { Zone(FileDropZoneAction) }
    /// fn view<'a>(theme: &'a Theme, state: &'a FileDropZoneState) -> Element<'a, Message> {
    ///     FileDropZone::new(theme, state)
    ///         .on_action(Message::Zone)
    ///         .trigger()
    /// }
    /// ```
    pub fn trigger(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let on_action = self.require_on_action();
        render::build_default_trigger(
            self.theme,
            &self.config,
            self.state,
            self.variant,
            self.width,
            on_action,
        )
    }

    /// Builds a custom trigger with caller-provided content.
    pub fn trigger_with(self, child: impl Into<Element<'a, Message>>) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let on_action = self.require_on_action();
        let height = self
            .height
            .unwrap_or(Length::Fixed(self.theme.style.file_drop_zone().height_px));
        render::build_surface(
            self.theme,
            &self.config,
            self.state,
            self.variant,
            self.width,
            height,
            child.into(),
            on_action,
        )
    }

    /// Builds the textarea composition: arbitrary content with drop / click.
    pub fn textarea(self, child: impl Into<Element<'a, Message>>) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        self.trigger_with(child)
    }

    fn require_on_action(&self) -> Rc<dyn Fn(FileDropZoneAction) -> Message + 'a> {
        self.on_action.clone().unwrap_or_else(|| {
            Rc::new(|_| {
                panic!("FileDropZone requires `.on_action(...)` before building an Element")
            })
        })
    }
}

/// Shorthand for [`FileDropZone::new`].
pub fn file_drop_zone<'a, Message>(
    theme: &'a Theme,
    state: &'a FileDropZoneState,
) -> FileDropZone<'a, Message> {
    FileDropZone::new(theme, state)
}
