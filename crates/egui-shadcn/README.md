# egui-shadcn

> Translations: [Русский](README.ru.md) · [Português (Brasil)](README.pt-BR.md)

## Overview
`egui-shadcn` is a set of egui form components styled after shadcn/ui. It mirrors the shadcn variants and sizes while exposing theme tokens for consistent visuals.

## Quick start
```rust
use egui_shadcn::{button, ControlSize, ControlVariant, Theme};

fn ui_example(ui: &mut egui::Ui, theme: &Theme) {
    button(ui, theme, "Primary", ControlVariant::Primary, ControlSize::Md, true);
}
```

## Components
- `button` — variants `Primary|Secondary|Ghost|Outline|Destructive|Link`; sizes `Sm|Md|Lg|IconSm|Icon|IconLg`.
- `text_input` — placeholder color, `is_invalid`, `enabled`, 3px ring, selection colors.
- `select` — options via `SelectProps`, placeholder, `is_invalid`, arrow glyph, disabled state.
- `checkbox` — sizes and variants.
- `toggle` — default/outline variants with accent colors.
- `switch` — track/thumb sizing aligned with shadcn.
- `textarea` — focus ring, invalid fill, optional counter.

## Theming
Visual states come from `Theme::control` and `Theme::input`, backed by `ColorPalette`.

## Examples
- `cargo run --example button` — all variants and sizes.
- `cargo run --example text_input` — sizes `Sm|Md|Lg`, invalid/disabled.
- `cargo run --example select` — grouped options, `SelectProps`, invalid/disabled, custom `SelectStyle`.
- `cargo run --example checkbox` — all variants/sizes, disabled.
- `cargo run --example toggle` — default/outline, icon sizes, disabled.
- `cargo run --example switch` — color variants, sizes `Sm|Md|Lg`, disabled.
- `cargo run --example textarea` — counter/limit, invalid, disabled, all sizes.
- `cargo run --example basic` — all components on one screen.

## Tests
`cargo test`

## Migration
- `select` now takes `SelectProps`: `select(ui, &theme, SelectProps { ... })`.
- `textarea` takes `TextareaProps`; pass placeholder as `WidgetText` (`"text".into()`).
- `SelectProps` includes `is_invalid`; set to `false` for legacy behavior.