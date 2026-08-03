# План портирования компонентов shadcn-svelte → iced-shadcn-v2

Источник: `nova_refs/shadcn-svelte` (59 компонентов).
Ключевая специфика порта на iced: всё, что в вебе строится на порталах/поповерах
(bits-ui + floating-ui), требует общей **overlay-инфраструктуры**, поэтому она
выделена в отдельную фазу и от неё зависит примерно треть библиотеки.

---

## Фаза 0 — уже готово

| Компонент | Тип |
|---|---|
| `button` ✅ | с нуля (базовый примитив, от него зависят ~10 других) |
| `spinner` ✅ | с нуля |
| `separator` ✅ | с нуля |
| `badge` ✅ | с нуля |
| `label` ✅ | с нуля |
| `form` ✅ | композит над `field` + `FormState` в `shadcn-common` |
| `file-drop-zone` ✅ | extras: Root/Trigger/Textarea + shared validation in `shadcn-common` |

## Фаза 1 — атомарные примитивы (всё с нуля, без зависимостей)

Порядок внутри фазы — от простого к сложному:

1. ~~**`separator`** — с нуля (тривиален: линия)~~ ✅
2. **`skeleton`** — с нуля (прямоугольник + анимация пульса)
3. ~~**`label`** — с нуля (стилизованный текст)~~ ✅
4. ~~**`badge`** — с нуля (варианты как у button, но проще)~~ ✅
5. ~~**`kbd`** — с нуля~~ ✅
6. ~~**`typography`** — с нуля (это не компонент, а набор текстовых стилей/хелперов)~~ ✅
7. **`aspect-ratio`** — с нуля (в iced — просто layout-обёртка) ✅
8. **`progress`** — с нуля (в iced есть `progress_bar` — обернуть и стилизовать) ✅
9. ~~**`input`** — с нуля (обёртка над `text_input` — критичный примитив)~~ ✅
10. **`textarea`** — с нуля (обёртка над `text_editor`)
11. ~~**`checkbox`** — с нуля~~ ✅
12. ~~**`switch`** — с нуля (checkbox-подобная логика + анимация)~~ ✅
13. ~~**`toggle`** — с нуля (кнопка с состоянием, переиспользует стили `button`)~~ ✅
14. **`slider`** — с нуля (обёртка над iced `slider`) ✅
15. ~~**`avatar`** — с нуля (image + fallback-логика)~~ ✅
16. ~~**`card`** — с нуля (контейнер: Header/Title/Description/Content/Footer)~~ ✅
17. ~~**`alert`** — с нуля (контейнер + иконка + typography)~~ ✅
18. ~~**`scroll-area`** — с нуля (обёртка над `scrollable` со стилизацией скроллбара)~~ ✅

## Фаза 2 — простые композиты (переиспользуют Фазу 0–1)

19. ~~**`button-group`** — переиспользует `button` (layout + слияние границ)~~ ✅
20. ~~**`toggle-group`** — переиспользует `toggle` (+ single/multiple selection state)~~ ✅
21. ~~**`radio-group`** — с нуля сама радио-кнопка, переиспользует `label`~~ ✅
22. ~~**`input-group`** — переиспользует `input`, `button`, `textarea` (addon-слоты)~~ ✅
23. ~~**`breadcrumb`** — с нуля (простой layout: link + separator-иконка)~~ ✅
24. ~~**`pagination`** — переиспользует варианты `button`~~ ✅
25. ~~**`item`** — с нуля (универсальная строка: media/content/actions)~~ ✅
26. ~~**`empty`** — переиспользует `item`-подобный layout + typography~~ ✅
27. ~~**`field`** — переиспользует `label`, `input`, `checkbox` и т.д. (форменная обвязка + ошибки)~~ ✅
27b. ~~**`form`** — переиспользует `field` + `button` + `FormState`/`FormRecipe` из `shadcn-common`~~ ✅
28. ~~**`table`** — с нуля (Row/Cell/Header на базе grid/column)~~ ✅
29. ~~**`tabs`** — с нуля (trigger переиспользует стили `button`/`toggle`)~~ ✅
30. ~~**`collapsible`** — с нуля (state + анимация высоты)~~ ✅
31. ~~**`accordion`** — переиспользует логику `collapsible` (+ single/multiple)~~ ✅
33. ~~**`input-otp`** — переиспользует `input`-логику (посимвольные слоты, фокус-менеджмент)~~ ✅
33b. ~~**`star-rating`** (extras) — с нуля на bits-ui RatingGroup + `StarRating` recipe/state в `shadcn-common`~~ ✅
33c. ~~**`phone-input`** (extras) — country selector (popover+command) + tel field; `PhoneInput` recipe/state в `shadcn-common`~~ ✅
33d. ~~**`password`** (extras) — Root/Input/Toggle/Copy/Strength + zxcvbn state в `shadcn-common`~~ ✅
34. **`resizable`** — в iced отображается на `pane_grid` — обёртка с нуля ✅

## Фаза 3 — overlay-инфраструктура ⚠️ (ключевая, самая рискованная)

Сначала пишется **общий overlay/portal-примитив** (позиционирование, click-outside,
Esc, z-слои) — аналог bits-ui Floating/Portal. Всё ниже — его потребители:

