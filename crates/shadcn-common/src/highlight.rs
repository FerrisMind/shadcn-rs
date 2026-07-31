//! Tree-sitter based syntax tokenizer shared by the egui and iced renderers.
//!
//! This module (feature `syntax`) turns a source string into a stream of
//! [`CodeToken`]s with semantic [`SyntaxKind`]s. Renderers map those kinds to
//! colors through [`crate::syntax::CodePalette`], which ports the GitHub
//! light/dark default themes the reference Svelte component produces with
//! `shiki`.
//!
//! The token stream is a full partition of the source: every byte (including
//! newlines) belongs to exactly one token. Diff additions/deletions carry a
//! line background through [`SyntaxKind::DiffAdd`]/[`SyntaxKind::DiffDelete`]
//! so renderers can paint the whole line.

use std::sync::OnceLock;

use tree_sitter_highlight::{Highlight, HighlightConfiguration, HighlightEvent, Highlighter};

use crate::syntax::{CodeToken, LanguageId, SyntaxKind};

/// Capture names enabled for every language. Names the grammars never emit
/// are inert; names missing from this list simply produce no highlight.
const CAPTURE_NAMES: &[&str] = &[
    "attribute",
    "boolean",
    "comment",
    "comment.documentation",
    "constant",
    "constant.builtin",
    "constructor",
    "constructor.builtin",
    "escape",
    "function",
    "function.builtin",
    "function.method",
    "function.macro",
    "keyword",
    "keyword.conditional",
    "keyword.coroutine",
    "keyword.debug",
    "keyword.exception",
    "keyword.repeat",
    "keyword.return",
    "keyword.storage",
    "label",
    "number",
    "number.float",
    "operator",
    "property",
    "property.builtin",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "self",
    "string",
    "string.escape",
    "string.regexp",
    "string.special",
    "string.special.key",
    "tag",
    "tag.delimiter",
    "type",
    "type.builtin",
    "type.qualifier",
    "variable",
    "variable.builtin",
    "variable.member",
    "variable.parameter",
    "diff.add",
    "diff.delete",
    "diff.header",
    "diff.range",
];

/// Highlight query for the diff grammar tuned to GitHub-style diff colors:
/// header lines, `---`/`+++` files, `@@` ranges and `+`/`-` changes.
const DIFF_HIGHLIGHTS_QUERY: &str = r#"
(command) @diff.header
(index) @diff.header
(similarity) @diff.header
(file_change) @diff.header
(binary_change) @diff.header
(old_file) @diff.delete
(new_file) @diff.add
(location) @diff.range
(addition) @diff.add
(deletion) @diff.delete
(comment) @comment
"#;

/// Lazily built configurations, one per supported language. Immutable once
/// built, so [`HighlightConfiguration`]s can be shared across render threads;
/// [`Highlighter`]s are cheap and created per call.
static CONFIGURATIONS: OnceLock<Vec<(&'static str, HighlightConfiguration)>> = OnceLock::new();

fn configurations() -> &'static Vec<(&'static str, HighlightConfiguration)> {
    CONFIGURATIONS.get_or_init(build_configurations)
}

fn push_config(
    out: &mut Vec<(&'static str, HighlightConfiguration)>,
    language: impl Into<tree_sitter::Language>,
    name: &'static str,
    highlights: &'static str,
    injections: &'static str,
    locals: &'static str,
) {
    // An invalid bundled query is a build-time concern; keep highlighting
    // usable for the remaining languages instead of panicking at runtime.
    if let Ok(mut config) =
        HighlightConfiguration::new(language.into(), name, highlights, injections, locals)
    {
        config.configure(CAPTURE_NAMES);
        out.push((name, config));
    }
}

fn build_configurations() -> Vec<(&'static str, HighlightConfiguration)> {
    let mut out = Vec::with_capacity(8);
    push_config(
        &mut out,
        tree_sitter_bash::LANGUAGE,
        "bash",
        tree_sitter_bash::HIGHLIGHT_QUERY,
        "",
        "",
    );
    push_config(
        &mut out,
        tree_sitter_css::LANGUAGE,
        "css",
        tree_sitter_css::HIGHLIGHTS_QUERY,
        "",
        "",
    );
    push_config(
        &mut out,
        tree_sitter_diff::LANGUAGE,
        "diff",
        DIFF_HIGHLIGHTS_QUERY,
        "",
        "",
    );
    push_config(
        &mut out,
        tree_sitter_javascript::LANGUAGE,
        "javascript",
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        tree_sitter_javascript::INJECTIONS_QUERY,
        tree_sitter_javascript::LOCALS_QUERY,
    );
    push_config(
        &mut out,
        tree_sitter_json::LANGUAGE,
        "json",
        tree_sitter_json::HIGHLIGHTS_QUERY,
        "",
        "",
    );
    push_config(
        &mut out,
        tree_sitter_rust::LANGUAGE,
        "rust",
        tree_sitter_rust::HIGHLIGHTS_QUERY,
        tree_sitter_rust::INJECTIONS_QUERY,
        "",
    );
    push_config(
        &mut out,
        tree_sitter_svelte_next::LANGUAGE,
        "svelte",
        tree_sitter_svelte_next::HIGHLIGHTS_QUERY,
        tree_sitter_svelte_next::INJECTIONS_QUERY,
        tree_sitter_svelte_next::LOCALS_QUERY,
    );
    push_config(
        &mut out,
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        "typescript",
        tree_sitter_typescript::HIGHLIGHTS_QUERY,
        "",
        tree_sitter_typescript::LOCALS_QUERY,
    );
    out
}

fn config_for(name: &str) -> Option<&'static HighlightConfiguration> {
    configurations()
        .iter()
        .find(|(candidate, _)| *candidate == name)
        .map(|(_, config)| config)
}

