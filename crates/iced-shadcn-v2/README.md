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
- `theme` — `shadcn-common` theme adapter for iced.
- `fonts` — font-face exports and the iced font adapter.

The root `button` and `spinner` modules are compatibility re-exports of
`components`, so existing v2 imports remain valid while new code can use the
feature-oriented `iced_shadcn_v2::components::*` paths.

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
