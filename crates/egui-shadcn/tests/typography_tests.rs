use egui_shadcn::{ShadcnTypographyVariant, TypographyColor, resolve_shadcn_style};

#[test]
fn shadcn_typography_h1_matches_reference_scale() {
    let style = resolve_shadcn_style(ShadcnTypographyVariant::H1);
    assert_eq!(style.size, 36.0);
    assert!(style.strong);
    assert_eq!(style.color, TypographyColor::Default);
}

#[test]
fn shadcn_typography_h2_matches_reference_scale() {
    let style = resolve_shadcn_style(ShadcnTypographyVariant::H2);
    assert_eq!(style.size, 30.0);
    assert!(style.strong);
}

#[test]
fn shadcn_typography_lead_is_muted_and_large() {
    let style = resolve_shadcn_style(ShadcnTypographyVariant::Lead);
    assert_eq!(style.size, 20.0);
    assert_eq!(style.color, TypographyColor::Muted);
}

#[test]
fn shadcn_typography_inline_code_is_monospace_and_emphasized() {
    let style = resolve_shadcn_style(ShadcnTypographyVariant::InlineCode);
    assert!(style.monospace);
    assert!(style.strong);
    assert_eq!(style.size, 14.0);
}

#[test]
fn shadcn_typography_blockquote_is_italic() {
    let style = resolve_shadcn_style(ShadcnTypographyVariant::Blockquote);
    assert!(style.italic);
}
