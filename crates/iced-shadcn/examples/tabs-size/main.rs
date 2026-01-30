use iced::border::Border;
use iced::widget::{column, container, text};
use iced::{Background, Element, Length};

use iced_shadcn::{
    TabsListProps, TabsListVariant, TabsRootProps, TabsSize, Theme, tabs_content, tabs_contents,
    tabs_list, tabs_root, tabs_trigger,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    small_active: String,
    large_active: String,
}

#[derive(Debug, Clone)]
enum Message {
    SmallChanged(String),
    LargeChanged(String),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::SmallChanged(value) => self.small_active = value,
            Message::LargeChanged(value) => self.large_active = value,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let small_active = if self.small_active.is_empty() {
            "account"
        } else {
            &self.small_active
        };
        let large_active = if self.large_active.is_empty() {
            "account"
        } else {
            &self.large_active
        };

        let small_list = tabs_list(
            vec![
                tabs_trigger("account", "Account"),
                tabs_trigger("documents", "Documents"),
                tabs_trigger("settings", "Settings"),
            ],
            small_active,
            Some(Message::SmallChanged),
            TabsRootProps::new(),
            TabsListProps::new()
                .variant(TabsListVariant::Pill)
                .size(TabsSize::One),
            theme,
        );

        let small_content = tabs_contents(
            vec![
                tabs_content("account", text("Small: Account").size(13)),
                tabs_content("documents", text("Small: Documents").size(13)),
                tabs_content("settings", text("Small: Settings").size(13)),
            ],
            small_active,
        );

        let large_list = tabs_list(
            vec![
                tabs_trigger("account", "Account"),
                tabs_trigger("documents", "Documents"),
                tabs_trigger("settings", "Settings"),
            ],
            large_active,
            Some(Message::LargeChanged),
            TabsRootProps::new(),
            TabsListProps::new()
                .variant(TabsListVariant::Pill)
                .size(TabsSize::Two),
            theme,
        );

        let large_content = tabs_contents(
            vec![
                tabs_content("account", text("Default: Account").size(14)),
                tabs_content("documents", text("Default: Documents").size(14)),
                tabs_content("settings", text("Default: Settings").size(14)),
            ],
            large_active,
        );

        let content = column![
            tabs_root(small_list, small_content),
            tabs_root(large_list, large_content),
        ]
        .spacing(24);

        app(theme, preview(theme, content).into())
    }
}

fn app<'a, Message: 'a>(theme: &Theme, content: Element<'a, Message>) -> Element<'a, Message> {
    let background = theme.palette.background;
    container(content)
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            ..iced::widget::container::Style::default()
        })
        .into()
}

fn preview<'a, Message: 'a>(
    theme: &Theme,
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Container<'a, Message> {
    let background = theme.palette.card;
    let border = theme.palette.border;
    let radius = theme.radius.md;

    container(content)
        .padding(20)
        .width(Length::Fixed(420.0))
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                radius: radius.into(),
                width: 1.0,
                color: border,
            },
            ..iced::widget::container::Style::default()
        })
}
