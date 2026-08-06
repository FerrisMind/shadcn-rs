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

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![egui-shadcn](https://img.shields.io/crates/v/egui-shadcn?label=egui-shadcn)](https://crates.io/crates/egui-shadcn)
[![iced-shadcn](https://img.shields.io/crates/v/iced-shadcn?label=iced-shadcn)](https://crates.io/crates/iced-shadcn)
[![iced-shadcn-v2](https://img.shields.io/crates/v/iced-shadcn-v2?label=iced-shadcn-v2)](https://crates.io/crates/iced-shadcn-v2)
[![shadcn-common](https://img.shields.io/crates/v/shadcn-common?label=shadcn-common)](https://crates.io/crates/shadcn-common)

> [!WARNING]
> ВНИМАНИЕ: API `shadcn-rs` сейчас нестабилен и может меняться от версии к версии, включая breaking changes.
> Фиксируйте точные версии зависимостей и проверяйте release notes перед обновлением.

## Кратко
- Workspace под библиотеки в стиле shadcn на Rust.

## Крейты
- `crates/egui-shadcn` — компоненты для egui (см. `crates/egui-shadcn/README.md`).
- `crates/iced-shadcn` — компоненты для iced, API v1 (см. `crates/iced-shadcn/README.md`).
- `crates/iced-shadcn-v2` — builder-first компоненты для iced, API v2; не зависит от v1 (см. `crates/iced-shadcn-v2/README.md`).
- `crates/shadcn-common` — общие design tokens, style packs и backend-agnostic хелперы для egui/iced (см. `crates/shadcn-common/README.md`).

## Лицензия
MIT (см. workspace `Cargo.toml`).

## Благодарности
- egui — immediate-mode GUI фреймворк для крейта egui-shadcn.
- iced — retained-mode GUI фреймворк для крейтов iced-shadcn и iced-shadcn-v2.
- twill — style packs и ядро design tokens для `shadcn-common`.
- Lucide Icons — набор иконок, используемый через `lucide-icons`.
- Radix UI — паттерны взаимодействия и доступности.
- shadcn/ui — дизайн-язык и вдохновение для компонентов.

