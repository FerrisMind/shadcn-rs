# egui-shadcn

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="../../.github/assets/icon-white.svg" />
    <source media="(prefers-color-scheme: light)" srcset="../../.github/assets/icon-black.svg" />
    <img alt="shadcn-rs logo" src="../../.github/assets/icon-black.svg" width="180" />
  </picture>
</p>

> Translations: [Русский](README.ru.md) · [Português (Brasil)](README.pt-BR.md)

## Overview
`egui-shadcn` is a set of egui form components styled after shadcn/ui. It mirrors the shadcn variants and sizes while exposing theme tokens for consistent visuals and per-control overrides (accent, radius, padding, slot spacing).

## Quick start
```rust
use egui_shadcn::{button, ControlSize, ControlVariant, Theme};

fn ui_example(ui: &mut egui::Ui, theme: &Theme) {
    button(ui, theme, "Primary", ControlVariant::Primary, ControlSize::Md, true);
}
```

## Components
- `button` — variants `Primary|Secondary|Ghost|Outline|Destructive|Link`; sizes `Sm|Md|Lg|IconSm|Icon|IconLg`.
- `label` — `for_id` to focus linked inputs, variants `Default|Secondary|Muted|Destructive`, descriptions, required marker.
- `text_input` — `text_input` wrapper + `text_input_with_config`/`InputConfig` (variants `Surface|Classic|Soft`, leading/trailing slots, password/read-only, fill width, invalid/disabled, selection colors) + accent override, radius/padding tweaks, slot gap/padding, `resolve_input_style` helper.
- `select` — options via `SelectProps`, placeholder, `is_invalid`, arrow glyph, disabled state, accent color override for trigger/content, ghost/classic/soft variants, high-contrast, клавиатурный typeahead (по префиксу).
- `checkbox` — sizes/variants, tri-state (`CheckboxState`), focus/invalid ring, high-contrast для токенов/кольца.
- `radio_group` — descriptions, vertical/horizontal layout, accent override, disabled options, high-contrast mode.
- `toggle` — default/outline variants с hover/bg/fg как в shadcn, on-state accent, размеры `Sm|Md|Lg|IconSm|Icon|IconLg`.
- `switch` — варианты `surface|classic|soft`, размеры `1|2|3` (map с `ControlSize`), `high_contrast`, кастом accent/thumb через `SwitchOptions`.
- `textarea` — focus ring, invalid fill, optional counter.
- `tooltip` — delayed hover/focus helper with positions and high-contrast styling.

Input config with slots and variant:
```rust
use egui_shadcn::{ControlSize, InputConfig, InputVariant, text_input_with_config};

let mut query = String::new();
let _response = text_input_with_config(
    ui,
    &theme,
    &mut query,
    InputConfig::new("Search docs", ControlSize::Md)
        .with_variant(InputVariant::Surface)
        .with_leading_slot(|slot_ui| slot_ui.label("[search]"))
        .with_trailing_slot(|slot_ui| slot_ui.label("Cmd+K")),
);
```

If you reuse the same value across multiple inputs, pass an explicit id to avoid `egui` ID clashes and keep focus stable:
```rust
let stable_id = ui.make_persistent_id("search_input");
let _response = text_input_with_config(
    ui,
    &theme,
    &mut query,
    InputConfig::new("Search docs", ControlSize::Md)
        .with_id(stable_id)
        .with_variant(InputVariant::Surface),
);
```

Custom accent/radius with tuned padding and slot spacing:
```rust
use egui::{Color32, CornerRadius, Vec2};
use egui_shadcn::{ControlSize, InputConfig, InputVariant, Theme, resolve_input_style};

let mut url = String::from("https://shadcn");
let config = InputConfig::new("Custom accent", ControlSize::Md)
    .with_variant(InputVariant::Soft)
    .with_accent(Color32::from_rgb(94, 234, 212))
    .with_radius(CornerRadius::same(12.0))
    .with_padding(Vec2::new(18.0, 10.0))
    .with_slot_gap(6.0)
    .with_trailing_slot(|slot_ui| slot_ui.label(".dev"));

let _resolved = resolve_input_style(&theme, &config); // snapshot of tokens/visuals for parity checks
let _response = text_input_with_config(ui, &theme, &mut url, config);
```

## Theming
Visual states come from `Theme::control` and `Theme::input`, backed by `ColorPalette`.

Toggle example:
```rust
use egui_shadcn::{toggle, ControlSize, Theme, ToggleVariant};

fn toggle_example(ui: &mut egui::Ui, theme: &Theme, pressed: &mut bool) {
    toggle(
        ui,
        theme,
        pressed,
        "Bold",
        ToggleVariant::Outline,
        ControlSize::Sm,
        true,
    );
}
```

Switch с кастомизацией:
```rust
use egui_shadcn::{
    switch_with_options, ControlVariant, SwitchOptions, SwitchSize, SwitchVariant, Theme,
};

fn switch_example(ui: &mut egui::Ui, theme: &Theme, on: &mut bool) {
    switch_with_options(
        ui,
        theme,
        on,
        "Surface",
        SwitchOptions {
            size: SwitchSize::Three,
            style: SwitchVariant::Classic,
            high_contrast: true,
            accent: None,
            ..SwitchOptions::default()
        },
    );
}
```

Tri-state checkbox:
```rust
use egui_shadcn::{
    checkbox_state, CheckboxCycle, CheckboxOptions, CheckboxState, ControlSize, ControlVariant,
};

let mut state = CheckboxState::Indeterminate;
checkbox_state(
    ui,
    &theme,
    &mut state,
    "Tri-state",
    CheckboxOptions {
        variant: ControlVariant::Secondary,
        size: ControlSize::Md,
        cycle: CheckboxCycle::TriState,
        invalid: false,
        ..CheckboxOptions::default()
    },
);
```

## Examples
- `cargo run --example button` — all variants and sizes.
- `cargo run --example label` — `Label` with descriptions, required marks, variants.
- `cargo run --example text_input` — sizes `Sm|Md|Lg`, invalid/disabled, slots, variants, password/read-only.
- `cargo run --example select` — grouped options, `SelectProps`, invalid/disabled, custom `SelectStyle`.
- `cargo run --example checkbox` — variants/sizes, tri-state + invalid ring, disabled.
- `cargo run --example radio` — vertical/horizontal groups, descriptions, disabled options.
- `cargo run --example toggle` — default/outline, icon sizes, disabled.
- `cargo run --example switch` — color variants, sizes `Sm|Md|Lg` → `SwitchSize 1|2|3`, variants `surface|classic|soft`, high-contrast/disabled.
- `cargo run --example textarea` — counter/limit, invalid, disabled, all sizes.
- `cargo run --example tooltip` — positioning, delay, high-contrast tooltip helper.
- `cargo run --example basic` — all components on one screen.

## Tests
`cargo test`

## Migration
- `select` now takes `SelectProps`: `select(ui, &theme, SelectProps { ... })`.
- `textarea` takes `TextareaProps`; pass placeholder as `WidgetText` (`"text".into()`).
- `SelectProps` includes `is_invalid`; set to `false` for legacy behavior.
- `CheckboxOptions` получил поле `high_contrast`; при ручной инициализации через литерал добавьте `high_contrast: false` (или `true` для повышенного контраста). Добавлен helper `checkbox_tokens_with_high_contrast`.
- `select` добавил helper `find_typeahead_match` и клавиатурный поиск по префиксу; при необходимости можете использовать его для пользовательских UX проверок.