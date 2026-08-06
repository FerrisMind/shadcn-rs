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

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![egui-shadcn](https://img.shields.io/crates/v/egui-shadcn?label=egui-shadcn)](https://crates.io/crates/egui-shadcn)
[![iced-shadcn](https://img.shields.io/crates/v/iced-shadcn?label=iced-shadcn)](https://crates.io/crates/iced-shadcn)
[![iced-shadcn-v2](https://img.shields.io/crates/v/iced-shadcn-v2?label=iced-shadcn-v2)](https://crates.io/crates/iced-shadcn-v2)
[![shadcn-common](https://img.shields.io/crates/v/shadcn-common?label=shadcn-common)](https://crates.io/crates/shadcn-common)

> [!WARNING]
> API STABILITY NOTICE: `shadcn-rs` API is currently unstable and may change between versions, including breaking changes.
> Always pin exact crate versions and review release notes before upgrading.

## Overview
- Rust workspace for shadcn-style UI component libraries.

## Crates
- `crates/egui-shadcn` — egui components (see `crates/egui-shadcn/README.md`).
- `crates/iced-shadcn` — iced components, v1 API (see `crates/iced-shadcn/README.md`).
- `crates/iced-shadcn-v2` — builder-first iced components, v2 API; does not depend on v1 (see `crates/iced-shadcn-v2/README.md`).
- `crates/shadcn-common` — shared design tokens, style packs, and backend-agnostic helpers for egui/iced (see `crates/shadcn-common/README.md`).

## License
MIT (see workspace `Cargo.toml`).

## Acknowledgements
- egui — immediate-mode GUI framework for the egui-shadcn crate.
- iced — retained-mode GUI framework for the iced-shadcn and iced-shadcn-v2 crates.
- twill — style packs and design-token core used by `shadcn-common`.
- Lucide Icons — icon set used via `lucide-icons`.
- Radix UI — interaction patterns and accessibility cues.
- shadcn/ui — design language and component inspiration.
