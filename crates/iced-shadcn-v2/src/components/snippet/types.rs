//! Configuration types used by the snippet component.

/// Text displayed by a [`super::Snippet`].
///
/// Mirrors the `text: string | string[]` prop of the reference Svelte
/// component: a single string renders as one `<pre>`-style block (embedded
/// newlines split into rows), while a list renders one block per entry. The
/// copy action always receives the raw text joined by `\n`, exactly like the
/// web component's `text.join('\n')`.
///
/// ```rust
/// use iced_shadcn_v2::SnippetText;
///
/// let single = SnippetText::from("npx jsrepo add ui/snippet");
/// let lines = SnippetText::from(vec![
///     "npx jsrepo add".to_owned(),
///     "npx jsrepo add ui/snippet".to_owned(),
/// ]);
/// assert_eq!(lines.copy_text(), "npx jsrepo add\nnpx jsrepo add ui/snippet");
/// assert!(single.copy_text().ends_with("snippet"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SnippetText {
    /// One code block; `\n` sequences split into separate rows.
    Single(String),
    /// One code block per entry.
    Lines(Vec<String>),
}

impl Default for SnippetText {
    fn default() -> Self {
        Self::Single(String::new())
    }
}

impl SnippetText {
    /// The rows displayed inside the frame, in order.
    pub fn lines(&self) -> Vec<&str> {
        match self {
            Self::Single(text) => text.split('\n').collect(),
            Self::Lines(lines) => lines.iter().flat_map(|line| line.split('\n')).collect(),
        }
    }

    /// The exact text handed to the copy action (`\n`-joined for lists).
    pub fn copy_text(&self) -> String {
        match self {
            Self::Single(text) => text.clone(),
            Self::Lines(lines) => lines.join("\n"),
        }
    }
}

impl From<&str> for SnippetText {
    fn from(text: &str) -> Self {
        Self::Single(text.to_owned())
    }
}

impl From<String> for SnippetText {
    fn from(text: String) -> Self {
        Self::Single(text)
    }
}

impl From<Vec<String>> for SnippetText {
    fn from(lines: Vec<String>) -> Self {
        Self::Lines(lines)
    }
}

impl From<Vec<&str>> for SnippetText {
    fn from(lines: Vec<&str>) -> Self {
        Self::Lines(lines.into_iter().map(str::to_owned).collect())
    }
}

/// Visual treatment of a [`super::Snippet`] frame.
///
/// ```rust
/// use iced_shadcn_v2::SnippetVariant;
///
/// assert_eq!(SnippetVariant::default(), SnippetVariant::Default);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SnippetVariant {
    /// `border-border bg-card` — neutral card surface.
    #[default]
    Default,
    /// `border-border bg-accent` — accent surface.
    Secondary,
    /// `border-destructive bg-destructive` — destructive surface.
    Destructive,
    /// `border-primary bg-primary text-primary-foreground` — primary surface.
    Primary,
}

/// Border radius preset for a [`super::Snippet`] frame.
///
/// ```rust
/// use iced_shadcn_v2::SnippetRadius;
///
/// assert!(SnippetRadius::None < SnippetRadius::Full);
/// assert_eq!(SnippetRadius::default(), SnippetRadius::Medium);
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SnippetRadius {
    /// No corner radius.
    None,
    /// Small corner radius.
    Small,
    /// Medium corner radius (`rounded-md` on the web).
    #[default]
    Medium,
    /// Large corner radius.
    Large,
    /// Fully rounded corners.
    Full,
}
