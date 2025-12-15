# egui-shadcn

<p align="center">
  <picture>
    <img alt="shadcn-egui logo" src="https://raw.githubusercontent.com/FerrisMind/shadcn-rs/master/crates/egui-shadcn/assets/icons/shadcn-egui/icon.svg" width="180" />
  </picture>
</p>

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
- Все компоненты поддерживают варианты, размеры и настройку темы.
- Checkbox: API Radix Themes (`size 1..=3`, варианты `surface|classic|soft`, `color`, `high_contrast`).
- Dialog: API Radix Themes Content (`size 1..=4`, align `start|center`, width/min/max/height, `as_child`).
- Label: API Radix Label (`as_child`, `html_for`) + варианты/description/required.
- Popover: API Radix Popover (Root state, позиционирование Popper `side/align/offsets/collision`, Portal `container`, `force_mount`, DismissableLayer callbacks).
- Tooltip: API Radix Tooltip (Provider задержки, Root state, позиционирование/коллизия Content, Portal container, dismiss callbacks).
- Select: API Radix Select (Root state, form-поля, позиционирование `position/side/align/offsets/collision`, dismiss callbacks, `text_value` для typeahead).
- Radio Group: API Radix Radio Group (`as_child`, контролируемый/неконтролируемый, `orientation`, `dir`, `loop_focus`, флаги на item) + карточный/сеточный варианты.
- Tabs: API Radix Tabs Root/List/Trigger/Content (`as_child`, controlled/uncontrolled, `orientation`, `dir`, `activation_mode`, list `loop`, content `force_mount`) + egui-расширения (variants, wrap/justify, scrollable, full_width, accent/high_contrast, compact/animate).
- Switch: API Radix Switch Root/Thumb (`as_child`, controlled/uncontrolled, `name/value`, `required`) + egui-расширения (size/style/high_contrast/animate/accent/custom radius/thumb color).
- Scroll Area: API Radix Scroll Area (`type` по умолчанию `hover`, `scroll_hide_delay` `600ms`, `as_child`, `dir`, `force_mount` по осям) + egui-расширения (size/radius/accent/high_contrast/colors_override/max_size/bar_visibility).

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
