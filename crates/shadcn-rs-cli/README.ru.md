# shadcn-rs-cli

CLI-утилита для установки компонентов `shadcn-rs`.

> Переводы: [![EN](https://img.shields.io/badge/EN-README-black)](README.md) [![PT-BR](https://img.shields.io/badge/PT--BR-README-green)](README.pt-BR.md)

## Что делает

- Устанавливает исходники компонентов из:
  - `egui-shadcn/src` и `iced-shadcn/src`, найденных по зависимостям проекта
  - порядок резолва: `path` dependency -> `cargo metadata` -> fallback на локальный workspace `shadcn-rs`
- Генерирует локальную структуру модулей в проекте:
  - `src/shadcn/<backend>/<component>.rs`
  - `src/shadcn/mod.rs`
  - `src/shadcn/<backend>/mod.rs`
- Переписывает внутренние пути `crate::...` в публичные импорты:
  - `egui_shadcn::...` или `iced_shadcn::...`

## Установка

Из crates.io (после публикации):

```powershell
cargo install shadcn-rs-cli
```

Локальная установка из workspace:

```powershell
cd references/shadcn-rs
cargo install --path ./crates/shadcn-rs-cli
```

Имя бинарника:

```text
shadcn-rs
```

## Команды

Инициализация конфига в целевом проекте:

```powershell
shadcn-rs init --project . --backend egui
```

Список доступных компонентов:

```powershell
shadcn-rs list --backend egui
shadcn-rs list --backend iced
```

Установка компонента:

```powershell
shadcn-rs add button --project . --backend egui
```

Установка с автоматическим добавлением зависимости в `Cargo.toml`:

```powershell
shadcn-rs add button --project . --backend egui --write-cargo
```

С `--write-cargo` CLI при необходимости также автоматически выполняет `cargo fetch`, поэтому на новом проекте компонент можно поставить одной командой.

Принудительная перезапись файла компонента:

```powershell
shadcn-rs add button --project . --backend egui --force
```

## Требование по зависимостям

Локально установленный файл компонента зависит от backend-крейта:

- `egui-shadcn` для `--backend egui`
- `iced-shadcn` для `--backend iced`

Можно добавить зависимость автоматически через `--write-cargo` или вручную.

## Текущее ограничение

Сейчас это single-file installer. Если компонент нельзя безопасно переписать из внутренних `crate::...` путей, CLI завершится явной ошибкой `unsupported`, чтобы не сгенерировать нерабочий код.
