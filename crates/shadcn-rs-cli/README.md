# shadcn-rs-cli

CLI installer for `shadcn-rs` components.

> Translations: [![RU](https://img.shields.io/badge/RU-README-blue)](README.ru.md) [![PT-BR](https://img.shields.io/badge/PT--BR-README-green)](README.pt-BR.md)

## What It Does

- Installs component source files from:
  - `egui-shadcn/src` and `iced-shadcn/src` resolved from your project dependencies
  - resolution order: `path` dependency -> `cargo metadata` package source -> local `shadcn-rs` workspace fallback
- Generates local module structure in your project:
  - `src/shadcn/<backend>/<component>.rs`
  - `src/shadcn/mod.rs`
  - `src/shadcn/<backend>/mod.rs`
- Rewrites internal `crate::...` paths to public imports:
  - `egui_shadcn::...` or `iced_shadcn::...`

## Installation

From crates.io (after publish):

```powershell
cargo install shadcn-rs-cli
```

Local install from workspace:

```powershell
cd references/shadcn-rs
cargo install --path ./crates/shadcn-rs-cli
```

Binary name:

```text
shadcn-rs
```

## Commands

Initialize config in your target project:

```powershell
shadcn-rs init --project . --backend egui
```

List available components:

```powershell
shadcn-rs list --backend egui
shadcn-rs list --backend iced
```

Install a component:

```powershell
shadcn-rs add button --project . --backend egui
```

Install and auto-add dependency in `Cargo.toml`:

```powershell
shadcn-rs add button --project . --backend egui --write-cargo
```

With `--write-cargo`, the CLI also runs `cargo fetch` automatically when needed, so a fresh project can install a component in one command.

Force overwrite existing component file:

```powershell
shadcn-rs add button --project . --backend egui --force
```

## Dependency Requirement

Installed local component files depend on the backend crate:

- `egui-shadcn` for `--backend egui`
- `iced-shadcn` for `--backend iced`

Use `--write-cargo` to add the missing dependency automatically, or add it manually.

## Current Limitation

This is a single-file installer. If a component cannot be safely rewritten from internal `crate::...` paths, the CLI will stop with an explicit `unsupported` error instead of generating broken code.
