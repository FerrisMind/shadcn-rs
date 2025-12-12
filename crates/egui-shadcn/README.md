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
- `card` — контейнер с вариантами `Surface|Classic|Ghost` (алиасы Outline/Subtle), заголовком/описанием, настраиваемыми паддингами/радиусом/тенью.
- `separator` - разделитель как в shadcn/Radix Themes: orientation `Horizontal|Vertical`, size `Size1..Size4` (1-3 фикс., 4 = fill), optional length override, кастом thickness/gap, color/accent override, high-contrast.
- `tabs` - underline (Radix)/soft (shadcn)/outline, horizontal/vertical, sizes `Size1|Size2`, wrap/justify, scrollable list (bars hidden), full-width triggers, per-tab disabled, accent override, high-contrast, arrow-key navigation.
- `scroll_area` - ScrollArea как в shadcn/Radix Themes: кастомные плавающие скроллбары с fade-анимацией, `size Size1..Size3`, `radius None|Small|Medium|Large|Full`, `type Hover|Scroll|Always|Auto`, optional hide delay, high-contrast/accent и override цветов.
- `popover` - align `Start|Center|End`, side offset/align offset, match ширина триггера, кламп к экрану, опциональная анимация, закрытие по Esc/клику вне.
- `dialog` - модальный оверлей как в Radix Themes/shadcn: size `DialogSize::Size1..Size4` (padding/radius), width/height + min/max limits, scrollable body, встроенная кнопка закрытия (опционально), scrim opacity/color, align `Start|Center`, offset, show/hide анимация, high-contrast и override токенов через `DialogTokens`.

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

Structural components example:
```rust
use egui_shadcn::{
    card, dialog, popover, separator, tabs, CardProps, CardSize, CardVariant, DialogProps,
    PopoverAlign, PopoverProps, SeparatorProps, SeparatorOrientation, TabItem, TabsProps, Theme,
};

let theme = Theme::default();
let mut dialog_open = true;
let mut popover_open = true;
let mut active_tab = "tab-1".to_string();
let tabs_items = [TabItem::new("tab-1", "General"), TabItem::new("tab-2", "Advanced")];

card(
    ui,
    &theme,
    CardProps::default()
        .with_heading("Card")
        .with_description("Container")
        .with_variant(CardVariant::Surface)
        .with_size(CardSize::Size3),
    |card_ui| {
        separator(
            card_ui,
            &theme,
            SeparatorProps {
                orientation: SeparatorOrientation::Horizontal,
                ..SeparatorProps::default()
            },
        );
        tabs(
            card_ui,
            &theme,
            TabsProps::new(ui.make_persistent_id("tabs"), &tabs_items, &mut active_tab),
            |tab_ui, active| {
                tab_ui.label(format!("Tab {}", active.id));
            },
        );
        let (_trigger, _content) = popover(
            card_ui,
            &theme,
            PopoverProps::new(ui.make_persistent_id("popover"), &mut popover_open)
                .with_align(PopoverAlign::End)
                .with_side_offset(8.0)
                .match_trigger_width(true),
            |t| t.button("Open popover"),
            |body| body.label("Popover body"),
        );
        let _dialog = dialog(
            card_ui,
            &theme,
            DialogProps::new(ui.make_persistent_id("dialog"), &mut dialog_open)
                .with_title("Dialog")
                .with_description("Modal with overlay")
                .with_scrim_opacity(160)
                .with_animation(true),
            |body| {
                body.label("Dialog content");
            },
        );
    },
);
```

`Card` поддерживает варианты `Surface/Classic/Ghost`, размеры `Size1..Size5` и опциональный интерактивный режим с hover/active/focus-анимациями через `CardProps`.

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

