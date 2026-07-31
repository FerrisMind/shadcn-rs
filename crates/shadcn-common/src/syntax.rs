//! Syntax-highlighting vocabulary shared by the egui and iced renderers.
//!
//! This module is backend-agnostic: it defines the set of supported
//! languages, the semantic token kinds, the per-token color palette (a port
//! of the GitHub light/dark default themes used by the reference Svelte
//! component through `shiki`), and the line-highlight helpers. The actual
//! tokenizer lives in [`crate::highlight`] (feature `syntax`); renderers
//! turn the produced [`CodeToken`] stream into their native rich text.

use crate::color::ThemeMode;
use crate::color_space::Rgba;

/// Languages the shared highlighter can tokenize.
///
/// The set mirrors the bundled languages of the reference Svelte component
/// (`bash`, `diff`, `javascript`, `json`, `svelte`, `typescript`, `text`)
/// plus `rust`, whose grammar the workspace already uses in egui. `text` is
/// special-cased by the tokenizer: it produces a single plain token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LanguageId {
    /// POSIX shell.
    Bash,
    /// Unified diff files.
    Diff,
    /// JavaScript.
    JavaScript,
    /// JSON.
    Json,
    /// Rust.
    Rust,
    /// Svelte components.
    Svelte,
    /// Plain text, no highlighting.
    Text,
    /// TypeScript (and TSX).
    TypeScript,
}

impl LanguageId {
    /// All tokenizable languages, in a stable order.
    pub const ALL: [LanguageId; 8] = [
        LanguageId::Bash,
        LanguageId::Diff,
        LanguageId::JavaScript,
        LanguageId::Json,
        LanguageId::Rust,
        LanguageId::Svelte,
        LanguageId::Text,
        LanguageId::TypeScript,
    ];

    /// Looks up a language by its `shiki`/common name (`"typescript"`,
    /// `"json"`, …). Accepts the usual short aliases (`"ts"`, `"js"`,
    /// `"rs"`, `"txt"`).
    #[must_use]
    pub fn parse_name(value: &str) -> Option<LanguageId> {
        match value {
            "bash" | "sh" | "shell" => Some(LanguageId::Bash),
            "diff" => Some(LanguageId::Diff),
            "javascript" | "js" | "jsx" => Some(LanguageId::JavaScript),
            "json" => Some(LanguageId::Json),
            "rust" | "rs" => Some(LanguageId::Rust),
            "svelte" => Some(LanguageId::Svelte),
            "text" | "plaintext" | "plain" | "txt" => Some(LanguageId::Text),
            "typescript" | "ts" | "tsx" => Some(LanguageId::TypeScript),
            _ => None,
        }
    }

    /// Canonical name for the language, usable as the value of the Svelte
    /// component's `lang` prop.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            LanguageId::Bash => "bash",
            LanguageId::Diff => "diff",
            LanguageId::JavaScript => "javascript",
            LanguageId::Json => "json",
            LanguageId::Rust => "rust",
            LanguageId::Svelte => "svelte",
            LanguageId::Text => "text",
            LanguageId::TypeScript => "typescript",
        }
    }
}

impl From<&str> for LanguageId {
    /// Converts a language name (see [`parse_name`](Self::parse_name)) into a
    /// [`LanguageId`], falling back to plain text for unknown names.
    ///
    /// This mirrors the reference Shiki behavior of rendering unknown
    /// languages as plain text, and makes builder APIs like
    /// `Code::new(source, "rust", theme)` ergonomic.
    fn from(value: &str) -> Self {
        LanguageId::parse_name(value).unwrap_or(LanguageId::Text)
    }
}

/// Semantic token kinds produced by the shared highlighter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SyntaxKind {
    /// No highlight; renderers fall back to the palette foreground.
    Plain,
    /// Comments.
    Comment,
    /// Constants, enum members, `this`/`undefined`-like builtin variables.
    Constant,
    /// Numeric literals.
    Number,
    /// Class/interface constructor names.
    Constructor,
    /// Function and method names.
    Function,
    /// Keywords (`if`, `const`, `import`, storage keywords).
    Keyword,
    /// Labels (e.g. Rust loop labels).
    Label,
    /// Operators.
    Operator,
    /// Object property names.
    Property,
    /// Punctuation (brackets, delimiters, tags delimiters).
    Punctuation,
    /// String literals.
    String,
    /// Escape sequences inside string literals.
    StringEscape,
    /// HTML/Svelte tag names.
    Tag,
    /// Type names (classes, interfaces, type aliases).
    Type,
    /// Builtin primitive types (`string`, `number`, …).
    TypeBuiltin,
    /// Local variable names.
    Variable,
    /// Builtin variables (`this`, `super`, …).
    VariableBuiltin,
    /// Function parameters.
    VariableParameter,
    /// HTML/Svelte attribute names.
    Attribute,
    /// Diff insertions (also carries a line background).
    DiffAdd,
    /// Diff deletions (also carries a line background).
    DiffDelete,
    /// Diff modifications (also carries a line background).
    DiffChanged,
    /// Diff header lines (`diff --git`, `index`, `---`, `+++`).
    DiffHeader,
    /// Diff hunk ranges (`@@ … @@`).
    DiffRange,
}

