# План работ

## Новая задача: 100% паритет примеров компонентов с shadcn/ui

- [x] Собрать список реализованных компонентов и их примеров в `crates/egui-shadcn/examples`.
- [x] Для каждого такого компонента найти оригинальный пример в `reference/shadcn-ui/apps/v4/registry/new-york-v4/examples`.
- [x] Привести наши примеры к 100% соответствию структуры, текстов и поведения оригиналам.
- [x] Прогнать `cargo test -p egui-shadcn` и сборку примеров, исправить расхождения.

## Архив: баг цикла открытия/закрытия Popover

План работы над багом цикла открытия/закрытия Popover:
- [x] Осмотреть реализацию popover и пример, сформировать гипотезы.
- [ ] Добавить точечное логирование/инструментацию и собрать логи воспроизведения.
- [ ] Определить причину по логам и внести минимальный фикс.
- [ ] Проверить фикс повторно с логами.
- [ ] Очистить временные логи/инструментацию, обновить документацию при необходимости.

## Текущая задача: Паритет структурных компонентов (Приоритет 2)

- [x] Привести `scripts/список_компонентов.md` к виду «Только Приоритет 2».
- [x] Сверить реализацию Dialog/Card/Separator/Tabs/ScrollArea/Popover с референсами shadcn/ui и Radix Themes (API, визуал, анимации, кастомизация).
- [x] Прогнать тесты `cargo test -p egui-shadcn` и исправить расхождения.
- [x] При необходимости обновить README и примеры.

## Новая задача: 100% паритет Card с shadcn/ui + Radix Themes

- [x] Изучить текущую реализацию `card.rs` и паттерны компонентов.
- [x] Сверить API/варианты/токены/анимации с официальными shadcn/ui и Radix Themes.
- [ ] Расширить Card: подкомпоненты, интерактивные состояния, гибкая кастомизация.
- [x] Расширить Card: подкомпоненты, интерактивные состояния, гибкая кастомизация.
- [x] Добавить/обновить тесты Card, прогнать `cargo test -p egui-shadcn`.
- [x] Обновить README/примеры по Card (если нужно).

## Новая задача: 100% паритет Tabs с shadcn/ui + Radix Themes

- [x] Изучить текущую реализацию `tabs.rs` и референсы shadcn/ui + Radix Themes.
- [x] Добавить/обновить тесты для Tabs (smoke + keyboard navigation).
- [x] Расширить Tabs API: size, wrap/justify, scrollable, full_width, disabled, accent, high_contrast, animate.
- [x] Привести визуал/анимации Tabs к паритету (Underline/Soft).
- [x] Обновить README/примеры и секцию миграции по Tabs.

## Новая задача: 100% паритет Separator с shadcn/ui + Radix Themes

- [x] Изучить текущую реализацию `separator.rs` и референсы shadcn/ui + Radix Themes.
- [x] Добавить/обновить тесты Separator (sizes/orientation/color/length overrides).
- [x] Расширить Separator API: size 1..4, accent/color, decorative, high_contrast, custom thickness.
- [x] Привести визуал/поведение Separator к паритету.
- [x] Обновить README/пример по Separator (если нужно).
- [x] Прогнать `cargo test -p egui-shadcn`.

## Новая задача: обновить пример Separator под референс shadcn/ui

- [x] Привести пример `examples/separator/main.rs` к структуре/текстам из @reference/shadcn-ui.
- [x] Ограничить ширину/сдвиг примера, чтобы сепаратор не тянулся на весь экран.
- [x] Проверить пример визуально/документационно и обновить README при необходимости.

## Новая задача: обновить пример Tabs под референс shadcn/ui

- [x] Привести пример `examples/tabs/main.rs` к структуре/текстам из @reference/shadcn-ui.
- [x] Проверить пример визуально/документационно и обновить README при необходимости.

## Новая задача: 100% паритет ScrollArea с shadcn/ui + Radix Themes

- [x] Изучить референсы shadcn/ui ScrollArea и Radix Themes ScrollArea.
- [x] Спроектировать расширенный API ScrollArea и написать тесты.
- [x] Реализовать ScrollArea с кастомными скроллбарами, анимациями и кастомизацией.
- [x] Обновить README/примеры по ScrollArea.
- [x] Прогнать `cargo test -p egui-shadcn`.

## Актуализация: восстановить ScrollArea и довести до 100% паритета

- [x] Восстановить модуль `scroll_area.rs` и экспортируемый API.
- [x] Сверить визуал/анимации/кастомизацию со свежими референсами Radix Themes + shadcn/ui.
- [x] Вернуть пример `examples/scroll_area/main.rs` и обновить README (если нужно).
- [x] Прогнать `cargo test -p egui-shadcn` и исправить расхождения.

## Новая задача: 100% паритет Dialog с shadcn/ui + Radix Themes

- [x] Изучить текущую реализацию `dialog.rs` и референсы shadcn/ui + Radix Themes.
- [x] Спроектировать расширенный API Dialog (sizes 1..4, align, width/height limits, overlay, close button, scroll).
- [x] Добавить/обновить тесты Dialog (geometry + open/close + size tokens).
- [x] Реализовать Dialog с анимациями show/hide, скроллом и кастомизацией в стиле Radix/Shadcn.
- [x] Обновить README/примеры по Dialog и секцию миграции (если API расширен).
- [x] Прогнать `cargo test -p egui-shadcn`.

## Срочная задача: починить предупреждения clippy в `egui-shadcn`

- [x] Проверить текущий список предупреждений `cargo clippy --package egui-shadcn --features examples --examples --release -- -D warnings`.
- [x] Исправить предупреждения в `dialog.rs`, `scroll_area.rs`, `separator.rs`, `tabs.rs`.
- [x] Повторно запустить `cargo clippy` и убедиться, что предупреждений нет.

## Новая задача: заменить прямое использование egui на внутренние UI-компоненты

- [x] Просканировать кодовую базу: найти все наши UI-обёртки над egui и все места прямого использования egui.
- [x] Для каждого прямого использования egui определить, есть ли наш аналог.
- [x] Заменить egui на наш компонент там, где аналог существует, без изменения логики/состояния.
- [x] Оставить прямое egui там, где аналога нет, и добавить TODO-комментарий с описанием требуемой обёртки.
- [x] Собрать проект и прогнать тесты (если есть) после серии изменений.

## Новая задача: привести пример Dialog к shadcn/ui структуре

- [x] Осмотреть текущий `examples/dialog` и блок Dialog в `examples/structural`.
- [x] Перестроить пример Dialog: Trigger (outline button) → Content → Header (Title/Description) → Form body (labels+inputs grid) → Footer (submit button).
- [x] Убедиться, что пример компилируется и визуально соответствует задумке.
- [x] Убрать наложение полей на хедер (отключить scrollable в примере).
- [x] Выровнять close-кнопку по заголовку и убрать лишнюю строку перед хедером.