/// Highlights `source` for `language`, returning a full partition of the
/// source into semantic tokens (see the module docs).
#[must_use]
#[allow(clippy::redundant_closure)]
pub fn highlight_code(source: &str, language: LanguageId) -> Vec<CodeToken> {
    let Some(config) = config_for(language.as_str()) else {
        return plain_tokens(source);
    };

    let mut highlighter = Highlighter::new();
    let events =
        match highlighter.highlight(config, source.as_bytes(), None, |name| config_for(name)) {
            Ok(events) => events,
            Err(_) => return plain_tokens(source),
        };

    let mut tokens: Vec<CodeToken> = Vec::new();
    let mut stack: Vec<SyntaxKind> = Vec::new();
    for event in events {
        match event {
            Ok(HighlightEvent::Source { start, end }) => {
                if start == end {
                    continue;
                }
                let kind = stack.last().copied().unwrap_or(SyntaxKind::Plain);
                push_token(&mut tokens, start, end, kind);
            }
            Ok(HighlightEvent::HighlightStart(Highlight(index))) => {
                let name = CAPTURE_NAMES.get(index).copied().unwrap_or("");
                stack.push(kind_for_capture(name));
            }
            Ok(HighlightEvent::HighlightEnd) => {
                if stack.len() > 1 {
                    let _ = stack.pop();
                }
            }
            Err(_) => break,
        }
    }

    if tokens.is_empty() {
        return plain_tokens(source);
    }
    tokens
}

/// Maps a tree-sitter capture name to a semantic [`SyntaxKind`], mirroring the
/// TextMate scope → theme color resolution of the reference component.
#[must_use]
pub fn kind_for_capture(name: &str) -> SyntaxKind {
    match name {
        "comment" | "comment.documentation" => SyntaxKind::Comment,
        "attribute" => SyntaxKind::Attribute,
        "tag" => SyntaxKind::Tag,
        "tag.delimiter" => SyntaxKind::Punctuation,
        "keyword"
        | "keyword.conditional"
        | "keyword.coroutine"
        | "keyword.debug"
        | "keyword.exception"
        | "keyword.repeat"
        | "keyword.return"
        | "keyword.storage"
        | "type.qualifier" => SyntaxKind::Keyword,
        "string" | "string.regexp" | "string.special" => SyntaxKind::String,
        "string.escape" | "escape" => SyntaxKind::StringEscape,
        "constant" | "constant.builtin" | "boolean" | "function.builtin" | "property.builtin" => {
            SyntaxKind::Constant
        }
        "number" | "number.float" => SyntaxKind::Number,
        "self" | "variable.builtin" => SyntaxKind::VariableBuiltin,
        "variable.parameter" => SyntaxKind::VariableParameter,
        "variable.member" => SyntaxKind::Property,
        "function" | "function.method" | "function.macro" => SyntaxKind::Function,
        "property" => SyntaxKind::Property,
        "operator" => SyntaxKind::Operator,
        "punctuation" | "punctuation.bracket" | "punctuation.delimiter" | "punctuation.special" => {
            SyntaxKind::Punctuation
        }
        "type" => SyntaxKind::Type,
        "type.builtin" | "constructor.builtin" => SyntaxKind::TypeBuiltin,
        "constructor" => SyntaxKind::Constructor,
        "label" => SyntaxKind::Label,
        "variable" => SyntaxKind::Variable,
        "string.special.key" => SyntaxKind::Tag,
        "diff.add" => SyntaxKind::DiffAdd,
        "diff.delete" => SyntaxKind::DiffDelete,
        "diff.header" => SyntaxKind::DiffHeader,
        "diff.range" => SyntaxKind::DiffRange,
        _ => SyntaxKind::Plain,
    }
}

fn plain_tokens(source: &str) -> Vec<CodeToken> {
    if source.is_empty() {
        return Vec::new();
    }
    vec![CodeToken {
        start: 0,
        end: source.len(),
        kind: SyntaxKind::Plain,
    }]
}

fn push_token(tokens: &mut Vec<CodeToken>, start: usize, end: usize, kind: SyntaxKind) {
    let merged = tokens
        .last_mut()
        .is_some_and(|last| last.kind == kind && last.end == start);
    if merged {
        if let Some(last) = tokens.last_mut() {
            last.end = end;
        }
        return;
    }
    tokens.push(CodeToken { start, end, kind });
}
