# shadcn-rs
> egui and iced component set with shadcn/ui aesthetics.

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/FerrisMind/shadcn-rs/master/.github/assets/icon-white.svg" />
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/FerrisMind/shadcn-rs/master/.github/assets/icon-black.svg" />
    <img alt="shadcn-rs logo" src="https://raw.githubusercontent.com/FerrisMind/shadcn-rs/master/.github/assets/icon-black.svg" width="200" />
  </picture>
</p>

> Translations: [![RU](https://img.shields.io/badge/RU-README-blue)](README.ru.md) [![PT-BR](https://img.shields.io/badge/PT--BR-README-green)](README.pt-BR.md)

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![egui-shadcn](https://img.shields.io/crates/v/egui-shadcn?label=egui-shadcn)](https://crates.io/crates/egui-shadcn)
[![iced-shadcn](https://img.shields.io/crates/v/iced-shadcn?label=iced-shadcn)](https://crates.io/crates/iced-shadcn)
[![iced-shadcn-v2](https://img.shields.io/crates/v/iced-shadcn-v2?label=iced-shadcn-v2)](https://crates.io/crates/iced-shadcn-v2)
[![shadcn-common](https://img.shields.io/crates/v/shadcn-common?label=shadcn-common)](https://crates.io/crates/shadcn-common)

> [!WARNING]
> API STABILITY NOTICE: `shadcn-rs` API is currently unstable and may change between versions, including breaking changes.
> Always pin exact crate versions and review release notes before upgrading.

## Overview
- Rust workspace for shadcn-style UI component libraries.
- Two public API styles: **props-first** (`*Props` / free functions) and **builder-first** (fluent `Component::new(…).variant(…)` chains).
- Theming and style packs in `shadcn-common` are built on [`twill`](https://github.com/FerrisMind/twill).

## Crates
- `egui-shadcn` — egui components, **props-first** API (some thin builder wrappers on top of props; see `crates/egui-shadcn/README.md`).
- `iced-shadcn` — iced components, **v1 props-first** API (see `crates/iced-shadcn/README.md`).
- `iced-shadcn-v2` — iced components, **v2 builder-first** API; does not depend on v1 (see `crates/iced-shadcn-v2/README.md`).
- `shadcn-common` — shared design tokens, style packs, and backend-agnostic helpers for egui/iced (see `crates/shadcn-common/README.md`).

## Demos

### egui-shadcn — WASM showcase

Full interactive showcase for `egui-shadcn` (and iced) in the browser:

**[ferrismind.github.io/shadcn-rs](https://ferrismind.github.io/shadcn-rs/)**

https://github.com/user-attachments/assets/8a3e7f25-fc51-4434-99d8-b9eb0de99e8f

Source: `crates/wasm-demo`.

### iced-shadcn v1 — desktop apps

These desktop apps use the **v1** (`iced-shadcn`) props-first API and are still **under development**. See also [issue #5](https://github.com/FerrisMind/shadcn-rs/issues/5).

#### Nova Code

Minimal VS Code–style editor UI on iced.

https://github.com/user-attachments/assets/04ddafcb-adf1-42fa-bb0e-97676792973b

#### Zver

Minimal desktop browser using system web engines via wry.

https://github.com/user-attachments/assets/0afa7180-efd5-496e-8f28-5a371fe2a12d

#### NeuroLang

Local desktop translator (text for now; more formats planned).

https://github.com/user-attachments/assets/e4908f23-5f14-486b-8200-9164f4136322

## License
Dual-licensed under [MIT](https://opensource.org/licenses/MIT) OR [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) (see workspace `Cargo.toml`).

## Acknowledgements
- [egui](https://github.com/emilk/egui) — immediate-mode GUI framework for the egui-shadcn crate.
- [iced](https://github.com/iced-rs/iced) — retained-mode GUI framework for the iced-shadcn and iced-shadcn-v2 crates.
- [Lucide Icons](https://github.com/lucide-icons/lucide) — icon set used via `lucide-icons`.
- [Radix UI](https://github.com/radix-ui/primitives) — interaction patterns and accessibility cues.
- [shadcn/ui](https://github.com/shadcn-ui/ui) — design language and component inspiration.
- [shadcn-svelte](https://github.com/huntabyte/shadcn-svelte) — Svelte port of shadcn/ui; reference for component patterns and API shape.
