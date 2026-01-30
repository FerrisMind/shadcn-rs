use iced::border::Border;
use iced::widget::{container, text};
use iced::{Background, Element, Length};

use iced_shadcn::{
    TabsListProps, TabsListVariant, TabsRootProps, Theme, tabs_content, tabs_contents, tabs_list,
    tabs_root, tabs_trigger,
};

pub fn main() -> iced::Result {
    iced::application(Example::default, Example::update, Example::view).run()
}

#[derive(Default)]
struct Example {
    theme: Theme,
    active: String,
}

#[derive(Debug, Clone)]
enum Message {
    TabChanged(String),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::TabChanged(value) => self.active = value,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let theme = &self.theme;
        let active = if self.active.is_empty() {
            "home"
        } else {
            &self.active
        };

        let list = tabs_list(
            vec![
                tabs_trigger("home", "Home"),
                tabs_trigger("settings", "Settings").disabled(true),
            ],
            active,
            Some(Message::TabChanged),
            TabsRootProps::new(),
            TabsListProps::new().variant(TabsListVariant::Pill),
            theme,
        );

        let content = tabs_contents(
            vec![
                tabs_content("home", text("Home content").size(14)),
                tabs_content("settings", text("Settings content").size(14)),
            ],
            active,
        );

        let tabs = tabs_root(list, content);
        app(theme, preview(theme, tabs).into())
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
        .padding(16)
        .width(Length::Fixed(360.0))
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
