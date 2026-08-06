# shadcn-rs

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/FerrisMind/shadcn-rs/master/.github/assets/icon-white.svg" />
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/FerrisMind/shadcn-rs/master/.github/assets/icon-black.svg" />
    <img alt="shadcn-rs logo" src="https://raw.githubusercontent.com/FerrisMind/shadcn-rs/master/.github/assets/icon-black.svg" width="200" />
  </picture>
</p>

> Набор компонентов egui и iced в эстетике shadcn/ui.

> Переводы: [![EN](https://img.shields.io/badge/EN-README-black)](README.md) [![PT-BR](https://img.shields.io/badge/PT--BR-README-green)](README.pt-BR.md)

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![egui-shadcn](https://img.shields.io/crates/v/egui-shadcn?label=egui-shadcn)](https://crates.io/crates/egui-shadcn)
[![iced-shadcn](https://img.shields.io/crates/v/iced-shadcn?label=iced-shadcn)](https://crates.io/crates/iced-shadcn)
[![iced-shadcn-v2](https://img.shields.io/crates/v/iced-shadcn-v2?label=iced-shadcn-v2)](https://crates.io/crates/iced-shadcn-v2)
[![shadcn-common](https://img.shields.io/crates/v/shadcn-common?label=shadcn-common)](https://crates.io/crates/shadcn-common)

> [!WARNING]
> ВНИМАНИЕ: API `shadcn-rs` сейчас нестабилен и может меняться от версии к версии, включая breaking changes.
> Фиксируйте точные версии зависимостей и проверяйте release notes перед обновлением.

## Кратко
- Workspace под библиотеки в стиле shadcn на Rust.
- Два публичных стиля API: **props-first** (`*Props` / свободные функции) и **builder-first** (цепочки `Component::new(…).variant(…)`).
- Темизация и style packs в `shadcn-common` построены на [`twill`](https://github.com/FerrisMind/twill).

## Крейты
- `egui-shadcn` — компоненты для egui, **props-first** API (поверх props есть тонкие builder-обёртки; см. `crates/egui-shadcn/README.md`).
- `iced-shadcn` — компоненты для iced, **v1 props-first** API (см. `crates/iced-shadcn/README.md`).
- `iced-shadcn-v2` — компоненты для iced, **v2 builder-first** API; не зависит от v1 (см. `crates/iced-shadcn-v2/README.md`).
- `shadcn-common` — общие design tokens, style packs и backend-agnostic хелперы для egui/iced (см. `crates/shadcn-common/README.md`).

## Демо

### egui-shadcn — WASM showcase

Полноценный интерактивный showcase для `egui-shadcn` (и iced) в браузере:

**[ferrismind.github.io/shadcn-rs](https://ferrismind.github.io/shadcn-rs/)**

Исходники: `crates/wasm-demo`.

### iced-shadcn v1 — desktop apps

Эти desktop-приложения используют **v1** (`iced-shadcn`) props-first API и пока **находятся в разработке**. См. также [issue #5](https://github.com/FerrisMind/shadcn-rs/issues/5).

#### Nova Code

Минималистичный редактор в духе VS Code на iced.

https://github.com/user-attachments/assets/04ddafcb-adf1-42fa-bb0e-97676792973b

#### Zver

Минималистичный desktop-браузер на системных web-движках через wry.

https://github.com/user-attachments/assets/0afa7180-efd5-496e-8f28-5a371fe2a12d

#### NeuroLang

Локальный desktop-переводчик (сейчас текст; другие форматы в планах).

https://github.com/user-attachments/assets/e4908f23-5f14-486b-8200-9164f4136322

## Лицензия
Двойная лицензия: [MIT](https://opensource.org/licenses/MIT) OR [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) (см. workspace `Cargo.toml`).

## Благодарности
- [egui](https://github.com/emilk/egui) — immediate-mode GUI фреймворк для крейта egui-shadcn.
- [iced](https://github.com/iced-rs/iced) — retained-mode GUI фреймворк для крейтов iced-shadcn и iced-shadcn-v2.
- [Lucide Icons](https://github.com/lucide-icons/lucide) — набор иконок, используемый через `lucide-icons`.
- [Radix UI](https://github.com/radix-ui/primitives) — паттерны взаимодействия и доступности.
- [shadcn/ui](https://github.com/shadcn-ui/ui) — дизайн-язык и вдохновение для компонентов.
- [shadcn-svelte](https://github.com/huntabyte/shadcn-svelte) — порт shadcn/ui на Svelte; референс паттернов компонентов и формы API.

