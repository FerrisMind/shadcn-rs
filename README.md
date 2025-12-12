# shadcn-rs

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset=".github/assets/icon-white.svg" />
    <source media="(prefers-color-scheme: light)" srcset=".github/assets/icon-black.svg" />
    <img alt="shadcn-rs logo" src=".github/assets/icon-black.svg" width="180" />
  </picture>
</p>

> Translations: [Русский](README.ru.md) · [Português (Brasil)](README.pt-BR.md)

## Overview
Rust workspace for shadcn-style UI component crates. Currently includes the `egui-shadcn` crate with core form elements for egui.

## Install
```
cargo add egui-shadcn --path crates/egui-shadcn
```

## Components
- `button` — variants `Primary|Secondary|Ghost|Outline|Destructive|Link`; sizes `Sm|Md|Lg|IconSm|Icon|IconLg`; supports `enabled`.
- `label` — `for_id` to focus inputs, variants `Default|Secondary|Muted|Destructive`, optional description, required marker.
- `text_input` — `text_input` wrapper + `text_input_with_config`/`InputConfig`: variants `Surface|Classic|Soft`, leading/trailing slots, password/read-only, fill width, invalid/disabled, 3px ring, selection colors, accent override, radius/padding overrides, slot gap/padding, `resolve_input_style` for parity checks.
- `select` — placeholder, options list, `enabled`, `is_invalid` (via `SelectProps`), arrow in text, color/accent override for trigger+content, ghost/classic/soft variants, high-contrast.
– `checkbox` — variants/sizes, три-стейт через `CheckboxState`, кольцо фокуса/invalid и анимации.
- `radio_group` — vertical/horizontal layouts, option descriptions, accent override, disabled options, high-contrast mode.
- `toggle` — button-like toggle (default/outline) с hover/bg/fg как в shadcn, анимация on/off через `animate_bool`, размеры `Sm|Md|Lg|IconSm|Icon|IconLg`.
- `switch` — варианты `surface|classic|soft`, размеры `1|2|3` (map с `ControlSize`), `high_contrast`, кастом accent/thumb через `SwitchOptions`.
- `textarea` — focus ring, `is_invalid` fill, optional counter and `max_len`.
- `tooltip` — delayed hover/focus helper with positions and high-contrast styling.
- `card` — контейнер с вариантами `Surface|Classic|Ghost` (алиасы Outline/Subtle), заголовком/описанием, настраиваемыми паддингами/радиусом/тенью.
- `separator` — горизонтальный/вертикальный разделитель с толщиной/отступами, явной длиной и цветом.
- `tabs` — underline/soft/outline варианты, горизонтальная/вертикальная ориентация, компактный режим.
- `scroll_area` — обёртка над `egui::ScrollArea` с контролем направления, размеров, видимости скролл-баров и `auto_shrink`.
- `popover` — align `Start|Center|End`, side/align offset, опционально ширина триггера, кламп к экрану, анимация, закрытие по Esc/клику вне.
- `dialog` — модальный оверлей с тайтлом/описанием, scrim opacity, центр/offset позиционирование, анимация, флаги закрытия по Esc/бэкдропу.

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
- `cargo run --example label` — linking labels to inputs with required markers and variants.
- `cargo run --example text_input` — sizes `Sm|Md|Lg`, `invalid`/`disabled`, slots, variants, password/read-only, accent/radius/padding overrides.
- `cargo run --example select` — legacy API (`SelectPropsSimple`), grouped lists, `invalid`, `disabled`, custom `SelectStyle`, size `Sm`.
- `cargo run --example checkbox` — variants/sizes, tri-state/invalid ring, `disabled`.
- `cargo run --example radio` — vertical/horizontal radio groups with descriptions and disabled options.
- `cargo run --example toggle` — variants `Default|Outline`, icon sizes, `disabled`.
- `cargo run --example switch` — color variants, sizes `Sm|Md|Lg` → `SwitchSize 1|2|3`, variants `surface|classic|soft`, `high_contrast`, `disabled`.
- `cargo run --example textarea` — counter and limit, `invalid`, `disabled`.
- `cargo run --example tooltip` — delayed hover/focus tooltip with positioning and high-contrast.
- `cargo run --example basic` — combined demo of all components.
- `cargo run --example card` — Surface/Classic/Ghost карточки, паддинги/радиус/тень.
- `cargo run --example separator` — гор./верт. разделители, толщина/длина/цвет.
- `cargo run --example tabs` — underline/soft/outline, активный таб в состоянии.
- `cargo run --example scroll_area` — вертикальный/двухосевой скролл с max_size и auto_shrink.
- `cargo run --example popover` — align, offsets, match ширины триггера, кламп к экрану.
- `cargo run --example dialog` — модальный оверлей со scrim, выравниванием и анимацией.
- `cargo run --example structural` — полный набор приоритет-2 компонентов на одном экране.

Combined screen (`basic.rs`, требуется импорт `PopoverAlign`):
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
        ToggleVariant::Default,
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
    let mut dialog_open = true;
    let mut popover_open = true;
    let mut active_tab = "tab-1".to_string();
    let tab_items = [
        TabItem::new("tab-1", "General"),
        TabItem::new("tab-2", "Advanced"),
    ];
    tabs(
        ui,
        &theme,
        TabsProps::new(ui.make_persistent_id("tabs"), &tab_items, &mut active_tab),
        |tab_ui, active| {
            tab_ui.label(format!("Active tab = {}", active.id));
        },
    );
    let (trigger, _popover_content) = popover(
        ui,
        &theme,
        PopoverProps::new(ui.make_persistent_id("popover"), &mut popover_open)
            .with_align(PopoverAlign::End)
            .with_side_offset(8.0)
            .match_trigger_width(true),
        |t_ui| t_ui.button("Open popover"),
        |body_ui| {
            body_ui.label("Popover body");
        },
    );
    if trigger.clicked() {
        popover_open = true;
    }
    let _dialog_content = dialog(
        ui,
        &theme,
        DialogProps::new(ui.make_persistent_id("dialog"), &mut dialog_open)
            .with_title("Dialog")
            .with_description("Parity with shadcn/radix")
            .with_scrim_opacity(160)
            .with_animation(true),
        |body_ui| {
            body_ui.label("Modal body");
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

