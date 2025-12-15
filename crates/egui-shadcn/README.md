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

All components support multiple variants, sizes, and theme customization. Checkbox aligns with Radix Themes: sizes `1..=3` (default `2`), variants `surface | classic | soft`, optional `color` override and `high_contrast`. Dialog aligns with Radix Themes Content API: sizes `1..=4` (default `3`), align `start | center` (default `center`), width/min/max/height props and optional `as_child` flag for parity. Label aligns with Radix Label: `as_child` opt-in, `html_for` to bind control, plus extras (variants/description/required) preserved. Popover aligns with Radix Popover (Primitives) API: Root (`default_open`, `on_open_change`, `modal`), Popper Content positioning (`side`, `align`, offsets, collision config), Portal (`container`) and `force_mount`, plus DismissableLayer handlers (`on_escape_key_down`, `on_pointer_down_outside`, `on_interact_outside`) where applicable in egui. Tooltip aligns with Radix Tooltip (Primitives) API: Provider/Root delays (`delay_duration`, `skip_delay_duration`, `disable_hoverable_content`), Root state (`open`, `default_open`, `on_open_change`), Content positioning/collision (`side`, `align`, offsets, `collision_padding`, `collision_boundary`, `sticky`, `hide_when_detached`, `update_position_strategy`), Portal container mapping, and dismiss callbacks (`on_escape_key_down`, `on_pointer_down_outside`) where applicable in egui. Select aligns with Radix Select (Primitives) API: Root state (`open/default_open`, `value/default_value`, change callbacks), shared form props (`name`, `auto_complete`, `required`, `disabled`, `form`, `dir`), and Content positioning/collision (`position`, `side`, `align`, offsets, collision padding/sticky/update strategy), plus dismiss callbacks (`on_close_auto_focus`, `on_escape_key_down`, `on_pointer_down_outside`), and per-item `text_value` support for typeahead.
Radio Group mirrors Radix Radio Group API: `as_child`, `default_value`, `value`, `on_value_change`, `disabled`, `name`, `required`, `orientation`, `dir`, `loop_focus` (Radix `loop`), and per-item `as_child`/`required`/`force_mount_indicator`, while keeping egui-specific extensions (card variant, grid layout, separators, accent color).
Tabs mirrors Radix Tabs Root/List/Trigger/Content API: `as_child` flags, `default_value`, controlled `value`, `on_value_change`, `orientation`, `dir`, `activation_mode`, list `loop`, content `force_mount`, and trigger-level `as_child`, while preserving egui extras (variants, wrap/justify, scrollable, full_width, accent/high_contrast, compact/animate).
Switch mirrors Radix Switch Root/Thumb API: `as_child`, `default_checked`, controlled `checked`, `on_checked_change`, `disabled`, `required`, `name`, `value` (default `\"on\"`), and `thumb_as_child`, with egui extensions for size/style/high_contrast/animate/accent/custom radius/thumb color.
Scroll Area mirrors Radix Scroll Area Root/View/Scrollbar/Thumb/Corner API: `type` (default `hover`), `scroll_hide_delay` (default `600ms`), `as_child`, `dir`, `force_mount` per axis, while keeping egui-specific extensions (size/radius/accent/high_contrast/colors_override/max_size/bar_visibility).

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
