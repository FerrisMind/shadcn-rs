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
- `select` — через `SelectProps`, плейсхолдер, `is_invalid`, стрелка, disabled, accent override, варианты триггера `surface/classic/soft/ghost`, варианты контента `soft/solid`, high-contrast, клавиатурный typeahead (по префиксу).
- `checkbox` — размеры и варианты, tri-state, фокус/invalid ring, high-contrast токены/кольцо.
- `toggle` — варианты default/outline с акцентными цветами.
- `switch` — размеры трека/ползунка как в shadcn.
- `textarea` — фокус, заливка при ошибке, опциональный счетчик.
- `card` — варианты `Surface|Classic|Ghost` (алиасы Outline/Subtle), заголовок/описание, настройка паддингов/радиуса/тени.
- `separator` — горизонтальный/вертикальный разделитель с толщиной/отступами, явной длиной и цветом.
- `tabs` — underline/soft/outline, горизонтальная/вертикальная ориентация, компактный режим.
- `scroll_area` — контролируемый скролл (vertical/horizontal/both), `bar_visibility`, `auto_shrink`.
- `popover` - `placement Above/Below/Left/Right`, align `Start|Center|End`, side/align offset, match ширина триггера, кламп к экрану, анимация, закрытие по Esc/клику вне.
- `dialog` — модальный оверлей с тайтлом/описанием, scrim opacity, offset позиционирование, анимация, флаги закрытия по Esc/бэкдропу.

## Тема
- Состояния берутся из `Theme::control` и `Theme::input`, основанных на `ColorPalette`.
- Токены темы: `ColorPalette`, `RadiusScale`, `MotionTokens` (150/200/250ms, cubic-bezier), `FocusTokens` (кольцо 3px). Анимации hover/press/open у button/select/checkbox/radio/tooltip используют `MotionTokens` и easing `ease_out_cubic`. Для кастомизации используйте `Theme::with_tokens(...)` c нужными радиусами/анимациями/шириной кольца.

## Примеры
- `cargo run --example button` — все варианты и размеры.
- `cargo run --example text_input` — размеры `Sm|Md|Lg`, invalid/disabled.
- `cargo run --example select` — групповые списки, `SelectProps`, invalid/disabled, свой `SelectStyle` с выбором trigger/content variants и accent.
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
- `CheckboxOptions` получил поле `high_contrast`; при ручной инициализации добавьте `high_contrast: false` (или `true` для повышенного контраста). Добавлен helper `checkbox_tokens_with_high_contrast`.
- `select` добавил helper `find_typeahead_match` и клавиатурный поиск по префиксу.
- `PopoverPlacement` получил новые варианты `Left` и `Right`; если вы матчите enum исчерпывающе, добавьте обработку новых сторон или wildcard arm.
