# egui-shadcn

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="../egui-shadcn/assets/icons/shadcn-egui/icon-white.svg" />
    <source media="(prefers-color-scheme: light)" srcset="../egui-shadcn/assets/icons/shadcn-egui/icon-black.svg" />
    <img alt="shadcn-egui logo" src="../egui-shadcn/assets/icons/shadcn-egui/icon-black.svg" width="180" />
  </picture>
</p>

## Visão geral
`egui-shadcn` é um conjunto de componentes de formulário para egui inspirados no shadcn/ui. Replica variantes e tamanhos do shadcn e expõe tokens de tema para visuais consistentes.

## Início rápido
```rust
use egui_shadcn::{button, ControlSize, ControlVariant, Theme};

fn ui_example(ui: &mut egui::Ui, theme: &Theme) {
    button(ui, theme, "Primary", ControlVariant::Primary, ControlSize::Md, true);
}
```

## Componentes
- Todos os componentes suportam variantes, tamanhos e personalização de tema.
- Checkbox: API Radix Themes (`size 1..=3`, variantes `surface|classic|soft`, `color`, `high_contrast`).
- Dialog: API Radix Themes Content (`size 1..=4`, alinhamento `start|center`, largura/min/max/altura, `as_child`).
- Label: API Radix Label (`as_child`, `html_for`) + variantes/descrição/required.
- Popover: API Radix Popover (estado Root, posicionamento Popper `side/align/offsets/collision`, Portal `container`, `force_mount`, callbacks de DismissableLayer).
- Tooltip: API Radix Tooltip (atrasos do Provider, estado Root, posicionamento/colisão do Content, container do Portal, callbacks de fechamento).
- Select: API Radix Select (estado Root, props de formulário, posicionamento `position/side/align/offsets/collision`, callbacks de fechamento, `text_value` por item para typeahead).
- Radio Group: API Radix Radio Group (`as_child`, controlado/não controlado, `orientation`, `dir`, `loop_focus`, flags por item) + variantes em cartão/grade.
- Tabs: API Radix Tabs Root/List/Trigger/Content (`as_child`, controlado/não controlado, `orientation`, `dir`, `activation_mode`, list `loop`, content `force_mount`) + extensões egui (variants, wrap/justify, scrollable, full_width, accent/high_contrast, compact/animate).
- Switch: API Radix Switch Root/Thumb (`as_child`, controlado/não controlado, `name/value`, `required`) + extensões egui (size/style/high_contrast/animate/accent/custom radius/thumb color).
- Scroll Area: API Radix Scroll Area (`type` padrão `hover`, `scroll_hide_delay` `600ms`, `as_child`, `dir`, `force_mount` por eixo) + extensões egui (size/radius/accent/high_contrast/colors_override/max_size/bar_visibility).
## Tema
Os estados visuais vêm de `Theme::control` e `Theme::input`, baseados em `ColorPalette`.

## Exemplos
- `cargo run --example button` — todas as variantes e tamanhos.
- `cargo run --example text_input` — tamanhos `Sm|Md|Lg`, estados inválido/desabilitado.
- `cargo run --example select` — grupos de opções, `SelectProps`, inválido/desabilitado, `SelectStyle` personalizado.
- `cargo run --example checkbox` — todas as variantes/tamanhos, desabilitado.
- `cargo run --example toggle` — default/outline, tamanhos de ícone, desabilitado.
- `cargo run --example switch` — variantes de cor, tamanhos `Sm|Md|Lg`, desabilitado.
- `cargo run --example textarea` — contador/limite, inválido, desabilitado, todos os tamanhos.
- `cargo run --example basic` — todos os componentes em uma única tela.

## Testes
`cargo test`

## Migração
- `select` agora usa `SelectProps`: `select(ui, &theme, SelectProps { ... })`.
- `textarea` usa `TextareaProps`; passe o placeholder como `WidgetText` (`"text".into()`).
- `SelectProps` inclui `is_invalid`; defina como `false` para o comportamento anterior.

