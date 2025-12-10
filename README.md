# shadcn-rs workspace

> Translations: [Русский](README.ru.md) · [Português (Brasil)](README.pt-BR.md)

## Overview
Rust workspace for shadcn-style UI component crates. Currently includes the `egui-shadcn` crate with core form elements for egui.

## Install
```
cargo add egui-shadcn --path crates/egui-shadcn
```

## Components
- `button` — variants `Primary|Secondary|Ghost|Outline|Destructive|Link`; sizes `Sm|Md|Lg|IconSm|Icon|IconLg`; supports `enabled`.
- `text_input` — custom placeholder color, `is_invalid`, `enabled`, 3px ring, selection colors.
- `select` — placeholder, options list, `enabled`, `is_invalid` (via `SelectProps`), arrow in text.
– `checkbox` — variants/sizes, три-стейт через `CheckboxState`, кольцо фокуса/invalid и анимации.
- `toggle` — button-like toggle (default/outline), accent colors, sizes `Sm|Md|Lg`.
- `switch` — track toggle with shadcn-aligned track/thumb sizes (32×18.4).
- `textarea` — focus ring, `is_invalid` fill, optional counter and `max_len`.

Tri-state checkbox with invalid ring:
```rust
use egui_shadcn::{
    checkbox_state, CheckboxCycle, CheckboxOptions, CheckboxState, ControlSize, ControlVariant,
};

let mut state = CheckboxState::Indeterminate;
checkbox_state(
    ui,
    &theme,
    &mut state,
    "Notifications",
    CheckboxOptions {
        variant: ControlVariant::Secondary,
        size: ControlSize::Md,
        cycle: CheckboxCycle::TriState,
        invalid: true,
        ..CheckboxOptions::default()
    },
);
```

## Examples
- `cargo run --example button` — variants `Primary|Secondary|Ghost|Outline|Destructive|Link` and all icon sizes.
- `cargo run --example text_input` — sizes `Sm|Md|Lg`, `invalid` and `disabled` states.
- `cargo run --example select` — legacy API (`SelectPropsSimple`), grouped lists, `invalid`, `disabled`, custom `SelectStyle`, size `Sm`.
- `cargo run --example checkbox` — variants/sizes, tri-state/invalid ring, `disabled`.
- `cargo run --example toggle` — variants `Default|Outline`, icon sizes, `disabled`.
- `cargo run --example switch` — color variants, sizes `Sm|Md|Lg`, `disabled`.
- `cargo run --example textarea` — counter and limit, `invalid`, `disabled`.
- `cargo run --example basic` — combined demo of all components.

Combined screen (`basic.rs`):
```rust
let theme = Theme::default();
let mut value = String::new();
let mut selected = None;
egui::CentralPanel::default().show(&ctx, |ui| {
    button(ui, &theme, "Primary", ControlVariant::Primary, ControlSize::Md, true);
    text_input(ui, &theme, &mut value, "Enter text", ControlSize::Md, false, true);
    let mut switch_on = false;
    let mut toggle_on = false;
    toggle(ui, &theme, &mut toggle_on, "Toggle", ToggleVariant::Outline, ControlSize::Md, true);
    toggle(
        ui,
        &theme,
        &mut switch_on,
        "Switch",
        ControlVariant::Primary,
        ControlSize::Sm,
        true,
    );
    select(
        ui,
        &theme,
        SelectProps {
            id_source: "demo",
            selected: &mut selected,
            options: &["One".to_string()],
            placeholder: "Choose",
            size: ControlSize::Sm,
            enabled: true,
            is_invalid: false,
        },
    );
});
```

## Tests
`cargo test`

## Migration
- `select` now accepts parameters via `SelectProps`; update calls to `select(ui, &theme, SelectProps { ... })`.
- `textarea` takes `TextareaProps`; pass placeholder as `WidgetText` (e.g., `"text".into()`).
- `SelectProps` includes `is_invalid`; set to `false` for legacy behavior.

## Mapping to shadcn
- Variants and sizes match shadcn/ui counterparts.
- Colors come from `Theme` and `ColorPalette`.
- Visual states mirror hover/active/disabled/focus.

