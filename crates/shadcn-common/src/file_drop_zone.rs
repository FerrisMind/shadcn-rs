//! Backend-agnostic file-drop-zone validation and helpers.
//!
//! Ported from shadcn-svelte-extras `FileDropZone` so iced and egui share the
//! same accept/size/count rules and human-readable size formatting.

use std::fmt;
use std::path::Path;

/// One byte.
pub const BYTE: u64 = 1;
/// One kilobyte (decimal, matching the extras `displaySize` helper).
pub const KILOBYTE: u64 = 1000;
/// One megabyte (decimal).
pub const MEGABYTE: u64 = 1000 * KILOBYTE;
/// One gigabyte (decimal).
pub const GIGABYTE: u64 = 1000 * MEGABYTE;

/// HTML `accept` shortcut for any image MIME type.
pub const ACCEPT_IMAGE: &str = "image/*";
/// HTML `accept` shortcut for any video MIME type.
pub const ACCEPT_VIDEO: &str = "video/*";
/// HTML `accept` shortcut for any audio MIME type.
pub const ACCEPT_AUDIO: &str = "audio/*";

/// Why a candidate file was rejected before upload.
///
/// Display strings match the extras `FileRejectedReason` literals so toast /
/// status copy stays identical across backends.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FileRejectedReason {
    /// `file.size > max_file_size`.
    MaximumFileSizeExceeded,
    /// File name / MIME did not match the `accept` list.
    FileTypeNotAllowed,
    /// `file_number > max_files` (1-based including already-uploaded count).
    MaximumFilesUploaded,
}

impl fmt::Display for FileRejectedReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::MaximumFileSizeExceeded => "Maximum file size exceeded",
            Self::FileTypeNotAllowed => "File type not allowed",
            Self::MaximumFilesUploaded => "Maximum files uploaded",
        })
    }
}

/// Configuration shared by every file-drop-zone instance.
///
/// Mirrors the extras `FileDropZone.Root` props (`disabled`, `maxFiles`,
/// `fileCount`, `maxFileSize`, `accept`) plus the internal `uploading` flag.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct FileDropZoneConfig {
    /// When true the zone ignores drop / pick / paste.
    pub disabled: bool,
    /// Hard cap on total uploaded files (`maxFiles`).
    pub max_files: Option<usize>,
    /// How many files are already uploaded (`fileCount`). Required for
    /// meaningful `max_files` enforcement.
    pub file_count: Option<usize>,
    /// Per-file size limit in bytes (`maxFileSize`).
    pub max_file_size: Option<u64>,
    /// Comma-separated accept list (`accept`) — extensions (`.png`), wildcards
    /// (`image/*`), or exact MIME types (`application/pdf`).
    pub accept: Option<String>,
    /// True while an upload callback is in flight (blocks further picks).
    pub uploading: bool,
}

impl FileDropZoneConfig {
    /// Creates an empty (unrestricted) configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            disabled: false,
            max_files: None,
            file_count: None,
            max_file_size: None,
            accept: None,
            uploading: false,
        }
    }

    /// Sets [`Self::disabled`].
    #[must_use]
    pub const fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets [`Self::max_files`].
    #[must_use]
    pub const fn with_max_files(mut self, max_files: usize) -> Self {
        self.max_files = Some(max_files);
        self
    }

    /// Sets [`Self::file_count`].
    #[must_use]
    pub const fn with_file_count(mut self, file_count: usize) -> Self {
        self.file_count = Some(file_count);
        self
    }

    /// Sets [`Self::max_file_size`].
    #[must_use]
    pub const fn with_max_file_size(mut self, max_file_size: u64) -> Self {
        self.max_file_size = Some(max_file_size);
        self
    }

    /// Sets [`Self::accept`].
    #[must_use]
    pub fn with_accept(mut self, accept: impl Into<String>) -> Self {
        self.accept = Some(accept.into());
        self
    }

    /// Sets [`Self::uploading`].
    #[must_use]
    pub const fn with_uploading(mut self, uploading: bool) -> Self {
        self.uploading = uploading;
        self
    }
}

/// Metadata for one file candidate under validation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FileCandidate {
    /// File name including extension (`avatar.PNG`).
    pub name: String,
    /// MIME type (`image/png`). Empty string is treated like the browser's
    /// unknown-type case and only matches extension patterns.
    pub mime: String,
    /// Byte length when known (required to enforce `max_file_size`).
    pub size: Option<u64>,
}

