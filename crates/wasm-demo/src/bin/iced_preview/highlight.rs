#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Comment,
    Keyword,
    String,
    Type,
    Function,
    Number,
    Attribute,
}

#[derive(Debug, Clone, Copy)]
pub struct TokenRange {
    pub start: usize,
    pub end: usize,
    pub kind: TokenKind,
}

#[cfg(not(target_arch = "wasm32"))]
const RUST_HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "function",
    "function.macro",
    "keyword",
    "label",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "string",
    "string.escape",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
];

#[cfg(not(target_arch = "wasm32"))]
pub fn rust_highlight_ranges(source: &str) -> Vec<TokenRange> {
    use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

    let mut config = match HighlightConfiguration::new(
        tree_sitter_rust::LANGUAGE.into(),
        "rust",
        tree_sitter_rust::HIGHLIGHTS_QUERY,
        tree_sitter_rust::INJECTIONS_QUERY,
        "",
    ) {
        Ok(config) => config,
        Err(_) => return Vec::new(),
    };
    config.configure(RUST_HIGHLIGHT_NAMES);

    let mut highlighter = Highlighter::new();
    let events = match highlighter.highlight(&config, source.as_bytes(), None, |_| None) {
        Ok(events) => events,
        Err(_) => return Vec::new(),
    };

    let mut stack: Vec<Option<TokenKind>> = Vec::new();
    let mut out = Vec::new();
    for event in events {
        let Ok(event) = event else {
            continue;
        };
        match event {
            HighlightEvent::Source { start, end } => {
                if let Some(Some(kind)) = stack.last()
                    && start < end
                    && end <= source.len()
                {
                    out.push(TokenRange {
                        start,
                        end,
                        kind: *kind,
                    });
                }
            }
            HighlightEvent::HighlightStart(index) => {
                let name = RUST_HIGHLIGHT_NAMES
                    .get(index.0)
                    .copied()
                    .unwrap_or_default();
                stack.push(map_native_highlight_name(name));
            }
            HighlightEvent::HighlightEnd => {
                let _ = stack.pop();
            }
        }
    }

    out
}

#[cfg(not(target_arch = "wasm32"))]
fn map_native_highlight_name(name: &str) -> Option<TokenKind> {
    match name {
        "comment" => Some(TokenKind::Comment),
        "keyword" | "operator" => Some(TokenKind::Keyword),
        "string" | "string.escape" => Some(TokenKind::String),
        "type" | "type.builtin" | "constructor" => Some(TokenKind::Type),
        "function" | "function.macro" => Some(TokenKind::Function),
        "constant" | "constant.builtin" | "variable.builtin" => Some(TokenKind::Number),
        "attribute" | "label" => Some(TokenKind::Attribute),
        _ => None,
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(inline_js = r#"
let parser = null;
let ready = false;
let bootPromise = null;

function classify(nodeType) {
  if (nodeType.includes('comment')) return 'comment';
  if (nodeType.includes('string') || nodeType === 'char_literal') return 'string';
  if (nodeType.includes('type') || nodeType === 'primitive_type') return 'type';
  if (nodeType.includes('attribute')) return 'attribute';
  if (nodeType.includes('float') || nodeType.includes('integer')) return 'number';
  if (
    nodeType === 'fn' || nodeType === 'let' || nodeType === 'pub' || nodeType === 'impl' ||
    nodeType === 'match' || nodeType === 'if' || nodeType === 'else' || nodeType === 'for' ||
    nodeType === 'while' || nodeType === 'loop' || nodeType === 'return' || nodeType === 'mod' ||
    nodeType === 'use' || nodeType === 'struct' || nodeType === 'enum' || nodeType === 'trait' ||
    nodeType === 'const' || nodeType === 'static'
  ) return 'keyword';
  if (nodeType === 'identifier' || nodeType === 'field_identifier') return 'function';
  return null;
}

async function boot() {
  if (ready) return;
  if (bootPromise) return bootPromise;
  bootPromise = (async () => {
    const ts = await import('https://cdn.jsdelivr.net/npm/web-tree-sitter@0.25.10/+esm');
    await ts.Parser.init({
      locateFile() {
        return 'https://cdn.jsdelivr.net/npm/web-tree-sitter@0.25.10/tree-sitter.wasm';
      },
    });
    const lang = await ts.Language.load('https://unpkg.com/tree-sitter-rust@0.24.0/tree-sitter-rust.wasm');
    parser = new ts.Parser();
    parser.setLanguage(lang);
    ready = true;
  })();
  return bootPromise;
}

export function ts_highlight_rust_ranges(source) {
  if (!ready || !parser || typeof source !== 'string' || source.length === 0) {
    boot();
    return '';
  }
  const tree = parser.parse(source);
  const out = [];
  const stack = [tree.rootNode];
  while (stack.length > 0) {
    const node = stack.pop();
    const kind = classify(node.type);
    if (kind) out.push(`${node.startIndex}:${node.endIndex}:${kind}`);
    for (let i = node.namedChildCount - 1; i >= 0; i -= 1) {
      const child = node.namedChild(i);
      if (child) stack.push(child);
    }
  }
  return out.join(';');
}
"#)]
extern "C" {
    fn ts_highlight_rust_ranges(source: &str) -> String;
}

#[cfg(target_arch = "wasm32")]
pub fn rust_highlight_ranges(source: &str) -> Vec<TokenRange> {
    let encoded = ts_highlight_rust_ranges(source);
    if encoded.is_empty() || source.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    for token in encoded.split(';') {
        let mut parts = token.split(':');
        let Some(start) = parts.next().and_then(|s| s.parse::<usize>().ok()) else {
            continue;
        };
        let Some(end) = parts.next().and_then(|s| s.parse::<usize>().ok()) else {
            continue;
        };
        let Some(kind) = parts.next().and_then(map_wasm_kind) else {
            continue;
        };
        if start < end && end <= source.len() {
            out.push(TokenRange { start, end, kind });
        }
    }
    out
}

#[cfg(target_arch = "wasm32")]
fn map_wasm_kind(kind: &str) -> Option<TokenKind> {
    match kind {
        "comment" => Some(TokenKind::Comment),
        "keyword" => Some(TokenKind::Keyword),
        "string" => Some(TokenKind::String),
        "type" => Some(TokenKind::Type),
        "function" => Some(TokenKind::Function),
        "number" => Some(TokenKind::Number),
        "attribute" => Some(TokenKind::Attribute),
        _ => None,
    }
}
