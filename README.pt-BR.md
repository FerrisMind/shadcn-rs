# shadcn-rs workspace

## Visão geral
Workspace Rust para crates de componentes de UI no estilo shadcn. Atualmente inclui o crate `egui-shadcn` com elementos básicos de formulário para egui.

## Instalação
```
cargo add egui-shadcn --path crates/egui-shadcn
```

## Componentes
- `button` — variantes `Primary`, `Secondary`, `Ghost`, `Outline`, `Destructive`, `Link`; tamanhos `Sm|Md|Lg|IconSm|Icon|IconLg`; suporta `enabled`.
- `text_input` — cor personalizada do placeholder, `is_invalid`, `enabled`, anel de 3px e cores de seleção.
- `select` — placeholder, lista de opções, `enabled`, `is_invalid` (via `SelectProps`), seta no texto.
- `checkbox` — controle de variante e tamanho.
- `toggle` — botão-toggle (default/outline), cores de destaque, tamanhos `Sm|Md|Lg`.
- `switch` — toggle de trilho com tamanhos de trilho/polegar alinhados ao shadcn (32×18.4).
- `textarea` — anel de foco, preenchimento `is_invalid`, contador opcional e `max_len`.

## Exemplos
- `cargo run --example button` — variantes `Primary|Secondary|Ghost|Outline|Destructive|Link` e todos os tamanhos de ícone.
- `cargo run --example text_input` — tamanhos `Sm|Md|Lg`, estados `invalid` e `disabled`.
- `cargo run --example select` — API legada (`SelectPropsSimple`), listas agrupadas, `invalid`, `disabled`, `SelectStyle` customizado, tamanho `Sm`.
- `cargo run --example checkbox` — todas as variantes e tamanhos, incluindo `disabled`.
- `cargo run --example toggle` — variantes `Default|Outline`, tamanhos de ícone, `disabled`.
- `cargo run --example switch` — variantes de cor, tamanhos `Sm|Md|Lg`, `disabled`.
- `cargo run --example textarea` — contador e limite, `invalid`, `disabled`.
- `cargo run --example basic` — demonstração combinada de todos os componentes.

Tela combinada (`basic.rs`):
```rust
let theme = Theme::default();
let mut value = String::new();
let mut selected = None;
egui::CentralPanel::default().show(&ctx, |ui| {
    button(ui, &theme, "Primary", ControlVariant::Primary, ControlSize::Md, true);
    text_input(ui, &theme, &mut value, "Digite o texto", ControlSize::Md, false, true);
    let mut switch_on = false;
    let mut toggle_on = false;
    toggle(ui, &theme, &mut toggle_on, "Toggle", ToggleVariant::Outline, ControlSize::Md, true);
    toggle(
        ui,
        &theme,
        &mut switch_on,
        "Switch",
        ControlVariant::Primary,
        ControlSize::Sm,
        true,
    );
    select(
        ui,
        &theme,
        SelectProps {
            id_source: "demo",
            selected: &mut selected,
            options: &["One".to_string()],
            placeholder: "Escolha",
            size: ControlSize::Sm,
            enabled: true,
            is_invalid: false,
        },
    );
});
```

## Testes
`cargo test`

## Migração
- `select` agora aceita parâmetros via `SelectProps`; atualize chamadas para `select(ui, &theme, SelectProps { ... })`.
- `textarea` usa `TextareaProps`; passe o placeholder como `WidgetText` (por exemplo, `"text".into()`).
- `SelectProps` inclui `is_invalid`; defina como `false` para o comportamento anterior.

## Mapeamento para shadcn
- Variantes e tamanhos correspondem aos equivalentes do shadcn/ui.
- As cores vêm de `Theme` e `ColorPalette`.
- Os estados visuais refletem hover/active/disabled/focus.

