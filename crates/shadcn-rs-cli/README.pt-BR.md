# shadcn-rs-cli

CLI para instalar componentes do `shadcn-rs`.

> Traducoes: [![EN](https://img.shields.io/badge/EN-README-black)](README.md) [![RU](https://img.shields.io/badge/RU-README-blue)](README.ru.md)

## O que faz

- Instala arquivos-fonte de componentes a partir de:
  - `egui-shadcn/src` e `iced-shadcn/src` resolvidos das dependencias do projeto
  - ordem de resolucao: dependency com `path` -> `cargo metadata` -> fallback para workspace local `shadcn-rs`
- Gera estrutura local de modulos no projeto:
  - `src/shadcn/<backend>/<component>.rs`
  - `src/shadcn/mod.rs`
  - `src/shadcn/<backend>/mod.rs`
- Reescreve caminhos internos `crate::...` para imports publicos:
  - `egui_shadcn::...` ou `iced_shadcn::...`

## Instalacao

Do crates.io (apos publicar):

```powershell
cargo install shadcn-rs-cli
```

Instalacao local a partir do workspace:

```powershell
cd references/shadcn-rs
cargo install --path ./crates/shadcn-rs-cli
```

Nome do binario:

```text
shadcn-rs
```

## Comandos

Inicializar configuracao no projeto alvo:

```powershell
shadcn-rs init --project . --backend egui
```

Listar componentes disponiveis:

```powershell
shadcn-rs list --backend egui
shadcn-rs list --backend iced
```

Instalar um componente:

```powershell
shadcn-rs add button --project . --backend egui
```

Instalar e adicionar dependencia automaticamente no `Cargo.toml`:

```powershell
shadcn-rs add button --project . --backend egui --write-cargo
```

Com `--write-cargo`, a CLI tambem executa `cargo fetch` automaticamente quando necessario, permitindo instalar componente em projeto novo com um unico comando.

Forcar sobrescrita de arquivo de componente existente:

```powershell
shadcn-rs add button --project . --backend egui --force
```

## Requisito de dependencia

Os arquivos locais instalados dependem do crate de backend:

- `egui-shadcn` para `--backend egui`
- `iced-shadcn` para `--backend iced`

Use `--write-cargo` para adicionar a dependencia automaticamente, ou adicione manualmente.

## Limitacao atual

No momento este e um instalador de arquivo unico (single-file). Se um componente nao puder ser reescrito com seguranca a partir de caminhos internos `crate::...`, a CLI encerra com erro explicito `unsupported` em vez de gerar codigo quebrado.