Tabs with shadcn-style pills:
```rust
use egui_shadcn::{
    tabs, TabItem, TabsJustify, TabsProps, TabsSize, TabsVariant, TabsWrap,
};

let mut active = "account".to_string();
let items = [
    TabItem::new("account", "Account"),
    TabItem::new("password", "Password"),
    TabItem::new("billing", "Billing").disabled(true),
];

let _ = tabs(
    ui,
    &theme,
    TabsProps::new(ui.make_persistent_id("profile-tabs"), &items, &mut active)
        .with_variant(TabsVariant::Soft)
        .with_size(TabsSize::Size2)
        .with_wrap(TabsWrap::NoWrap)
        .with_justify(TabsJustify::Start)
        .full_width(true),
    |content_ui, tab| {
        content_ui.label(format!("Content for {}", tab.id));
    },
);
```

## Examples
- `cargo run --example button` - all variants and sizes.
- `cargo run --example label` - `Label` with descriptions, required marks, variants.
- `cargo run --example text_input` — sizes `Sm|Md|Lg`, invalid/disabled, slots, variants, password/read-only.
- `cargo run --example select` — grouped options, `SelectProps`, invalid/disabled, custom `SelectStyle`.
- `cargo run --example checkbox` — variants/sizes, tri-state + invalid ring, disabled.
- `cargo run --example radio` — vertical/horizontal groups, descriptions, disabled options.
- `cargo run --example toggle` — default/outline, icon sizes, disabled.
- `cargo run --example switch` — color variants, sizes `Sm|Md|Lg` → `SwitchSize 1|2|3`, variants `surface|classic|soft`, high-contrast/disabled.
- `cargo run --example textarea` — counter/limit, invalid, disabled, all sizes.
- `cargo run --example tooltip` — positioning, delay, high-contrast tooltip helper.
- `cargo run --example basic` — all components on one screen.
- `cargo run --example card` — Surface/Classic/Ghost карточки, паддинги/радиус/тень.
- `cargo run --example separator` - гор./верт. разделители, size 1..4, thickness/length/color.
- `cargo run --example tabs` - underline/soft/outline, активный таб в состоянии.
- `cargo run --example scroll_area` - паритетный ScrollArea с кастомными барами: size/radius/type, max_size и auto_shrink.
- `cargo run --example popover` - align, offsets, match ширины триггера, кламп к экрану.
- `cargo run --example dialog` - модальный оверлей со scrim, выравниванием и анимацией.
- `cargo run --example structural` - полный набор приоритет-2 компонентов на одном экране.

## Tests
`cargo test`

## Migration
- `select` now takes `SelectProps`: `select(ui, &theme, SelectProps { ... })`.
- `textarea` takes `TextareaProps`; pass placeholder as `WidgetText` (`"text".into()`).
- `SelectProps` includes `is_invalid`; set to `false` for legacy behavior.
- `SeparatorProps::default()` now uses `gap = 0.0` (no implicit spacing); set `with_gap(6.0)` or a custom value to restore previous padding.
- `CheckboxOptions` получил поле `high_contrast`; при ручной инициализации через литерал добавьте `high_contrast: false` (или `true` для повышенного контраста). Добавлен helper `checkbox_tokens_with_high_contrast`.
- `select` добавил helper `find_typeahead_match` и клавиатурный поиск по префиксу; при необходимости можете использовать его для пользовательских UX проверок.
- `PopoverPlacement` adds `Left` and `Right` variants; update exhaustive matches accordingly (handle new sides or add a wildcard arm).
- `tabs` обновлены для паритета с shadcn/ui и Radix Themes: `TabItem` получил поле `disabled` (добавьте `disabled: false` при использовании литералов). Список табов теперь по умолчанию scrollable; отключите через `TabsProps::scrollable(false)` при необходимости.
- `scroll_area` теперь рендерит кастомные скроллбары в стиле shadcn/Radix; `bar_visibility` управляет их показом. Для прежнего always-visible поведения используйте `ScrollAreaType::Always` или `ScrollBarVisibility::AlwaysVisible`.
- `DialogProps` расширен (size scale, width/height limits, scrollable, close button, high_contrast, tokens_override). При ручной инициализации литералом добавьте новые поля или перейдите на `DialogProps::new(...).with_*` билдера.
