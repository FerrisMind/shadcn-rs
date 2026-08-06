# iced-shadcn-v2

Builder-first shadcn-inspired components for `iced`.

This crate is the v2 API and does not depend on the v1 `iced-shadcn` crate.
Theme tokens are resolved by `shadcn-common`; backend rendering is implemented
with native `iced` types.

## Module layout

- `components` — component implementations grouped by feature:
  - `components::alert` — callout with variants, icon, typed text, and action slots.
  - `components::accordion` — controlled single/multiple disclosure with items,
    custom trigger/content slots, disabled states, and navigation helpers.
  - `components::aspect_ratio` — layout wrapper that preserves a width-to-height ratio.
  - `components::avatar` — image/fallback roots, badges, overlapping groups, and counts.
  - `components::badge` — status label with shadcn variants, icons, and as-link.
  - `components::breadcrumb` — navigation trail with list, items, links, current
    page, chevron/custom separators, and a collapsed-steps ellipsis.
  - `components::button` — public button API and private geometry, rendering,
    style, and error modules.
  - `components::card` — composable card root with header, title,
    description, action, content, and footer slots.
  - `components::checkbox` — controlled checked, unchecked, and indeterminate input.
  - `components::data_table` — sortable, filterable, paginated table with row selection;
    forwards the existing Input, Button, Checkbox, and Table style parameters.
  - `components::collapsible` — controlled disclosure with trigger, content, and
    chevron indicator (height/width transition).
  - `components::field` — composable field roots, groups, labels, descriptions,
    separators, validation errors, and responsive layout support.
  - `components::input` — controlled text field over iced `text_input` with `.cn-input` styling.
  - `components::input_group` — composable input/textarea groups with inline and block addons.
  - `components::input_otp` — one-time-password slots with focus, keyboard
    editing, paste, pattern filters, groups, and a blinking fake caret.
  - `components::item` — media/content/actions row, item group, and separators.
  - `components::kbd` — keyboard-shortcut chip and grouped key sequences.
  - `components::label` — form label with style-pack typography and `for` / click.
  - `components::pagination` — controlled page window with prev/next and ellipsis.
  - `components::password` — extras password suite (input, visibility toggle, copy,
    zxcvbn strength meter) with shared state in `shadcn-common`.
  - `components::progress` — theme-aware determinate and indeterminate progress bar.
  - `components::radio_group` — controlled single-value radio selection with
    orientation, focus ring, descriptions, and arrow-key helpers.
  - `components::scroll_area` — themed rail and thumb over iced's own scrolling.
  - `components::separator` — horizontal/vertical rule.
  - `components::skeleton` — theme-aware pulse and static placeholders.
  - `components::slider` — single- and multi-thumb canvas slider with steps.
  - `components::sonner` — stacked toast notifications with typed actions,
    cancel buttons, promise updates, positions, timers, and theme-aware surfaces.
  - `components::spinner` — canvas-based loading indicator.
  - `components::switch` — controlled on/off toggle with animated thumb.
  - `components::table` — responsive compositional table with typed slots,
    spanning cells, alignment, hover, selected rows, and horizontal overflow.
  - `components::tabs` — controlled tab list/triggers/content with orientation,
    line variant, keyboard activation, and wrap options.
  - `components::toggle` — pressed/unpressed toggle button with variants.
  - `components::toggle_group` — controlled single/multiple toggle selection
    with orientation, spacing, and item composition.
  - `components::typography` — prose text, lists, and tables.
- `theme` — `shadcn-common` theme adapter for iced:
  - `theme::palette` — semantic colors and OKLCH-to-iced conversion.
  - `theme::tokens` — theme mode, style, base, accent, radius, and semantic APIs.
  - `theme::typography` — body, heading, and font-pack selection APIs.
- `fonts` — font-face exports and the iced font adapter.

The root `accordion`, `alert`, `aspect_ratio`, `avatar`, `badge`, `breadcrumb`, `button`, `card`, `checkbox`,
`collapsible`, `data_table`, `field`, `input`, `input_group`, `item`, `kbd`, `label`, `pagination`, `progress`,
`radio_group`, `scroll_area`, `separator`, `skeleton`, `slider`, `sonner`, `spinner`, `switch`, `table`,
  `tabs`, `toggle`, `toggle_group`, and `typography` modules are compatibility re-exports of
`components`, so existing
v2 imports remain valid while new code can use the feature-oriented
`iced_shadcn_v2::components::*` paths.

## Theming

Unlike shadcn on the web (CSS variables on `:root`), iced has no ambient theme.
**Your app owns a `Theme`** — usually in application state — and passes `&Theme`
into every component. Style packs (Vega, Nova, …) set defaults for fonts/radius;
`Theme::with_*` overrides win over the pack. Per-control knobs (`Button::variant`,
`color`, `radius`, …) win over that `Theme` for one widget.

### 1. One theme for the whole app

