# shadcn-rs

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/FerrisMind/shadcn-rs/master/.github/assets/icon-white.svg" />
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/FerrisMind/shadcn-rs/master/.github/assets/icon-black.svg" />
    <img alt="shadcn-rs logo" src="https://raw.githubusercontent.com/FerrisMind/shadcn-rs/master/.github/assets/icon-black.svg" width="200" />
  </picture>
</p>

> Conjunto de componentes egui e iced com estética shadcn/ui.

> Traduções: [![EN](https://img.shields.io/badge/EN-README-black)](README.md) [![RU](https://img.shields.io/badge/RU-README-blue)](README.ru.md)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![egui-shadcn](https://img.shields.io/crates/v/egui-shadcn?label=egui-shadcn)](https://crates.io/crates/egui-shadcn)
[![iced-shadcn](https://img.shields.io/crates/v/iced-shadcn?label=iced-shadcn)](https://crates.io/crates/iced-shadcn)
[![iced-shadcn-v2](https://img.shields.io/crates/v/iced-shadcn-v2?label=iced-shadcn-v2)](https://crates.io/crates/iced-shadcn-v2)
[![shadcn-common](https://img.shields.io/crates/v/shadcn-common?label=shadcn-common)](https://crates.io/crates/shadcn-common)

> [!WARNING]
> AVISO: a API do `shadcn-rs` é atualmente instável e pode mudar entre versões, incluindo breaking changes.
> Fixe versões exatas das dependências e revise as release notes antes de atualizar.

## Visão geral
- Workspace para bibliotecas de UI no estilo shadcn, em Rust.

## Crates
- `crates/egui-shadcn` — componentes para egui (veja `crates/egui-shadcn/README.md`).
- `crates/iced-shadcn` — componentes para iced, API v1 (veja `crates/iced-shadcn/README.md`).
- `crates/iced-shadcn-v2` — componentes iced builder-first, API v2; não depende da v1 (veja `crates/iced-shadcn-v2/README.md`).
- `crates/shadcn-common` — design tokens compartilhados, style packs e helpers agnósticos de backend para egui/iced (veja `crates/shadcn-common/README.md`).

## Licença
MIT (veja `Cargo.toml` do workspace).

## Agradecimentos
- egui — framework GUI em modo imediato para o crate egui-shadcn.
- iced — framework GUI em modo retido para os crates iced-shadcn e iced-shadcn-v2.
- twill — style packs e núcleo de design tokens usados por `shadcn-common`.
- Lucide Icons — conjunto de ícones usado via `lucide-icons`.
- Radix UI — padrões de interação e acessibilidade.
- shadcn/ui — linguagem de design e inspiração dos componentes.
