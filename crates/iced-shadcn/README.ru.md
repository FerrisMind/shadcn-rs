# iced-shadcn

<p align="center">
  <img src="https://raw.githubusercontent.com/FerrisMind/shadcn-rs/master/crates/iced-shadcn/assets/icons/shadcn-iced/icon.svg" width="200" alt="shadcn-iced logo" />
</p>

## Обзор

`iced-shadcn` задуман как набор компонентов для [iced](https://github.com/iced-rs/iced) в стиле [shadcn/ui](https://ui.shadcn.com).  
Цель — общий визуальный язык и единые токены темы, согласованные с остальной экосистемой `shadcn-rs`.

## Статус

Этот крейт находится **в активной разработке**. Публичное API, модель темизации и набор компонентов пока не стабильны и могут меняться.

**Coming soon**:

- Каталог компонентов (по возможности с паритетом к `egui-shadcn`)
- Гайд по теме и токенам
- Примеры использования и лучшие практики

## Empty

Композиционный пример для `Empty` в той же структуре, что и в `shadcn-svelte`:

```rust
use lucide_icons::Icon;
use iced_shadcn::{
    EmptyContentProps, EmptyHeaderProps, EmptyMediaProps, EmptyMediaVariant, EmptyRootProps,
    EmptyTitleProps, Theme, button, empty_content, empty_description, empty_header, empty_media,
    empty_root, empty_title,
};

fn view<'a, Message: Clone + 'a>(theme: &'a Theme) -> iced::Element<'a, Message> {
    empty_root(
        iced::widget::column![
            empty_header(
                vec![
                    empty_media(
                        iced::widget::text(char::from(Icon::Folder).to_string()),
                        EmptyMediaProps::new().variant(EmptyMediaVariant::Icon),
                        theme,
                    ),
                    empty_title("Нет данных", EmptyTitleProps::new(), theme),
                    empty_description("Пока здесь ничего нет", Default::default(), theme),
                ],
                EmptyHeaderProps::new(),
            ),
            empty_content(
                vec![button("Добавить", None::<Message>, Default::default(), theme).into()],
                EmptyContentProps::new(),
            ),
        ]
        .spacing(24),
        EmptyRootProps::new(),
        theme,
    )
}
```

Пример:
- `crates/iced-shadcn/examples/empty`

## Лицензия

MIT

---

**Inspired by** [shadcn/ui](https://ui.shadcn.com) · **Icons by** [Lucide](https://lucide.dev)


