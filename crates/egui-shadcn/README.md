# egui-shadcn

<p align="center">
  <img src="https://raw.githubusercontent.com/FerrisMind/shadcn-rs/master/crates/egui-shadcn/assets/icons/shadcn-egui/icon.svg" width="200" alt="shadcn-egui logo" />
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
egui-shadcn = "0.3.1"
egui = "0.33"
```

Or from git:

```toml
[dependencies]
egui-shadcn = { git = "https://github.com/FerrisMind/shadcn-rs", package = "egui-shadcn" }
```

Or from crates.io:

```toml
cargo add egui-shadcn
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
- **Layout**: `card`, `separator`, `tabs`, `scroll_area`, `collapsible`
- **Overlays**: `dialog`, `popover`, `tooltip`
- **Typography**: `label`, `text`, `heading`, `link`, `code`, `kbd`, `blockquote`, `typography` (shadcn variants)

- All components support variants, sizes, and theme customization.
- Checkbox: Radix Themes API (`size 1..=3`, variants `surface|classic|soft`, `color`, `high_contrast`).
- Dialog: Radix Themes Content API (`size 1..=4`, align `start|center`, width/min/max/height, `as_child`).
- Label: Radix Label API (`as_child`, `html_for`) plus variants/description/required.
- Popover: Radix Popover API (Root state, Popper positioning `side/align/offsets/collision`, Portal `container`, `force_mount`, DismissableLayer callbacks).
- Tooltip: Radix Tooltip API (Provider delays, Root state, Content positioning/collision, Portal container, dismiss callbacks).
- Select: Radix Select API (Root state, form props, positioning `position/side/align/offsets/collision`, dismiss callbacks, per-item `text_value`).
- Radio Group: Radix Radio Group API (`as_child`, controlled/uncontrolled, `orientation`, `dir`, `loop_focus`, per-item flags) plus card/grid variants.
- Tabs: Radix Tabs Root/List/Trigger/Content API (`as_child`, controlled/uncontrolled, `orientation`, `dir`, `activation_mode`, list `loop`, content `force_mount`) plus egui extensions (variants, wrap/justify, scrollable, full_width, accent/high_contrast, compact/animate).
- Switch: Radix Switch Root/Thumb API (`as_child`, controlled/uncontrolled, `name/value`, `required`) plus egui extensions (size/style/high_contrast/animate/accent/custom radius/thumb color).
- Scroll Area: Radix Scroll Area API (`type` default `hover`, `scroll_hide_delay` default `600ms`, `as_child`, `dir`, `force_mount` per axis) plus egui extensions (size/radius/accent/high_contrast/colors_override/max_size/bar_visibility).
- Typography: Radix Themes-like API for `Text`, `Heading`, `Link`, `Code`, `Kbd`, `Blockquote` plus shadcn-aligned `typography` variants (`H1/H2/H3/H4/P/Lead/Large/Small/Muted/InlineCode/Blockquote`).

## Theming

Visual states come from `Theme::control` and `Theme::input`, backed by `ColorPalette`:

```rust
use egui_shadcn::Theme;

let theme = Theme::default();
// Customize via theme tokens
```

`ColorPalette` defaults match shadcn theming variables (OKLCH, `Neutral`). For other documented base palettes, use `ShadcnBaseColor` with `ColorPalette::shadcn_light(...)` / `ColorPalette::shadcn_dark(...)`.

## Examples

Run examples to see components in action:

```bash
cargo run --example button      # Button variants and sizes
cargo run --example collapsible # Collapsible (Radix-like)
cargo run --example input       # Text input with config
cargo run --example select      # Select dropdown
cargo run --example checkbox    # Checkbox with tri-state
cargo run --example switch      # Switch component
cargo run --example toggle      # Toggle button
cargo run --example popover     # Popover component
cargo run --example typography  # Typography system demo
```

See `examples/` directory for all available examples.

## Documentation

See individual component examples in `examples/` directory for detailed usage.

## License

MIT

---

**Inspired by** [shadcn/ui](https://ui.shadcn.com) · **Icons by** [Lucide](https://lucide.dev)
