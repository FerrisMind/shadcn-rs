## Tabs example

Демо повторяет shadcn/ui Tabs: два таба Account/Password, контент в Card с формой.

Запуск:
```bash
cargo run --example tabs
```

Что показано:
- Tabs в варианте `Soft` (список в muted-фоне, активный таб на bg).
- Header/description карточки + поля `Name`, `Username`, `Current password`, `New password`.
- Инпуты используют `InputType::Password` для скрытия текста на нужных полях.
- Состояние всех полей и активного таба хранится в `String` и сохраняется при переключениях.
