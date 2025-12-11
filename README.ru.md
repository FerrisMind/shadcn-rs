# shadcn-rs workspace

## Обзор
Rust-workspace под UI-библиотеки в стиле shadcn. Сейчас включает crate `egui-shadcn` с базовыми элементами форм для egui.

## Установка
```
cargo add egui-shadcn --path crates/egui-shadcn
```

## Компоненты
- `button` — варианты `Primary`, `Secondary`, `Ghost`, `Outline`, `Destructive`, `Link`; размеры `Sm|Md|Lg|IconSm|Icon|IconLg`; поддерживает `enabled`.
- `text_input` — плейсхолдер с кастомным цветом, `is_invalid`, `enabled`, 3px ring и цвет selection.
- `select` — плейсхолдер, список опций, `enabled`, `is_invalid` (через `SelectProps`), стрелка в тексте, кастом accent для триггера/контента, варианты `ghost|classic|soft`, high-contrast.
- `checkbox` — вариант и размер.
- `toggle` — кнопка-toggle (default/outline), цвета accent, размеры `Sm|Md|Lg`.
- `switch` — трековый переключатель, размеры трека/ползунка совпадают с shadcn (32×18.4).
- `textarea` — подсветка фокуса, `is_invalid` (заливка), опциональный счетчик и `max_len`.

## Примеры
- `cargo run --example button` — варианты `Primary|Secondary|Ghost|Outline|Destructive|Link` и все размеры включая icon.
- `cargo run --example text_input` — размеры `Sm|Md|Lg`, состояния `invalid` и `disabled`.
- `cargo run --example select` — legacy API (`SelectPropsSimple`), групповые списки, `invalid`, `disabled`, кастомный `SelectStyle`, размер `Sm`.
- `cargo run --example checkbox` — все варианты и размеры, включая `disabled`.
- `cargo run --example toggle` — варианты `Default|Outline`, icon-размеры, `disabled`.
- `cargo run --example switch` — варианты цвета, размеры `Sm|Md|Lg`, `disabled`.
- `cargo run --example textarea` — счетчик и лимит, `invalid`, `disabled`.
- `cargo run --example basic` — сводный пример всех компонентов.

Комбинированный экран (`basic.rs`):
```rust
let theme = Theme::default();
let mut value = String::new();
let mut selected = None;
egui::CentralPanel::default().show(&ctx, |ui| {
    button(ui, &theme, "Primary", ControlVariant::Primary, ControlSize::Md, true);
    text_input(ui, &theme, &mut value, "Введите текст", ControlSize::Md, false, true);
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
            placeholder: "Выберите",
            size: ControlSize::Sm,
            enabled: true,
            is_invalid: false,
        },
    );
});
```

## Тесты
`cargo test`

## Миграция
- `select` теперь принимает параметры через `SelectProps`; замените вызовы на `select(ui, &theme, SelectProps { ... })`.
- `textarea` принимает `TextareaProps`; передавайте плейсхолдер как `WidgetText` (например, `"text".into()`).
- `SelectProps` включает `is_invalid`; для прежнего поведения установите `false`.

## Маппинг на shadcn
- Варианты и размеры соответствуют аналогам shadcn/ui.
- Цвета задаются через `Theme` и `ColorPalette`.
- Визуальные состояния воспроизводят hover/active/disabled/focus.