/// Per-token text colors for syntax-highlighted code.
///
/// Values mirror the `github-light-default` / `github-dark-default` themes
/// used by the reference Svelte component. The palette is opaque per
/// [`ThemeMode`]; use [`code_palette`] to build it.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct CodePalette {
    /// Plain foreground used for unhighlighted code and parameters.
    pub foreground: Rgba,
    /// Comments.
    pub comment: Rgba,
    /// Constants and numeric literals.
    pub constant: Rgba,
    /// Constructor names.
    pub constructor: Rgba,
    /// Function and method names.
    pub function: Rgba,
    /// Keywords and operators.
    pub keyword: Rgba,
    /// Object property names.
    pub property: Rgba,
    /// String literals.
    pub string: Rgba,
    /// String escape sequences.
    pub string_escape: Rgba,
    /// Tag names.
    pub tag: Rgba,
    /// Type names.
    pub ty: Rgba,
    /// Builtin primitive types.
    pub type_builtin: Rgba,
    /// Local variables.
    pub variable: Rgba,
    /// Builtin variables.
    pub variable_builtin: Rgba,
    /// Diff insertion foreground and line background.
    pub diff_add: Rgba,
    /// Diff insertion line background.
    pub diff_add_background: Rgba,
    /// Diff deletion foreground.
    pub diff_delete: Rgba,
    /// Diff deletion line background.
    pub diff_delete_background: Rgba,
    /// Diff modification foreground.
    pub diff_changed: Rgba,
    /// Diff modification line background.
    pub diff_changed_background: Rgba,
    /// Diff header foreground.
    pub diff_header: Rgba,
    /// Diff range (`@@ … @@`) foreground.
    pub diff_range: Rgba,
}

impl CodePalette {
    /// Text color and optional line background for a token [`SyntaxKind`].
    ///
    /// The background is only ever `Some` for the diff marker kinds
    /// (`DiffAdd`, `DiffDelete`, `DiffChanged`) and applies to the whole
    /// line the token lives on.
    #[must_use]
    pub const fn token_color(&self, kind: SyntaxKind) -> (Rgba, Option<Rgba>) {
        match kind {
            SyntaxKind::Plain => (self.foreground, None),
            SyntaxKind::Comment => (self.comment, None),
            SyntaxKind::Constant | SyntaxKind::Number | SyntaxKind::VariableBuiltin => {
                (self.constant, None)
            }
            SyntaxKind::Constructor | SyntaxKind::Variable => (self.constructor, None),
            SyntaxKind::Function => (self.function, None),
            SyntaxKind::Keyword | SyntaxKind::Operator | SyntaxKind::StringEscape => {
                (self.keyword, None)
            }
            SyntaxKind::Label => (self.foreground, None),
            SyntaxKind::Property => (self.property, None),
            SyntaxKind::Punctuation => (self.foreground, None),
            SyntaxKind::String => (self.string, None),
            SyntaxKind::Tag => (self.tag, None),
            SyntaxKind::Type => (self.ty, None),
            SyntaxKind::TypeBuiltin => (self.type_builtin, None),
            SyntaxKind::VariableParameter => (self.foreground, None),
            SyntaxKind::Attribute => (self.foreground, None),
            SyntaxKind::DiffAdd => (self.diff_add, Some(self.diff_add_background)),
            SyntaxKind::DiffDelete => (self.diff_delete, Some(self.diff_delete_background)),
            SyntaxKind::DiffChanged => (self.diff_changed, Some(self.diff_changed_background)),
            SyntaxKind::DiffHeader => (self.diff_header, None),
            SyntaxKind::DiffRange => (self.diff_range, None),
        }
    }
}

