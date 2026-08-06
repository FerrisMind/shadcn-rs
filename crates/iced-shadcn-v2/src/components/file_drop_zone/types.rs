//! Public types for the file-drop-zone component.

use std::path::PathBuf;

use shadcn_common::FileRejectedReason;

/// Interaction actions emitted by a file drop zone.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum FileDropZoneAction {
    /// User clicked the zone — open a native file picker.
    PickerRequested,
    /// One or more filesystem paths were accepted after validation.
    DropPaths(Vec<PathBuf>),
    /// A path failed validation (size, type, or count).
    Rejected {
        /// Path that was rejected.
        path: PathBuf,
        /// Why the path was rejected.
        reason: FileRejectedReason,
    },
    /// Pointer / file-drag hover entered or left the zone.
    Hovered(bool),
}

/// Application-owned hover / upload bookkeeping for one zone instance.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FileDropZoneState {
    /// True while the pointer or a file drag is over the zone.
    pub hovered: bool,
}

impl FileDropZoneState {
    /// Creates a fresh idle state.
    #[must_use]
    pub const fn new() -> Self {
        Self { hovered: false }
    }

    /// Applies hover updates from a [`FileDropZoneAction`].
    pub fn apply(&mut self, action: &FileDropZoneAction) {
        if let FileDropZoneAction::Hovered(value) = action {
            self.hovered = *value;
        }
    }
}

/// Bytes-loaded representation of a dropped or picked file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileDropZoneFile {
    /// File name including extension.
    pub name: String,
    /// Absolute or relative path the bytes were read from.
    pub path: PathBuf,
    /// File contents.
    pub bytes: Vec<u8>,
    /// Guessed MIME type from the path extension.
    pub mime: String,
}

/// Visual treatment of the drop surface.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FileDropZoneVariant {
    /// Transparent fill; dashed border only (matches the extras default).
    #[default]
    Default,
    /// Uses the card surface behind the dashed border.
    Surface,
    /// Uses the muted surface behind the dashed border.
    Soft,
}

/// How the zone presents itself — default trigger chrome vs custom child.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FileDropZoneMode {
    /// Default extras trigger (upload icon + label + optional hint).
    #[default]
    Trigger,
    /// Custom child wrapped with drop / click behaviour (textarea composition).
    Surface,
}
