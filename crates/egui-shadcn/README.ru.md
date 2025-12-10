# egui-shadcn

## Обзор
`egui-shadcn` — набор компонентов ввода для egui в стиле shadcn/ui. Повторяет варианты и размеры shadcn и использует токены темы для единых визуальных состояний.

## Быстрый старт
```rust
use egui_shadcn::{button, ControlSize, ControlVariant, Theme};

fn ui_example(ui: &mut egui::Ui, theme: &Theme) {
    button(ui, theme, "Primary", ControlVariant::Primary, ControlSize::Md, true);
}
```

## Компоненты
- `button` — варианты `Primary|Secondary|Ghost|Outline|Destructive|Link`; размеры `Sm|Md|Lg|IconSm|Icon|IconLg`.
- `text_input` — цвет плейсхолдера, `is_invalid`, `enabled`, кольцо 3px, цвета выделения.
- `select` — через `SelectProps`, плейсхолдер, `is_invalid`, стрелка, disabled.
- `checkbox` — размеры и варианты.
- `toggle` — варианты default/outline с акцентными цветами.
- `switch` — размеры трека/ползунка как в shadcn.
- `textarea` — фокус, заливка при ошибке, опциональный счетчик.

## Тема
Состояния берутся из `Theme::control` и `Theme::input`, основанных на `ColorPalette`.

## Примеры
- `cargo run --example button` — все варианты и размеры.
- `cargo run --example text_input` — размеры `Sm|Md|Lg`, invalid/disabled.
- `cargo run --example select` — групповые списки, `SelectProps`, invalid/disabled, свой `SelectStyle`.
- `cargo run --example checkbox` — все варианты/размеры, disabled.
- `cargo run --example toggle` — default/outline, icon-размеры, disabled.
- `cargo run --example switch` — цветовые варианты, размеры `Sm|Md|Lg`, disabled.
- `cargo run --example textarea` — счетчик/лимит, invalid, disabled, все размеры.
- `cargo run --example basic` — все компоненты на одном экране.

## Тесты
`cargo test`

## Миграция
- `select` принимает `SelectProps`: `select(ui, &theme, SelectProps { ... })`.
- `textarea` принимает `TextareaProps`; плейсхолдер как `WidgetText` (`"text".into()`).
- В `SelectProps` есть `is_invalid`; установите `false` для прежнего поведения.

