# iced-shadcn

<p align="center">
  <img src="https://raw.githubusercontent.com/FerrisMind/shadcn-rs/master/crates/iced-shadcn/assets/icons/shadcn-iced/icon.svg" width="200" alt="shadcn-iced logo" />
</p>

<p align="center">
  <a href="https://crates.io/crates/iced-shadcn"><img alt="iced-shadcn version" src="https://img.shields.io/crates/v/iced-shadcn?label=iced-shadcn"></a>
</p>

> [!WARNING]
> ВНИМАНИЕ: API `iced-shadcn` сейчас нестабилен и может меняться от версии к версии, включая breaking changes.
> Фиксируйте точные версии зависимостей и проверяйте release notes перед обновлением.

## Обзор

`iced-shadcn` задуман как набор компонентов для [iced](https://github.com/iced-rs/iced) в стиле [shadcn/ui](https://ui.shadcn.com).  
Цель — общий визуальный язык и единые токены темы, согласованные с остальной экосистемой `shadcn-rs`.

## Статус

Этот крейт находится **в активной разработке**. Публичное API, модель темизации и набор компонентов пока не стабильны и могут меняться.

**Coming soon**:

- Каталог компонентов (по возможности с паритетом к `egui-shadcn`)
- Гайд по теме и токенам
- Примеры использования и лучшие практики

## Демо

Desktop-приложения на этом **v1** props-first крейте. Они пока **находятся в разработке**. Подробнее: [issue #5](https://github.com/FerrisMind/shadcn-rs/issues/5).

### Nova Code

Минималистичный редактор в духе VS Code на iced.

https://github.com/user-attachments/assets/04ddafcb-adf1-42fa-bb0e-97676792973b

### Zver

Минималистичный desktop-браузер на системных web-движках через wry.

https://github.com/user-attachments/assets/0afa7180-efd5-496e-8f28-5a371fe2a12d

### NeuroLang

Локальный desktop-переводчик (сейчас текст; другие форматы в планах).

https://github.com/user-attachments/assets/e4908f23-5f14-486b-8200-9164f4136322

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


