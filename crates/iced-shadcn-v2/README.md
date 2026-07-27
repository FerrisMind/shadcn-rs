# iced-shadcn-v2

Builder-first shadcn-inspired components for `iced`.

This crate is the v2 API and does not depend on the v1 `iced-shadcn` crate.
Theme tokens are resolved by `shadcn-common`; backend rendering is implemented
with native `iced` types.

## Module layout

- `components` — component implementations grouped by feature:
  - `components::button` — public button API and private geometry, rendering,
    style, and error modules.
  - `components::spinner` — canvas-based loading indicator.
- `theme` — `shadcn-common` theme adapter for iced:
  - `theme::palette` — semantic colors and OKLCH-to-iced conversion.
  - `theme::tokens` — theme mode, style, base, accent, radius, and semantic APIs.
  - `theme::typography` — body, heading, and font-pack selection APIs.
- `fonts` — font-face exports and the iced font adapter.

The root `button` and `spinner` modules are compatibility re-exports of
`components`, so existing v2 imports remain valid while new code can use the
feature-oriented `iced_shadcn_v2::components::*` paths.

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

## Example

```rust,no_run
use iced_shadcn_v2::{Button, ButtonVariant, Theme};

#[derive(Debug, Clone)]
enum Message {
    Save,
}

fn view(theme: &Theme) -> iced::Element<'_, Message> {
    Button::text("Save", theme)
        .variant(ButtonVariant::Default)
        .on_press(Message::Save)
        .into()
}
```
