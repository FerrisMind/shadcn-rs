# shadcn-common

Shared shadcn design tokens for `iced-shadcn` and `egui-shadcn`.

Built on [`twill-core`](https://github.com/FerrisMind/twill): styles, base/accent colors,
theme mode, typography, radius, and icon catalog — without GUI backend deps.

```rust
use shadcn_common::{AccentColor, BaseColor, ResolvedTheme, StyleId, ThemeMode};

let theme = ResolvedTheme::new(
    StyleId::Vega,
    BaseColor::Neutral,
    Some(AccentColor::Amber),
    ThemeMode::Light,
);
let primary = theme.color_value(shadcn_common::SemanticColor::Primary);
```
