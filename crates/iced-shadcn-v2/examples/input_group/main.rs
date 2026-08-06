//! Interactive playground for `iced-shadcn-v2::InputGroup`.
//!
//! The examples cover inline and block addons, text and action slots, the
//! controlled `Input`, the multi-line editor, invalid/disabled propagation,
//! and every shadcn-svelte style pack.
//!
//! Run with:
//! `cargo run -p iced-shadcn-v2 --example input_group`

use iced::widget::{column, container, row, scrollable, text, text_editor};
use iced::{Alignment, Background, Element, Length, Task};

use iced_shadcn_v2::{
    Button, ButtonVariant, FontId, Input, InputGroup, InputGroupAddon, InputGroupAddonAlign,
    InputGroupButton, InputGroupButtonSize, InputGroupProps, InputGroupText, InputGroupTextarea,
    InputGroupTextareaProps, StyleId, Theme, fonts, iced_font, input_group_textarea_apply_action,
};

pub fn main() -> iced::Result {
    let mut app = iced::application(Example::default, Example::update, Example::view)
        .title(Example::title)
        .default_font(iced_font(FontId::Geist));

    for face in fonts::ALL_FACES {
        app = app.font(*face);
    }

    app.run()
}

struct Example {
    theme: Theme,
    query: String,
    url: String,
    editor: text_editor::Content,
    last_action: &'static str,
    invalid: bool,
    disabled: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Query(String),
    Url(String),
    Editor(text_editor::Action),
    Action(&'static str),
    Style(StyleId),
    ToggleInvalid,
    ToggleDisabled,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::light(),
            query: String::new(),
            url: "https://shadcn-svelte.com".to_owned(),
            editor: text_editor::Content::with_text(""),
            last_action: "—",
            invalid: false,
            disabled: false,
        }
    }
}

impl Example {
    fn title(&self) -> String {
        "iced-shadcn-v2 InputGroup".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Query(value) => self.query = value,
            Message::Url(value) => self.url = value,
            Message::Editor(action) => {
                let props = InputGroupTextareaProps::new().max_len(120);
                input_group_textarea_apply_action(&mut self.editor, action, props);
            }
            Message::Action(action) => self.last_action = action,
            Message::Style(style) => self.theme = self.theme.clone().with_style(style),
            Message::ToggleInvalid => self.invalid = !self.invalid,
            Message::ToggleDisabled => self.disabled = !self.disabled,
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let palette = theme.palette;

        let theme_controls = row(StyleId::ALL
            .into_iter()
            .map(|style| {
                Button::text(style.as_str(), theme)
                    .variant(if style == theme.style_id() {
                        ButtonVariant::Default
                    } else {
                        ButtonVariant::Outline
                    })
                    .on_press(Message::Style(style))
                    .into()
            })
            .collect::<Vec<Element<'_, Message>>>())
        .spacing(6)
        .wrap();

        let state_controls = row![
            Button::text(
                if self.invalid {
                    "Clear invalid"
                } else {
                    "Mark invalid"
                },
                theme
            )
            .variant(ButtonVariant::Outline)
            .on_press(Message::ToggleInvalid),
            Button::text(if self.disabled { "Enable" } else { "Disable" }, theme)
                .variant(ButtonVariant::Outline)
                .on_press(Message::ToggleDisabled),
        ]
        .spacing(8);

        let search = InputGroup::new(theme)
            .push_input(
                Input::new(theme)
                    .value(self.query.as_str())
                    .placeholder("Search...")
                    .id("input-group-search")
                    .disabled(self.disabled)
                    .invalid(self.invalid)
                    .on_input(Message::Query),
            )
            .push_addon(InputGroupAddon::text("⌕", theme).align(InputGroupAddonAlign::InlineStart))
            .push_addon(
                InputGroupAddon::empty(theme)
                    .align(InputGroupAddonAlign::InlineEnd)
                    .push(InputGroupText::text("⌘K", theme))
                    .push(
                        InputGroupButton::icon(text("↗"), theme)
                            .size(InputGroupButtonSize::IconXs)
                            .on_press(Message::Action("Open search")),
                    ),
            );

        let url_group = InputGroup::new(theme)
            .push_input(
                Input::new(theme)
                    .value(self.url.as_str())
                    .placeholder("example.com")
                    .id("input-group-url")
                    .disabled(self.disabled)
                    .on_input(Message::Url),
            )
            .push_addon(InputGroupAddon::text("https://", theme))
            .push_addon(
                InputGroupAddon::empty(theme)
                    .align(InputGroupAddonAlign::InlineEnd)
                    .push(InputGroupButton::text("Copy", theme).on_press(Message::Action("Copy"))),
            );

        let textarea = InputGroup::with_props(
            theme,
            InputGroupProps::new()
                .invalid(self.invalid)
                .disabled(self.disabled),
        )
        .push(
            InputGroupTextarea::new(&self.editor, theme)
                .placeholder("Write a message...")
                .id("input-group-editor")
                .rows(3)
                .max_rows(5)
                .disabled(self.disabled)
                .on_action(Message::Editor),
        )
        .push_addon(
            InputGroupAddon::empty(theme)
                .align(InputGroupAddonAlign::BlockEnd)
                .push(InputGroupText::text(
                    format!("{} / 120 characters", self.editor.text().chars().count()),
                    theme,
                ))
                .push(InputGroupButton::text("Send", theme).on_press(Message::Action("Send"))),
        );

        let invalid = InputGroup::new(theme)
            .push_input(Input::new(theme).value("Invalid value").invalid(true))
            .push_addon(InputGroupAddon::text("!", theme));

        let disabled = InputGroup::new(theme)
            .push_input(Input::new(theme).value("Disabled value").disabled(true))
            .push_addon(InputGroupAddon::text("Locked", theme));

        let content = column![
            text("Input Group")
                .size(32)
                .font(iced_font(theme.font_pack().heading))
                .color(palette.foreground),
            text("Composable controls and addons, with the same four alignment slots as shadcn-svelte.")
                .size(14)
                .font(iced_font(theme.font_pack().sans))
                .color(palette.muted_foreground),
            section("Style pack", theme_controls, theme),
            section("State", state_controls, theme),
            section("Inline addons", search, theme),
            section("Prefix and action", url_group, theme),
            section("Textarea with block-end actions", textarea, theme),
            section("Invalid", invalid, theme),
            section("Disabled", disabled, theme),
            text(format!("Last action: {}", self.last_action))
                .size(12)
                .font(iced_font(theme.font_pack().mono))
                .color(palette.muted_foreground),
        ]
        .spacing(18)
        .max_width(900)
        .padding(8);

        container(
            scrollable(
                container(content)
                    .width(Length::Fill)
                    .center_x(Length::Fill)
                    .padding(24),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(palette.background)),
            text_color: Some(palette.foreground),
            ..container::Style::default()
        })
        .into()
    }
}

fn section<'a>(
    label: &'static str,
    content: impl Into<Element<'a, Message>>,
    theme: &'a Theme,
) -> Element<'a, Message> {
    column![
        text(label)
            .size(17)
            .font(iced_font(theme.font_pack().heading))
            .color(theme.palette.muted_foreground),
        content.into(),
    ]
    .spacing(8)
    .align_x(Alignment::Start)
    .into()
}
