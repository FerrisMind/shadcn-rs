# shadcn-rs

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset=".github/assets/icon-white.svg" />
    <source media="(prefers-color-scheme: light)" srcset=".github/assets/icon-black.svg" />
    <img alt="shadcn-rs logo" src=".github/assets/icon-black.svg" width="180" />
  </picture>
</p>

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
- `tooltip` — задержки открытия/закрытия, позиции, high-contrast.
- `card` — контейнер `Surface|Outline|Subtle`, заголовок/описание, настраиваемые паддинги/тень.
- `separator` — горизонтальный/вертикальный разделитель с толщиной/отступами.
- `tabs` — underline/soft/outline, горизонтальные/вертикальные, компактный режим.
- `scroll_area` — вертикальный/горизонтальный/оба скролла с контролем баров.
- `popover` — триггер + контент Above/Below, закрытие по Esc/клику вне, кастом ширина/высота.
- `dialog` — модальный оверлей с тайтлом/описанием, центр/offset позиционирование, закрытие по Esc/бэкдропу.

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
let mut active_tab = "tab-1".to_string();
let mut dialog_open = true;
let mut popover_open = true;
let tabs_items = [
    TabItem::new("tab-1", "Основное"),
    TabItem::new("tab-2", "Расширенное"),
];
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
    tabs(
        ui,
        &theme,
        TabsProps::new(ui.make_persistent_id("tabs"), &tabs_items, &mut active_tab),
        |tab_ui, active| {
            tab_ui.label(format!("Активен {}", active.id));
        },
    );
    let (_trigger, _content) = popover(
        ui,
        &theme,
        PopoverProps::new(ui.make_persistent_id("popover"), &mut popover_open),
        |t| t.button("Открыть popover"),
        |body| body.label("Тело popover"),
    );
    let _dialog = dialog(
        ui,
        &theme,
        DialogProps::new(ui.make_persistent_id("dialog"), &mut dialog_open)
            .with_title("Диалог")
            .with_description("Модальное окно"),
        |body| {
            body.label("Контент диалога");
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

