let parser = null;
let ready = false;
let bootPromise = null;

function classify(nodeType) {
  if (nodeType.includes("comment")) return "comment";
  if (nodeType.includes("string") || nodeType === "char_literal") return "string";
  if (nodeType.includes("type") || nodeType === "primitive_type") return "type";
  if (nodeType.includes("attribute")) return "attribute";
  if (nodeType.includes("float") || nodeType.includes("integer")) return "number";
  if (
    nodeType === "fn" ||
    nodeType === "let" ||
    nodeType === "pub" ||
    nodeType === "impl" ||
    nodeType === "match" ||
    nodeType === "if" ||
    nodeType === "else" ||
    nodeType === "for" ||
    nodeType === "while" ||
    nodeType === "loop" ||
    nodeType === "return" ||
    nodeType === "mod" ||
    nodeType === "use" ||
    nodeType === "struct" ||
    nodeType === "enum" ||
    nodeType === "trait" ||
    nodeType === "const" ||
    nodeType === "static"
  ) {
    return "keyword";
  }
  if (nodeType === "identifier" || nodeType === "field_identifier") return "function";
  return null;
}

async function boot() {
  if (ready) return;
  if (bootPromise) return bootPromise;
  bootPromise = (async () => {
    const ts = await import("https://cdn.jsdelivr.net/npm/web-tree-sitter@0.25.10/+esm");
    await ts.Parser.init({
      locateFile() {
        return "https://cdn.jsdelivr.net/npm/web-tree-sitter@0.25.10/tree-sitter.wasm";
      },
    });
    const lang = await ts.Language.load(
      "https://unpkg.com/tree-sitter-rust@0.24.0/tree-sitter-rust.wasm",
    );
    parser = new ts.Parser();
    parser.setLanguage(lang);
    ready = true;
  })();
  return bootPromise;
}

boot();

export function ts_highlight_rust_ranges(source) {
  if (!ready || !parser || typeof source !== "string" || source.length === 0) {
    boot();
    return "";
  }
  const tree = parser.parse(source);
  const out = [];
  const stack = [tree.rootNode];
  while (stack.length > 0) {
    const node = stack.pop();
    const kind = classify(node.type);
    if (kind) {
      out.push(`${node.startIndex}:${node.endIndex}:${kind}`);
    }
    for (let i = node.namedChildCount - 1; i >= 0; i -= 1) {
      const child = node.namedChild(i);
      if (child) stack.push(child);
    }
  }
  return out.join(";");
}