impl FileCandidate {
    /// Builds a candidate from a filesystem path using extension-based MIME
    /// guessing. Size is read from metadata when available.
    #[must_use]
    pub fn from_path(path: &Path) -> Self {
        let name = path
            .file_name()
            .map(|value| value.to_string_lossy().into_owned())
            .unwrap_or_default();
        let mime = guess_mime(path).to_owned();
        let size = std::fs::metadata(path).ok().map(|meta| meta.len());
        Self { name, mime, size }
    }
}

/// Returns whether the zone currently accepts new files.
///
/// Matches the extras `canUploadFiles` derived value: disabled, uploading, or
/// already at `max_files` all return `false`.
#[must_use]
pub const fn can_upload(config: &FileDropZoneConfig) -> bool {
    if config.disabled || config.uploading {
        return false;
    }
    !matches!(
        (config.max_files, config.file_count),
        (Some(max), Some(count)) if count >= max
    )
}

/// Whether the hidden file input should allow multiple selection.
///
/// Matches extras: `maxFiles - fileCount > 1` (or unlimited when `maxFiles` is
/// unset).
#[must_use]
pub const fn accepts_multiple(config: &FileDropZoneConfig) -> bool {
    match (config.max_files, config.file_count) {
        (Some(max), Some(count)) => max.saturating_sub(count) > 1,
        (Some(max), None) => max > 1,
        (None, _) => true,
    }
}

/// Validates one candidate against `config`.
///
/// `file_number` is 1-based and should already include already-uploaded files
/// (`fileCount + index + 1` in the extras source).
#[must_use]
pub fn should_accept_file(
    candidate: &FileCandidate,
    file_number: usize,
    config: &FileDropZoneConfig,
) -> Option<FileRejectedReason> {
    if let Some(max_size) = config.max_file_size
        && candidate.size.is_some_and(|size| size > max_size)
    {
        return Some(FileRejectedReason::MaximumFileSizeExceeded);
    }

    if let Some(max_files) = config.max_files
        && file_number > max_files
    {
        return Some(FileRejectedReason::MaximumFilesUploaded);
    }

    if let Some(accept) = config.accept.as_deref()
        && !accept_matches(&candidate.name, &candidate.mime, accept)
    {
        return Some(FileRejectedReason::FileTypeNotAllowed);
    }

    None
}

/// Splits candidates into accepted / rejected lists using [`should_accept_file`].
#[must_use]
pub fn partition_candidates(
    candidates: impl IntoIterator<Item = FileCandidate>,
    config: &FileDropZoneConfig,
) -> (Vec<FileCandidate>, Vec<(FileCandidate, FileRejectedReason)>) {
    let base = config.file_count.unwrap_or(0);
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for (index, candidate) in candidates.into_iter().enumerate() {
        let file_number = base + index + 1;
        if let Some(reason) = should_accept_file(&candidate, file_number, config) {
            rejected.push((candidate, reason));
        } else {
            accepted.push(candidate);
        }
    }

    (accepted, rejected)
}

/// Formats a byte length the way extras `displaySize` does (rounded whole units).
#[must_use]
pub fn display_size(bytes: u64) -> String {
    if bytes < KILOBYTE {
        return format!("{bytes} B");
    }
    if bytes < MEGABYTE {
        return format!("{} KB", (bytes as f64 / KILOBYTE as f64).round() as u64);
    }
    if bytes < GIGABYTE {
        return format!("{} MB", (bytes as f64 / MEGABYTE as f64).round() as u64);
    }
    format!("{} GB", (bytes as f64 / GIGABYTE as f64).round() as u64)
}

/// Builds the default trigger subtitle when limits are configured.
///
/// Mirrors the conditional copy in `file-drop-zone-trigger.svelte`.
#[must_use]
pub fn default_hint(config: &FileDropZoneConfig) -> Option<String> {
    match (config.max_files, config.max_file_size) {
        (Some(max), Some(size)) => Some(format!(
            "You can upload {max} files (up to {} each)",
            display_size(size)
        )),
        (Some(max), None) => Some(format!("You can upload {max} files")),
        (None, Some(size)) => Some(format!("Maximum size {}", display_size(size))),
        (None, None) => None,
    }
}

