# egui-shadcn

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/icons/shadcn-egui/icon-white.svg" />
    <source media="(prefers-color-scheme: light)" srcset="assets/icons/shadcn-egui/icon-black.svg" />
    <img alt="egui-shadcn logo" src="assets/icons/shadcn-egui/icon-black.svg" width="180" />
  </picture>
</p>

<p align="center">
  <strong>Shadcn-inspired component kit for egui</strong>
</p>

<p align="center">
  <a href="README.ru.md">Русский</a> · <a href="README.pt-BR.md">Português (Brasil)</a>
</p>

---

## Overview

`egui-shadcn` provides a set of form components for [egui](https://github.com/emilk/egui) styled after [shadcn/ui](https://ui.shadcn.com). It mirrors shadcn variants and sizes while exposing theme tokens for consistent visuals and per-control customization.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
egui-shadcn = "0.1.0"
egui = "0.33"
```

Or from git:

```toml
[dependencies]
egui-shadcn = { git = "https://github.com/FerrisMind/shadcn-rs", package = "egui-shadcn" }
```

## Quick Start

```rust
use egui_shadcn::{button, ControlSize, ControlVariant, Theme};

fn ui_example(ui: &mut egui::Ui, theme: &Theme) {
    button(
        ui,
        theme,
        "Click me",
        ControlVariant::Primary,
        ControlSize::Md,
        true,
    );
}
```

## Components

- **Form Controls**: `button`, `text_input`, `select`, `checkbox`, `radio_group`, `switch`, `toggle`, `textarea`
- **Layout**: `card`, `separator`, `tabs`, `scroll_area`
- **Overlays**: `dialog`, `popover`, `tooltip`
- **Typography**: `label`

All components support multiple variants, sizes, and theme customization.

## Theming

Visual states come from `Theme::control` and `Theme::input`, backed by `ColorPalette`:

```rust
use egui_shadcn::Theme;

let theme = Theme::default();
// Customize via theme tokens
```

## Examples

Run examples to see components in action:

```bash
cargo run --example button --features examples      # Button variants and sizes
cargo run --example input --features examples       # Text input with config
cargo run --example select --features examples      # Select dropdown
cargo run --example checkbox --features examples    # Checkbox with tri-state
cargo run --example switch --features examples      # Switch component
cargo run --example toggle --features examples      # Toggle button
cargo run --example popover --features examples     # Popover component
```

See `examples/` directory for all available examples.

## Documentation

See individual component examples in `examples/` directory for detailed usage.

## License

MIT

---

**Inspired by** [shadcn/ui](https://ui.shadcn.com) · **Icons by** [Lucide](https://lucide.dev)
