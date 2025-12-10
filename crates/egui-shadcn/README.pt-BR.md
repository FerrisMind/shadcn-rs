# egui-shadcn

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
- `button` — variantes `Primary|Secondary|Ghost|Outline|Destructive|Link`; tamanhos `Sm|Md|Lg|IconSm|Icon|IconLg`.
- `text_input` — cor do placeholder, `is_invalid`, `enabled`, anel de 3px e cores de seleção.
- `select` — via `SelectProps`, placeholder, `is_invalid`, seta no texto e estado desabilitado.
- `checkbox` — tamanhos e variantes.
- `toggle` — variantes default/outline com cores de destaque.
- `switch` — tamanhos de trilho/polegar alinhados ao shadcn.
- `textarea` — anel de foco, preenchimento para erro e contador opcional.

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