```rust,no_run
use iced_shadcn_v2::{AccentColor, Button, StyleId, Theme, ThemeMode};

struct App {
    theme: Theme,
}

impl Default for App {
    fn default() -> Self {
        Self {
            theme: Theme::light()
                .with_style(StyleId::Vega)
                .with_accent(Some(AccentColor::Blue))
                .with_mode(ThemeMode::Light),
        }
    }
}

// All buttons share &self.theme and restyle when you replace self.theme.
```

### 2. Two different style systems on screen at once

Pass a different `&Theme` into each button (clone + `with_style` is fine):

```rust,no_run
use iced::widget::row;
use iced_shadcn_v2::{Button, StyleId, Theme};

fn two_styles() -> iced::Element<'static, ()> {
    let vega = Theme::light().with_style(StyleId::Vega);
    let nova = Theme::light().with_style(StyleId::Nova);

    row![
        Button::text("Vega", &vega).into(),
        Button::text("Nova", &nova).into(),
    ]
    .into()
}
```

`StyleId` is **not** a `Button` prop — only a property of `Theme`.

### 3. Same theme, different button treatments

Keep one `Theme`; vary `variant` / `color` / `radius` / `size` per button:

```rust,no_run
use iced::widget::row;
use iced_shadcn_v2::{AccentColor, Button, ButtonRadius, ButtonVariant, Theme};

fn variants(theme: &Theme) -> iced::Element<'_, ()> {
    row![
        Button::text("Primary", theme)
            .variant(ButtonVariant::Default)
            .into(),
        Button::text("Ghost amber", theme)
            .variant(ButtonVariant::Ghost)
            .color(AccentColor::Amber)
            .radius(ButtonRadius::Full)
            .into(),
    ]
    .into()
}
```

`Button::style_override` is unrelated to Vega/Nova: it only tweaks the resolved
iced `button::Style` (colors, border, shadow) after our resolver runs.

## Examples

```bash
cargo run -p iced-shadcn-v2 --example alert
cargo run -p iced-shadcn-v2 --example accordion
cargo run -p iced-shadcn-v2 --example aspect_ratio
cargo run -p iced-shadcn-v2 --example avatar
cargo run -p iced-shadcn-v2 --example badge
cargo run -p iced-shadcn-v2 --example breadcrumb
cargo run -p iced-shadcn-v2 --example button
cargo run -p iced-shadcn-v2 --example card
cargo run -p iced-shadcn-v2 --example checkbox
cargo run -p iced-shadcn-v2 --example collapsible
cargo run -p iced-shadcn-v2 --example data_table
cargo run -p iced-shadcn-v2 --example field
cargo run -p iced-shadcn-v2 --example input
cargo run -p iced-shadcn-v2 --example input_group
cargo run -p iced-shadcn-v2 --example input_otp
cargo run -p iced-shadcn-v2 --example item
cargo run -p iced-shadcn-v2 --example kbd
cargo run -p iced-shadcn-v2 --example label
cargo run -p iced-shadcn-v2 --example pagination
cargo run -p iced-shadcn-v2 --example progress
cargo run -p iced-shadcn-v2 --example radio_group
cargo run -p iced-shadcn-v2 --example scroll_area
cargo run -p iced-shadcn-v2 --example separator
cargo run -p iced-shadcn-v2 --example skeleton
cargo run -p iced-shadcn-v2 --example slider
cargo run -p iced-shadcn-v2 --example sonner
cargo run -p iced-shadcn-v2 --example spinner
cargo run -p iced-shadcn-v2 --example switch
cargo run -p iced-shadcn-v2 --example table
cargo run -p iced-shadcn-v2 --example tabs
cargo run -p iced-shadcn-v2 --example toggle
cargo run -p iced-shadcn-v2 --example toggle_group
cargo run -p iced-shadcn-v2 --example typography
```

Sonner is mounted once in the root stack. Show or update toasts from an update
handler, then keep the toaster in the view:

```rust,no_run
use iced::widget::{button, stack, text};
use iced::{Element, Task};
use iced_shadcn_v2::{Theme, Toaster, toast};

#[derive(Debug, Clone)]
enum Message {
    ShowToast,
}

fn update(message: Message) -> Task<Message> {
    if matches!(message, Message::ShowToast) {
        let _ = toast("Event has been created")
            .description("Sunday, December 03, 2023 at 9:00 AM")
            .show();
    }
    Task::none()
}

fn view(theme: &Theme) -> Element<'_, Message> {
    let content = button(text("Show toast")).on_press(Message::ShowToast);
    stack![content, Toaster::new(theme).into()].into()
}
```

```rust,no_run
use iced_shadcn_v2::{
    Badge, BadgeVariant, Button, ButtonVariant, Spinner, SpinnerVariant, Theme, spinner,
};

#[derive(Debug, Clone)]
enum Message {
    Save,
}

fn view(theme: &Theme) -> iced::Element<'_, Message> {
    iced::widget::row![
        Badge::text("New", theme).variant(BadgeVariant::Secondary),
        Button::text("Save", theme)
            .variant(ButtonVariant::Default)
            .on_press(Message::Save),
        spinner(
            Spinner::new(theme)
                .variant(SpinnerVariant::AiLoaderIcon)
                .animated(true),
        ),
    ]
    .into()
}
```