/// Builds the code palette for a light or dark theme.
#[must_use]
pub fn code_palette(mode: ThemeMode) -> CodePalette {
    match mode {
        ThemeMode::Light => CodePalette {
            foreground: rgba(0x1f, 0x23, 0x28),
            comment: rgba(0x6e, 0x77, 0x81),
            constant: rgba(0x05, 0x50, 0xae),
            constructor: rgba(0x95, 0x38, 0x00),
            function: rgba(0x82, 0x50, 0xdf),
            keyword: rgba(0xcf, 0x22, 0x2e),
            property: rgba(0x05, 0x50, 0xae),
            string: rgba(0x0a, 0x30, 0x69),
            string_escape: rgba(0xcf, 0x22, 0x2e),
            tag: rgba(0x11, 0x63, 0x29),
            ty: rgba(0x95, 0x38, 0x00),
            type_builtin: rgba(0x05, 0x50, 0xae),
            variable: rgba(0x95, 0x38, 0x00),
            variable_builtin: rgba(0x05, 0x50, 0xae),
            diff_add: rgba(0x11, 0x63, 0x29),
            diff_add_background: rgba(0xda, 0xfb, 0xe1),
            diff_delete: rgba(0x82, 0x07, 0x1e),
            diff_delete_background: rgba(0xff, 0xeb, 0xe9),
            diff_changed: rgba(0x95, 0x38, 0x00),
            diff_changed_background: rgba(0xff, 0xd8, 0xb5),
            diff_header: rgba(0x05, 0x50, 0xae),
            diff_range: rgba(0x82, 0x50, 0xdf),
        },
        ThemeMode::Dark => CodePalette {
            foreground: rgba(0xe6, 0xed, 0xf3),
            comment: rgba(0x91, 0x98, 0xa1),
            constant: rgba(0x79, 0xc0, 0xff),
            constructor: rgba(0xff, 0xa6, 0x57),
            function: rgba(0xd2, 0xa8, 0xff),
            keyword: rgba(0xff, 0x7b, 0x72),
            property: rgba(0x79, 0xc0, 0xff),
            string: rgba(0xa5, 0xd6, 0xff),
            string_escape: rgba(0xff, 0x7b, 0x72),
            tag: rgba(0x7e, 0xe7, 0x87),
            ty: rgba(0xff, 0xa6, 0x57),
            type_builtin: rgba(0x79, 0xc0, 0xff),
            variable: rgba(0xff, 0xa6, 0x57),
            variable_builtin: rgba(0x79, 0xc0, 0xff),
            diff_add: rgba(0x7e, 0xe7, 0x87),
            diff_add_background: rgba(0x04, 0x26, 0x0f),
            diff_delete: rgba(0xff, 0xa1, 0x98),
            diff_delete_background: rgba(0x49, 0x02, 0x02),
            diff_changed: rgba(0xff, 0xa6, 0x57),
            diff_changed_background: rgba(0x5a, 0x1e, 0x02),
            diff_header: rgba(0x79, 0xc0, 0xff),
            diff_range: rgba(0xd2, 0xa8, 0xff),
        },
    }
}

/// One line of the reference component's `highlight` prop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CodeLineHighlight {
    /// A single 1-based line number.
    Single(u32),
    /// An inclusive 1-based line range.
    Range(u32, u32),
}

impl From<u32> for CodeLineHighlight {
    fn from(value: u32) -> Self {
        Self::Single(value)
    }
}

impl From<(u32, u32)> for CodeLineHighlight {
    fn from((start, end): (u32, u32)) -> Self {
        Self::Range(start, end)
    }
}

impl From<std::ops::RangeInclusive<u32>> for CodeLineHighlight {
    fn from(range: std::ops::RangeInclusive<u32>) -> Self {
        Self::Range(*range.start(), *range.end())
    }
}

/// Returns whether `line` (1-based) matches any of the `highlights`.
///
/// Mirrors the `within` helper of the reference `code.svelte.ts`.
#[must_use]
pub fn line_is_highlighted(highlights: &[CodeLineHighlight], line: u32) -> bool {
    highlights.iter().any(|highlight| match *highlight {
        CodeLineHighlight::Single(number) => line == number,
        CodeLineHighlight::Range(start, end) => start <= line && line <= end,
    })
}

/// A run of source text with a semantic [`SyntaxKind`].
///
/// Offsets are byte offsets into the original source, and the token stream
/// produced by [`crate::highlight::highlight_code`] is a full partition of
/// the source (every byte belongs to exactly one token, including newlines).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeToken {
    /// Start byte offset into the source (inclusive).
    pub start: usize,
    /// End byte offset into the source (exclusive).
    pub end: usize,
    /// Semantic kind of this run.
    pub kind: SyntaxKind,
}

/// Opaque sRGB color.
fn rgba(red: u8, green: u8, blue: u8) -> Rgba {
    Rgba::new(f32::from(red), f32::from(green), f32::from(blue), 1.0)
}
