use iced::widget::{column, container, row, text};
use iced::{Alignment, Background, Element, Length};

use iced_shadcn::{
    ButtonProps, ButtonSize, ButtonVariant, CardProps, CardSize, InputProps, InputSize,
    InputVariant, TabsListProps, TabsListVariant, TabsRootProps, Theme, button, card, input, label,
    tabs_content, tabs_contents, tabs_list, tabs_root, tabs_trigger,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

struct Example {
    theme: Theme,
    active: String,
    name: String,
    username: String,
    current_password: String,
    new_password: String,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            theme: Theme::dark(),
            active: String::new(),
            name: "Pedro Duarte".to_string(),
            username: "@peduarte".to_string(),
            current_password: String::new(),
            new_password: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TabChanged(String),
    NameChanged(String),
    UsernameChanged(String),
    CurrentPasswordChanged(String),
    NewPasswordChanged(String),
    SaveAccount,
    SavePassword,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::TabChanged(id) => self.active = id,
            Message::NameChanged(value) => self.name = value,
            Message::UsernameChanged(value) => self.username = value,
            Message::CurrentPasswordChanged(value) => self.current_password = value,
            Message::NewPasswordChanged(value) => self.new_password = value,
            Message::SaveAccount | Message::SavePassword => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let active = if self.active.is_empty() {
            "account"
        } else {
            &self.active
        };

        let list = tabs_list(
            vec![
                tabs_trigger("account", "Account"),
                tabs_trigger("password", "Password"),
            ],
            active,
            Some(Message::TabChanged),
            TabsRootProps::new(),
            TabsListProps::new().variant(TabsListVariant::Pill),
            theme,
        );

        let field_width = Length::Fixed(320.0);

        let account_card = card(
            column![
                column![
                    text("Account").size(16),
                    text("Make changes to your account here. Click save when you're done.")
                        .size(13)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(theme.palette.muted_foreground),
                        }),
                ]
                .spacing(6),
                column![
                    column![
                        label("Name", theme),
                        input(
                            &self.name,
                            "Name",
                            Some(Message::NameChanged),
                            InputProps::new()
                                .size(InputSize::Size2)
                                .variant(InputVariant::Surface),
                            theme,
                        )
                        .width(field_width),
                    ]
                    .spacing(12),
                    column![
                        label("Username", theme),
                        input(
                            &self.username,
                            "Username",
                            Some(Message::UsernameChanged),
                            InputProps::new()
                                .size(InputSize::Size2)
                                .variant(InputVariant::Surface),
                            theme,
                        )
                        .width(field_width),
                    ]
                    .spacing(12),
                ]
                .spacing(24),
                row![button(
                    "Save changes",
                    Some(Message::SaveAccount),
                    ButtonProps::new()
                        .variant(ButtonVariant::Solid)
                        .size(ButtonSize::Size2),
                    theme,
                ),]
                .align_y(Alignment::Center),
            ]
            .spacing(24),
            CardProps::new().size(CardSize::Size4),
            theme,
        );

        let password_card = card(
            column![
                column![
                    text("Password").size(16),
                    text("Change your password here. After saving, you'll be logged out.")
                        .size(13)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(theme.palette.muted_foreground),
                        }),
                ]
                .spacing(6),
                column![
                    column![
                        label("Current password", theme),
                        input(
                            &self.current_password,
                            "Current password",
                            Some(Message::CurrentPasswordChanged),
                            InputProps::new()
                                .size(InputSize::Size2)
                                .variant(InputVariant::Surface),
                            theme,
                        )
                        .width(field_width),
                    ]
                    .spacing(12),
                    column![
                        label("New password", theme),
                        input(
                            &self.new_password,
                            "New password",
                            Some(Message::NewPasswordChanged),
                            InputProps::new()
                                .size(InputSize::Size2)
                                .variant(InputVariant::Surface),
                            theme,
                        )
                        .width(field_width),
                    ]
                    .spacing(12),
                ]
                .spacing(24),
                row![button(
                    "Save password",
                    Some(Message::SavePassword),
                    ButtonProps::new()
                        .variant(ButtonVariant::Solid)
                        .size(ButtonSize::Size2),
                    theme,
                ),]
                .align_y(Alignment::Center),
            ]
            .spacing(24),
            CardProps::new().size(CardSize::Size4),
            theme,
        );

        let content = tabs_contents(
            vec![
                tabs_content("account", account_card),
                tabs_content("password", password_card),
            ],
            active,
        );

        let tabs = tabs_root(list, content);

        preview(theme, container(tabs).width(Length::Fixed(384.0)))
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into()
    }
}

fn preview<'a, Message: 'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.background;
    container(content)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
}