/// Default primary label for the empty trigger surface.
pub const DEFAULT_TRIGGER_LABEL: &str = "Drag 'n' drop files here, or click to select files";

/// Returns whether `name`/`mime` match a comma-separated HTML `accept` list.
#[must_use]
pub fn accept_matches(name: &str, mime: &str, accept: &str) -> bool {
    let patterns: Vec<&str> = accept
        .split(',')
        .map(str::trim)
        .filter(|pattern| !pattern.is_empty())
        .collect();
    if patterns.is_empty() {
        return true;
    }

    let file_name = name.to_ascii_lowercase();
    let file_type = mime.to_ascii_lowercase();

    patterns.iter().any(|pattern| {
        let pattern = pattern.to_ascii_lowercase();
        if file_type.is_empty() || pattern.starts_with('.') {
            file_name.ends_with(&pattern)
        } else if let Some(base) = pattern.strip_suffix("/*") {
            file_type.starts_with(&format!("{base}/"))
        } else {
            file_type == pattern
        }
    })
}

/// Guesses a MIME type from a path extension (common image/audio/video/docs).
#[must_use]
pub fn guess_mime(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|value| value.to_ascii_lowercase())
        .as_deref()
    {
        Some("png") | Some("apng") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("avif") => "image/avif",
        Some("svg") => "image/svg+xml",
        Some("bmp") => "image/bmp",
        Some("txt") => "text/plain",
        Some("pdf") => "application/pdf",
        Some("json") => "application/json",
        Some("mp4") => "video/mp4",
        Some("mov") => "video/quicktime",
        Some("mp3") => "audio/mpeg",
        Some("wav") => "audio/wav",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_extension_is_case_insensitive() {
        assert!(accept_matches("avatar.PNG", "", ".png"));
    }

    #[test]
    fn accept_wildcard_matches_family() {
        assert!(accept_matches("a.jpg", "image/jpeg", "image/*"));
        assert!(!accept_matches("a.mp4", "video/mp4", "image/*"));
    }

    #[test]
    fn accept_exact_mime() {
        assert!(accept_matches(
            "a.pdf",
            "application/pdf",
            "application/pdf"
        ));
    }

    #[test]
    fn rejects_over_max_files() {
        let config = FileDropZoneConfig::new().with_max_files(2);
        let candidate = FileCandidate {
            name: "a.txt".into(),
            mime: "text/plain".into(),
            size: Some(1),
        };
        assert_eq!(
            should_accept_file(&candidate, 3, &config),
            Some(FileRejectedReason::MaximumFilesUploaded)
        );
    }

    #[test]
    fn rejects_over_max_size() {
        let config = FileDropZoneConfig::new().with_max_file_size(5);
        let candidate = FileCandidate {
            name: "a.bin".into(),
            mime: "application/octet-stream".into(),
            size: Some(10),
        };
        assert_eq!(
            should_accept_file(&candidate, 1, &config),
            Some(FileRejectedReason::MaximumFileSizeExceeded)
        );
    }

    #[test]
    fn can_upload_respects_count_and_uploading() {
        let at_cap = FileDropZoneConfig::new()
            .with_max_files(2)
            .with_file_count(2);
        assert!(!can_upload(&at_cap));
        assert!(!can_upload(&FileDropZoneConfig::new().with_uploading(true)));
        assert!(can_upload(&FileDropZoneConfig::new()));
    }

    #[test]
    fn display_size_matches_extras() {
        assert_eq!(display_size(500), "500 B");
        assert_eq!(display_size(3_000), "3 KB");
        assert_eq!(display_size(3 * MEGABYTE), "3 MB");
    }

    #[test]
    fn default_hint_composes_limits() {
        let both = FileDropZoneConfig::new()
            .with_max_files(4)
            .with_max_file_size(3 * MEGABYTE);
        assert_eq!(
            default_hint(&both).as_deref(),
            Some("You can upload 4 files (up to 3 MB each)")
        );
    }

    #[test]
    fn rejected_reason_display_matches_extras() {
        assert_eq!(
            FileRejectedReason::FileTypeNotAllowed.to_string(),
            "File type not allowed"
        );
    }
}
