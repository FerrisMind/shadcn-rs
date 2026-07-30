# shadcn-common

Shared shadcn design tokens for `iced-shadcn` and `egui-shadcn`.

Built on [`twill-core`](https://github.com/FerrisMind/twill): styles, base/accent colors,
theme mode, typography, radius, icon catalog, and **component recipes**
(`recipes::{label,button,badge,kbd,skeleton,…}`) — without GUI backend deps.

Also includes backend-agnostic behaviour helpers ported from Zag utilities:
value/step mapping, selection sets, pagination ranges, presence lifecycle,
calendar navigation, and RGB/HSB/HSL color-space math.

```rust
use shadcn_common::{
    AccentColor, BaseColor, ControlSize, LabelContext, ResolvedTheme, StyleId, ThemeMode,
};

let theme = ResolvedTheme::new(
    StyleId::Vega,
    BaseColor::Neutral,
    Some(AccentColor::Amber),
    ThemeMode::Light,
);
let primary = theme.color_value(shadcn_common::SemanticColor::Primary);
let label = theme.style_pack().label(LabelContext::Field);
let button = theme.style_pack().button_size(ControlSize::Md);
```