35. ~~**`tooltip`** — с нуля на overlay (самый простой потребитель, есть база в iced)~~ ✅
36. **`popover`** — с нуля на overlay (**фундамент**: от него зависят select, combobox, date-picker) ✅
37. ~~**`hover-card`** — переиспользует `popover` (hover-триггер вместо клика)~~ ✅
38. ~~**`dialog`** — с нуля на overlay (модальность + backdrop + фокус-ловушка)~~ ✅
39. **`alert-dialog`** — переиспользует `dialog` + варианты `button` ✅
40. ~~**`sheet`** — переиспользует `dialog` (позиционирование у края + slide-анимация)~~ ✅
41. ~~**`drawer`** — переиспользует `sheet`/`dialog` (в вебе это vaul, у нас — вариант sheet)~~ ✅
42. **Меню-примитив** (общий) → далее три потребителя:
    - ~~**`dropdown-menu`** — с нуля на popover + menu-примитив~~ ✅
    - **`context-menu`** — переиспользует внутренности `dropdown-menu` (правый клик)
    - ~~**`menubar`** — переиспользует `dropdown-menu` (горизонтальная полоса триггеров)~~ ✅
43. ~~**`select`** — переиспользует `popover` + список item'ов~~ ✅
44. ~~**`navigation-menu`** — переиспользует popover/menu-логику~~ ✅
45. ~~**`sonner`** (toast) — с нуля на overlay (в вебе внешняя либа svelte-sonner; у нас — очередь + стек + таймеры)~~ ✅

## Фаза 4 — тяжёлые композиты (финал)

46. ~~**`command`** — переиспользует `dialog` + `input` + список (fuzzy-фильтрация в `shadcn-common`)~~ ✅
47. **`combobox`** — переиспользует `popover` + `command` (сам почти ничего не добавляет)
48. **`carousel`** — с нуля (в вебе embla; у нас — свой scroll/snap) + `button` ✅
49. **`calendar`** — с нуля (date-логика!) + варианты `button` + `select` ✅
50. **`range-calendar`** — переиспользует `calendar` (range-состояние) ✅
51. **`date-picker`** — переиспользует `popover` + `calendar`/`range-calendar` + `input-group` ✅
52. **`data-table`** — переиспользует `table`, `checkbox`, `button`, `dropdown-menu`, `select`, `input` (в вебе TanStack Table — нужен свой мини-движок: sort/filter/pagination state) ✅ [chorale-core 0.2.3]
53. **`chart`** — с нуля на `Canvas` (в вебе layerchart; самый «неперетаскиваемый» компонент) + `tooltip`
54. ~~**`sidebar`** — переиспользует `sheet`, `tooltip`, `separator`, `skeleton`, `input`, `button` (самый большой композит — строго последним)~~ ✅

---

## Сводка

### С нуля (собственные примитивы)

button ✅, spinner ✅, separator ✅, skeleton, label ✅, badge ✅, kbd ✅, typography ✅,
aspect-ratio, progress, input ✅, textarea, checkbox ✅, switch ✅, toggle ✅, slider,
avatar ✅, card ✅, alert ✅, scroll-area ✅, radio-group ✅, breadcrumb ✅, item ✅, table ✅, tabs ✅,
collapsible ✅, accordion ✅, pagination ✅, field ✅, empty ✅, resizable, tooltip ✅, popover, dialog ✅, sonner ✅,
carousel ✅, calendar, chart, command ✅, overlay ✅, dropdown-menu ✅, navigation-menu ✅, menubar ✅.

### Переиспользуют готовые

| База | Потребители |
|---|---|
| `button` | button-group, pagination, alert-dialog, carousel, calendar, data-table, sidebar |
| `toggle` | toggle-group |
| `label` | radio-group |
| `input` | input-group, input-otp, field, command, sidebar |
| `collapsible` | accordion |
| `popover` | hover-card, select, combobox, date-picker, dropdown-menu |
### Переиспользуют готовые

| База | Потребители |
|---|---|
| `button` | button-group, pagination, alert-dialog, carousel, calendar, data-table, sidebar |
| `toggle` | toggle-group |
| `label` | radio-group |
| `input` | input-group, input-otp, field, command, sidebar |
| `collapsible` | accordion |
| `popover` | hover-card, select, combobox, date-picker, dropdown-menu |
| `dialog` | alert-dialog, sheet ✅, command |
| `sheet` ✅ | drawer ✅, sidebar |
| `dropdown-menu` ✅ | context-menu, menubar ✅ |
| `calendar` | range-calendar, date-picker |
| `table` | data-table |
| `command` | combobox |

### Критический путь

`input` → overlay-примитив → `popover`/`dialog` — от этих трёх зависит
~20 компонентов, поэтому их стоит проектировать особенно тщательно
(API, темизация, фокус), а не «лишь бы работало».

### Отдельные риски для iced

- **Overlay-слой** (Фаза 3) — позиционирование, click-outside, Esc, z-слои.
- **Date-логика** для calendar — нужен аналог `@internationalized/date`
  (например, крейт `time`/`chrono` + своя локализация).
- **Chart** — полностью ручной рендер на Canvas.
