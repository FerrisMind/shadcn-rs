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

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![egui-shadcn](https://img.shields.io/crates/v/egui-shadcn?label=egui-shadcn)](https://crates.io/crates/egui-shadcn)
[![iced-shadcn](https://img.shields.io/crates/v/iced-shadcn?label=iced-shadcn)](https://crates.io/crates/iced-shadcn)
[![iced-shadcn-v2](https://img.shields.io/crates/v/iced-shadcn-v2?label=iced-shadcn-v2)](https://crates.io/crates/iced-shadcn-v2)
[![shadcn-common](https://img.shields.io/crates/v/shadcn-common?label=shadcn-common)](https://crates.io/crates/shadcn-common)

> [!WARNING]
> AVISO: a API do `shadcn-rs` é atualmente instável e pode mudar entre versões, incluindo breaking changes.
> Fixe versões exatas das dependências e revise as release notes antes de atualizar.

## Visão geral
- Workspace para bibliotecas de UI no estilo shadcn, em Rust.
- Dois estilos de API pública: **props-first** (`*Props` / funções livres) e **builder-first** (cadeias `Component::new(…).variant(…)`).
- Temas e style packs em `shadcn-common` são construídos sobre [`twill`](https://github.com/FerrisMind/twill).

## Crates
- `egui-shadcn` — componentes para egui, API **props-first** (alguns wrappers builder finos sobre props; veja `crates/egui-shadcn/README.md`).
- `iced-shadcn` — componentes para iced, API **v1 props-first** (veja `crates/iced-shadcn/README.md`).
- `iced-shadcn-v2` — componentes para iced, API **v2 builder-first**; não depende da v1 (veja `crates/iced-shadcn-v2/README.md`).
- `shadcn-common` — design tokens compartilhados, style packs e helpers agnósticos de backend para egui/iced (veja `crates/shadcn-common/README.md`).

## Demos

### egui-shadcn — WASM showcase

Showcase interativo completo de `egui-shadcn` (e iced) no navegador:

**[ferrismind.github.io/shadcn-rs](https://ferrismind.github.io/shadcn-rs/)**

Código-fonte: `crates/wasm-demo`.

### iced-shadcn v1 — apps desktop

Estes apps desktop usam a API **v1** (`iced-shadcn`) props-first e ainda estão **em desenvolvimento**. Veja também a [issue #5](https://github.com/FerrisMind/shadcn-rs/issues/5).

#### Nova Code

Editor minimalista no estilo VS Code em iced.

https://github.com/user-attachments/assets/04ddafcb-adf1-42fa-bb0e-97676792973b

#### Zver

Navegador desktop minimalista com engines web do sistema via wry.

https://github.com/user-attachments/assets/0afa7180-efd5-496e-8f28-5a371fe2a12d

#### NeuroLang

Tradutor desktop local (texto por enquanto; outros formatos planejados).

https://github.com/user-attachments/assets/e4908f23-5f14-486b-8200-9164f4136322

## Licença
Licença dupla: [MIT](https://opensource.org/licenses/MIT) OR [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) (veja `Cargo.toml` do workspace).

## Agradecimentos
- [egui](https://github.com/emilk/egui) — framework GUI em modo imediato para o crate egui-shadcn.
- [iced](https://github.com/iced-rs/iced) — framework GUI em modo retido para os crates iced-shadcn e iced-shadcn-v2.
- [Lucide Icons](https://github.com/lucide-icons/lucide) — conjunto de ícones usado via `lucide-icons`.
- [Radix UI](https://github.com/radix-ui/primitives) — padrões de interação e acessibilidade.
- [shadcn/ui](https://github.com/shadcn-ui/ui) — linguagem de design e inspiração dos componentes.
- [shadcn-svelte](https://github.com/huntabyte/shadcn-svelte) — port de shadcn/ui para Svelte; referência de padrões de componentes e formato de API.
